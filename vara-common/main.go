package main

import (
	"bytes"
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/parser"
	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/streamingfast/substreams-foundational-modules/vara-common/metadata"
	"github.com/streamingfast/substreams-foundational-modules/vara-common/metadata/spec"
	pbgear "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/gear/type/v1"
	pbvara "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/substreams/gear/type/v1"
)

var specVersions = map[uint32]string{
	100:  spec.V100,
	120:  spec.V120,
	130:  spec.V130,
	140:  spec.V140,
	210:  spec.V210,
	310:  spec.V310,
	320:  spec.V320,
	330:  spec.V330,
	340:  spec.V340,
	350:  spec.V350,
	1000: spec.V1000,
	1010: spec.V1010,
	1020: spec.V1020,
	1030: spec.V1030,
	1040: spec.V1040,
	1050: spec.V1050,
	1110: spec.V1110,
	1200: spec.V1200,
	1210: spec.V1210,
	1300: spec.V1300,
	1310: spec.V1310,
	1400: spec.V1400,
	1410: spec.V1410,
	1420: spec.V1420,
}
var metadataRegistry = map[uint32]*types.Metadata{}

func init() {
	for version, data := range specVersions {
		metadataRegistry[version] = metadata.Load(data)
	}
}

// this will actually return a decodedBlock containing all the decoded calls and events
func map_decoded_block(block *pbgear.Block) (*pbvara.Block, error) {
	factory := registry.NewFactory()
	meta := metadataRegistry[block.Header.SpecVersion]

	callRegistry, err := factory.CreateCallRegistry(meta)
	if err != nil {
		return nil, fmt.Errorf("creating call registry: %w", err)
	}

	extrinsics, err := ToExtrinsics(block.Extrinsics, callRegistry, meta)
	if err != nil {
		return nil, fmt.Errorf("converting extrinsics: %w", err)
	}

	eventRegistry, err := factory.CreateEventRegistry(meta)
	if err != nil {
		return nil, fmt.Errorf("creating event registry: %w", err)
	}

	events, err := toEvents(eventRegistry, block.RawEvents, meta)
	if err != nil {
		return nil, fmt.Errorf("converting events: %w", err)
	}

	return &pbvara.Block{
		Number:        block.Number,
		Hash:          block.Hash,
		Header:        block.Header,
		DigestItems:   block.DigestItems,
		Justification: block.Justification,
		Extrinsics:    extrinsics,
		Events:        events,
	}, nil
}

func ToExtrinsics(extrinsics []*pbgear.Extrinsic, callRegistry registry.CallRegistry, metadata *types.Metadata) ([]*pbvara.Extrinsic, error) {
	var out []*pbvara.Extrinsic

	for _, extrinsic := range extrinsics {
		c, err := toCall(extrinsic, callRegistry, metadata)
		if err != nil {
			return nil, fmt.Errorf("converting call: %w", err)
		}
		e := pbvara.Extrinsic{
			Version:   extrinsic.Version,
			Signature: extrinsic.Signature,
			Call:      c,
		}

		out = append(out, &e)
	}

	return out, nil
}

func toCall(extrinsic *pbgear.Extrinsic, callRegistry registry.CallRegistry, metadata *types.Metadata) (*pbvara.Call, error) {
	name, decodedCall, err := decodeExtrinsic(extrinsic, callRegistry)
	if err != nil {
		return nil, fmt.Errorf("decoding call: %w", err)
	}

	fields := toFields(decodedCall, metadata)

	return &pbvara.Call{
		Name:   name,
		Fields: fields,
	}, nil
}

func toEvents(eventRegistry registry.EventRegistry, storageEvents []byte, metadata *types.Metadata) ([]*pbvara.Event, error) {
	var out []*pbvara.Event

	evts, err := decodeEvents(eventRegistry, storageEvents)
	if err != nil {
		return nil, fmt.Errorf("decoding call: %w", err)
	}

	for _, event := range evts {
		out = append(out, &pbvara.Event{
			Name:   event.Name,
			Fields: toFields(event.Fields, metadata),
		})
	}

	return out, nil
}

