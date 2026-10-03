# Specification: Tensor Storage, Strides, and Memory Layout

This document defines the normative memory layout, striding rules, view transformations,
and broadcasting semantics for `Tensor` in `src/math/tensor.rs`.

---

## 1. Data Structure Invariants

A `Tensor` represents an n-dimensional view over a shared, linear storage buffer.

```rust
pub struct Tensor {
    storage: Arc<Vec<f32>>,
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

---

## 2. Zero-Copy View Transformations

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

## 3. Storage Mutation and Copy-on-Write (COW)

Shared storage buffers are protected through atomic reference counting (`Arc`).

1. **In-Place Mutation:** Operations modifying tensor values require exclusive ownership of a contiguous, packed storage buffer.
2. **Copy-on-Write (COW) Protocol:** When attempting in-place mutation on a tensor with shared storage (`Arc::strong_count > 1`) or on a non-contiguous strided view, the implementation must first allocate a fresh contiguous buffer, copy logical elements in order, reset `strides` to canonical C-order and `offset` to zero, before performing the mutation.
3. Direct indexing panics (`Arc::get_mut().expect(...)`) on shared views are prohibited.

---

## 4. Multidimensional Broadcasting Rules

Two shapes are compatible for broadcasting if, aligning dimensions from right to left:
1. They are equal, or
2. One of them is 1, or
3. One of the shapes has fewer dimensions (padded with 1s on the left).

### Stride Computation for Broadcast:
When an axis of size 1 is expanded to size $M$:
- Expanded shape: $\text{shape}'[k] = M$.
- Expanded stride: $\text{strides}'[k] = 0$.

A stride of `0` ensures that advancing along dimension $k$ accesses the same physical memory location without allocating duplicate storage.
