//go:build tinygo || wasip1

package main

import (
	"fmt"
	"os"
	"reflect"
	"unsafe"

	pbgear "github.com/streamingfast/firehose-gear/pb/sf/gear/type/v1"
)

//go:generate substreams protogen ./substreams.yaml --with-tinygo-maps // creates genre substreams.gen.go

// Dans WASI: _start
func main() {
	// m := load_metadata()
	// fmt.Println("metadata loaded", m.Version)
}

func panic(a any) {
	os.Exit(2) //fail-safe so we know someone call panic
}

//export db_get_i64
func _db_get_i64(code, scope, key uint64) []byte

//export output
func _output(ptr, len uint32)

//go:wasm-module logger
//export println
func _log(ptr, len uint32)

// Log a line to the Substreams engine
func Logf(message string, args ...any) {
	_log(stringToPtr(fmt.Sprintf(message, args...)))
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
func _map_decoded_block(blockPtr, blockLen uint32) (retval uint32) {
	// defer func() {
	// 	if r := recover(); r != nil {
	// 		logf("%#v", r)
	// 		retval = 1
	// 	}
	// }()

	a := ptrToString(blockPtr, blockLen)
	b := []byte(a)
	dest := &pbgear.Block{}
	if err := dest.UnmarshalVT(b); err != nil {
		Logf("failed unmarshal: %w, %d", err, len(a), len(b), b[:20])
		return 1
	}

	ret, err := map_decoded_block(dest)
	if err != nil {
		panic(fmt.Errorf("map_extrinsics failed: %w", err))
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

// ptrToString returns a string from WebAssembly compatible numeric types
// representing its pointer and length.
func ptrToString(ptr uint32, size uint32) string {
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
func stringToPtr(s string) (uint32, uint32) {
	buf := []byte(s)
	ptr := &buf[0]
	unsafePtr := uintptr(unsafe.Pointer(ptr))
	return uint32(unsafePtr), uint32(len(buf))
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
