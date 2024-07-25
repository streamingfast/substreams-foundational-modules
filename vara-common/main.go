package main

import (
	"bytes"
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	pbgear "github.com/streamingfast/firehose-gear/pb/sf/gear/type/v1"
)

// this will actually return a decodedBlock containing all the decoded calls and events
func map_decoded_block(block *pbgear.Block) (*pbgear.Block, error) {
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
