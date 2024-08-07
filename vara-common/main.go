package main

import (
	"bytes"
	"fmt"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
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
	events := toEvents(block.Events)

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

func toEvents(events []*pbgear.Event) []*pbvara.Event {
	var out []*pbvara.Event

	return out
}

func toFields(fields registry.DecodedFields, metadata *types.Metadata) *pbvara.Fields {
	m := map[string]*pbvara.Value{}

	for _, field := range fields {
		v := toValue(field, metadata)
		m[field.Name] = v
	}

	return &pbvara.Fields{
		Map: nil,
	}
}

func toValue(field *registry.DecodedField, metadata *types.Metadata) *pbvara.Value {
	lookupType := metadata.AsMetadataV14.EfficientLookup[field.LookupIndex]
	var value *pbvara.Value
	switch {
	case lookupType.Def.IsPrimitive:
	case lookupType.Def.IsSequence:
	case lookupType.Def.IsArray:
	case lookupType.Def.IsTuple:
	case lookupType.Def.IsVariant:

	case lookupType.Def.IsCompact:
	case lookupType.Def.IsComposite:
		value.Typed = &pbvara.Value_Fields{
			Fields: toFields(field.Value.(registry.DecodedFields), metadata),
		}
	default:
		panic("unknown type")
	}

	return &pbvara.Value{}
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

//func decodeEvents(eventRegistry registry.EventRegistry, storageEvents []byte) ([]*parser.Event, error) {
//	decoder := scale.NewDecoder(bytes.NewReader(storageEvents))
//
//	eventsCount, err := decoder.DecodeUintCompact()
//	if err != nil {
//		return nil, fmt.Errorf("failed to decode events count: %w", err)
//	}
//
//	var events []*parser.Event
//
//	for i := uint64(0); i < eventsCount.Uint64(); i++ {
//		var phase types.Phase
//
//		err := decoder.Decode(&phase)
//		if err != nil {
//			return nil, fmt.Errorf("failed to decode phase: %w", err)
//		}
//
//		var eventID types.EventID
//
//		err = decoder.Decode(&eventID)
//		if err != nil {
//			return nil, fmt.Errorf("failed to decode event ID: %w", err)
//		}
//
//		eventDecoder, ok := eventRegistry[eventID]
//		if !ok {
//			return nil, fmt.Errorf("failed to get event decoder")
//		}
//
//		eventFields, err := eventDecoder.Decode(decoder)
//		if err != nil {
//			return nil, fmt.Errorf("failed to decode event fields: %w", err)
//		}
//
//		var topics []types.Hash
//
//		err = decoder.Decode(&topics)
//		if err != nil {
//			return nil, fmt.Errorf("failed to decode topics: %w", err)
//		}
//
//		event := &parser.Event{
//			Name:    eventDecoder.Name,
//			Fields:  eventFields,
//			EventID: eventID,
//			Phase:   &phase,
//			Topics:  topics,
//		}
//
//		events = append(events, event)
//	}
//
//	return events, nil
//}

func ToCallIndex(ci *pbgear.CallIndex) types.CallIndex {
	return types.CallIndex{
		SectionIndex: uint8(ci.SectionIndex),
		MethodIndex:  uint8(ci.MethodIndex),
	}
}
