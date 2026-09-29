# Implementation Status

The repository is an early experimental implementation. The Rust model and the
SystemVerilog NPU prototype execute independently and do not currently provide
numerically equivalent inference.

## Rust package

| Component | Status | Implementation |
|---|---|---|
| Sequential model | Implemented | `Sequential` stores `Vec<AnyModule>` |
| Dense layer | Implemented | `Linear` uses `Vec<Vec<f32>>` weights and `Vec<f32>` biases |
| Activations | Implemented | ReLU and sigmoid |
| Loss | Implemented | Mean squared error over outputs and samples |
| Gradient | Implemented | Centered finite differences with `h = 1e-3` |
| Optimizer | Implemented | Batch gradient descent |
| Experiments | Implemented | Interactive OR and XOR selection |
| Weight output | Partial | Writes only IEEE-754 weight bits to `npu/weights.mem` |
| Automated tests | Not implemented | No Rust `#[test]` functions are present |

The Rust implementation uses `f32`, `std`, and dynamically allocated vectors. It has
no analytical backpropagation, quantization-aware training, fixed-point arithmetic,
or `no_std` runtime.

## Visualization

The package exports a `view` module backed by `petgraph`. The visualization binary
creates a hard-coded graph, saves DOT text, and invokes Graphviz to generate a PNG.
It does not inspect or render a `Sequential` model. `plotters` and `petgraph` are
unconditional package dependencies.

## SystemVerilog prototype

| Component | Status | Implementation |
|---|---|---|
| Dot product | Implemented | One signed 32-bit product and accumulation per clock |
| Vector length | Parameterized | `N`, instantiated as `4` by the testbench |
| Bias | Implemented | External signed 32-bit input added to the final result |
| Control | Implemented | `rst`, `start`, and `done` signals |
| Memory loading | Partial | Testbench loads arrays through hierarchical access |
| Result validation | Not implemented | Testbench prints without asserting an expected value |
| Waveform output | Not implemented | Testbench has no `$dumpfile` or `$dumpvars` calls |

The module has no numeric-mode parameter, widened accumulator, saturation logic,
hardware activation function, model controller, or layer engine.

## Rust and NPU compatibility

The Rust executable writes weights with `f32::to_bits()`. The SystemVerilog module
interprets loaded words as signed integers. Biases and topology are not exported, and
the testbench does not consume Rust-generated inputs or expected outputs. Bit-exact
parity is therefore not defined or tested.

The checked-in NPU configuration also expects four weights, while the checked-in
`weights.mem` contains two words.

## Command status

| Command | Status | Notes |
|---|---|---|
| `cargo build` | Available | Builds the Rust package and binaries |
| `cargo run --bin main` | Available | Prompts for OR or XOR and overwrites `npu/weights.mem` |
| `cargo run --bin visualization` | Available | Requires Graphviz and writes fixed artifact paths |
| `cargo test` | Available | No automated tests are defined |
| `make sim` | Partial | Runs the testbench without pass/fail validation; current weights are incomplete |
| `make sim2` | Broken | References absent `main2.sv` and `tb2.sv` |
| `make test` / `make all` | Broken | Depend on `sim2` |
| `make wave` | Broken | Expects a VCD file that the testbench does not generate |
| `make wave2` | Broken | Depends on absent ternary sources and waveform output |
| `make lint` | Broken | Its second invocation references absent ternary sources |
| `make clean` | Available | Removes NPU build artifacts |

## Known technical limitations

- Model dimensions are not validated against input, target, weight, or bias lengths.
- `AnyModule::as_linear` and `as_mut_linear` panic when called for an activation.
- Training cost scales poorly because each parameter requires two complete cost
  evaluations per gradient step.
- Weight initialization uses unscaled uniform random values.
- The visualization uses fixed paths and reports Graphviz process failures only at
  runtime.
- The NPU uses internal arrays loaded directly by the testbench rather than an
  external memory interface.
