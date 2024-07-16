//go:build tinygo || wasip1

package main

import (
	"fmt"
	"reflect"
	"unsafe"

	pbstarknet "github.com/streamingfast/substreams-foundational-modules/starknet-common/pb/sf/starknet/type/v1"
	v1 "github.com/streamingfast/substreams-foundational-modules/starknet-common/pb/sf/substreams/starknet/type/v1"
)

//go:generate substreams-starknet protogen ./substreams-starknet.yaml --with-tinygo-maps // creates genre substreams-starknet.gen.go

// Dans WASI: _start
func main() {}

////export db_get_i64
//func _db_get_i64(code, scope, key uint64) []byte {}

//export output
func _output(ptr, len int32) {}

//go:wasm-module logger
//export println
func _log(ptr int32, len int32) {}

// Output the serialized protobuf byte array to the Substreams engine
func output(out []byte) {
	_output(byteArrayToPtr(out))
}

// Log a line to the Substreams engine
func Logf(message string, args ...any) {
	_log(stringToPtr(fmt.Sprintf(message, args...)))
}

//export all_transactions
func _all_transactions(blockPtr, blockLen int32) (retval int32) {
	defer func() {
		if r := recover(); r != nil {
			Logf("%#v", r)
			retval = 1
		}
	}()

	a := ptrToString(blockPtr, blockLen)
	b := []byte(a)
	dest := &pbstarknet.Block{}
	if err := dest.UnmarshalVT(b); err != nil {
		Logf("failed unmarshal: %s", err)
		return 1
	}

	ret, err := AllTransactions(dest)
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

//export index_transactions
func _index_transactions(ptr, len int32) (retval int32) {
	defer func() {
		if r := recover(); r != nil {
			Logf("%#v", r)
			retval = 1
		}
	}()

	a := ptrToString(int32(ptr), int32(len))
	b := []byte(a)
	dest := &v1.Transactions{}
	if err := dest.UnmarshalVT(b); err != nil {
		Logf("failed unmarshal: %s", err)
		return 1
	}

	ret, err := IndexTransaction(dest)
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

//export filtered_transactions
func _filtered_transactions(queryPtr, queryLen int32, transactionsPtr, transactionsLen int32) (ret int32) {
	defer func() {
		if r := recover(); r != nil {
			Logf("%#v", r)
			ret = 1
		}
	}()

	query := ptrToString(queryPtr, queryLen)
	txsData := ptrToBytes(transactionsPtr, transactionsLen)
	txs := &v1.Transactions{}

	if err := txs.UnmarshalVT(txsData); err != nil {
		Logf("failed unmarshal: %s", err)
		return 1
	}

	out, err := FilteredTransactions(query, txs)
	if err != nil {
		panic(fmt.Errorf("map_extrinsics failed: %w", err))
	}

	if out != nil {
		cnt, err := out.MarshalVT()
		if err != nil {
			panic(fmt.Errorf("marshal output: %w", err))
		}
		output(cnt)
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
func byteArrayToPtr(buf []byte) (int32, int32) {
	ptr := &buf[0]
	unsafePtr := uintptr(unsafe.Pointer(ptr))
	return int32(unsafePtr), int32(len(buf))
}
