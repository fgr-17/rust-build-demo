-include ../.env

FLATC = flatc

FLATB_FILE = $(MODELS_DIR)/flatbdata.fbs

DATA_JSON_FILES = $(wildcard $(DATA_INPUT)/*.json)
DATA_BIN_FILES = $(patsubst $(DATA_INPUT)/%.json,$(DATA_OUTPUT)/%.bin,$(DATA_JSON_FILES))

FLATB_GENERATED = $(DATA_SOURCES_OUTPUT)/flatb_generated.rs

all: gensrc genbin

gensrc: $(FLATB_GENERATED)

$(FLATB_GENERATED): $(FLATB_FILE)
	@echo "=== Generating FlatBuffers Rust source ==="
	@echo "Schema: $(FLATB_FILE)"
	@echo "Output: $(DATA_SOURCES_OUTPUT)"
	@mkdir -p $(DATA_SOURCES_OUTPUT)
	$(FLATC) --rust \
		--gen-object-api \
		--gen-mutable \
		-o $(DATA_SOURCES_OUTPUT) \
		$(FLATB_FILE)
	@echo "Generated: $(FLATB_GENERATED)"

genbin: $(DATA_BIN_FILES)

$(DATA_OUTPUT)/%.bin: $(DATA_INPUT)/%.json $(FLATB_FILE)
	@echo "=== Generating binary: $@ ==="
	@mkdir -p $(DATA_OUTPUT)
	$(FLATC) --binary \
		--force-defaults \
		-o $(DATA_OUTPUT) \
		$(FLATB_FILE) \
		$<
	@echo "Generated: $@"

cleanfb:
	rm -f $(FLATB_GENERATED)
	rm -f $(DATA_BIN_FILES)
