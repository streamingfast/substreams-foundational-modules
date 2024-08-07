package main

import (
	"fmt"
	pbgear "github.com/streamingfast/firehose-gear/pb/sf/gear/type/v1"
	"github.com/stretchr/testify/require"
	"testing"
)

func Test_MapDecodedBlock(t *testing.T) {
	blk := &pbgear.Block{}
	dblock, err := map_decoded_block(blk)
	require.NoError(t, err)
	fmt.Println(dblock.Number)
}
