//go:build tinygo || wasip1

package main

import (
	"fmt"
	"reflect"
	"unsafe"

	pbgear "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/gear/type/v1"
	pbvara "github.com/streamingfast/substreams-foundational-modules/vara-common/pb/sf/substreams/vara/type/v1"
)

func main() {}

// func panic(a any) {
// 	Logf("panic: %v", a)
// 	os.Exit(2) //fail-safe so we know someone call panic
// }

//export db_get_i64
func _db_get_i64(code, scope, key uint64) []byte

//export output
func _output(ptr, len uint32)

//go:wasm-module logger
//export println
func _log(ptr, len uint32)

// Log a line to the Substreams engine
func Logf(message string, args ...any) {
	unsafePtr, bufPtr := stringToPtr(fmt.Sprintf(message, args...))
	_log(uint32(unsafePtr), uint32(bufPtr))
}

// Output the serialized protobuf byte array to the Substreams engine
func output(out []byte) {
	_output(byteArrayToPtr(out))
}

type OutputError struct {
	msg string
}

func (o OutputError) Error() string {
	return o.msg
}

func outputVT(out vtMessage) error {
	if out == nil || reflect.ValueOf(out).IsNil() {
		return nil
	}
	b, err := out.MarshalVT()
	if err != nil {
		return fmt.Errorf("marshalling output: %w", err)
	}
	if len(b) == 0 {
		return nil
	}
	output(b)
	return nil
}

//export map_decoded_block
func _map_decoded_block(blockPtr, blockLen int32) int32 {
	a := ptrToString(blockPtr, blockLen)
	b := []byte(a)

	in := &pbgear.Block{}
	if err := in.UnmarshalVT(b); err != nil {
		Logf("failed unmarshal: %q", err)
		return 1
	}

	ret, err := MapDecodedBlock(in)
	if err != nil {
		Logf("map_decoded_block failed: %s", err)
		return 1
	}
	if ret != nil {
		cnt, err := ret.MarshalVT()
		if err != nil {
			panic(fmt.Errorf("marshal output: %w", err))
		}
		output(cnt)
	}
	return 0
}

//export all_extrinsics
func all_extrinsics(blockPtr, blockLen int32) (retval int32) {
	block := &pbvara.Block{}
	if err := unmarshalVT(blockPtr, blockLen, block); err != nil {
		Logf("failed unmarshal: %q", err)
		return 1
	}

	ret, err := AllExtrinsics(block)

	if err != nil {
		panic(fmt.Errorf("AllExtrinsics: failed: %w", err))
	}
	if ret != nil {
		cnt, err := ret.MarshalVT()
		if err != nil {
			panic(fmt.Errorf("AllExtrinsics: marshal output: %w", err))
		}
		output(cnt)
	}
	return 0
}

//export index_extrinsics
func index_extrinsics(extrinsicsPtr, extrinsicsLen int32) (retval int32) {
	extrinsics := &pbvara.Extrinsics{}
	if err := unmarshalVT(extrinsicsPtr, extrinsicsLen, extrinsics); err != nil {
		Logf("unmarshalling extrinsics: %s", err)
		return 1
	}

	output, err := IndexExtrinsics(extrinsics)
	if err != nil {
		Logf("executing index_block: %s", err)
		return 1

	}
	if err := outputVT(output); err != nil {
		Logf("outputing vt message: %s", err)
		return 1
	}
	return 0
}

//export filtered_extrinsics
func filtered_extrinsics(queryPtr, queryLen int32, blockPtr, blockLen int32) (ret int32) {
	query := ptrToString(queryPtr, queryLen)

	block := &pbvara.Block{}
	if err := unmarshalVT(blockPtr, blockLen, block); err != nil {
		Logf("failed unmarshal transactions: %s", err)
		return 1
	}

	output, err := FilteredExtrinsics(query, block)
	if err != nil {
		Logf("calling filtered_transactions: %s", err)
		return 1
	}

	if err := outputVT(output); err != nil {
		Logf("outputing vt message: %s", err)
		return 1
	}
	return 0
}

// ptrToString returns a string from WebAssembly compatible numeric types
// representing its pointer and length.
func ptrToString(ptr int32, size int32) string {
	// Get a slice view of the underlying bytes in the stream. We use SliceHeader, not StringHeader
	// as it allows us to fix the capacity to what was allocated.
	return *(*string)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(ptr),
		Len:  int(size), // Tinygo requires these as uintptrs even if they are int fields.
		Cap:  int(size), // ^^ See https://github.com/tinygo-org/tinygo/issues/1284
	}))
}

func ptrToBytes(ptr int32, size int32) []byte {
	// Get a slice view of the underlying bytes in the stream. We use SliceHeader, not StringHeader
	// as it allows us to fix the capacity to what was allocated.
	return *(*[]byte)(unsafe.Pointer(&reflect.SliceHeader{
		Data: uintptr(ptr),
		Len:  int(size), // Tinygo requires these as uintptrs even if they are int fields.
		Cap:  int(size), // ^^ See https://github.com/tinygo-org/tinygo/issues/1284
	}))
}

// stringToPtr returns a pointer and size pair for the given string in a way
// compatible with WebAssembly numeric types.
func stringToPtr(s string) (int32, int32) {
	buf := []byte(s)
	ptr := &buf[0]
	unsafePtr := uintptr(unsafe.Pointer(ptr))
	return int32(unsafePtr), int32(len(buf))
}

// byteArrayToPtr returns a pointer and size pair for the given byte array, for WASM compat.
func byteArrayToPtr(buf []byte) (uint32, uint32) {
	ptr := &buf[0]
	unsafePtr := uintptr(unsafe.Pointer(ptr))
	return uint32(unsafePtr), uint32(len(buf))
}

type vtMessage interface {
	MarshalVT() ([]byte, error)
	UnmarshalVT([]byte) error
}

func unmarshalVT(prt, len int32, dest vtMessage) error {
	b := ptrToBytes(prt, len)
	return dest.UnmarshalVT(b)
}
