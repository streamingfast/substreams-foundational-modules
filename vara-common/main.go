package main

import (
	"bytes"
	"context"
	"encoding/hex"
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/parser"
	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/streamingfast/substreams-foundational-modules/vara-common/metadata"
	"github.com/streamingfast/substreams-foundational-modules/vara-common/metadata/spec"
	pbgear "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/gear/type/v1"
	pbindex "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/substreams/index/v1"
	pbsubstreams "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/substreams/v1"
	pbvara "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/substreams/vara/type/v1"
	"github.com/streamingfast/substreams-foundational-modules/vara-common/sqe"
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
	1421: spec.V1421,
	1500: spec.V1500,
	1501: spec.V1501,
}

type regItem struct {
	meta          *types.Metadata
	callRegistry  registry.CallRegistry
	eventRegistry registry.EventRegistry
}

var metadataRegistry = map[uint32]*regItem{}

func init() {
	factory := registry.NewFactory()
	for version, data := range specVersions {
		meta := metadata.Load(data)

		callRegistry, err := factory.CreateCallRegistry(meta)
		if err != nil {
			panic(fmt.Errorf("creating call registry: %w", err))
		}

		eventRegistry, err := factory.CreateEventRegistry(meta)
		if err != nil {
			panic(fmt.Errorf("creating event registry: %w", err))
		}
		metadataRegistry[version] = &regItem{
			meta:          meta,
			callRegistry:  callRegistry,
			eventRegistry: eventRegistry,
		}
	}
}

// this will actually return a decodedBlock containing all the decoded calls and events
func MapDecodedBlock(block *pbgear.Block) (*pbvara.Block, error) {
	reg := metadataRegistry[block.Header.SpecVersion]

	extrinsics, err := ToExtrinsics(block.Extrinsics, reg.callRegistry, reg.meta)
	if err != nil {
		return nil, fmt.Errorf("converting extrinsics: %w", err)
	}

	events, err := toEvents(reg.eventRegistry, block.RawEvents, reg.meta)
	if err != nil {
		return nil, fmt.Errorf("converting events: %w", err)
	}

	for _, event := range events {
		if event.Phase.IsApplyExtrinsic {
			ex := extrinsics[int(event.Phase.AsApplyExtrinsic)]
			ex.Events = append(ex.Events, event)
		}
	}

	return &pbvara.Block{
		Number:        block.Number,
		Hash:          block.Hash,
		Header:        block.Header,
		Timestamp:     block.Timestamp,
		DigestItems:   block.DigestItems,
		Justification: block.Justification,
		Extrinsics:    extrinsics,
		Events:        events,
	}, nil
}

func AllExtrinsics(block *pbvara.Block) (*pbvara.Extrinsics, error) {
	clock := &pbsubstreams.Clock{
		Id:        hex.EncodeToString(block.Hash),
		Number:    block.Number,
		Timestamp: block.Timestamp,
	}

	return &pbvara.Extrinsics{
		Clock:      clock,
		Extrinsics: block.Extrinsics,
	}, nil
}

func IndexExtrinsics(extrinsics *pbvara.Extrinsics) (*pbindex.Keys, error) {
	out := &Index{
		Keys: &pbindex.Keys{},
	}
	for _, extrinsic := range extrinsics.Extrinsics {
		idx := indexExtrinsic(extrinsic)
		out.Keys.Keys = append(out.Keys.Keys, idx.Keys.Keys...)
	}
	return out.Keys, nil
}

func indexExtrinsic(extrinsic *pbvara.Extrinsic) *Index {
	index := &Index{
		Keys: &pbindex.Keys{},
	}

	k := "extrinsic:" + extrinsic.Call.Name
	index.AddKey(k)
	for _, event := range extrinsic.Events {
		ek := "event:" + event.Name
		index.AddKey(k + ":" + ek)
		//extrinsic:Gear.run:event:Success
	}

	return index
}

func FilteredExtrinsics(query string, block *pbvara.Block) (*pbvara.Extrinsics, error) {
	clock := &pbsubstreams.Clock{
		Id:        hex.EncodeToString(block.Hash),
		Number:    block.Number,
		Timestamp: block.Timestamp,
	}

	out := &pbvara.Extrinsics{
		Clock: clock,
	}

	for _, extrinsic := range block.Extrinsics {
		index := indexExtrinsic(extrinsic)
		match, err := applyQuery(query, index.Keys)
		if err != nil {
			return nil, fmt.Errorf("applying query: %w", err)
		}
		if match {
			out.Extrinsics = append(out.Extrinsics, extrinsic)
		}

	}

	return out, nil
}

