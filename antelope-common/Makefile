.PHONY: protogen
protogen:
	substreams protogen --exclude-paths="sf/substreams/antelope,sf/antelope,sf/substreams/rpc,sf/substreams/sink,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package: build
	substreams pack substreams.yaml

.PHONY: index
index: build
	substreams gui -e eos.substreams.pinax.network:443 index_stuff -s -10