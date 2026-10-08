# Normative Specification: TARS v0.1.0 Release Contract (V0 Scope)

**Document type:** Project release target. The capabilities in this contract are not
claims about the current public library.

This document defines the normative scope, benchmark criteria, hardware-software capabilities,
visualization requirements, and research logging protocol for TARS Release v0.1.0 (v0).

---

## 1. Primary Benchmark Goal

**The flagship milestone of TARS v0.1.0 is solving the MNIST Handwritten Digit Recognition task.**

- **Input Dimension:** 784 features ($28 \times 28$ normalized grayscale pixels).
- **Output Dimension:** 10 classes ($0$ through $9$ softmax/one-hot logits).
- **Target Accuracy:** $\ge 95\%$ classification accuracy on the MNIST validation set in Rust simulation.
- **Hardware Target:** Full inference of the trained MNIST model executing on the SystemVerilog NPU with bit-exact numeric parity.

---

## 2. Core Library Architecture and Functional Capabilities

TARS v0.1.0 must be structured as an elegant, clean Rust crate (`tars`) with a public API designed for future extensibility (CNNs, Closed-Form Continuous / CfC networks, Neural ODEs).

```text
tars/
├── src/
│   ├── lib.rs                   # Public crate re-exports and prelude
│   ├── math/                    # Tensor engine (strides, views, broadcasting, matmul)
│   ├── modules/                 # Module trait, Linear, Activations, Sequential
│   ├── model/                   # Model<M> container and IR graph extraction
│   ├── optim/                   # Tensorized Backprop, BGD, Adam, Xavier/He init
│   ├── quant/                   # Quantization engine (Q8.24 fixed-point & ternary {-1, 0, +1})
│   ├── export/                  # Hardware manifest & memory artifact generators (.mem)
│   ├── observe/                 # Observer trait, telemetry event stream, metrics
│   └── view/                    # Graphviz topology & node activation visualization
```

### 2.1 Complete Tensor Engine (`src/math/tensor.rs`)
- Full N-dimensional contiguous and strided `Tensor` implementation with `Arc` storage.
- Explicit data type representation via `DType` enum (`F32`, `Q8_24`, `Ternary`).
- Zero-copy view transformations: `slice`, `transpose`, `reshape`, `permute`.
- Multidimensional NumPy-compliant broadcasting.
- High-performance row-major matrix multiplication (`matmul`) for float tensors, and mixed-precision matrix multiplication for ternary weight tensors.
- Axis reductions (`sum`, `mean`).
- Copy-on-Write (COW) mutation semantics via `make_contiguous_mut`.

### 2.2 Quantization & Ternary Engine (`src/quant/`)
- **Q8.24 Fixed-Point Engine:** Quantization of continuous floats into signed 32-bit Q8.24 integers.
- **Ternary Quantization Engine ($\{-1, 0, +1\}$):** Weight quantization mapping parameters to ternary states $\{-W_0, 0, +W_0\}$ with scaling factor $W_0$.
- **Ternary Compute Operations:** Tensor-level mixed-precision dot products reducing continuous/fixed-point activations against ternary weights via conditional add/sub/zero accumulation.
- **Quantization-Aware Export:** Export pipeline emitting layer manifests, Q8.24 files (`weights.mem`, `bias.mem`), and ternary codebook parameters.

### 2.3 Respective NPU Hardware (`npu/`)
- Multi-cycle **Layer Engine FSM** in SystemVerilog capable of processing $784 \to H \to 10$ layer shapes.
- Configurable arithmetic mode supporting both **32-bit Q8.24 MAC** and **Ternary Multiply-Accumulate** operations.
- 64-bit saturating accumulator to prevent overflow during 784-element dot products.
- Self-checking simulation testbench (`tb.sv`) verifying NPU execution against Rust golden vectors with $0$ bit errors.

### 2.4 Tensorized Analytical Backpropagation
- Pure tensor-based automatic gradient computation (`backward()`), eliminating all `Vec<Vec<f32>>` allocations.
- Dual-mode verification: `num_grad` (finite differences) executes in regression test suites to guarantee analytical gradient correctness.

---

## 3. Visualization and Observability Specifications

The visualization subsystem (`src/view/`) must enhance the topology renderer:

1. **Topology Graph Rendering:** Render neurons organized in layer columns (Input 784 $\to$ Hidden $H$ $\to$ Output 10, with configurable subsampling for display).
2. **Dynamic Node Activation Color Intensity:**
   - Each neuron vertex in the graph must visually encode its activation level $a \in [0, 1]$ (or normalized range).
   - The fill color hue is chosen per-network (e.g., deep cobalt blue or emerald green), while the **color intensity / saturation / alpha** scales dynamically with $a$:
     $$\text{Color}(a) = \text{RGBA}(\text{R}_{\text{base}}, \text{G}_{\text{base}}, \text{B}_{\text{base}}, \alpha = a)$$
3. **Edge Weight Rendering:** Edge stroke width and color polarity scale with $w_{ij}$ magnitude and sign.

---

## 4. Academic Research Logging Protocol (Paper Prerequisites)

To support the subsequent publication of an academic research paper on the TARS architecture and hardware-software co-design, all engineering discoveries, design trade-offs, and empirical results must be formally recorded.

### 4.1 Research Artifact Directory (`docs/project/research/`)

Every non-trivial engineering decision or discovery must produce a numbered log in
`docs/project/research/`:

```text
docs/project/research/
├── README.md                      # Index of research logs and paper outline
├── LOG-001-q8_24_truncation_error.md
├── LOG-002-ternary_quantization_loss.md
├── LOG-003-npu_accumulator_overflow.md
└── LOG-004-mnist_accuracy_vs_bitwidth.md
```

### 4.2 Research Log Requirements
Each log entry must follow a standardized academic structure:
1. **Title and Context:** Problem statement or hypothesis.
2. **Theoretical Formulation:** Mathematical equations or hardware state transition models.
3. **Empirical Benchmarks:** Numerical tables, accuracy graphs, or RTL cycle counts comparing alternatives.
4. **Conclusion & Architectural Implication:** Final decision and impact on TARS v0.1.0.

---

## 5. Future-Proofing Invariants (Preparing for v1.0+)

To ensure v0.1.0 abstractions seamlessly scale to CNNs, Closed-Form Continuous (CfC) networks, and Neural ODEs:

1. **Graph Agnosticism:** The `Module` trait must enforce `forward(&self, input: &Tensor) -> Tensor` without assuming 1D or 2D vector shapes.
2. **Tensor Dimensionality:** `Tensor` must support arbitrary rank $N \ge 1$ (e.g., $B \times C \times H \times W$ for CNNs, $B \times T \times D$ for sequence/CfC networks).
3. **Intermediate Representation (IR):** The `HardwareManifest` exported by `Model` must represent layers as a directed acyclic graph (DAG) of tensor operations, not hard-coded linear chains.
