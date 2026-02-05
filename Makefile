.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package:
	@projects=$$(find . -name 'substreams.yaml' -not -path './.git/*' -not -path './target/*' | xargs -n1 dirname | grep "$(PROJECT)"); \
	for project in $$projects; do \
		set -e ; \
		echo "Substreams packing $$project..."; \
		pushd $$project > /dev/null; \
		substreams build; \
		popd > /dev/null; \
	done

.PHONY: format
format:
	cargo fmt
