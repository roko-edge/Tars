# SystemVerilog NPU Prototype

This directory contains a signed 32-bit dot-product module and its simulation
testbench.

## Files

| File | Purpose |
|---|---|
| `main.sv` | `npu` module parameterized by vector length `N` |
| `tb.sv` | Clock, reset, memory loading, and result display |
| `activations.mem` | Four hexadecimal activation words |
| `weights.mem` | Hexadecimal weight words |
| `Makefile` | Simulation, waveform, lint, and cleanup targets |

## Interface and arithmetic

The testbench instantiates `npu` with `N = 4`. The module processes one activation
and weight per asserted `start` clock, accumulates into a signed 32-bit register, and
adds the signed 32-bit `bias` input to the last product. The testbench sets `bias` to
`7`.

There is no fixed-point scaling, floating-point decoder, saturation, widened
accumulator, activation function, or automated expected-result check.

## Environment

From the repository root:

```bash
nix develop .#hardware
```

From this directory:

```bash
nix develop ..#hardware
```

The shell provides GNU Make, Icarus Verilog, Verilator, GTKWave, and Verible.
The Makefile automatically invokes the parent hardware shell when `iverilog` is not
already available on `PATH`.

## Commands

Run these commands from `npu/`:

```bash
make sim
make clean
```

`make sim` compiles `main.sv` and `tb.sv` with Icarus Verilog and prints the result.
It does not report pass or fail. The checked-in `weights.mem` contains two words,
while the DUT requires four, so uninitialized values may affect the result.

## Incomplete targets

| Target | Limitation |
|---|---|
| `sim2` | References absent `main2.sv` and `tb2.sv` files |
| `test` and `all` | Depend on `sim2` |
| `wave` | Expects `build/dump.vcd`, but the testbench does not generate a VCD file |
| `wave2` | Depends on the absent ternary simulation and `build/dump2.vcd` |
| `lint` | Runs a second Verilator command against absent ternary files |

## Memory representation

`$readmemh` loads both memory files directly into signed 32-bit arrays. The Rust
training executable writes IEEE-754 `f32` bit patterns to `weights.mem`, but the NPU
uses those words as signed integers. See [`../docs/MEM_FORMAT.md`](../docs/MEM_FORMAT.md)
for the exact current behavior.
