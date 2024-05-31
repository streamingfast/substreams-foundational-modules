generated-buf-build.binpb: buf.yaml buf.gen.yaml proto/cosmos/*/*.proto
	buf build --as-file-descriptor-set -o generated-buf-build.binpb

.PHONY: build
build: generated-buf-build.binpb
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen: generated-buf-build.binpb
	buf generate --exclude-path=gogoproto --exclude-path=google/protobuf --exclude-path=cosmos/msg/v1 --exclude-path=amino generated-buf-build.binpb

.PHONY: package
package: build
	substreams pack ./substreams.yaml
