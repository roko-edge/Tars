# tars-ml

**TinyML Hardware/Software Co-Design** library built from scratch: small neural networks trained in Rust (Edition 2024, zero ML dependencies), executed on a parameterized SystemVerilog NPU or on microcontrollers through a fixed-point runtime.

---

## Environment

With Nix (recommended — includes hardware tooling):

```bash
nix develop             # Rust + hardware tools + support
nix develop .#hardware  # only iverilog / verilator / gtkwave
```

Without Nix: install [rustup](https://rustup.rs). Note: outside the Nix environment, `cargo build` fails when building `plotters`, which requires the `fontconfig` system library.

## Build and Run (Rust)

```bash
cargo build
cargo run --bin main           # demo training
cargo run --bin visualization  # visualization prototype
cargo test
```

## NPU (SystemVerilog)

```bash
cd npu
make sim     # builds with iverilog (-g2012) and runs in the terminal
make wave    # opens generated waveforms in GTKWave
make lint    # static analysis with Verilator
make clean   # cleans the build/ directory
```

## Documentation

Full index at [docs/README.md](docs/README.md). Main documents:

| Document | Content |
|---|---|
| [docs/STATUS.md](docs/STATUS.md) | Current codebase status and next steps |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Architecture and the three core pillars |
| [docs/ROADMAP_NPU.md](docs/ROADMAP_NPU.md) | Evolution from v0 to v8 with completion criteria |
| [docs/DECISIONS.md](docs/DECISIONS.md) | Architecture Decision Records (ADRs) |

## Principles

1. **100% human-written code** — AI does not write code in this repository ([AGENTS.md](AGENTS.md)).
2. **Zero ML libraries** — algebra, optimizers, and activation functions are implemented from scratch.
3. **Training at target precision (QAT)** — Q8.24, INT8, and ternary; never post-training quantization.
4. **Zero-error parity** — whatever Rust computes, the NPU reproduces bit-for-bit.
