package main

import (
	"bufio"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path"
	"strings"
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

	j, err := json.MarshalIndent(dblock, "", "  ")
	require.NoError(t, err)
	fmt.Println(string(j))
}

func Test_MapDecodedBlocks(t *testing.T) {

	dataDir := "./testdata"
	files, err := os.ReadDir(dataDir)
	if err != nil {
		fmt.Println(err)
		return
	}

	for _, file := range files {
		//check for extension .blocks.jsonl
		if strings.HasSuffix(file.Name(), ".blocks.jsonl") {

			fmt.Println("processing:", file.Name())
			//open file
			f, err := os.Open(path.Join(dataDir, file.Name()))
			require.NoError(t, err)

			scanner := bufio.NewScanner(f)
			for scanner.Scan() {
				// Unmarshal each line into &pbgear.Block{}
				block := &pbgear.Block{}
				err = json.Unmarshal([]byte(scanner.Text()), block)
				require.NoError(t, err)

				dblock, err := map_decoded_block(block)
				require.NoError(t, err)
				fmt.Println("decoded block", dblock.Number, hex.EncodeToString(block.Hash))
				j, err := json.MarshalIndent(dblock, "", "  ")
				require.NoError(t, err)
				fmt.Println(string(j))
			}

			//closing the file
			if err := f.Close(); err != nil {
				fmt.Println(err)
			}
		}
	}
}
