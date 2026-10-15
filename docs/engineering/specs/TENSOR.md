# Specification: Tensor Storage, Strides, Data Types, and Memory Layout

**Status:** Target contract for the core tensor subsystem. This specification defines
the normative architecture, minimum viable surface, memory layout, striding rules,
type representations, and linear algebra contracts for `Tensor` in `src/math/tensor.rs`.

---

## 1. Architectural Foundations and Design Rationale

The tensor abstraction in TARS decouples structural indexing, numerical representation,
and physical storage. It adopts structural patterns established by production Rust
tensor runtimes (`huggingface/candle` and `tracel-ai/burn`) and the BitNet b1.58
reference implementations (`microsoft/BitNet` and `llama.cpp` TQ2_0).

### 1.1 Structural Decoupling: DType vs. Storage vs. Layout
In contrast to element-level tagged unions (`Vec<Enum>`), which inflate memory consumption
through variant discriminants and alignment padding (up to 8 bytes per element) and defeat
SIMD vectorization, TARS enforces container-level storage dispatch:

| Layer | Responsibility | Pattern Source |
|---|---|---|
| `DType` | 1-byte copyable metadata tag identifying the numerical type | Candle `DType`, Burn `DType` |
| `Storage` | Heterogeneous enum owning contiguous, type-specific `Arc` buffers | Candle `CpuStorage`, Burn `NdArrayTensor` |
| `Tensor` | Shared geometrical metadata (`shape`, `strides`, `offset`) over `Storage` | NumPy ndarray, PyTorch ATen, Candle `Tensor_` |

Fixed-point arithmetic (`Q8_24`) is strictly excluded from `DType` and `Storage`. It is
retained solely as a serialization format within `src/export.rs` for generating SystemVerilog
`$readmemh` test artifacts under `npu/data/`. Simulating fixed-point inside the tensor
operator graph imposes artificial software overhead without parity benefit.

---

## 2. Data Structure Invariants and Representations

```text
pub enum DType {
    F32,
    I8,
    T3,
}

pub enum Storage {
    F32(Arc<Vec<f32>>),
    I8 {
        data: Arc<Vec<i8>>,
        scale: f32,
    },
    T3 {
        data: Arc<Vec<u8>>,
        scale: f32,
    },
}

pub struct Tensor {
    storage: Storage,
    shape: Vec<usize>,
    strides: Vec<usize>,
    offset: usize,
}
```

### Invariants:
1. `shape.len() == strides.len()`.
2. Total logical elements: $N = \prod_{i=0}^{D-1} \text{shape}[i]$. For scalar tensors ($D=0$), $N=1$.
3. Storage bounds: for all multi-indices $(i_0, \dots, i_{D-1})$ with $0 \le i_k < \text{shape}[k]$, the physical index
   $$\text{idx} = \text{offset} + \sum_{k=0}^{D-1} i_k \cdot \text{strides}[k]$$
   must satisfy $0 \le \text{idx} < \text{buffer\_len}$.
4. A tensor is **contiguous (row-major / C-order)** if and only if:
   $$\text{strides}[D-1] = 1 \quad \text{and} \quad \text{strides}[k] = \text{strides}[k+1] \cdot \text{shape}[k+1] \quad \forall k \in [0, D-2]$$
5. Buffer capacity constraints per storage variant:
   - `Storage::F32`: buffer contains at least $N$ elements (`f32`).
   - `Storage::I8`: buffer contains at least $N$ elements (`i8`).
   - `Storage::T3`: buffer contains at least $\lceil N / 4 \rceil$ bytes (`u8`), packing 4 trits per byte.

---

## 3. Ternary Encoding and Storage Contracts

TARS supports ternary weights $W \in \{-W_0, 0, +W_0\}$. The design supports a standard
two-phase evolution to prevent future architectural rework:

### 3.1 Canonical Bit-Packed Ternary (BitNet b1.58 / TQ2_0 Standard)
Each trit is encoded using biased 2-bit unsigned integers ($\text{uint2} = \text{trit} + 1$):

| Trit Value | Code (Binary) | Semantics |
| :---: | :---: | :--- |
| $-1$ | `00` | Negative weight: subtract activation |
| $0$ | `01` | Zero: skip (sparsity gating) |
|$+1$ | `10` | Positive weight: add activation |
| Reserved | `11` | Invalid / padding |

