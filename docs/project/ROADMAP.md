# Project Roadmap and Engineering Organization

**Document type:** Internal project plan. Planned work is not part of the current
public API unless separately documented under [`docs/`](../README.md).

This document establishes the technical milestones, cross-functional responsibilities,
and integration contracts for TARS development.

---

## 1. System Vision and Boundaries

TARS unifies an experimental deep learning engine written in Rust with a synthesizable
hardware neural processing unit (NPU) specified in SystemVerilog.

```text
┌─────────────────────────────────────────────────────────────┐
│                        RUST RUNTIME                         │
│                                                             │
│   Model<M> ──► Module Composition ──► Tensor Engine         │
│        │                                     ▲              │
│        ▼ (Train)                             │ (Verify)     │
│   Analytic Backprop ──► Optimizer (BGD) ─────┘              │
│        │                                                    │
│        ▼ (Export)                                           │
│   Hardware Manifest + Q8.24 Memory Artifacts (.mem)         │
└──────────────────────────────┬──────────────────────────────┘
                               │ File / IPC Boundary
┌──────────────────────────────▼──────────────────────────────┐
│                    SYSTEMVERILOG NPU (DUT)                  │
│                                                             │
│   Memory Controller ──► Layer Engine FSM ──► MAC Array       │
│                                                   │         │
│   Self-Checking Testbench (Bit-Exact Parity) ◄────┘         │
└─────────────────────────────────────────────────────────────┘
```

The system requires strict parity between software inference and RTL simulation.
To achieve this without blocking parallel velocity, work is distributed across three
core engineers with explicit cross-domain intersections.

---

## 2. Engineering Matrix and Cross-Functional Intersections

Work is structured around five primary intersections where each domain features an
**Owner** (responsible for design and delivery) and a **Peer Reviewer** (responsible
for contract sign-off and verification).

```text
                [ GUSTAVO ]
      Stochasticity, Numerics, Testing & Analytics
                  /         \
   Intersection 1 /           \ Intersection 5
 (Network Arch.)/             \(Stochastics & Benchmarks)
               /               \
        [ GILDO ] ─────────── [ ARTHUR ]
 Engine, Backprop &           NPU, Hardware Tests &
 NPU RTL Reviewer             Systems Integration
        \                       /
   Intersection 2 \           / Intersection 4
  (Tensor Core)  (Owner:     (NPU & Parity)
                  Gildo)    /
               \           /
            Intersection 3
        (Compute & Backprop)
```

### Intersections Specification

| Intersection | Subsystem | Lead (Owner) | Peer Reviewer | Joint Scope & Deliverables |
| --- | --- | --- | --- | --- |
| **1** | **Network Architecture (`Model` vs `Sequential`)** | Arthur | Gustavo | `Module` trait definition, parameter tracking, topologically ordered IR extraction, and deprecation of monolithic `Sequential`. |
| **2** | **Tensor Core & Mathematical Foundations** | Gildo | Gustavo | Strided memory layout, zero-copy views (`strides`, `offset`), multidimensional broadcasting rules, and copy-on-write semantics, verified against `docs/engineering/specs/TENSOR.md`. |
| **3** | **Compute Engine & Tensorized Backprop** | Gildo | Arthur | Batched tensor arithmetic (`matmul`, element-wise), elimination of `Vec<Vec<f32>>`, tensorized analytical backprop, and memory export mapping. |
| **4** | **NPU Co-Processor & Parity Co-Simulation** | Arthur | Gildo | Multi-neuron Layer Engine FSM, 64-bit saturating accumulator, self-checking testbench (`tb.sv`), and golden-model bit-exact test suite. |
| **5** | **Stochasticity, Numerical Controls & Benchmark Analytics** | Gustavo | Gildo | Deterministic PRNG harness, random weight initialization algorithms (Xavier/Glorot, He/Kaiming, ternary), Rust test suite, dataset loading, metric computation, and empirical research analysis. |

### Domain Responsibility Summary

