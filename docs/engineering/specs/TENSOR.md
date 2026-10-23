# Specification: Tensor Storage, Strides, Data Types, and Memory Layout

**Status:** Target contract for the core tensor subsystem. This specification defines
the normative architecture, minimum viable surface, memory layout, striding rules,
type representations, and linear algebra contracts for `Tensor` in `src/math/tensor.rs`.
Section 4.2 records the delivery phase in progress and the surface that remains
deferred.

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
| `DType` | One-byte copyable metadata tag identifying the numerical type | Candle `DType`, Burn `DType` |
| `Storage` | Heterogeneous enum owning contiguous, type-specific `Arc` buffers | Candle `CpuStorage`, Burn `NdArrayTensor` |
| `Tensor` | Shared geometrical metadata (`shape`, `strides`, `offset`) over `Storage` | NumPy ndarray, PyTorch ATen, Candle `Tensor_` |

`DType` is a fieldless enumeration with a one-byte representation. It derives `Clone`,
`Copy`, `PartialEq`, `Eq`, and `Debug`, so that the tag is returned by value and is
directly comparable. The numeric value of each discriminant is an implementation
detail and is not part of this contract.

Fixed-point arithmetic (`Q8_24`) is strictly excluded from `DType` and `Storage`. It is
retained solely as a serialization format within `src/export.rs` for generating SystemVerilog
`$readmemh` test artifacts under `npu/data/`. Simulating fixed-point inside the tensor
operator graph imposes artificial software overhead without parity benefit.

### 1.2 Implementation Status Vocabulary
Section 4 annotates each member of the minimum viable surface with an implementation
status. The vocabulary is normative:

| Status | Meaning |
|---|---|
| `Implemented` | The behavior described by this specification is present and exercised by the test suite |
| `Partial` | The operation exists but does not yet satisfy every clause of its contract |
| `Not implemented` | The operation is specified here and absent from the runtime |
| `Deferred` | The operation is outside the scope of the current delivery phase |

---

## 2. Data Structure Invariants and Representations

### 2.1 Type Representations

