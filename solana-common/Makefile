.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run substreams.yaml $(if $(MODULE),$(MODULE),map_block_meta) $(if $(START_BLOCK),-s $(START_BLOCK)) $(if $(STOP_BLOCK),-t $(STOP_BLOCK)) $(if $(PARAMS),-p '$(PARAMS)')

.PHONY: package
package: build
	substreams pack substreams.yaml
