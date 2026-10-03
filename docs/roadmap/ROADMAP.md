# Project Roadmap and Engineering Organization

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

Work is structured around four primary intersections where each domain features an
**Owner** (responsible for design and delivery) and a **Peer Reviewer** (responsible
for contract sign-off and verification).

```text
               [ GUSTAVO ]
         Architecture & Mathematical Foundations
                 /         \
   Intersection 1 /           \ Intersection 2
  (Tensor Core)/             \(Model IR & Topology)
              /               \
       [ GILDO ] ─────────── [ ARTHUR ]
     Engine & Algorithms       Hardware & Systems Integration
               Intersection 3
          (Compute & Backprop)
```

### Intersections Specification

| Intersection | Subsystem | Lead (Owner) | Peer Reviewer | Joint Scope & Deliverables |
|---|---|---|---|---|
| **1** | **Network Architecture (`Model` vs `Sequential`)** | Arthur | Gustavo | `Module` trait definition, parameter tracking, topologically ordered IR extraction, and deprecation of monolithic `Sequential`. |
| **2** | **Tensor Core & Mathematical Foundations** | Gustavo | Gildo | Memory layout, zero-copy views (`strides`, `offset`), multidimensional broadcasting rules, and deterministic PRNG harness. |
| **3** | **Compute Engine & Tensorized Backprop** | Gildo | Arthur | Batched tensor arithmetic (`matmul`, element-wise), elimination of `Vec<Vec<f32>>`, tensorized analytical backprop, and memory export mapping. |
| **4** | **NPU Co-Processor & Parity Co-Simulation** | Arthur | Gustavo | Multi-neuron Layer Engine FSM, 64-bit saturating accumulator, self-checking testbench, and golden-model bit-exact test suite. |

---

## 3. Milestone Execution Plan (6-Week Horizon)

### Phase 1: Contracts and Deterministic Baselines (Weeks 1–2)

- **Arthur:**
  - Author normative specifications: `docs/specs/MODEL_TRAIT.md` and `docs/specs/ARITHMETIC_Q8_24.md`.
  - Sanitize `npu/Makefile`: eliminate broken simulation/ternary targets, wire up automated Verilator linting (`make lint`).
  - Upgrade `npu/tb.sv` to support `$dumpfile` and `$dumpvars` for GTKWave tracing.
- **Gustavo:**
  - Author normative specification: `docs/specs/TENSOR.md`.
  - Implement deterministic pseudo-random number generator (PRNG) with explicit seed support (replacing unseeded uniform randoms).
  - Construct baseline characterization tests: capture loss trajectories and canonical weights for OR and XOR models using current engine.
- **Gildo:**
  - Review `docs/specs/TENSOR.md` for algorithm suitability.
  - Implement tensor element-wise arithmetic kernels (`add`, `sub`, `mul`, `div`) respecting strided storage.
  - Implement memory reduction operations (`sum`, `mean`) along arbitrary axes.

**Phase 1 Gate:** All specification documents merged into `main`. Canonical XOR loss curves locked in regression fixtures.

---

### Phase 2: Core Abstractions and Hardware Upgrades (Weeks 3–4)

- **Gustavo:**
  - Deliver zero-copy tensor views (`slice`, `transpose`, `reshape`, `permute`) via strided index projection.
  - Implement NumPy-compliant multidimensional broadcasting resolution.
  - Provide safe copy-on-write (COW) mutation semantics for shared storage buffers (`Arc::make_mut`).
- **Gildo:**
  - Implement cache-friendly row-major matrix multiplication (`matmul`) with dimension assertions.
  - Port initialization algorithms (Xavier/Glorot and He/Kaiming) to `Tensor`.
  - Validate math kernels against Gustavo's analytical test harness.
- **Arthur:**
  - Refactor `Sequential` to implement the new `Module` trait.
  - Introduce `Model<M>` container owning parameter lifetime management.
  - Upgrade NPU microarchitecture: expand accumulator to 64 bits with saturation logic; integrate signed ReLU activation unit in RTL.

**Phase 2 Gate:** Matrix multiplication and tensor views pass automated unit tests. NPU RTL compiles cleanly under Verilator without warnings.

---

### Phase 3: Engine Convergence and Tensorized Backpropagation (Week 5)

- **Gildo (Lead) & Arthur (Pair):**
  - Migrate `Linear` layer to store weights and biases as contiguous `Tensor` instances.
  - Re-engineer `backward()`: perform full backward pass using batched matrix multiplications, eliminating `Vec<Vec<f32>>` allocations.
  - Eliminate redundant model cloning in gradient structures (`Grad`).
- **Gustavo:**
  - Execute numerical gradient validation: verify analytical tensor backprop against finite differences (`num_grad`) with tolerance $\epsilon \le 10^{-4}$.
  - Verify convergence stability on multi-layer XOR network.
- **Arthur:**
  - Design multi-cycle Layer Engine FSM in `npu/main.sv` capable of computing $M$ output neurons from $N$ input activations sequentially.
  - Implement `Model::compile_to_manifest()` exporting layer descriptors and Q8.24 weights.

**Phase 3 Gate:** Training loop converges on XOR using pure `Tensor` backend. Numerical gradient check passes.

---

### Phase 4: Integrated System Verification (Week 6)

- **Arthur (Lead) & Gustavo (Pair):**
  - Export trained XOR model from Rust runtime into unified `.mem` format with manifest.
  - Implement self-checking testbench in `npu/tb.sv`: reads input activations, computes hardware output, and asserts equivalence against exported golden vectors.
  - Document bit-exact parity proof across all 4 XOR input combinations.
- **Gildo:**
  - Deprecate and remove legacy vector math code paths.
  - Finalize documentation benchmarks and training profiles.

**Phase 4 Gate:** End-to-end demonstration: Rust train ──► Export ──► NPU simulation PASS with zero mismatches.

---

## 4. Engineering Invariants and Anti-Regression Policy

1. **Continuous Verification:** The legacy training path must remain functional until Phase 3 convergence is verified against the golden characterization fixtures.
2. **Deterministic Reproducibility:** Every automated test and experiment must initialize PRNG state with a hard-coded seed.
3. **Interface Sign-Off:** No modifications to public structs or traits in `src/math/` or `src/modules/` are permitted without approval from the respective intersection owners.
4. **Hardware-Software Dual Sign-Off:** Any modification to numeric representation, scaling, or rounding behavior requires simultaneous updates to both `src/export.rs` and `npu/main.sv`.
