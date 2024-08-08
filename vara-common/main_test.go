package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path"
	"strings"
	"testing"

	pbgear "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/gear/type/v1"
	"github.com/stretchr/testify/require"
)

var testBlocks = loadBlocks()

func Test_MapDecodedBlock_EmptyBlock(t *testing.T) {
	blk := &pbgear.Block{}
	dblock, err := map_decoded_block(blk)
	require.NoError(t, err)
	fmt.Println(dblock.Number)
}

func Test_MapDecodedBlock(t *testing.T) {
	tests := []struct {
		name string
		key  string
	}{
		{name: "spec100", key: "100"},
		{name: "spec120", key: "17600"},
		{name: "spec130", key: "138000"},
		{name: "spec140", key: "1380000"},
		{name: "spec210", key: "4182200"},
		{name: "spec310", key: "4433500"},
		{name: "spec320", key: "4641600"},
		{name: "spec330", key: "4961300"},
		{name: "spec340", key: "4986900"},
		{name: "spec350", key: "5046400"},
		{name: "spec1000", key: "5079000"},
		{name: "spec1010", key: "6760600"},
		{name: "spec1020", key: "7196000"},
		{name: "spec1030", key: "8384900"},
		{name: "spec1040", key: "8754200"},
		{name: "spec1050", key: "8775700"},
		{name: "spec1110", key: "9876600"},
		{name: "spec1200", key: "10849200"},
		{name: "spec1210", key: "10934500"},
		{name: "spec1300", key: "11627100"},
		{name: "spec1310", key: "12224800"},
		{name: "spec1400", key: "12629600"},
		{name: "spec1410", key: "12885100"},
		{name: "spec1420", key: "14463900"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			key := tt.key
			blocks, ok := testBlocks[key]
			require.True(t, ok)

			for _, b := range blocks {
				dblock, err := map_decoded_block(b)
				require.NoError(t, err)
				require.NotNil(t, dblock)
				//j, err := json.MarshalIndent(dblock, "", "  ")
				//require.NoError(t, err)
				//fmt.Println(string(j))
			}
		})
	}
}

func loadBlocks() map[string][]*pbgear.Block {
	dataDir := "./testdata"
	files, err := os.ReadDir(dataDir)
	if err != nil {
		panic(err)
	}

	blockMap := make(map[string][]*pbgear.Block, 0)
	for _, file := range files {
		if strings.HasSuffix(file.Name(), ".blocks.jsonl") {
			f, err := os.Open(path.Join(dataDir, file.Name()))
			if err != nil {
				panic(err)
			}

			scanner := bufio.NewScanner(f)
			blocks := make([]*pbgear.Block, 0)

			for scanner.Scan() {
				block := &pbgear.Block{}
				err = json.Unmarshal([]byte(scanner.Text()), block)
				if err != nil {
					panic(err)
				}
				blocks = append(blocks, block)
			}

			blockMap[strings.TrimSuffix(file.Name(), ".blocks.jsonl")] = blocks

			if err := f.Close(); err != nil {
				panic(err)
			}
		}
	}
	return blockMap
}
