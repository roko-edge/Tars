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
| Tensor Core | In Progress | `Tensor` struct in `src/math/tensor.rs` with `Arc` storage, shapes, strides, and `is_contiguous()` check |
| Loss Function | Implemented | Mean Squared Error (MSE) across outputs and dataset samples |
| Analytical Backpropagation | Implemented | Full analytical backprop in `src/optim/grad.rs` (`backward()`) |
| Finite-Difference Gradient | Implemented | Centered finite differences in `src/optim/grad.rs` (`num_grad()`), retained for verification |
| Optimizer | Implemented | Batch Gradient Descent (BGD) applying computed gradients |
| Experiments | Implemented | Built-in OR and XOR experiment factories |
| Fixed-Point Export | Implemented | `export_model()` and `export_data()` convert values to Q8.24 fixed-point hex files (`weights.mem`, `bias.mem`, `activations.mem`, `target.mem`) |
| Automated Tests | Broken | Tensor tests exist but do not compile against the current `Tensor` interface |

The Rust implementation uses `f32`, `std`, and dynamically allocated vectors. Analytical backpropagation is present, but operates on nested vectors rather than contiguous tensors.

---

## Visualization

The package exports a `view` module backed by `petgraph`. `NetGraph::network_to_graph` extracts graph topology directly from a `Sequential` model and input slice. The visualization binary saves Graphviz DOT files (`graph_to_dot` / `save_dot`) and generates PNG diagrams via `plot()`. `plotters` and `petgraph` are required package dependencies.

---

## SystemVerilog NPU Prototype

| Component | Status | Implementation Details |
|---|---|---|
| Processing Element | Implemented | `pe.sv` in `npu/src/` multiplies signed 32-bit Q8.24 values and accumulates reduced products in 64 bits |
| NPU Top-Level | In Progress | `npu.sv` in `npu/src/` implements FSM control (`start`/`done`) and instantiates `pe` |
| Vector Length | Parameterized | Parameter `N` (instantiated as `N=2` in testbench) |
| Internal Storage | Implemented | Internal `bias[1]`, `weights[N]`, and `act[N]` arrays loaded via `$readmemh` |
| Control Logic | Implemented | `clk`, `rst`, `start`, and `done` handshaking signals |
| Testbench | Partial | `tests/tb.sv` instantiates DUT, enables `$dumpfile`/`$dumpvars`, but lacks automated pass/fail assertions and non-zero exit codes |

---

## Command Status

| Command | Status | Notes |
|---|---|---|
| `cargo build` | Available | Compiles the Rust package and binaries |
| `cargo run --bin main` | Available | Trains the source-selected OR model and exports four Q8.24 files under `npu/` |
| `cargo run --bin visualization` | Available | Requires Graphviz; generates graph DOT and PNG artifacts |
| `cargo test` | Broken | Existing tensor tests do not compile against the current implementation |
| `make sim` (from `npu/`) | Partial | Compiles `src/pe.sv`, `src/npu.sv`, and `tests/tb.sv` using Icarus Verilog and prints output; lacks pass/fail checks |
| `make test` / `make all` | Partial | Aliases `sim` |
| `make wave` | Available | Runs GTKWave against `build/dump.vcd` generated from simulation |
| `make lint` | Available | Runs Verilator lint-only checks on `src/` and `tests/` sources |
| `make clean` | Available | Cleans NPU build outputs |

---

## Known Technical Limitations

1. **Testbench Self-Checking:** The NPU testbench (`tests/tb.sv`) prints result signals but does not assert equality against expected values or return a non-zero exit code on failure.
2. **Q8.24 Rounding Strategy:** Rust export (`src/export.rs`) and SystemVerilog arithmetic (`npu/src/npu.sv`, `npu/src/pe.sv`) perform truncation (`as i32` / bit slicing) rather than round-half-to-even.
3. **NPU Result Saturation:** The processing element accumulates in 64 bits, but the top-level 32-bit result has no explicit signed saturation.
4. **Vector Tensor Storage:** Linear layers and gradients currently allocate nested vectors (`Vec<Vec<f32>>`), requiring migration to contiguous `Tensor` storage.
5. **Artifact Integration:** Rust exports to `npu/`, while the testbench consumes checked-in files under `npu/data/`.