```text
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

### 2.2 Invariants
1. `shape.len() == strides.len()`.
2. Total logical elements: $N = \prod_{i=0}^{D-1} \text{shape}[i]$. For scalar tensors ($D=0$), $N=1$.
3. Storage bounds: for all multi-indices $(i_0, \dots, i_{D-1})$ with $0 \le i_k < \text{shape}[k]$, the physical index
   $$\text{idx} = \text{offset} + \sum_{k=0}^{D-1} i_k \cdot \text{strides}[k]$$
   is a **logical element index**, not a byte offset, and must satisfy
   $0 \le \text{idx} < \text{element\_capacity}$ of the backing buffer.
4. A tensor is **contiguous (row-major / C-order)** if and only if both of the following clauses hold:
   $$\text{strides}[D-1] = 1 \quad \text{and} \quad \text{strides}[k] = \text{strides}[k+1] \cdot \text{shape}[k+1] \quad \forall k \in [0, D-2]$$
   $$\text{offset} = 0$$
   The first clause establishes the row-major element order. The second clause establishes that the tensor owns a packed buffer that begins at element zero, which is the precondition of every kernel that addresses the buffer directly. A tensor whose physical offset is non-zero is therefore not contiguous, even when its stride vector is canonical. A tensor whose logical extent is empty is contiguous whenever both clauses hold; the vacuous case carries no behavioral consequence, because such a tensor has no element to transfer.
5. Buffer capacity constraints per storage variant, expressed as a logical element capacity:
   - `Storage::F32`: at least $N$ elements (`f32`).
   - `Storage::I8`: at least $N$ elements (`i8`).
   - `Storage::T3`: at least $N$ logical elements, materialized as $\lceil N / 4 \rceil$ bytes (`u8`), packing four trits per byte.
   A buffer whose element capacity is below $N$ is a precondition violation at construction.
6. `DType` occupies exactly one byte in memory and is copyable by value.
7. A `Storage::T3` tensor is **byte-aligned** when its `offset` and every entry of its
   `strides` are multiples of four. A ternary view that is not byte-aligned is not a
   valid view, because the four trits sharing a byte cannot be addressed
   independently. Such a view is not produced by the view operations of Section 4 and
   is not eligible for in-place mutation.

### 2.3 Index and Addressing Model
The logical element index of Section 2.2, Invariant 3, is resolved into a physical
address according to the storage variant. This model is the single normative
statement of how the packing of Section 3.1 reaches the byte buffer.

| Variant | Byte address of element `idx` | Bit position within byte |
|---|---|---|
| `F32` | $\text{idx} \cdot 4$ | Not applicable |
| `I8` | $\text{idx}$ | Not applicable |
| `T3` | $\lfloor \text{idx} / 4 \rfloor$ | $(\text{idx} \bmod 4) \cdot 2$ |

`offset` is always expressed in logical element units, in every variant.

---

## 3. Ternary and Packed Encoding Contracts

TARS supports ternary weights $W \in \{-W_0, 0, +W_0\}$. The design supports a standard
two-phase evolution to prevent future architectural rework.

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
- **Dequantization:** The value represented by a trit code $c$ is $\text{trit}(c) \cdot W_0$, where $W_0$ is the scale held by the storage.
- **Reserved Encoding:** The code `11` is an invalid encoding. Reading an element whose code is `11` is a precondition violation and terminates the operation. A ternary buffer allocated by the runtime never contains the reserved code, and a buffer intended to hold zero-valued trits is filled with the code `01` in every bit pair.

### 3.2 Ternary Affine Evaluation
When evaluating an affine projection with ternary weights, inner products eliminate floating-point multiplications:
$$y_i = b_i + W_0 \left( \sum_{j \in S_+} a_j - \sum_{j \in S_-} a_j \right)$$
where $S_+ = \{j \mid w_{ij} = +1\}$ and $S_- = \{j \mid w_{ij} = -1\}$. The scale factor $W_0$ is multiplied once per output accumulation.

The bias term $b_i$ belongs to the forward contract of the linear layer, not to the
tensor product. `Tensor::matmul` is a pure product and returns no bias contribution;
the ternary evaluation above is the specialization that a ternary-aware linear layer
applies to the result of that product.

### 3.3 I8 Scaling Semantics
The scale held by `Storage::I8` is a multiplicative factor. A stored integer $q$
represents the value $q \cdot \text{scale}$. The scale is a property of the storage
buffer and is not altered by element access. In the current scope the scale is held
per tensor; a per-channel scale is a subsequent extension.

### 3.4 Extensibility to Succinct Mask + Sign Vectors (POPCNT)
The `Storage::T3` encapsulation guarantees that migrating internally to a Position Bitmap ($N$ bits) plus a Compact Sign Vector ($K$ bits) evaluated via `POPCNT` requires zero changes to the public `Tensor` API, `Linear` layer, or `backward()` routines.

---

## 4. Minimum Viable Surface for Repository Convergence

To unblock the migration of `Linear`, `backward()`, activations, and optimizers without
premature optimization, the tensor core must provide strictly the following methods:

| Method Category | Method Signature | Operational Behavior | Status |
|---|---|---|---|
| **Constructors** | `new(Storage, Vec<usize>) -> Self` | Wraps storage in canonical C-order strided view at offset 0 | Partial |
| | `zeros(Vec<usize>, DType) -> Self` | Allocates zeroed buffer sized to shape and data type | Partial |
| **Geometry** | `shape(&self) -> &[usize]` | Borrows shape vector | Implemented |
| | `strides(&self) -> &[usize]` | Borrows strides vector | Implemented |
| | `offset(&self) -> usize` | Returns physical offset | Implemented |
| | `numel(&self) -> usize` | Computes logical product $\prod \text{shape}[i]$ | Implemented |
| | `is_contiguous(&self) -> bool` | Validates the contiguity definition of Section 2.2, Invariant 4 | Partial |
| | `dtype(&self) -> DType` | Returns copyable data type discriminant | Partial |
| **Element Access** | `get(&self, &[usize]) -> f32` | Logical view: dequantizes value to `f32` for generic consumption | Partial |
| | `set(&mut self, &[usize], f32)` | Mutates storage in-place via `Arc::make_mut` (COW) | Partial |
| **Views & Memory** | `transpose(&self, usize, usize) -> Self` | Zero-copy axis swap modifying `shape` and `strides` | Partial |
| | `transpose_mut(&mut self, usize, usize)` | In-place axis swap of `shape` and `strides`; alters metadata only and does not mutate the storage buffer | Partial |
| | `contiguous(&self) -> Self` | Returns a tensor whose strides are canonical row-major and whose physical offset is zero; shares the source storage when the source already satisfies Section 2.2, Invariant 4, and re-packs the elements by coordinate mapping otherwise | Partial |
| **Linear Algebra** | `matmul(&self, &Tensor) -> Self` | Computes 2D matrix multiplication ($[M, K] \times [K, N] \to [M, N]$) with specialized ternary fast-path | Partial |
| | `matvec(&self, &Tensor) -> Self` | Multiplies matrix $[M, K]$ by vector $[K]$ | Partial |
| | `outer(&self, &Tensor) -> Self` | Outer product of 1D vectors $[M] \times [N] \to [M, N]$ | Partial |
| **Element-wise** | `add(&self, &Tensor) -> Self` | Pointwise logical addition respecting strides | Partial |
| | `sub(&self, &Tensor) -> Self` | Pointwise logical subtraction respecting strides | Partial |
| | `mul_scalar(&self, f32) -> Self` | Pointwise scalar multiplication | Partial |
| | `div_scalar(&self, f32) -> Self` | Pointwise scalar division | Partial |
| | `map(&self, F) -> Self` | Applies a scalar function to logical elements, where $F$ is any callable over `f32` that may capture state | Partial |
| | `sum(&self) -> f32` | Full reduction across all logical coordinates | Partial |

#### 4.0.1 Gap Notes
- `new` does not yet validate the element capacity of the supplied storage against the shape product, as required by Section 2.2, Invariant 5.
- `zeros` does not yet materialize the ternary fill encoding of Section 3.1, and assigns a degenerate zero scale to the packed variants, which contradicts the scale definition of Sections 3.1 and 3.3.
- `get` and `set` are restricted to the `F32` variant. Access on `I8` and `T3` storage is a precondition violation under Section 4.1, and is not a silent no-operation.
- `is_contiguous` couples a geometric predicate to the storage variant: it terminates the operation on `I8` and `T3` storage without a diagnostic that names the violated precondition, which is contrary to Section 4.1. It also applies a third clause, a non-empty backing buffer, which is not part of the definition of Section 2.2, Invariant 4.
- The in-place axis swap alters the shape and stride vectors only. It does not write to the storage buffer and therefore does not trigger the buffer duplication of Section 5. Its diagnostic, shared with the non-in-place variant, is not yet worded according to the policy of Section 4.1, and the operation has no test coverage.
- The stride computation that establishes canonical row-major strides carries an identifier distinct from the re-packing operation, so the two entities coexist without collision. The re-packing operation is delivered under the identifier `make_contiguous`; its reconciliation with the `contiguous` name of this table remains open. The operation has no test coverage.
- The element-wise kernels and the products are implemented over `F32` and honour the stride vectors of their operands. Their shape and dimension preconditions are reported through default assertion text or empty diagnostics that do not name the violated precondition; `div_scalar` does not reject a zero divisor; `map` accepts non-mutating callables only; the kernels and the products have no test coverage.
- The full reduction `sum` is implemented and honours the stride vector. It terminates on a rank-zero operand through the logical iteration helper (defect D-06 of the delivery plan) and has no test coverage.

### 4.1 Preconditions and Error Policy
The tensor core reports precondition violations by terminating the operation with a
descriptive diagnostic. It does not return a recoverable error value in the current
delivery phase. A diagnostic is never empty, and a precondition violation is never
reported as a successful operation.

| Operation | Precondition |
|---|---|
| `new` | The element capacity of the storage is at least the shape product (Section 2.2, Invariant 5) |
| `is_contiguous` | The storage variant supports the check; the packed variants are outside the active phase (Section 4.2) |
| `get`, `set` | The index tuple has one entry per dimension, and each entry is within the corresponding shape extent |
| `get`, `set` | The storage variant is supported by the access path of the active delivery phase (Section 4.2) |
| `set` | The buffer is not shared with a view that requires exclusive ownership; otherwise the buffer is duplicated before mutation (Section 5) |
| `transpose` | Both axis identifiers are within the rank of the tensor; a ternary result is byte-aligned (Section 2.2, Invariant 7) |
| `transpose_mut` | Both axis identifiers are within the rank of the tensor; a ternary result is byte-aligned (Section 2.2, Invariant 7) |
| `contiguous` | The storage variant supports element transfer in the active delivery phase |
| `matmul` | The left operand has rank 2, the right operand has rank 2, and the inner dimensions agree |
| `matvec` | The left operand has rank 2, the right operand has rank 1, and the inner dimensions agree |
| `outer` | Both operands have rank 1 |
| `add`, `sub` | Both operands have identical shapes; broadcasting is not provided in the current phase |
| `mul_scalar`, `div_scalar`, `map` | The operand is a valid tensor; `div_scalar` requires a non-zero divisor |
| `sum` | The operand is a valid tensor |

### 4.2 Delivery Phases
The minimum viable surface of this section is delivered incrementally. The phases
below partition the surface and state the exit condition of each.

| Phase | Scope | Exit Condition |
|---|---|---|
| **Current: F32 core** | Constructors, geometry accessors, element access over `F32`, the axis swap in both variants, `contiguous`, the element-wise kernels (`add`, `sub`, `mul_scalar`, `div_scalar`, `map`), `sum`, the products `matmul`, `matvec`, `outer` over `F32`, and the bulk access surface | Every operation of the phase satisfies Section 4.1 and is covered by the tensor test suite |
| **Subsequent: packed variants** | Element access over `T3` (within the v0.1.0 arc), the ternary dequantization of Section 3.1, the ternary fast path of Section 3.2, and the alignment constraint of Section 2.2, Invariant 7. The `I8` variant and the scaling of Section 3.3 are deferred beyond v0.1.0 (maintainer decision, October 2026) | Packed round trips and dequantized values are covered by the tensor test suite |
| **Deferred** | Reductions along arbitrary axes, `mean`, `slice`, `permute`, `reshape`, and multidimensional broadcasting | Each deferred operation receives its own contract in this specification before implementation |

The deferral of axis reductions and of the view and broadcasting operations is
consistent with the delivery gates recorded in the project roadmap. The
corresponding roadmap entries and this table must be reviewed together whenever
either is amended.

---

## 5. Storage Mutation and Copy-on-Write (COW)

1. Storage buffers are guarded by atomic reference counting (`Arc`).
2. In-place mutation via `set` must invoke `Arc::make_mut`, guaranteeing that shared
   buffers (`strong_count > 1`) are cloned before modification without panic.
3. Direct `Arc::get_mut().expect(...)` calls on shared or view-derived tensors are strictly prohibited.
4. Mutation of a view that is not byte-aligned under Section 2.2, Invariant 7, is
   prohibited in the current delivery phase.
