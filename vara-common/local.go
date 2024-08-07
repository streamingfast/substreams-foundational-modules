//go:build !tinygo && !wasip1

package main

import (
	"fmt"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

var metadata *types.Metadata

func init() {
	metadata = loadMetadata()
}

// Log a line to the Substreams engine
func Logf(message string, args ...any) {
	fmt.Printf(message+"\n", args...)
}
