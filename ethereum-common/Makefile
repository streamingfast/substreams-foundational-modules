.PHONY: protogen
protogen:
	substreams protogen --exclude-paths="sf/ethereum/substreams,sf/substreams/rpc,sf/substreams/sink,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package: build
	substreams pack substreams.yaml
