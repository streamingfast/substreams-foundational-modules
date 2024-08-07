package main

import (
	"encoding/json"
	"fmt"
	"os"
	"testing"

	pbgear "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/gear/type/v1"
	"github.com/stretchr/testify/require"
)

func Test_MapDecodedBlock_EmptyBlock(t *testing.T) {
	blk := &pbgear.Block{}
	dblock, err := map_decoded_block(blk)
	require.NoError(t, err)
	fmt.Println(dblock.Number)
}

func Test_MapDecodedBlock(t *testing.T) {
	blk := &pbgear.Block{}
	b, err := os.ReadFile("testdata/block_14463900.json")
	require.NoError(t, err)
	err = json.Unmarshal(b, blk)
	require.NoError(t, err)

	dblock, err := map_decoded_block(blk)
	require.NoError(t, err)
	fmt.Println(dblock.Number)
}
