# Getting Started

This guide describes the commands and behavior available in the current repository.
TARS is experimental: public interfaces may change, and some subsystems are incomplete.

---

## 1. Environment

The Nix flake provides development shells for `x86_64-linux`:

```bash
nix develop
nix develop .#rust
nix develop .#hardware
```

The default shell contains the Rust and hardware toolchains. The Rust shell includes
Graphviz and the visualization dependencies. The hardware shell includes GNU Make,
Icarus Verilog, Verilator, GTKWave, and Verible.

Without Nix, provide a Rust 2024 toolchain and the corresponding system tools manually.

---

## 2. Build the Rust Package

Run commands from the repository root:

```bash
cargo build
```

The package contains the library and two binaries, `main` and `visualization`.

`cargo test` is not currently a successful verification command. Tensor tests exist,
but they do not compile against the current `Tensor` interface.

---

## 3. Run the Training Binary

```bash
cargo run --bin main
```

The current binary:

1. Selects the OR experiment in source; it has no prompt or command-line selector.
2. Trains for the configured 100,000 epochs with non-deterministic initialization.
3. Prints periodic mean squared error values and final floating-point predictions.
4. Writes Q8.24 artifacts to `npu/weights.mem`, `npu/bias.mem`,
   `npu/activations.mem`, and `npu/target.mem`.
5. Constructs a model graph in memory without saving or rendering it.

The NPU testbench reads different files under `npu/data/`. Running the Rust binary
does not update the data consumed by `make sim`.

See [`EXPERIMENTS.md`](EXPERIMENTS.md) for the experiment contract and selection
limitations.

---

## 4. Run the Visualization Binary

```bash
cargo run --bin visualization
```

The visualization binary requires Graphviz. It builds its own hard-coded random model;
it does not consume the model trained by `main`. It writes:

| Artifact | Path |
|---|---|
| Graphviz source | `src/view/artifacts/graph.dot` |
| Rendered image | `src/view/artifacts/network.png` |

---

## 5. Run the Hardware Prototype

Hardware setup, simulation, waveform inspection, and lint commands are documented in
[`npu/README.md`](../npu/README.md). The current prototype computes a length-two Q8.24
dot product plus bias and prints results without automatic target assertions.
