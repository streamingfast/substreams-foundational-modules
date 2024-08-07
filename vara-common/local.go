//go:build !tinygo && !wasip1

package main

import (
	"fmt"
)

func init() {
}

// Log a line to the Substreams engine
func Logf(message string, args ...any) {
	fmt.Printf(message+"\n", args...)
}
