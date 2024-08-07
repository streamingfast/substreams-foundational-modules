package main

import (
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

func To_bytes(a any) []byte {
	var items []any
	switch in := a.(type) {
	case registry.DecodedFields:
		fields := []*registry.DecodedField(in)
		items = fields[0].Value.([]any)
	case []any:
		items = in
	default:
		panic(fmt.Sprintf("Unsupported type %T", in))
	}
	var out []byte
	for _, i := range items {
		u := uint8(i.(types.U8))
		out = append(out, byte(u))
	}
	return out
}

func To_string(i any) string {
	switch v := i.(type) {
	case types.UCompact:
		return fmt.Sprintf("%d", v.Int64())
	case types.Text:
		return string(v)
	case types.U128:
		return v.String()
	case types.U256:
		return v.String()
	case types.I128:
		return v.String()
	case types.I256:
		return v.String()
	default:
		panic("Unknown type")
	}
}

func To_int32(i any) int32 {
	switch v := i.(type) {
	case types.UCompact:
		return int32(v.Int64())
	case types.I8:
		return int32(v)
	case types.I16:
		return int32(v)
	case types.I32:
		return int32(v)
	default:
		panic("Unknown type")
	}
}

func To_uint32(i any) uint32 {
	switch v := i.(type) {
	case types.UCompact:
		return uint32(v.Int64())
	case types.U8:
		return uint32(v)
	case types.U16:
		return uint32(v)
	case types.U32:
		return uint32(v)
	default:
		panic("Unknown type")
	}
}

func To_Optional_uint32(i any) *uint32 {
	o := To_uint32(i)
	return &o
}

func To_uint64(a any) uint64 {
	switch v := a.(type) {
	case types.UCompact:
		return uint64(v.Int64())
	case types.U8:
		return uint64(v)
	case types.U16:
		return uint64(v)
	case types.U32:
		return uint64(v)
	case types.U64:
		return uint64(v)
	case uint8:
		return uint64(v)
	case uint16:
		return uint64(v)
	case uint32:
		return uint64(v)
	case uint64:
		return uint64(v)
	default:
		panic("Unknown type")
	}
}

func To_int64(a any) int64 {
	switch v := a.(type) {
	case types.UCompact:
		return int64(v.Int64())
	case types.I8:
		return int64(v)
	case types.I16:
		return int64(v)
	case types.I32:
		return int64(v)
	case types.I64:
		return int64(v)
	case int8:
		return int64(v)
	case int16:
		return int64(v)
	case int32:
		return int64(v)
	case int64:
		return int64(v)
	default:
		panic("Unknown type")
	}
}

func To_Optional_uint64(i any) *uint64 {
	o := To_uint64(i)
	return &o
}

func To_Optional_string(a any) *string {
	s := To_string(a)
	return &s
}

func To_bool(b any) bool {
	return b.(bool)
}

func To_Optional_bool(b any) *bool {
	o := To_bool(b)
	return &o
}