func applyQuery(query string, keys *pbindex.Keys) (bool, error) {
	keyQuerier := sqe.NewFromKeys(keys.Keys)
	q, err := sqe.Parse(context.Background(), query)
	if err != nil {
		return false, fmt.Errorf("parsing query %q: %w", query, err)
	}

	return sqe.KeysApply(q, keyQuerier), nil
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

	fields, err := toFields(decodedCall, metadata)
	if err != nil {
		return nil, fmt.Errorf("converting call fields: %w", err)
	}

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
		f, err := toFields(event.Fields, metadata)
		if err != nil {
			return nil, fmt.Errorf("converting fields: %w", err)
		}

		var topics [][]byte

		for _, topic := range event.Topics {
			h := [32]byte(topic)
			topics = append(topics, h[:])
		}

		out = append(out, &pbvara.Event{
			Name: event.Name,
			Phase: &pbgear.Phase{
				IsApplyExtrinsic: event.Phase.IsApplyExtrinsic,
				AsApplyExtrinsic: event.Phase.AsApplyExtrinsic,
				IsFinalization:   event.Phase.IsFinalization,
				IsInitialization: event.Phase.IsFinalization,
			},
			Fields: f,
			Topics: topics,
		})
	}

	return out, nil
}

func toFields(in any, metadata *types.Metadata) (*pbvara.Fields, error) {
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
			return nil, fmt.Errorf("expected an array of 1 element")
		}
		fields = i[0].(registry.DecodedFields)
	case nil:
		return &pbvara.Fields{}, nil
	default:
		return nil, fmt.Errorf("unexpected type %T", i)
	}
	m := map[string]*pbvara.Value{}

	for _, field := range fields {
		if _, found := m[field.Name]; found {
			return nil, fmt.Errorf("duplicate field: " + field.Name)
		}

		v, err := toValue(field, metadata)

		if err != nil {
			return nil, fmt.Errorf("converting field %q: %w", field.Name, err)
		}

		m[field.Name] = v
	}

	return &pbvara.Fields{
		Map: m,
	}, nil
}

func toValue(decodedField *registry.DecodedField, metadata *types.Metadata) (*pbvara.Value, error) {
	lookupType := metadata.AsMetadataV14.EfficientLookup[decodedField.LookupIndex]

	switch {
	case lookupType.Def.IsPrimitive:
		return toPrimitiveValue(lookupType, decodedField.Value)

	case lookupType.Def.IsSequence, lookupType.Def.IsArray:
		return toSequenceValue(decodedField.Value, metadata, lookupType)

	case lookupType.Def.IsTuple:
		return toTupleValue(decodedField, metadata, lookupType)

	case lookupType.Def.IsVariant:
		return toVariantValue(decodedField, metadata, lookupType)

	case lookupType.Def.IsCompact:
		return toCompactValue(decodedField, metadata, lookupType)

	case lookupType.Def.IsComposite:
		return toCompositeValue(decodedField, metadata, lookupType)
	default:
		return nil, fmt.Errorf("unknown type")
	}
}

func toTupleValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) (*pbvara.Value, error) {
	return nil, fmt.Errorf("not implemented")
}

func toCompositeValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) (*pbvara.Value, error) {
	compositeFields := decodedField.Value.(registry.DecodedFields)

	if len(compositeFields) == 1 {
		f := compositeFields[0]
		if f.Name == "[u8; 32]" {
			v, err := toValue(f, metadata)
			if err != nil {
				return nil, fmt.Errorf("converting composite field: %w", err)
			}
			return &pbvara.Value{
				Type: &pbvara.Value_Bytes{
					Bytes: v.Type.(*pbvara.Value_Bytes).Bytes,
				},
			}, nil
		}
	}

	values := map[string]*pbvara.Value{}
	for _, field := range compositeFields {
		if field.Name == "" {
			panic("composite field name not set")
		}

		switch v := field.Value.(type) {
		case []interface{}: //this is a Sequence
			var err error
			values[field.Name], err = toValue(field, metadata)
			if err != nil {

			}
		case registry.DecodedFields:
			var err error
			fs, err := toFields(v, metadata)
			if err != nil {
				return nil, fmt.Errorf("converting composite field: %w", err)
			}
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
	}, nil
}

func toCompactValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) (*pbvara.Value, error) {
	childType := metadata.AsMetadataV14.EfficientLookup[lookupType.Def.Compact.Type.Int64()]
	if childType.Def.IsPrimitive {
		return toPrimitiveValue(childType, decodedField.Value)
	} else {
		fields, err := toFields(decodedField.Value, metadata)
		if err != nil {
			return nil, fmt.Errorf("converting none primitive: %w", err)
		}
		if len(fields.Map) == 1 {
			for _, field := range fields.Map {
				return &pbvara.Value{
					Type: field.Type,
				}, nil
			}
		}

		return &pbvara.Value{
			Type: &pbvara.Value_Fields{
				Fields: fields,
			},
		}, nil
	}
}

