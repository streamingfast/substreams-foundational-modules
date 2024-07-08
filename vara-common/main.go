package main

import (
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	pbgear "github.com/streamingfast/tinygo-test/pb"
)

func map_extrinsics(block *pbgear.Block) (*pbgear.ParsedExtrinsics, error) {
	parsedExtrinsics, err := convertExtrinsics(block.Extrinsics)
	if err != nil {
		return nil, fmt.Errorf("failed to convert extrinsics: %w", err)
	}

	return &pbgear.ParsedExtrinsics{
		BlockHash:  block.Hash,
		Extrinsics: parsedExtrinsics,
	}, nil
}

func convertExtrinsics(extrinsics []*pbgear.Extrinsic) ([]*pbgear.ParsedExtrinsic, error) {
	gearExtrinsics := make([]*pbgear.ParsedExtrinsic, 0, len(extrinsics))
	for _, extrinsic := range extrinsics {
		callFields, err := convertDecodedFields(extrinsic)
		if err != nil {
			return nil, fmt.Errorf("failed to convert decoded fields: %w", err)
		}
		_ = callFields
		// gearExtrinsics = append(gearExtrinsics, &pbgear.ParsedExtrinsic{
		// 	Name:       extrinsic.Name,
		// 	CallFields: callFields,
		// 	CallIndex:  convertCallIndex(extrinsic.CallIndex),
		// 	Version:    uint32(extrinsic.Version),
		// 	Signature:  convertExtrinsicsSignature(extrinsic.Signature),
		// })
	}

	return gearExtrinsics, nil
}

func convertDecodedFields(extrinsic *pbgear.Extrinsic) (*pbgear.ParsedExtrinsic, error) {
	return &pbgear.ParsedExtrinsic{
		Name: "",
		CallFields: ,
		CallIndex: ,
		Version: extrinsic.Version,
		Signature: convertSignature(extrinsic.Signature),
	}, nil
}

func convertSignature(signature *pbgear.ExtrinsicSignature) *pbgear.ParsedExtrinsicSignature {
	return &pbgear.ParsedExtrinsicSignature{
		Signer: signature.Signer,
		Signature: signature.Signature,
	}
}