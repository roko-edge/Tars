# Implementation Status

This document reflects the current state of the repository as of October 2026.

---

## Rust Package

| Component | Status | Implementation Details |
|---|---|---|
| Sequential Model | Implemented | `Sequential` stores `Vec<AnyModule>`; migration to `Module` trait scheduled in `docs/roadmap/ROADMAP.md` |
| Dense Layer | Implemented | `Linear` uses `Vec<Vec<f32>>` weights and `Vec<f32>` biases |
| Activations | Implemented | ReLU and Sigmoid modules |
| Tensor Core | In Progress | `Tensor` struct in `src/math/tensor.rs` with `Arc` storage, shapes, strides, and `is_contiguous()` check |
| Loss Function | Implemented | Mean Squared Error (MSE) across outputs and dataset samples |
| Analytical Backpropagation | Implemented | Full analytical backprop in `src/optim/grad.rs` (`backward()`) |
| Finite-Difference Gradient | Implemented | Centered finite differences in `src/optim/grad.rs` (`num_grad()`), retained for verification |
| Optimizer | Implemented | Batch Gradient Descent (BGD) applying computed gradients |
| Experiments | Implemented | Built-in OR and XOR experiment factories |
| Fixed-Point Export | Implemented | `export_model()` and `export_data()` convert values to Q8.24 fixed-point hex files (`weights.mem`, `bias.mem`, `activations.mem`, `target.mem`) |
| Automated Tests | Not Implemented | Unit tests scheduled for introduction alongside deterministic PRNG harness |

The Rust implementation uses `f32`, `std`, and dynamically allocated vectors. Analytical backpropagation is present, but operates on nested vectors rather than contiguous tensors.

---

## Visualization

The package exports a `view` module backed by `petgraph`. `NetGraph::from_sequential` extracts graph topology directly from a trained `Sequential` model. The visualization binary saves Graphviz DOT files and generates PNG diagrams. `plotters` and `petgraph` are required package dependencies.

---

## SystemVerilog NPU Prototype

| Component | Status | Implementation Details |
|---|---|---|
| Dot Product | Implemented | Signed 32-bit product and accumulation operating on Q8.24 fixed-point words (`raw_mult[55:24]`) |
| Vector Length | Parameterized | Parameter `N` (instantiated as `N=2` in testbench) |
| Internal Storage | Implemented | Internal `bias[1]`, `weights[N]`, and `act[N]` arrays loaded via `$readmemh` |
| Control Logic | Implemented | `clk`, `rst`, `start`, and `done` handshaking signals |
| Testbench | Partial | Instantiates DUT and prints results; lacks self-checking assertions and VCD dump calls |

---

## Command Status

| Command | Status | Notes |
|---|---|---|
| `cargo build` | Available | Compiles the Rust package and binaries |
| `cargo run --bin main` | Available | Trains configured model and exports Q8.24 memory files to `npu/` |
| `cargo run --bin visualization` | Available | Requires Graphviz; generates graph DOT and PNG artifacts |
| `cargo test` | Available | Currently reports 0 tests; test suite introduction in progress |
| `make sim` (from `npu/`) | Partial | Compiles `main.sv` and `tb.sv` using Icarus Verilog and prints output; lacks pass/fail checks |
| `make sim2` | Broken | References absent `main2.sv` and `tb2.sv` files |
| `make test` / `make all` | Broken | Depends on `sim2` |
| `make wave` / `make wave2` | Broken | References absent VCD dump files |
| `make lint` | Broken | Invocations reference absent ternary sources |
| `make clean` | Available | Cleans NPU build outputs |

---

## Known Technical Limitations

1. **Testbench Self-Checking:** The NPU testbench (`tb.sv`) prints result signals but does not assert equality against expected values or return a non-zero exit code on failure.
2. **Q8.24 Rounding Strategy:** Rust export (`src/export.rs`) and SystemVerilog arithmetic (`npu/main.sv`) perform truncation (`as i32` / bit slicing) rather than round-half-to-even.
3. **NPU Accumulator Width:** The NPU accumulator is 32 bits without saturation logic, susceptible to overflow on large vector sizes.
4. **Vector Tensor Storage:** Linear layers and gradients currently allocate nested vectors (`Vec<Vec<f32>>`), requiring migration to contiguous `Tensor` storage.
