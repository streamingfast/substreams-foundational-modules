package main

import (
	"testing"

	pbgear "github.com/streamingfast/firehose-gear/pb/sf/gear/type/v1"
)

func Test_MapDecodedBlock(t *testing.T) {
	blk := &pbgear.Block{}
	dblock, err := map_decoded_block(blk)
	if err != nil {
		t.Fatalf("failed to map decoded block: %s", err)
	}
	_ = dblock
}