- **Arthur** owns NPU microarchitecture (`npu.sv`, `pe.sv`), hardware testing (`npu/tests/tb.sv`), Verilator linting, VCD tracing, `Module` trait design, `Model<M>` container, and Q8.24 export tooling (`src/export.rs`). He supplies raw hardware metrics (cycle latency, resource utilization, parity outcomes) to Gustavo for benchmark aggregation.
- **Gustavo** owns stochastic weight generation (seeded PRNG, Xavier/He, ternary sampling), numerical controls (gradient validation with $\epsilon \le 10^{-4}$, Q8.24 conversion error analysis, saturation checks), the automated Rust test suite (`src/tests/`, `cargo test`), dataset loaders (MNIST), and all benchmark harnesses, metric computation, and empirical analyses published to `docs/project/research/`.
- **Gildo** owns the tensorized execution engine (`matmul`, element-wise kernels, axis reductions), `Linear` layer migration to contiguous `Tensor`, tensorized `backward()`, and serves as official RTL reviewer for the NPU (`npu.sv`, `pe.sv`, `tb.sv`), auditing FSM correctness, saturation logic, and parity co-simulation. He supplies engine profiling data to Gustavo for benchmark aggregation.

---

## 3. Milestone Execution Plan (Sequential Phases)

Phases are strictly ordered by dependency, not by calendar. Each phase is completed when its gate condition is met.

### Phase 1: Contracts and Deterministic Baselines

- **Arthur:**
  x Sanitize `npu/Makefile`: eliminate broken simulation/ternary targets, wire up automated Verilator linting (`make lint`).
  x Upgrade `npu/tests/tb.sv` to support `$dumpfile` and `$dumpvars` for GTKWave tracing.
  - Author normative specifications: `docs/engineering/specs/MODEL_TRAIT.md` and `docs/engineering/specs/ARITHMETIC_Q8_24.md`.

- **Gustavo:**
  x Author normative specification: `docs/engineering/specs/TENSOR.md`.
  x Implement deterministic pseudo-random number generator (PRNG) with explicit seed support (replacing unseeded uniform randoms).
  x Implement baseline `cargo test` harness in `src/tests/` for automated Rust regression testing.
  - Construct baseline characterization tests: capture loss trajectories and canonical weights for OR and XOR models using current engine.

- **Gildo:**
  - Review `docs/engineering/specs/TENSOR.md` for algorithm suitability.
  - Implement tensor element-wise arithmetic kernels (`add`, `sub`, `mul`, `div`) respecting strided storage.
  - Implement memory reduction operations (`sum`, `mean`) along arbitrary axes.
  - Perform initial peer review of NPU RTL files (`npu/src/npu.sv`, `npu/src/pe.sv`, `npu/tests/tb.sv`, `npu/Makefile`).

**Phase 1 Gate:** All specification documents merged into `main`. Canonical XOR loss curves locked in regression fixtures via `cargo test`. NPU testbench compiles and traces cleanly under Icarus Verilog and passes initial RTL review.

---

### Phase 2: Core Abstractions and Hardware Upgrades

- **Gustavo:**
  . Implement weight initialization algorithms with deterministic seed suport:
  x Xavier.
  - Kaiming.

  - Build comprehensive unit test suite in `src/tests/` for tensor storage, shapes, strides, and dimension assertions.
  - Design benchmark harness for tracking loss curves, training convergence rates, and memory allocations.
  - Implement dataset loading and batching pipeline for MNIST.
- **Gildo:**
  - Deliver zero-copy tensor views (`slice`, `transpose`, `reshape`, `permute`) via strided index projection.
  - Implement NumPy-compliant multidimensional broadcasting resolution and safe copy-on-write (COW) mutation semantics (`Arc::make_mut`).
  - Implement cache-friendly row-major matrix multiplication (`matmul`) with dimension assertions.
  - Review Arthur's NPU microarchitecture updates: audit FSM transitions, 64-bit saturating accumulator arithmetic, and signed ReLU activation logic.
