HAVE_IVERILOG := $(shell command -v iverilog 2> /dev/null)

ifeq ($(HAVE_IVERILOG),)
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
	$(RUN) iverilog -g2012 -o $(BUILD_DIR)/sim_binary.out $(SRC) $(TB)
	$(RUN) vvp $(BUILD_DIR)/sim_binary.out

test: sim

wave: sim
	$(RUN) gtkwave $(BUILD_DIR)/dump.vcd

lint:
	$(RUN) verilator --lint-only -Wall -Wno-DECLFILENAME -Wno-TIMESCALEMOD $(SRC) $(TB)

clean:
	rm -rf $(BUILD_DIR) *.vvp
