.PHONY: build
build: protobuild
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protobuild
protobuild:
	buf build --as-file-descriptor-set -o generated-buf-build.binpb

.PHONY: protogen
protogen: protobuild
	buf generate --exclude-path=gogoproto --exclude-path=google/protobuf --exclude-path=cosmos/msg/v1 --exclude-path=amino generated-buf-build.binpb

.PHONY: package
package: build
	substreams pack ./substreams.yaml
