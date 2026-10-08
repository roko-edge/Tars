# Memory Files

**Audience:** Engineers working on the Rust-to-RTL boundary.

**Document type:** Current artifact-format reference. This is not a user guide for
the Rust library.

The NPU prototype uses hexadecimal text files accepted by SystemVerilog
`$readmemh`. Artifacts are generated from the Rust runtime (`src/export.rs`)
in Q8.24 fixed-point format with truncation.

The exporter and the testbench operate on the same files under `npu/data/`: a training
run overwrites the checked-in memory files consumed by simulation. The shared encoding
and paths do not constitute an automated integration path.

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

`src/export.rs` exports trained weights and biases converted through
`to_q8_24(x: f32) -> u32` as eight-digit hexadecimal words.

The SystemVerilog DUT loads these files into internal memory arrays:

```systemverilog
$readmemh("data/weights.mem", dut.w);
$readmemh("data/bias.mem", dut.b);
```

Values are loaded directly as signed 32-bit Q8.24 integers. Multiplication produces a 64-bit product (Q16.48); the processing element reduces it with an arithmetic right shift by 24 bits and accumulates the full-width reduced product in a 64-bit accumulator. The top level adds the per-output bias and saturates the sum to the signed 32-bit Q8.24 range.

## `target.mem`

`export_data()` also exports the dataset target labels in Q8.24 hex format (`0x00000000` or `0x01000000`). These are training labels, not golden inference outputs; automated bit-exact assertion against golden Q8.24 vectors remains a project target defined in [`specs/ARITHMETIC_Q8_24.md`](specs/ARITHMETIC_Q8_24.md).

## Size and Configuration

The testbench configures the top-level NPU with `IN = 2` and `OUT = 1`. The module supports sequential computation of `OUT` output neurons from `IN` input activations within a single affine layer. Multi-layer chaining and activation units are scheduled for Phase 3 via the Layer Engine FSM.

## Current Limitations

- **Layer Manifest:** Model topology, layer counts, and per-layer dimensions are not yet encapsulated in a formal manifest (`Model::compile_to_manifest()`).
- **Rounding Mode:** Rust and RTL both use simple truncation rather than round-half-to-even.
- **Automated Assertions:** `npu/tests/tb.sv` logs outputs and dumps waveforms but does not yet assert bit-exact equality against golden vectors with non-zero exit codes.
- **Per-Product Reduction:** The processing element accumulates the full-width arithmetic-shifted product rather than the 32-bit slice of the arithmetic contract ([`specs/ARITHMETIC_Q8_24.md`](specs/ARITHMETIC_Q8_24.md), Section 3.1); the deviation is recorded there.
