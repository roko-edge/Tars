# Memory Files

The NPU prototype uses hexadecimal text files accepted by SystemVerilog
`$readmemh`. Artifacts are generated from the Rust runtime (`src/export.rs`)
in Q8.24 fixed-point format with truncation.

## `activations.mem`

`npu/tests/tb.sv` loads input activations into an array for batch evaluation:

```systemverilog
$readmemh("data/activations.mem", data);
```

Each 32-bit hexadecimal word represents a Q8.24 fixed-point value ($1.0 \times 2^{24} = \text{0x01000000}$). For a 2-input network evaluating the 4 XOR/OR cases, the file contains 8 words (4 pairs of $x_1, x_2$):

```text
00000000
00000000
00000000
01000000
01000000
00000000
01000000
01000000
```

The NPU processes these entries as signed 32-bit Q8.24 words.

## `weights.mem` and `bias.mem`

`src/export.rs` exports trained weights and biases converted via `to_q8_24(x: f32) -> u32`:

```rust
writeln!(file_weights, "{:08X}", to_q8_24(*weight))?;
writeln!(file_bias, "{:08X}", to_q8_24(linear.bias[o]))?;
```

The SystemVerilog DUT loads these files into internal memory arrays:

```systemverilog
$readmemh("data/weights.mem", dut.w);
$readmemh("data/bias.mem", dut.b);
```

Values are loaded directly as signed 32-bit Q8.24 integers. Multiplication produces a 64-bit product (Q16.48), and bit slicing `[55:24]` returns the truncated Q8.24 word to the accumulator.

## `target.mem`

`export_data()` also exports expected target labels in Q8.24 hex format (`0x00000000` or `0x01000000`), intended for automated assertion testing against NPU predictions.

## Size and Configuration

The testbench configures the top-level NPU with parameter $N = 2$, matching a single neuron processing 2 input activations (such as the OR model or a single neuron in an XOR layer). Multi-neuron and multi-cycle layer execution is scheduled for Phase 3 via the Layer Engine FSM.

## Current Limitations

- **Layer Manifest:** Model topology, layer counts, and per-layer dimensions are not yet encapsulated in a formal manifest (`Model::compile_to_manifest()`).
- **Rounding Mode:** Rust and RTL both use simple truncation rather than round-half-to-even.
- **Automated Assertions:** `npu/tests/tb.sv` logs outputs and dumps waveforms but does not yet assert bit-exact equality against `target.mem` with non-zero exit codes.
- **Accumulator Saturation:** 64-bit saturation logic on the NPU accumulator is scheduled for Phase 2 implementation.
