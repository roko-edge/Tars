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

## SystemVerilog module

`npu/main.sv` defines `npu`, parameterized by vector length `N`:

```systemverilog
module npu #(
    parameter int N
) (
    input  logic               clk,
    input  logic               rst,
    input  logic               start,
    input  logic signed [31:0] bias,
    output logic               done,
    output logic signed [31:0] result
);
```

The module contains internal `weights[N]` and `act[N]` arrays. While `start` is
asserted, it processes one array element per clock and adds `bias` when processing the
last element. Multiplication and accumulation use signed 32-bit signals; there is no
fixed-point scaling, widened accumulator, saturation, or activation function.

`npu/tb.sv` instantiates `N = 4`, loads the internal arrays with `$readmemh`, sets the
bias input to `7`, waits for `done`, and prints `result`. It does not compare the
result with an expected value.

## Integration boundary

The two implementations currently disagree at the file boundary:

| Producer or consumer | Interpretation |
|---|---|
| Rust executable | Writes each weight as the hexadecimal bits of an IEEE-754 `f32` |
| SystemVerilog module | Uses each loaded word as a signed 32-bit integer |

Consequently, the generated Rust weights do not represent equivalent numeric values
inside the NPU. There is no model-topology parser, parity test, or shared inference
pipeline.
