module github.com/streamingfast/substreams-foundational-modules/vara-common

go 1.22.0

require github.com/centrifuge/go-substrate-rpc-client/v4 v4.2.1

replace github.com/centrifuge/go-substrate-rpc-client/v4 => github.com/streamingfast/go-substrate-rpc-client/v4 v4.0.0-20240731152751-689d07763128

replace github.com/streamingfast/firehose-gear => ../../firehose-gear

require (
	github.com/holiman/uint256 v1.2.4 // indirect
	github.com/planetscale/vtprotobuf v0.6.0 // indirect
	golang.org/x/exp v0.0.0-20231110203233-9a3e6036ecaa // indirect
	google.golang.org/protobuf v1.33.0 // indirect
)

require (
	github.com/ChainSafe/go-schnorrkel v1.0.0 // indirect
	github.com/cosmos/go-bip39 v1.0.0 // indirect
	github.com/decred/base58 v1.0.4 // indirect
	github.com/decred/dcrd/crypto/blake256 v1.0.0 // indirect
	github.com/ethereum/go-ethereum v1.13.8 // indirect
	github.com/gtank/merlin v0.1.1 // indirect
	github.com/gtank/ristretto255 v0.1.2 // indirect
	github.com/mimoo/StrobeGo v0.0.0-20220103164710-9a04d6ca976b // indirect
	github.com/pierrec/xxHash v0.1.5 // indirect
	github.com/streamingfast/firehose-gear v0.0.0-20240731132012-850add1a5ef7
	github.com/vedhavyas/go-subkey/v2 v2.0.0 // indirect
	golang.org/x/crypto v0.23.0 // indirect
	golang.org/x/sys v0.20.0 // indirect
)