- **Packing Density:** 4 trits per byte (`u8`). Byte index: $\lfloor \text{idx} / 4 \rfloor$. Bit shift: $(\text{idx} \pmod 4) \times 2$.
- **Memory Compression:** Consumes $2.0$ bits per element ($16\times$ reduction relative to `f32`, $4\times$ relative to `i8`).
- **Scale Factor:** $W_0 = \frac{1}{mn} \sum_{ij} |W_{ij}|$ is stored per tensor (or per output channel).

### 3.2 Multiplication-Free Matrix Multiplication ($A_{\text{f32}} \times W_{\text{T3}}$)
When evaluating an affine projection with ternary weights, inner products eliminate floating-point multiplications:
$$y_i = b_i + W_0 \left( \sum_{j \in S_+} a_j - \sum_{j \in S_-} a_j \right)$$
where $S_+ = \{j \mid w_{ij} = +1\}$ and $S_- = \{j \mid w_{ij} = -1\}$. The scale factor $W_0$ is multiplied once per output accumulation.

### 3.3 Extensibility to Succinct Mask + Sign Vectors (POPCNT)
The `Storage::T3` encapsulation guarantees that migrating internally to a Position Bitmap ($N$ bits) plus a Compact Sign Vector ($K$ bits) evaluated via `POPCNT` requires zero changes to the public `Tensor` API, `Linear` layer, or `backward()` routines.

---

## 4. Minimum Viable Surface for Repository Convergence

To unblock the migration of `Linear`, `backward()`, activations, and optimizers without
premature optimization, the tensor core must provide strictly the following methods:

| Method Category | Method Signature | Operational Behavior |
|---|---|---|
| **Constructors** | `new(Storage, Vec<usize>) -> Self` | Wraps storage in canonical C-order strided view at offset 0 |
| | `zeros(Vec<usize>, DType) -> Self` | Allocates zeroed buffer sized to shape and data type |
| **Geometry** | `shape(&self) -> &[usize]` | Borrows shape vector |
| | `strides(&self) -> &[usize]` | Borrows strides vector |
| | `offset(&self) -> usize` | Returns physical offset |
| | `numel(&self) -> usize` | Computes logical product $\prod \text{shape}[i]$ |
| | `is_contiguous(&self) -> bool` | Validates C-order stride invariant |
| | `dtype(&self) -> DType` | Returns copyable data type discriminant |
| **Element Access** | `get(&self, &[usize]) -> f32` | Logical view: dequantizes value to `f32` for generic consumption |
| | `set(&mut self, &[usize], f32)` | Mutates `F32` storage in-place via `Arc::make_mut` (COW) |
| **Views & Memory** | `transpose(&self, usize, usize) -> Self` | Zero-copy axis swap modifying `shape` and `strides` |
| | `contiguous(&self) -> Self` | Returns `self.clone()` if contiguous; re-packs elements via coordinate mapping otherwise |
| **Linear Algebra** | `matmul(&self, &Tensor) -> Self` | Computes 2D matrix multiplication ($[M, K] \times [K, N] \to [M, N]$) with specialized ternary fast-path |
| | `matvec(&self, &Tensor) -> Self` | Multiplies matrix $[M, K]$ by vector $[K]$ |
| | `outer(&self, &Tensor) -> Self` | Outer product of 1D vectors $[M] \times [N] \to [M, N]$ |
| **Element-wise** | `add(&self, &Tensor) -> Self` | Pointwise logical addition respecting strides |
| | `sub(&self, &Tensor) -> Self` | Pointwise logical subtraction respecting strides |
| | `mul_scalar(&self, f32) -> Self` | Pointwise scalar multiplication |
| | `div_scalar(&self, f32) -> Self` | Pointwise scalar division |
| | `map(&self, fn(f32) -> f32) -> Self` | Applies scalar function to logical elements |
| | `sum(&self) -> f32` | Full reduction across all logical coordinates |

Higher-order features—including arbitrary multidimensional broadcasting with 0-strides,
arbitrary slicing (`slice`), multi-axis permutation (`permute`), and non-contiguous
in-place mutations—are deferred to subsequent phases.

---

## 5. Storage Mutation and Copy-on-Write (COW)

1. Storage buffers are guarded by atomic reference counting (`Arc`).
2. In-place mutation via `set` must invoke `Arc::make_mut`, guaranteeing that shared
   buffers (`strong_count > 1`) are cloned before modification without panic.
3. Direct `Arc::get_mut().expect(...)` calls on shared or view-derived tensors are strictly prohibited.