func toFields(in any, metadata *types.Metadata) *pbvara.Fields {
	var fields registry.DecodedFields
	switch i := in.(type) {
	case *registry.VariantWTF:
		fields = i.Value.(registry.DecodedFields)
	case registry.DecodedFields:
		fields = i
	case []*registry.DecodedField:
		fields = i
	case []interface{}:
		if len(i) != 1 {
			panic("expected an array of 1 element")
		}
		fields = i[0].(registry.DecodedFields)
	case nil:
		return &pbvara.Fields{}
	default:
		panic(fmt.Sprintf("unknown type %T", in))
	}
	m := map[string]*pbvara.Value{}

	for _, field := range fields {
		v := toValue(field, metadata)
		if _, found := m[field.Name]; found {
			panic("duplicate field: " + field.Name)
		}

		m[field.Name] = v
	}

	return &pbvara.Fields{
		Map: m,
	}
}

func toValue(decodedField *registry.DecodedField, metadata *types.Metadata) *pbvara.Value {
	lookupType := metadata.AsMetadataV14.EfficientLookup[decodedField.LookupIndex]
	value := &pbvara.Value{}

	switch {
	case lookupType.Def.IsPrimitive:
		value = toPrimitiveValue(lookupType, decodedField.Value)

	case lookupType.Def.IsSequence, lookupType.Def.IsArray:
		value = toSequenceValue(decodedField, metadata, lookupType)

	case lookupType.Def.IsTuple:
		panic("not implemented")

	case lookupType.Def.IsVariant:
		value = toVariantValue(decodedField, metadata, lookupType)

	case lookupType.Def.IsCompact:
		value = toCompactValue(decodedField, metadata, lookupType)

	case lookupType.Def.IsComposite:
		value = toCompositeValue(decodedField, metadata, lookupType)
	default:
		panic("unknown type")
	}

	return value
}

func toCompositeValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) *pbvara.Value {
	compositeFields := decodedField.Value.(registry.DecodedFields)

	if len(compositeFields) == 1 {
		f := compositeFields[0]
		if f.Name == "[u8; 32]" {
			v := toValue(f, metadata)
			return &pbvara.Value{
				Type: &pbvara.Value_Bytes{
					Bytes: v.Type.(*pbvara.Value_Bytes).Bytes,
				},
			}
		}
	}

	values := map[string]*pbvara.Value{}
	for _, field := range compositeFields {
		if field.Name == "" {
			panic("composite field name not set")
		}

		switch v := field.Value.(type) {
		case []interface{}: //this is a Sequence
			values[field.Name] = toValue(field, metadata)
		case registry.DecodedFields:
			fs := toFields(v, metadata)
			values[field.Name] = &pbvara.Value{
				Type: &pbvara.Value_Fields{
					Fields: &pbvara.Fields{
						Map: fs.Map,
					},
				},
			}
		case uint8: //enum
			enumType := metadata.AsMetadataV14.EfficientLookup[field.LookupIndex]
			for _, e := range enumType.Def.Variant.Variants {
				if uint8(e.Index) == v {
					values[field.Name] = &pbvara.Value{
						Type: &pbvara.Value_String_{
							String_: string(e.Name),
						},
					}
					break
				}
			}
		}
	}
	return &pbvara.Value{
		Type: &pbvara.Value_Fields{
			Fields: &pbvara.Fields{
				Map: values,
			},
		},
	}
}

func toCompactValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) *pbvara.Value {
	childType := metadata.AsMetadataV14.EfficientLookup[lookupType.Def.Compact.Type.Int64()]
	if childType.Def.IsPrimitive {
		return toPrimitiveValue(childType, decodedField.Value)
	} else {
		fields := toFields(decodedField.Value, metadata)
		if len(fields.Map) == 1 {
			for _, field := range fields.Map {
				return &pbvara.Value{
					Type: field.Type,
				}
			}
		}

		return &pbvara.Value{
			Type: &pbvara.Value_Fields{
				Fields: fields,
			},
		}
	}
}

func toVariantValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) *pbvara.Value {
	value := &pbvara.Value{}

	switch v := decodedField.Value.(type) {
	case *registry.VariantWTF:
		wtf := decodedField.Value.(*registry.VariantWTF)
		matched := false
		for _, variant := range lookupType.Def.Variant.Variants {
			if byte(variant.Index) == wtf.VariantByte {
				matched = true
				if len(variant.Fields) == 1 {
					childType := metadata.AsMetadataV14.EfficientLookup[variant.Fields[0].Type.Int64()]
					if childType.Def.IsPrimitive {
						value = toPrimitiveValue(childType, v.ValueAt(0))
						break
					}
				}

				if _, ok := v.ValueAt(0).(uint8); ok {
					value.Type = &pbvara.Value_String_{
						String_: string(variant.Name),
					}

					break
				}
				value.Type = &pbvara.Value_Fields{

					Fields: toFields(v.ValueAt(0), metadata),
				}
				break
			}
		}
		if !matched {
			panic(fmt.Sprintf("variant not found for: %d", wtf.VariantByte))
		}
	case uint8: //this is an enum
		//todo: we should add a enum type with both name and index
		for _, variant := range lookupType.Def.Variant.Variants {
			if byte(variant.Index) == decodedField.Value {
				value.Type = &pbvara.Value_String_{
					String_: string(variant.Name),
				}
			}
		}
	default:
		panic(fmt.Sprintf("unknown type %T", decodedField.Value))
	}

	return value
}

func toSequenceValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) *pbvara.Value {
	var arrayOfTypeID int64
	switch {
	case lookupType.Def.IsSequence:
		arrayOfTypeID = lookupType.Def.Sequence.Type.Int64()
	case lookupType.Def.IsArray:
		arrayOfTypeID = lookupType.Def.Array.Type.Int64()
	}
	arrayOfType := metadata.AsMetadataV14.EfficientLookup[arrayOfTypeID]
	array := &pbvara.Array{}
	if arrayOfType.Def.IsPrimitive && (arrayOfType.Def.Primitive.Si0TypeDefPrimitive == types.IsU8 || arrayOfType.Def.Primitive.Si0TypeDefPrimitive == types.IsI8) {
		data := To_bytes(decodedField.Value)
		return &pbvara.Value{
			Type: &pbvara.Value_Bytes{
				Bytes: data,
			},
		}

	} else {
		for _, item := range decodedField.Value.([]interface{}) {
			if arrayOfType.Def.IsPrimitive {
				val := toPrimitiveValue(arrayOfType, item.(*registry.DecodedField).Value)
				array.Items = append(array.Items, val)
			} else if arrayOfType.Def.IsComposite {
				fields := toFields(item, metadata)
				if len(fields.Map) == 1 {
					for _, field := range fields.Map { //composite was just a wrapper
						val := &pbvara.Value{
							Type: field.Type,
						}
						array.Items = append(array.Items, val)
						continue
					}
				}

				val := &pbvara.Value{
					Type: &pbvara.Value_Fields{
						Fields: toFields(item, metadata),
					},
				}
				array.Items = append(array.Items, val)
				continue
			}
			v := item.(registry.Valuable).ValueAt(0)
			val := &pbvara.Value{
				Type: &pbvara.Value_Fields{
					Fields: toFields(v, metadata),
				},
			}
			array.Items = append(array.Items, val)
		}

		return &pbvara.Value{
			Type: &pbvara.Value_Array{
				Array: array,
			},
		}
	}
}

