package main

import (
	"bytes"
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/parser"
	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	pbgear "github.com/streamingfast/firehose-gear/pb/sf/gear/type/v1"
)

// this will actually return a decodedBlock containing all the decoded calls and events
func map_decoded_block(block *pbgear.Block) (*pbgear.Block, error) {
	factory := registry.NewFactory()
	callRegistry, err := factory.CreateCallRegistry(metadata)
	if err != nil {
		return nil, fmt.Errorf("failed to create call registry: %w", err)
	}

	decodedExtrinsic := make([]registry.DecodedFields, 0)
	for _, extrinsic := range block.Extrinsics {
		decodedField, err := decodeCallExtrinsics(callRegistry, extrinsic)
		if err != nil {
			return nil, fmt.Errorf("failed to decode extrinsic: %w", err)
		}
		decodedExtrinsic = append(decodedExtrinsic, decodedField)
	}

	eventRegistry, err := factory.CreateEventRegistry(metadata)
	if err != nil {
		return nil, fmt.Errorf("failed to create event registry: %w", err)
	}

	decodedEvents, err := decodeEvents(eventRegistry, block.RawEvents)
	if err != nil {
		return nil, fmt.Errorf("failed to decode events: %w", err)
	}
	_ = decodedEvents

	// TODO: here we want to call the code from the gen_types.go and decode the extrinsics and events
	// then we want to return *pbgear.DecodedBlock containing the decoded calls and events

	return nil, nil
}

func decodeCallExtrinsics(callRegistry registry.CallRegistry, extrinsic *pbgear.Extrinsic) (registry.DecodedFields, error) {
	callIndex := extrinsic.Method.CallIndex
	args := extrinsic.Method.Args

	callDecoder, ok := callRegistry[convertCallIndex(callIndex)]
	if ok != true {
		return nil, fmt.Errorf("failed to get call decoder")
	}

	decoder := scale.NewDecoder(bytes.NewReader(args))

	callFields, err := callDecoder.Decode(decoder)
	if err != nil {
		return nil, fmt.Errorf("failed to decode call: %w", err)
	}

	return callFields, nil
}

func convertCallIndex(ci *pbgear.CallIndex) types.CallIndex {
	return types.CallIndex{
		SectionIndex: uint8(ci.SectionIndex),
		MethodIndex:  uint8(ci.MethodIndex),
	}
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
