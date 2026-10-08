# Implementation Status

**Document type:** Project status record. User instructions are maintained in
[`docs/`](../README.md).

This document reflects the current state of the repository as of October 2026.

---

## Rust Package

| Component | Status | Implementation Details |
|---|---|---|
| Sequential Model | Implemented | `Sequential` stores `Vec<AnyModule>`; the tensor-based replacement is specified under `docs/engineering/` |
| Dense Layer | Implemented | `Linear` uses `Vec<Vec<f32>>` weights and `Vec<f32>` biases |
| Activations | Implemented | ReLU and Sigmoid modules |
| Tensor Core | In Progress | `Tensor` struct in `src/math/tensor.rs` with `Arc` storage, shapes, strides, contiguity check, zero-copy `transpose` variants, element-wise `add`/`sub`, scalar multiply/divide with in-place variants, `zeros`, `random_uniform`, and `dot_product`; element-wise kernels iterate raw storage and do not respect arbitrary strides; `DType` abstraction and ternary mixed-precision kernels are planned under Gustavo's domain |
| Loss Function | Implemented | Mean Squared Error (MSE) across outputs and dataset samples |
| Analytical Backpropagation | Implemented | Full analytical backprop in `src/optim/grad.rs` (`backward()`) |
| Finite-Difference Gradient | Implemented | Centered finite differences in `src/optim/grad.rs` (`num_grad()`), retained for verification |
| Optimizer | Implemented | Batch Gradient Descent (BGD) applying computed gradients |
| Experiments | Implemented | Built-in OR and XOR experiment factories |
| Fixed-Point Export | Implemented | `export_model()` and `export_data()` convert values to Q8.24 fixed-point hex files under `npu/data/` (`weights.mem`, `bias.mem`, `activations.mem`, `target.mem`) |
| Automated Tests | Broken | Tensor tests exist but do not compile against the current `Tensor` interface; `cargo check --tests` reports ten interface errors |

The Rust implementation uses `f32`, `std`, and dynamically allocated vectors. Analytical backpropagation is present, but operates on nested vectors rather than contiguous tensors.

---

## Visualization

The package exports a `view` module backed by `petgraph`. `NetGraph::network_to_graph` extracts graph topology directly from a `Sequential` model and input slice. The visualization binary saves Graphviz DOT files (`graph_to_dot` / `save_dot`) and generates PNG diagrams via `plot()`. `plotters` and `petgraph` are required package dependencies.

---

## SystemVerilog NPU Prototype

| Component | Status | Implementation Details |
|---|---|---|
| Processing Element | Implemented | `pe.sv` in `npu/src/` multiplies signed 32-bit Q8.24 values and accumulates reduced products in 64 bits |
| NPU Top-Level | Implemented | `npu.sv` in `npu/src/` implements the single-layer affine engine: `IDLE`/`BUSY` FSM with `start`/`done`, sequential computation of `OUT` outputs from `IN` activations, per-output bias, and final signed 64-to-32-bit saturation |
| Layer Dimensions | Parameterized | Parameters `IN` and `OUT` (testbench instantiates `IN=2`, `OUT=1`) |
| Internal Storage | Implemented | Internal `b[OUT]`, `w[IN*OUT]`, and `a[IN]` arrays loaded via `$readmemh` |
| Control Logic | Implemented | `clk`, `rst`, `start`, and `done` handshaking signals |
| Testbench | Partial | `tests/tb.sv` instantiates DUT, enables `$dumpfile`/`$dumpvars`, but lacks automated pass/fail assertions and non-zero exit codes |

---

## Command Status

| Command | Status | Notes |
|---|---|---|
| `cargo build` | Available | Compiles the Rust package and binaries |
| `cargo run --bin main` | Available | Trains the source-selected OR model and exports four Q8.24 files under `npu/data/`, overwriting the files read by the NPU testbench |
| `cargo run --bin visualization` | Available | Requires Graphviz; generates graph DOT and PNG artifacts |
| `cargo test` | Broken | Existing tensor tests do not compile against the current implementation |
| `make sim` (from `npu/`) | Partial | Compiles `src/pe.sv`, `src/npu.sv`, and `tests/tb.sv` using Icarus Verilog and prints output; lacks pass/fail checks |
| `make test` / `make all` | Partial | Aliases `sim` |
| `make wave` | Available | Runs GTKWave against `build/dump.vcd` generated from simulation |
| `make lint` | Partial | Runs Verilator lint-only checks on `src/` and `tests/` sources; currently exits non-zero with four index-width warnings (`WIDTHTRUNC`/`WIDTHEXPAND`) in `npu/src/npu.sv` |
| `make clean` | Available | Cleans NPU build outputs |

---

## Known Technical Limitations

1. **Testbench Self-Checking:** The NPU testbench (`tests/tb.sv`) prints result signals but does not assert equality against expected values or return a non-zero exit code on failure.
2. **Q8.24 Rounding Strategy:** Rust export (`src/export.rs`) and SystemVerilog arithmetic (`npu/src/npu.sv`, `npu/src/pe.sv`) perform truncation (`as i32` / bit slicing) rather than round-half-to-even.
3. **NPU RTL Lint:** Verilator reports four index-width warnings on the `i` and `j` index expressions in `npu/src/npu.sv`, so `make lint` exits non-zero.
4. **Vector Tensor Storage:** Linear layers and gradients currently allocate nested vectors (`Vec<Vec<f32>>`), requiring migration to contiguous `Tensor` storage.
5. **Artifact Integration:** Rust export and the testbench share `npu/data/`; a training run overwrites the checked-in memory files. Exports still omit activation functions and topology, and `target.mem` contains dataset labels rather than golden Q8.24 inference outputs.
