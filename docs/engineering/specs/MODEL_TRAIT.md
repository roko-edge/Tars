# Specification: Model and Module Interface Hierarchy

**Status:** Target architecture. The current runtime still uses `Sequential` with
`Vec<AnyModule>` and does not provide the `Model<M>` container specified here.

This document specifies the structural interface contract for neural network
layers, composite blocks, and execution containers in TARS.

---

## 1. Motivation and Architectural Scope

The initial TARS runtime coupled model topology, layer storage, and training state
inside a monolithic `Sequential` struct storing `Vec<AnyModule>`. This structure exhibits
several limitations:

1. **Topology Rigidity:** Cannot represent branching graphs, skip-connections (e.g., ResNet),
   or recurrent/continuous-time architectures (e.g., Liquid/CfC networks).
2. **Type Erasure Overhead:** Relies on manual enum dispatch (`AnyModule`) rather than
   trait-based polymorphism or statically typed pipelines.
3. **Hardware Coupling:** Prevents clean separation between internal Rust execution
   and external hardware graph compilation.

This specification decouples these concerns into three abstractions: `Module`,
`Sequential` (as a composite `Module`), and `Model<M>`.

---

## 2. Core Abstractions

### 2.1 The `Module` Trait Interface

Every differentiable layer, activation, or composite block must implement the `Module` trait interface.

```rust
pub trait Module: std::fmt::Debug {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn parameters(&self) -> Vec<&Tensor>;
    fn parameters_mut(&mut self) -> Vec<&mut Tensor>;
}
```

#### Contract Expectations
- `forward`: Accepts an input tensor (contiguous or strided, verifying compatibility across `DType` representations) and evaluates the transformation, returning the resulting tensor without mutating internal layer state.
- `parameters`: Collects immutable references to all parameter tensors owned by the module (e.g., weights and biases, which may carry continuous `F32` or quantized `Ternary` `DType` variants).
- `parameters_mut`: Collects mutable references to all parameter tensors in identical order, enabling in-place parameter updates during optimization.

---

### 2.2 `Sequential` as a Composite Module

`Sequential` is re-architected as a composite `Module` that owns an ordered collection of submódules (`Vec<Box<dyn Module>>`).

#### Structural Requirements
- **Encapsulation:** Holds an ordered vector of dynamic module trait objects.
- **Forward Composition:** Evaluating `forward` on `Sequential` recursively pipes the output tensor of module $k$ as the input tensor of module $k+1$.
- **Parameter Aggregation:** Evaluating `parameters` or `parameters_mut` flattens and concatenates the parameter vectors across all inner submódules in execution order.

---

### 2.3 The `Model<M>` Execution Container

`Model<M>` encapsulates a root `Module` `M`, serving as the top-level container for training execution, serialization, and hardware compilation.

#### Responsibilities
- **Root Ownership:** Holds the top-level `Module` instance representing the entire network topology.
- **Parameter Management:** Delegates parameter access to the root module's `parameters_mut()`.
- **Hardware IR Compilation:** Exposes a compilation interface (`compile_to_manifest(&self) -> HardwareManifest`) that traverses the module topology and parameters to construct the flat layer descriptors consumed by the NPU.

---

## 3. Extensibility Design

By restricting `Module` to an abstract trait interface, future architectural expansions
require no changes to the training container or optimizer:

- **Convolutional Layers (`Conv2d`):** Implements `Module`, exposing kernel weights and bias parameters.
- **Closed-Form Continuous Cells (`CfcCell`):** Implements `Module`, handling state transition and input concatenation.
- **Branching Networks (`DagModel`):** Implements `Module`, executing a directed acyclic graph of submódules.
