//go:build !tinygo

package main

import "fmt"

// Log a line to the Substreams engine
func Logf(message string, args ...any) {
	fmt.Printf(message+"\n", args...)
}
