# Specification: Tensor Storage, Strides, Data Types, and Memory Layout

**Status:** Target contract for an incomplete subsystem. The current `Tensor`
implementation provides construction (including zeros), indexing, mutation, element
count, contiguity inspection, zero-copy transpose variants, element-wise add/sub,
scalar multiply and divide with in-place variants, a storage-based dot product, and
shape, strides, and offset accessors. Element-wise kernels iterate raw storage and do
not respect arbitrary strides; slicing, reshaping, permutation, broadcasting,
reductions, `DType` abstractions, and ternary/mixed-precision operations are not
implemented.

This document defines the normative memory layout, striding rules, view transformations,
data type abstractions (`DType`), and broadcasting semantics for `Tensor` in `src/math/tensor.rs`.

---

## 1. Data Structure Invariants and Data Type Representation

A `Tensor` represents an n-dimensional view over a shared, linear storage buffer with an explicit element data type (`DType`).

```rust
pub enum DType {
    F32,
    Q8_24,
    Ternary,
}

pub struct Tensor {
    storage: Arc<Vec<f32>>,
    dtype: DType,
    shape: Vec<usize>,
    strides: Vec<usize>,
    offset: usize,
}
```

### Invariants:
1. `shape.len() == strides.len()`.
2. Total logical elements: $N = \prod_{i=0}^{D-1} \text{shape}[i]$. For scalar tensors ($D=0$), $N=1$.
3. Storage bounds: for all valid multi-indices $(i_0, \dots, i_{D-1})$ where $0 \le i_k < \text{shape}[k]$, the physical index
   $$\text{idx} = \text{offset} + \sum_{k=0}^{D-1} i_k \cdot \text{strides}[k]$$
   must satisfy $0 \le \text{idx} < \text{storage.len()}$.
4. A tensor is **contiguous (row-major / C-order)** if and only if:
   $$\text{strides}[D-1] = 1 \quad \text{and} \quad \text{strides}[k] = \text{strides}[k+1] \cdot \text{shape}[k+1] \quad \forall k \in [0, D-2]$$
5. `dtype` indicates the numerical interpretation of the storage. For `DType::Ternary`, logical elements represent states in $\{-1, 0, +1\}$ scaled by a parameter-level scale factor $W_0$.

---

## 2. Ternary and Mixed-Precision Compute Engine Contract

Operations involving ternary weight tensors ($\{-1, 0, +1\}$) and continuous or fixed-point activation tensors must comply with the following computation rules:

### 2.1 Mixed-Precision Matrix Multiplication ($A_{\text{float/Q8.24}} \times B_{\text{ternary}}$)
When evaluating a layer with ternary weights $W \in \{-W_0, 0, +W_0\}^{O \times I}$ against activation $A$:
1. Floating-point/Q8.24 multiplications between activation elements and ternary weight states must be reduced to conditional addition, subtraction, or zero-accumulation:
   $$y_i = b_i + W_0 \sum_{j \in S_+} a_j - W_0 \sum_{j \in S_-} a_j$$
   where $S_+ = \{j \mid w_{ij} = +1\}$ and $S_- = \{j \mid w_{ij} = -1\}$.
2. The scale factor $W_0$ is applied once per dot-product reduction or accumulated output to minimize floating-point scaling operations.

---

## 3. Zero-Copy View Transformations

View operations modify `shape`, `strides`, and `offset` without copying or reallocating the underlying storage buffer.

### 2.1 Transpose and Permute
Transposition swaps two axes; permutation reorders all axes according to a given permutation $\pi$:
- New shape: $\text{shape}'[k] = \text{shape}[\pi(k)]$.
- New strides: $\text{strides}'[k] = \text{strides}[\pi(k)]$.
- Storage buffer and `offset` remain unchanged.

### 2.2 Slicing
Slicing axis $k$ from $start$ to $end$ with step $step$:
- New shape: $\text{shape}'[k] = \lceil (end - start) / step \rceil$.
- New strides: $\text{strides}'[k] = \text{strides}[k] \cdot step$.
- New offset: $\text{offset}' = \text{offset} + start \cdot \text{strides}[k]$.

### 2.3 Reshape
- If `is_contiguous()` is true, a reshape updates `shape` and computes new canonical C-order strides without copying.
- If `is_contiguous()` is false, `reshape` must explicitly invoke `.contiguous()` (cloning the data into a newly allocated, packed buffer) before modifying shape.

---

## 4. Storage Mutation and Copy-on-Write (COW)

Shared storage buffers are protected through atomic reference counting (`Arc`).

1. **In-Place Mutation:** Operations modifying tensor values require exclusive ownership of a contiguous, packed storage buffer.
2. **Copy-on-Write (COW) Protocol:** When attempting in-place mutation on a tensor with shared storage (`Arc::strong_count > 1`) or on a non-contiguous strided view, the implementation must first allocate a fresh contiguous buffer, copy logical elements in order, reset `strides` to canonical C-order and `offset` to zero, before performing the mutation.
3. Direct indexing panics (`Arc::get_mut().expect(...)`) on shared views are prohibited.

---

## 5. Multidimensional Broadcasting Rules

Two shapes are compatible for broadcasting if, aligning dimensions from right to left:
1. They are equal, or
2. One of them is 1, or
3. One of the shapes has fewer dimensions (padded with 1s on the left).

### Stride Computation for Broadcast:
When an axis of size 1 is expanded to size $M$:
- Expanded shape: $\text{shape}'[k] = M$.
- Expanded stride: $\text{strides}'[k] = 0$.

A stride of `0` ensures that advancing along dimension $k$ accesses the same physical memory location without allocating duplicate storage.
