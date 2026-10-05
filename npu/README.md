# SystemVerilog NPU Prototype

This directory contains a synthesizable SystemVerilog neural processing unit (NPU)
prototype and its simulation testbench.

## Files

| File / Directory | Purpose |
|---|---|
| `src/npu.sv` | Top-level `npu` module with FSM control (`IDLE`, `BUSY`) parameterized by length `N` |
| `src/pe.sv` | Processing Element (PE) performing multiply-accumulate with 64-bit accumulator in Q8.24 |
| `tests/tb.sv` | Testbench with clock generation, reset, memory loading, waveform dumping, and display |
| `data/activations.mem` | Q8.24 hexadecimal activation words (8 entries for 4 evaluation pairs) |
| `data/weights.mem` | Q8.24 hexadecimal weight words |
| `data/bias.mem` | Q8.24 hexadecimal bias word |
| `data/target.mem` | Q8.24 hexadecimal target outputs |
| `Makefile` | Simulation (`sim`), waveform inspection (`wave`), linting (`lint`), and cleanup |

## Interface and Arithmetic

The testbench instantiates `npu` with parameter `N = 2`. The top-level module coordinates with `pe.sv` using an `init` and `en` handshake over the vector elements. The processing element computes 64-bit products ($Q16.48$) and extracts bits `[55:24]` to accumulate in Q8.24 format. On vector completion, the stored bias (`b[0]`) is added to produce the final `result`.

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
make sim     # Compiles and runs simulation using Icarus Verilog
make wave    # Opens GTKWave with build/dump.vcd
make lint    # Runs Verilator lint-only checks on src/ and tests/
make clean   # Removes build/ artifacts and vvp binaries
```

`make sim` compiles `src/pe.sv`, `src/npu.sv`, and `tests/tb.sv` with Icarus Verilog and prints output. Waveforms are dumped to `build/dump.vcd`. Automated golden assertion checks with non-zero exit codes are scheduled for Phase 4.

## Memory Representation

`$readmemh` loads memory files directly into signed 32-bit Q8.24 arrays. The Rust export module (`src/export.rs`) writes matching Q8.24 hexadecimal values. See [`../docs/MEM_FORMAT.md`](../docs/MEM_FORMAT.md) for full format details.
