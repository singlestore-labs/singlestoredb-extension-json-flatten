MODULE := json_flatten

.PHONY: debug
debug: $(eval TGT:=debug)
debug: MODE = debug
debug: all

.PHONY: release
release: $(eval TGT:=release)
release: RELFLAGS = --release
release: MODE = release
release: all

.PHONY: all
all: loader_standalone loader_extension

.PHONY: wasm
wasm:
	cargo wasi build --lib $(RELFLAGS)

.PHONY: loader_standalone
loader_standalone: wasm
	@echo "Generating standalone loader."
	@./create_loader.sh $(MODE) standalone

.PHONY: loader_extension_embed
loader_extension_embed: wasm
	@echo "Generating extension embedded loader."
	@./create_loader.sh $(MODE) extension_embed

.PHONY: loader_extension
loader_extension: loader_extension_embed extension
	@echo "Generating extension main loader."
	@./create_loader.sh $(MODE) extension

.PHONY: extension
extension: wasm
	@echo "Building extension."
	@tar --transform 's/.*\///g' -cvf target/$(MODULE).tar \
		$(MODULE).sql \
		$(MODULE).wit \
		target/wasm32-wasi/$(MODE)/$(MODULE).wasm \
		    > /dev/null

.PHONY: clean
clean:
	@cargo clean
	@rm -f load_standalone.sql load_extension.sql $(MODULE).sql