- **Arthur:**
  - Refactor `Sequential` to implement the new `Module` trait.
  - Introduce `Model<M>` container owning parameter lifetime management.
  - Upgrade NPU microarchitecture: expand accumulator to 64 bits with saturation logic; integrate signed ReLU activation unit in RTL (`npu/src/pe.sv` and `npu/src/npu.sv`).
  - Maintain hardware testbench in `npu/tests/tb.sv` and supply raw cycle count metrics to Gustavo.

**Phase 2 Gate:** Matrix multiplication and tensor views pass automated unit tests (`cargo test`). NPU RTL compiles cleanly under Verilator without warnings and passes Gildo's peer review.

---

### Phase 3: Engine Convergence and Tensorized Backpropagation

- **Gildo (Lead) & Arthur (Pair):**
  - Migrate `Linear` layer to store weights and biases as contiguous `Tensor` instances.
  - Re-engineer `backward()`: perform full backward pass using batched matrix multiplications, eliminating `Vec<Vec<f32>>` allocations.
  - Eliminate redundant model cloning in gradient structures (`Grad`).
  - Review NPU multi-cycle Layer Engine FSM implementation.
- **Gustavo:**
  - Execute numerical gradient validation: verify analytical tensor backprop against finite differences (`num_grad`) with tolerance $\epsilon \le 10^{-4}$.
  - Conduct quantitative analysis of quantization error ($f32 \to \text{Q8.24}$) and saturation effects.
  - Run benchmark suite on XOR and multi-layer configurations; log performance metrics, convergence curves, and memory profiles into `docs/project/research/`.
- **Arthur:**
  - Design multi-cycle Layer Engine FSM in `npu/src/npu.sv` capable of computing $M$ output neurons from $N$ input activations sequentially.
  - Implement `Model::compile_to_manifest()` exporting layer descriptors and Q8.24 weights.
  - Maintain hardware simulation suite and deliver cycle timing data to Gustavo.

**Phase 3 Gate:** Training loop converges on XOR using pure `Tensor` backend. Numerical gradient check passes ($\epsilon \le 10^{-4}$). NPU Layer Engine FSM passes simulation and Gildo's RTL review.

---

### Phase 4: Integrated System Verification

- **Arthur (Lead) & Gildo (RTL Reviewer):**
  - Export trained XOR model from Rust runtime into unified `.mem` format with manifest.
  - Implement self-checking testbench in `npu/tests/tb.sv`: reads input activations, computes hardware output, and asserts equivalence against exported golden vectors with non-zero exit codes on failure.
  - Document bit-exact parity proof across all 4 XOR input combinations.
- **Gustavo:**
  - Lead final evaluation, benchmark reporting, and scientific analysis for the v0.1.0 release.
  - Consolidate software accuracy (MNIST $\ge 95\%$), hardware cycle latency, and quantization loss metrics into `docs/project/research/`.
  - Validate end-to-end regression fixtures ensuring bit-exact alignment between software Q8.24 inference and RTL simulation.
- **Gildo:**
  - Deprecate and remove legacy vector math code paths.
  - Audit finalized NPU testbench results and sign off on RTL hardware verification.

**Phase 4 Gate:** End-to-end demonstration: Rust train ──► Export ──► NPU simulation PASS with zero bit mismatches. Full benchmark report and research logs consolidated in `docs/project/research/`.

---

## 4. Engineering Invariants and Anti-Regression Policy

1. **Continuous Verification:** The legacy training path must remain functional until Phase 3 convergence is verified against the golden characterization fixtures.
2. **Deterministic Reproducibility:** Every automated test, benchmark, and experiment must initialize PRNG state with a hard-coded seed.
3. **Interface Sign-Off:** No modifications to public structs or traits in `src/math/` or `src/modules/` are permitted without approval from the respective intersection owners.
4. **Hardware-Software Dual Sign-Off:** Any modification to numeric representation, scaling, or rounding behavior requires simultaneous updates to both `src/export.rs` and `npu/src/npu.sv`, with sign-off from Arthur (Hardware Owner), Gildo (NPU Reviewer), and Gustavo (Numerical Controls).
5. **Empirical Data Provenance:** All published benchmarks, convergence metrics, and research claims must be generated from version-controlled fixtures managed under `docs/project/research/` with documented random seeds.
