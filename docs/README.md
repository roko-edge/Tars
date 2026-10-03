# Documentation Index

This directory contains technical specifications, architectural designs, implementation status, and project roadmaps for TARS.

---

## Navigation by Task

| Objective | Document | Scope |
|---|---|---|
| **Target Milestone & V0 Scope** | [`specs/V0_CONTRACT.md`](specs/V0_CONTRACT.md) | Release contract: MNIST benchmark, NPU hardware, quantization, and paper requirements |
| **Team Division & Schedule** | [`roadmap/ROADMAP.md`](roadmap/ROADMAP.md) | Work allocation across Arthur, Gustavo, and Gildo; 6-week milestones and intersections |
| **Software & RTL Architecture** | [`ARCHITECTURE.md`](ARCHITECTURE.md) | Structure of the Rust package (`src/`) and SystemVerilog prototype (`npu/`) |
| **Implementation Status** | [`STATUS.md`](STATUS.md) | Current status of modules, commands, and known technical limitations |
| **Research Logs for Publication** | [`research/README.md`](research/README.md) | Empirical research logs and mapping to academic paper sections |

---

## Navigation by Domain Lead

### Arthur (NPU Microarchitecture, Hardware & Systems Integration)
- **Primary Specification:** [`specs/ARITHMETIC_Q8_24.md`](specs/ARITHMETIC_Q8_24.md) — Fixed-point arithmetic and bit-exact RTL parity protocol.
- **Software Subsystem:** [`specs/MODEL_TRAIT.md`](specs/MODEL_TRAIT.md) — `Module` trait definition and `Model<M>` intermediate representation export.
- **Source Paths:** `npu/main.sv`, `npu/tb.sv`, `src/export.rs`.

### Gustavo (Tensor Core & Software Architecture)
- **Primary Specification:** [`specs/TENSOR.md`](specs/TENSOR.md) — Strided memory layout, zero-copy views, broadcasting, and Copy-on-Write semantics.
- **Software Subsystem:** [`specs/MODEL_TRAIT.md`](specs/MODEL_TRAIT.md) — Type safety and trait bound validation for `Module`.
- **Source Paths:** `src/math/tensor.rs`, deterministic PRNG test harness.

### Gildo (Execution Engine & Backpropagation)
- **Primary Specification:** [`specs/TENSOR.md`](specs/TENSOR.md) — Matrix multiplication (`matmul`), reductions, and tensor operations.
- **Software Subsystem:** `src/optim/grad.rs` — Tensorized analytical backpropagation (`backward()`).
- **Source Paths:** `src/modules/linear.rs`, `src/optim/grad.rs`, `src/optim/bgd.rs`.

---

## Normative Specifications (`docs/specs/`)

1. [`specs/V0_CONTRACT.md`](specs/V0_CONTRACT.md): Comprehensive v0.1.0 release contract (MNIST, Tensor, NPU, Quantization, and Paper).
2. [`specs/MODEL_TRAIT.md`](specs/MODEL_TRAIT.md): `Module` trait hierarchy, `Sequential` refactoring, and `Model<M>` container.
3. [`specs/TENSOR.md`](specs/TENSOR.md): Tensor storage invariants, strides, zero-copy views, broadcasting, and Copy-on-Write.
4. [`specs/ARITHMETIC_Q8_24.md`](specs/ARITHMETIC_Q8_24.md): Fixed-point Q8.24 numeric contract and bit-exact RTL simulation alignment.