func decodeExtrinsic(extrinsic *pbgear.Extrinsic, callRegistry registry.CallRegistry) (string, registry.DecodedFields, error) {
	callIndex := extrinsic.Method.CallIndex
	args := extrinsic.Method.Args

	callDecoder, ok := callRegistry[ToCallIndex(callIndex)]
	if !ok {
		return "", nil, fmt.Errorf("failed to get call decoder")
	}

	decoder := scale.NewDecoder(bytes.NewReader(args))

	callFields, err := callDecoder.Decode(decoder)
	if err != nil {
		return "", nil, fmt.Errorf("failed to decode call: %w", err)
	}

	return callDecoder.Name, callFields, nil
}

func decodeEvents(eventRegistry registry.EventRegistry, storageEvents []byte) ([]*parser.Event, error) {
	decoder := scale.NewDecoder(bytes.NewReader(storageEvents))

	eventsCount, err := decoder.DecodeUintCompact()
	if err != nil {
		return nil, fmt.Errorf("failed to decode events count: %w", err)
	}

	var events []*parser.Event

	for i := uint64(0); i < eventsCount.Uint64(); i++ {
		var phase types.Phase

		err := decoder.Decode(&phase)
		if err != nil {
			return nil, fmt.Errorf("failed to decode phase: %w", err)
		}

		var eventID types.EventID

		err = decoder.Decode(&eventID)
		if err != nil {
			return nil, fmt.Errorf("failed to decode event ID: %w", err)
		}

		eventDecoder, ok := eventRegistry[eventID]
		if !ok {
			return nil, fmt.Errorf("failed to get event decoder")
		}

		eventFields, err := eventDecoder.Decode(decoder)
		if err != nil {
			return nil, fmt.Errorf("failed to decode event fields: %w", err)
		}

		var topics []types.Hash

		err = decoder.Decode(&topics)
		if err != nil {
			return nil, fmt.Errorf("failed to decode topics: %w", err)
		}

		event := &parser.Event{
			Name:    eventDecoder.Name,
			Fields:  eventFields,
			EventID: eventID,
			Phase:   &phase,
			Topics:  topics,
		}

		events = append(events, event)
	}

	return events, nil
}

func toPrimitiveValue(in *types.Si1Type, value any) *pbvara.Value {
	var val *pbvara.Value
	switch in.Def.Primitive.Si0TypeDefPrimitive {
	case types.IsBool:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bool{
				Bool: To_bool(value),
			},
		}
	case types.IsChar:
		val = &pbvara.Value{
			Type: &pbvara.Value_String_{
				String_: To_string(value),
			},
		}
	case types.IsStr:
		val = &pbvara.Value{
			Type: &pbvara.Value_String_{
				String_: To_string(value),
			},
		}
	case types.IsU8: // TODO: not sure about this one, should it be bytes ??
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_uint32(value)),
			},
		}
	case types.IsU16:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_uint32(value)),
			},
		}

	case types.IsU32:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_uint32(value)),
			},
		}

	case types.IsU64:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_uint64(value)),
			},
		}
	case types.IsU128:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_string(value)),
			},
		}
	case types.IsU256:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_string(value)),
			},
		}
	case types.IsI8:
		val = &pbvara.Value{
			Type: &pbvara.Value_Int32{
				Int32: To_int32(value),
			},
		}
	case types.IsI16:
		val = &pbvara.Value{
			Type: &pbvara.Value_Int32{
				Int32: To_int32(value),
			},
		}
	case types.IsI32:
		val = &pbvara.Value{
			Type: &pbvara.Value_Int32{
				Int32: To_int32(value),
			},
		}
	case types.IsI64:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_int64(value)),
			},
		}
	case types.IsI128:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_string(value)),
			},
		}
	case types.IsI256:
		val = &pbvara.Value{
			Type: &pbvara.Value_Bigint{
				Bigint: fmt.Sprint(To_string(value)),
			},
		}
	}
	return val
}

func ToCallIndex(ci *pbgear.CallIndex) types.CallIndex {
	return types.CallIndex{
		SectionIndex: uint8(ci.SectionIndex),
		MethodIndex:  uint8(ci.MethodIndex),
	}
}
