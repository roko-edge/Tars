# Documentation

These documents describe the implementation currently present in the repository.
They do not specify unimplemented quantization modes, fixed-point runtimes, or NPU
architectures.

## Index

| Document | Scope |
|---|---|
| [`STATUS.md`](STATUS.md) | Current implementation, command status, and known limitations |
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | Rust modules, public interfaces, execution flow, and NPU structure |
| [`experiments/README.md`](experiments/README.md) | Experiment modules, Train configuration, selection, and training flow |
| [`MEM_FORMAT.md`](MEM_FORMAT.md) | Existing `weights.mem` and `activations.mem` behavior |
| [`DECISIONS.md`](DECISIONS.md) | Architectural decisions reflected in the repository |

The root [`README.md`](../README.md) contains environment and command instructions.
Repository automation policy is defined separately in [`AGENTS.md`](../AGENTS.md).

## Terminology

| Term | Meaning in the current implementation |
|---|---|
| BGD | Batch gradient descent |
| DOT | Graphviz graph-description format |
| MSE | Mean squared error |
| NPU | The SystemVerilog dot-product module in `npu/main.sv` |
