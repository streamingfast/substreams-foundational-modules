package metadata

import (
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

func Load(data string) *types.Metadata {
	metadata := &types.Metadata{}
	err := codec.DecodeFromHex(data, metadata)
	if err != nil {
		panic(err)
	}
	return metadata
}
