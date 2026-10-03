HAVE_IVERILOG := $(shell command -v iverilog 2> /dev/null)

ifeq ($(HAVE_IVERILOG),)
  RUN = nix develop ..\#hardware -c
else
  RUN =
endif

BUILD_DIR = build

.PHONY: all sim test wave lint clean

all: test
$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

sim: $(BUILD_DIR)
	$(RUN) iverilog -g2012 -o $(BUILD_DIR)/sim_binary.out main.sv tb.sv
	$(RUN) vvp $(BUILD_DIR)/sim_binary.out

test: sim

wave: sim
	$(RUN) gtkwave $(BUILD_DIR)/dump.vcd

lint:
	$(RUN) verilator --lint-only -Wall -Wno-DECLFILENAME -Wno-TIMESCALEMOD main.sv tb.sv

clean:
	rm -rf $(BUILD_DIR) *.vvp
