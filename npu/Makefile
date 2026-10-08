HAVE_VERILATOR := $(shell command -v verilator 2> /dev/null)

ifeq ($(HAVE_VERILATOR),)
  RUN = nix develop ..\#hardware -c
else
  RUN =
endif

BUILD_DIR = build
SRC = src/pe.sv src/npu.sv
TB  = tests/tb.sv

.PHONY: all sim test wave lint clean

all: test

$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

sim: $(BUILD_DIR)
	@$(RUN) verilator --binary --timing --trace \
		--quiet --quiet-build --quiet-stats \
		-Wno-fatal -Wno-WIDTHTRUNC -Wno-WIDTHEXPAND -Wno-TIMESCALEMOD \
		$(SRC) $(TB) --top-module tb \
		-o npu_sim --Mdir $(BUILD_DIR)
	@$(RUN) ./$(BUILD_DIR)/npu_sim +verilator+quiet

test: sim

wave: sim
	@$(RUN) gtkwave $(BUILD_DIR)/dump.vcd

lint:
	@$(RUN) verilator --lint-only -Wall -Wno-DECLFILENAME -Wno-TIMESCALEMOD $(SRC) $(TB)

clean:
	@rm -rf $(BUILD_DIR)
