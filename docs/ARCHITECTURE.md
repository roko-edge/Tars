# Architecture

The repository contains one Rust package and one SystemVerilog prototype. They share
the repository but do not yet share a compatible numeric representation or model
format.

## Repository structure

```text
Cargo.toml
src/
  lib.rs                  Public module exports and re-exports
  train.rs                Shared Train configuration
  experiments/
    mod.rs                Public experiment modules
    experiment_type.rs    OR/XOR identifiers
    or.rs                 OR configuration factory
    xor.rs                XOR configuration factory
    local.rs.template     Incomplete local-experiment starting point
  data.rs                 Training samples
  math.rs                 Scalar activations and random initialization
  modules.rs              Module trait and concrete-module enum
  modules/
    linear.rs             Dense layer
    activations.rs        ReLU and sigmoid modules
    sequential.rs         Ordered module composition
  optim.rs                Optimizer module exports
  optim/
    cost.rs               Mean squared error
    grad.rs               Finite-difference gradients
    bgd.rs                Batch gradient descent
  view.rs                 Visualization module export
  view/plot.rs            petgraph and Graphviz integration
  bin/
    main.rs               Training executable with source-selected experiment
    visualization.rs      Hard-coded graph visualization executable
npu/
  main.sv                 Parameterized dot-product module
  tb.sv                   Simulation testbench
  Makefile                Simulation, waveform, and lint targets
  activations.mem         Testbench activation values
  weights.mem             Weight values
```

## Rust model

`Module` defines the inference interface:

```rust
pub trait Module {
    fn forward(&self, input: &[f32]) -> Vec<f32>;
}
```

`AnyModule` provides runtime dispatch over the implemented module types:

```rust
pub enum AnyModule {
    Linear(Linear),
    Activation(Activation),
}
```

`Sequential` stores an ordered `Vec<AnyModule>`. Its builder methods update the
expected feature count when a linear layer is appended.

```rust
impl Sequential {
    pub fn new(input_size: usize) -> Self;
    pub fn linear(self, output_size: usize) -> Self;
    pub fn relu(self) -> Self;
    pub fn sigmoid(self) -> Self;
}
```

`Linear` stores weights as `Vec<Vec<f32>>`, indexed by output feature and then input
feature. Biases are stored as `Vec<f32>`.

```rust
impl Linear {
    pub fn new(
        in_sz: usize,
        out_sz: usize,
        weights: Vec<Vec<f32>>,
        bias: Vec<f32>,
    ) -> Self;
    pub fn random(in_sz: usize, out_sz: usize) -> Self;
}
```

The remaining training interfaces are:

```rust
impl Data {
    pub fn new(input: &[f32], target: &[f32]) -> Self;
}

pub fn cost(model: &Sequential, data: &[Data]) -> f32;
pub fn num_grad(model: &Sequential, data: &[Data]) -> Grad;

impl BGD {
    pub fn new(lr: f32) -> Self;
    pub fn step(&self, model: &mut Sequential, grad: &Grad);
}
```

`cost` computes the mean squared error across output elements and samples.
`num_grad` computes centered finite differences with `h = 1e-3` for every weight and
bias. `BGD::step` applies the resulting gradient to each linear layer.

## Training executable

`src/bin/main.rs` performs the following operations:

1. Calls `experiments::xor::train()` to construct the selected `Train` configuration.
2. Prints its `ExperimentType` identifier and initial MSE.
3. Trains with full-dataset analytical backpropagation and BGD, reporting progress.
4. Prints predictions for the configured dataset.
5. Builds a `NetGraph` from the trained model without saving or rendering it.

`Train` is defined in `src/train.rs` and re-exported as `tars::Train`. Experiment
factories provide the model, epochs, learning rate, dataset, and identifier. Selection
is fixed by the factory call in the executable source; there is no interactive
selector. The NPU export calls are currently commented out.

See [`experiments/README.md`](experiments/README.md) for the configuration contract,
built-in experiments, reporting behavior, and manual extension workflow.

## Visualization

The visualization code is part of the `tars` package and is exported through
`tars::view`. `NetGraph` wraps `petgraph::graph::DiGraph<Perceptron, f32>` and exposes
the following interface:

```rust
impl NetGraph {
    pub fn new() -> Self;
    pub fn to_dot(&self) -> String;
    pub fn save_dot(&self, path: &str) -> std::io::Result<()>;
    pub fn add_node(&mut self, p: Perceptron);
    pub fn add_edge(&mut self, from: usize, to: usize, w: f32);
    pub fn plot(&self);
}
```

Node indices are retained in insertion order. `plot` invokes the external `dot`
command with fixed input and output paths. The visualization executable constructs a
graph directly and is not connected to `Sequential`.

## SystemVerilog Hardware Subsystem

`npu/src/npu.sv` defines the top-level `npu` module, parameterized by vector length `N`:

```systemverilog
module npu #(
    parameter int N = 2
) (
    input  logic               clk,
    input  logic               rst,
    input  logic               start,
    output logic               done,
    output logic signed [31:0] result
);
```

The top-level NPU module uses a state machine (`IDLE`, `BUSY`) and delegates multiply-accumulate operations to the Processing Element (`pe.sv` in `npu/src/`), which maintains a 64-bit signed accumulator (`pe_acc`) operating on Q8.24 fixed-point words. Internal memory arrays (`b`, `w`, `a`) are populated via `$readmemh` from `data/` memory files.

`npu/tests/tb.sv` instantiates the DUT (`N = 2`), loads `data/bias.mem`, `data/weights.mem`, and `data/activations.mem`, executes multi-sample dot product evaluations, logs trace outputs via `$dumpfile` / `$dumpvars`, and displays formatted Q8.24 results.

## Integration Boundary

The Rust export module (`src/export.rs`) converts floating-point weights, biases, inputs, and targets into Q8.24 fixed-point hexadecimal words (`to_q8_24` with truncation):

| Artifact File | Format | Source Generator | Consumer |
|---|---|---|---|
| `weights.mem` | Q8.24 Hex | `export_model()` | `npu/src/npu.sv` (`w` array) |
| `bias.mem` | Q8.24 Hex | `export_model()` | `npu/src/npu.sv` (`b` array) |
| `activations.mem` | Q8.24 Hex | `export_data()` | `npu/tests/tb.sv` (`a` array) |
| `target.mem` | Q8.24 Hex | `export_data()` | Testbench verification |

The integration boundary shares the Q8.24 numeric representation with truncation semantics across Rust and SystemVerilog. Full end-to-end topology compilation and bit-exact golden vector assertions are governed by Phase 3 and Phase 4 roadmap milestones.
