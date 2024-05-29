.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	buf build --as-file-descriptor-set -o /tmp/injectivecommon.binpb && buf generate /tmp/injectivecommon.binpb#format=bin

.PHONY: package
package: build
	substreams pack ./substreams.yaml
