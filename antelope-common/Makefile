# .PHONY: protogen
# protogen:
# 	substreams protogen --exclude-paths="sf/antelope,sf/substreams/index,sf/substreams/v1,sf/substreams/rpc,sf/substreams/sink,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package: build
	substreams pack substreams.yaml

.PHONY: index
index: build
	substreams gui -e eos.substreams.pinax.network:443 index_actions -s -10


.PHONY: gui
gui: build
	substreams gui -e eos.substreams.pinax.network:443 filtered_transactions -s 372667500