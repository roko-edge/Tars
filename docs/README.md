# Documentation Index

This directory contains technical specifications, architectural designs, implementation status, and project roadmaps for TARS.

---

## Navigation by Task

| Objective | Document | Scope |
|---|---|---|
| **Target Milestone & V0 Scope** | [`specs/V0_CONTRACT.md`](specs/V0_CONTRACT.md) | Release contract: MNIST benchmark, NPU hardware, quantization, and paper requirements |
| **Team Division & Milestones** | [`roadmap/ROADMAP.md`](roadmap/ROADMAP.md) | Work allocation across Arthur, Gustavo, and Gildo; sequential phase milestones and intersections |
| **Software & RTL Architecture** | [`ARCHITECTURE.md`](ARCHITECTURE.md) | Structure of the Rust package (`src/`) and SystemVerilog prototype (`npu/`) |
| **Implementation Status** | [`STATUS.md`](STATUS.md) | Current status of modules, commands, and known technical limitations |
| **Research Logs for Publication** | [`research/README.md`](research/README.md) | Empirical research logs and mapping to academic paper sections |

---

## Navigation by Domain Lead

### Arthur (NPU Microarchitecture, Hardware Tests & Systems Integration)
- **Primary Specification:** [`specs/ARITHMETIC_Q8_24.md`](specs/ARITHMETIC_Q8_24.md) — Fixed-point arithmetic and bit-exact RTL parity protocol.
- **Software Subsystem:** [`specs/MODEL_TRAIT.md`](specs/MODEL_TRAIT.md) — `Module` trait definition and `Model<M>` intermediate representation export.
- **Hardware & Test Paths:** `npu/src/npu.sv`, `npu/src/pe.sv`, `npu/tests/tb.sv`, `src/export.rs`.
- **Hardware Metrics:** Supplies raw cycle latency, resource utilization, and simulation parity outputs for benchmark aggregation.

### Gustavo (Stochasticity, Numerical Controls, Rust Testing, Benchmarks & Analytics)
- **Primary Specifications:** [`specs/TENSOR.md`](specs/TENSOR.md) — Mathematical storage invariants and broadcasting contracts; [`research/README.md`](research/README.md) — Empirical research methodology.
- **Software Subsystems:** Deterministic PRNG harness (explicit seed), random weight initializations (Xavier/Glorot, He/Kaiming, ternary), numerical gradient validation (`num_grad` vs `backward()` with $\epsilon \le 10^{-4}$), quantization error analysis ($f32 \to \text{Q8.24}$), dataset loaders (MNIST), and automated Rust test suite (`cargo test`).
- **Benchmarks & Analytics:** Lead custodian of benchmark harnesses, metric computation (accuracy, loss curves, gradient norms), hardware/software comparative analysis, and research logs for academic publication.
- **Source Paths:** `src/math/tensor.rs`, `src/tests/`, `docs/research/`.

### Gildo (Execution Engine, Tensorized Backpropagation & NPU RTL Reviewer)
- **Primary Specification:** [`specs/TENSOR.md`](specs/TENSOR.md) — Strided tensor kernels, matrix multiplication (`matmul`), reductions, and execution efficiency.
- **Software Subsystem:** `src/optim/grad.rs`, `src/modules/linear.rs`, `src/optim/bgd.rs` — Tensorized analytical backpropagation (`backward()`), elimination of `Vec<Vec<f32>>`, and contiguous tensor storage.
- **Hardware Review Role:** Official peer reviewer for SystemVerilog NPU RTL (`npu/src/npu.sv`, `npu/src/pe.sv`, `npu/tests/tb.sv`), auditing FSM correctness, 64-bit saturating accumulation, and simulation parity.
- **Source Paths:** `src/modules/linear.rs`, `src/optim/grad.rs`, `src/optim/bgd.rs`, `npu/src/`, `npu/tests/`.

---

## Normative Specifications (`docs/specs/`)

1. [`specs/V0_CONTRACT.md`](specs/V0_CONTRACT.md): Comprehensive v0.1.0 release contract (MNIST, Tensor, NPU, Quantization, and Paper).
2. [`specs/MODEL_TRAIT.md`](specs/MODEL_TRAIT.md): `Module` trait hierarchy, `Sequential` refactoring, and `Model<M>` container.
3. [`specs/TENSOR.md`](specs/TENSOR.md): Tensor storage invariants, strides, zero-copy views, broadcasting, and Copy-on-Write.
4. [`specs/ARITHMETIC_Q8_24.md`](specs/ARITHMETIC_Q8_24.md): Fixed-point Q8.24 numeric contract and bit-exact RTL simulation alignment.
