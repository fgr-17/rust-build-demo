-include ../.env

PROTOC = protoc
FLATC = flatc

PROTO_FILE = $(MODELS_DIR)/sensor.proto
CONFIG_TXT = $(CONFIG_DIR)/sensor_config.pbtxt
CONFIG_BIN = $(CONFIG_OUTPUT)/sensor_config.pb
DATA_BIN = $(CONFIG_OUTPUT)/data_example.fb

all: $(CONFIG_BIN)

$(CONFIG_BIN): $(PROTO_FILE) $(CONFIG_TXT)
	$(info MODELS_DIR is: $(MODELS_DIR))
	$(info CONFIG_DIR is: $(CONFIG_DIR))
	$(info CONFIG_OUTPUT is: $(CONFIG_OUTPUT))
	@mkdir -p $(CONFIG_OUTPUT)
	$(PROTOC) 	--proto_path=$(MODELS_DIR) \
				--encode=sensor.SensorData \
				$(PROTO_FILE) < $(CONFIG_TXT) > $(CONFIG_BIN)
	@echo "Generated binary config: $(CONFIG_BIN)"

cleanpb:
	rm -rf $(CONFIG_OUTPUT)