func toVariantValue(decodedField *registry.DecodedField, metadata *types.Metadata, lookupType *types.Si1Type) (*pbvara.Value, error) {
	switch v := decodedField.Value.(type) {
	case *registry.VariantWTF:
		wtf := decodedField.Value.(*registry.VariantWTF)
		for _, variant := range lookupType.Def.Variant.Variants {
			if byte(variant.Index) == wtf.VariantByte {
				if len(variant.Fields) == 1 {
					childType := metadata.AsMetadataV14.EfficientLookup[variant.Fields[0].Type.Int64()]
					if childType.Def.IsPrimitive {
						value, err := toPrimitiveValue(childType, v.ValueAt(0))
						if err != nil {
							return nil, fmt.Errorf("converting variant primitive field: %w", err)
						}
						return value, nil
					}
					if childType.Def.IsSequence || childType.Def.IsArray {
						value, err := toSequenceValue(v.ValueAt(0), metadata, childType)
						if err != nil {
							return nil, fmt.Errorf("converting variant sequence field: %w", err)
						}
						return value, nil
					}
				}

				if _, ok := v.ValueAt(0).(uint8); ok {
					value := &pbvara.Value{}
					value.Type = &pbvara.Value_String_{
						String_: string(variant.Name),
					}

					return value, nil
				}

				f, err := toFields(v.ValueAt(0), metadata)
				if err != nil {
					return nil, fmt.Errorf("converting variant field item: %w", err)
				}
				value := &pbvara.Value{}
				value.Type = &pbvara.Value_Fields{
					Fields: f,
				}

				return value, nil
			}
		}
		return nil, fmt.Errorf("variant not found for: %d", wtf.VariantByte)
	case uint8: //this is an enum
		//todo: we should add a enum type with both name and index
		for _, variant := range lookupType.Def.Variant.Variants {
			if byte(variant.Index) == decodedField.Value {
				value := &pbvara.Value{}
				value.Type = &pbvara.Value_String_{
					String_: string(variant.Name),
				}
				return value, nil
			}
		}
		return nil, fmt.Errorf("enum variant not found for: %d", decodedField.Value)
	default:
		return nil, fmt.Errorf("unknown type %T", decodedField.Value)
	}
}

func toSequenceValue(value any, metadata *types.Metadata, lookupType *types.Si1Type) (*pbvara.Value, error) {
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
		data := To_bytes(value)
		return &pbvara.Value{
			Type: &pbvara.Value_Bytes{
				Bytes: data,
			},
		}, nil

	} else {
		for _, item := range value.([]interface{}) {
			if arrayOfType.Def.IsPrimitive {
				val, err := toPrimitiveValue(arrayOfType, item.(*registry.DecodedField).Value)
				if err != nil {
					return nil, fmt.Errorf("decoding primitive value: %w", err)
				}
				array.Items = append(array.Items, val)
				continue
			} else if arrayOfType.Def.IsTuple {
				fields, err := toFields(item, metadata)
				if err != nil {
					return nil, fmt.Errorf("converting tuple: %w", err)
				}
				val := &pbvara.Value{
					Type: &pbvara.Value_Fields{
						Fields: fields,
					},
				}
				array.Items = append(array.Items, val)
				continue
			} else if arrayOfType.Def.IsSequence || arrayOfType.Def.IsArray {
				val, err := toSequenceValue(item, metadata, arrayOfType)
				if err != nil {
					return nil, fmt.Errorf("converting sequence: %w", err)
				}
				array.Items = append(array.Items, val)
				continue

			} else if arrayOfType.Def.IsComposite {
				fields, err := toFields(item, metadata)
				if err != nil {
					return nil, fmt.Errorf("converting composite: %w", err)
				}
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
						Fields: fields,
					},
				}
				array.Items = append(array.Items, val)
				continue
			}

			valuable, ok := item.(registry.Valuable)
			if !ok {
				fmt.Println("wtf")
			}
			v := valuable.ValueAt(0)
			f, err := toFields(v, metadata)
			if err != nil {
				return nil, fmt.Errorf("converting composite field item: %w", err)
			}
			val := &pbvara.Value{
				Type: &pbvara.Value_Fields{
					Fields: f,
				},
			}
			array.Items = append(array.Items, val)
		}

		return &pbvara.Value{
			Type: &pbvara.Value_Array{
				Array: array,
			},
		}, nil
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
	if storageEvents == nil {
		return nil, nil
	}

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

func toPrimitiveValue(in *types.Si1Type, value any) (*pbvara.Value, error) {
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
	return val, nil
}

func ToCallIndex(ci *pbgear.CallIndex) types.CallIndex {
	return types.CallIndex{
		SectionIndex: uint8(ci.SectionIndex),
		MethodIndex:  uint8(ci.MethodIndex),
	}
}
