delete-binpb:
ifneq ("$(wildcard $(PATH_TO_FILE))","")
	rm proto/generated-buf-build.binpb
endif

generated-buf-build.binpb: proto/buf.gen.yaml proto/buf.yaml delete-binpb
	cd proto; buf mod update; buf build --as-file-descriptor-set -o generated-buf-build.binpb

.PHONY: build
build: generated-buf-build.binpb
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen: generated-buf-build.binpb
	cd proto; buf generate  generated-buf-build.binpb -o ../

.PHONY: package
package: build
	substreams pack ./substreams.yaml