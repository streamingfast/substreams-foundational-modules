ENDPOINT ?= eth.substreams.pinax.network:443 # OR eth.substreams.pinax.network:443
START_BLOCK ?= 483000
STOP_BLOCK ?= 486000
MODULE ?= metadata_to_foundational_store

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml $(MODULE) -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml $(MODULE) -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf"

.PHONY: pack
pack: build
	substreams pack substreams.yaml 