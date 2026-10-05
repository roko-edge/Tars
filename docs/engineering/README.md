# Internal Engineering Records

This area contains architecture descriptions, implemented decisions, technical
contracts, and proposals for TARS maintainers. It is not user documentation and does
not establish that a planned capability is available.

---

## Navigation by Task

| Objective | Document | State |
|---|---|---|
| Understand the current repository structure | [`ARCHITECTURE.md`](ARCHITECTURE.md) | Current implementation record |
| Review implemented architectural decisions | [`DECISIONS.md`](DECISIONS.md) | Implemented decisions only |
| Inspect the Rust-to-RTL artifact format | [`MEM_FORMAT.md`](MEM_FORMAT.md) | Current format with disconnected paths |
| Evaluate the observability design | [`OBSERVABILITY_PROPOSAL.md`](OBSERVABILITY_PROPOSAL.md) | Proposal, not implemented |
| Review normative and target contracts | [`specs/README.md`](specs/README.md) | Mixed current and target contracts with explicit status |

---

## Navigation by Engineering Domain

| Domain | Primary records |
|---|---|
| Rust model architecture | [`ARCHITECTURE.md`](ARCHITECTURE.md), [`specs/MODEL_TRAIT.md`](specs/MODEL_TRAIT.md) |
| Tensor engine | [`specs/TENSOR.md`](specs/TENSOR.md) |
| NPU arithmetic and integration | [`MEM_FORMAT.md`](MEM_FORMAT.md), [`specs/ARITHMETIC_Q8_24.md`](specs/ARITHMETIC_Q8_24.md) |
| Visualization and telemetry | [`OBSERVABILITY_PROPOSAL.md`](OBSERVABILITY_PROPOSAL.md) |

Current project progress and scheduling belong in
[`../project/`](../project/README.md).
