# Current Public API

This document identifies the implemented public surface of the TARS Rust package.
It is a usage reference, not a stability guarantee. Internal design targets are
maintained under [`engineering/`](engineering/README.md).

---

## 1. Data and Experiment Configuration

| Resource | Current role | Important constraint |
|---|---|---|
| `Data` | Stores one input vector and one target vector | Dimensions are not validated by the constructor |
| `Train` | Groups a model, epoch count, learning rate, dataset, and experiment identifier | It is configuration only; it does not execute training |
| `ExperimentType` | Identifies the OR or XOR configuration in reporting | It does not select or dispatch a factory |
| `experiments::or::train()` | Creates the built-in OR configuration | Weights and biases initialize to zero; there is no configurable seed |
| `experiments::xor::train()` | Creates the built-in XOR configuration | The main binary does not select it currently |

See [`EXPERIMENTS.md`](EXPERIMENTS.md) for topologies, datasets, and execution behavior.

---

## 2. Models and Modules

| Resource | Implemented operations | Important constraint |
|---|---|---|
| `Module` | Forward evaluation over `&[f32]` | The current interface is vector-based, not tensor-based |
| `Sequential` | Construction, linear layers, ReLU, sigmoid, and forward evaluation | Supports an ordered linear chain only |
| `Linear` | Construction from explicit parameters, zeros, or random initialization | `Sequential` uses the random constructor; dimensions are assumed to be consistent |
| `Activation` | ReLU and sigmoid evaluation | No additional activation variants are implemented |
| `AnyModule` | Runtime dispatch over linear and activation modules | Typed accessors panic when used with the wrong variant |

The tensor-based `Module` and `Model<M>` hierarchy described in the engineering
specification is a target architecture, not the current public interface.

---

## 3. Training Operations

| Resource | Current role | Important constraint |
|---|---|---|
| `cost` | Computes mean squared error over outputs and samples | Empty or dimensionally inconsistent data is not guarded |
| `backward` | Computes analytical gradients for the supported sequential modules | Operates on the full dataset and supported module variants only |
| `num_grad` | Computes centered finite-difference gradients | Intended for verification and not used by the main binary |
| `BGD` | Applies batch gradient descent updates | Model and gradient structure compatibility is assumed |

There is no high-level public trainer that owns the complete loop. The executable
implements the loop using these operations.

---

## 4. Export and Visualization

| Resource | Current role | Important constraint |
|---|---|---|
| `to_q8_24` | Converts an `f32` value to the current Q8.24 word representation | Uses truncation rather than round-half-to-even |
| `export_model` | Writes linear weights and biases | Does not emit topology or a layer manifest |
| `export_data` | Writes inputs and targets | Output paths are fixed relative to the working directory |
| `NetGraph` | Extracts linear-layer nodes and edges and emits Graphviz artifacts | Activations and biases are not represented |

The Rust exporter and NPU testbench use compatible numeric words but different active
artifact paths. They are not an automated end-to-end pipeline.

---

## 5. Experimental Tensor Surface

`Tensor` is public but incomplete. The current implementation provides the following
surface:

| Resource | Implemented operations | Important constraint |
|---|---|---|
| `Tensor::new` | Wraps a storage in a canonical row-major view | The element capacity of the storage is not validated against the shape product |
| `Tensor::zeros` | Allocates a zeroed buffer for a shape and data type | The packed variants receive a degenerate zero scale, and the ternary fill encoding is not yet materialized |
| `Tensor::get`, `Tensor::set` | Index lookup and mutation over `f32` storage | The packed variants are not supported; mutation duplicates a shared buffer before writing |
| `Tensor::transpose` | Zero-copy axis swap of shape and stride vectors | The diagnostic of an out-of-range axis identifier does not name the violated precondition |
| `Tensor::transpose_mut` | In-place axis swap of shape and stride vectors; alters metadata only and does not write to the storage buffer | The diagnostic of an out-of-range axis identifier does not name the violated precondition; the operation has no test coverage |
| `Tensor::shape`, `Tensor::strides`, `Tensor::offset`, `Tensor::numel`, `Tensor::dtype` | Geometry and type accessors | The type tag is not yet a one-byte copyable value |
| `Tensor::is_contiguous` | Row-major stride check | Requires canonical strides and a zero physical offset; the check additionally requires a non-empty backing buffer, and terminates on the packed variants |

The element-wise kernels, the reductions, the products, and the packed access paths
are specified but absent. The complete storage, view, broadcasting, and
copy-on-write behavior under
[`engineering/specs/TENSOR.md`](engineering/specs/TENSOR.md) is a target contract.
