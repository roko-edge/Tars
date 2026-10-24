# Architectural Decisions

**Audience:** TARS maintainers and engineering reviewers.

**Document type:** Record of implemented architectural decisions.

This file contains only decisions reflected in the current repository. Proposed
quantization modes, runtime targets, and future NPU organizations are not recorded as
implemented decisions.

## ADR-002: Keep Rust and SystemVerilog in one repository

**Status:** Implemented

**Decision:** Maintain the Rust package and SystemVerilog prototype in the same
repository.

**Rationale:**

- Software, hardware, and interface documentation can be reviewed together.
- Changes to memory-file production and consumption are visible in one revision.
- The development shell can provide both Rust and hardware tools.

**Current consequences:**

- Rust sources and the SystemVerilog prototype have independent build commands.
- There is no automated integration or parity test between them.
- Rust export and the testbench address the same files under `npu/data/`. The
  export calls in the training binary are commented out, so the checked-in files
  remain from an earlier export and are not overwritten by a training run. The
  remaining gaps toward a shared pipeline are the absence of topology export and
  automated parity assertions.

## ADR-003: Distribute TARS under the Apache License 2.0

**Status:** Implemented

**Decision:** The Rust package, the SystemVerilog NPU prototype, and the
documentation are distributed under the Apache License, Version 2.0.

**Rationale:**

- The project is a research initiative and requires maximal reuse, citation, and
  adoption without per-user negotiation.
- Apache-2.0 grants an explicit patent license and permits proprietary derivatives,
  which preserves a commercial licensing path for future proprietary modules while
  the published corpus remains open.
- Authorship is held jointly by the three named authors (see `AUTHORS`); external
  contributions require a contributor agreement before acceptance, to preserve
  relicensing flexibility.

**Current consequences:**

- `LICENSE`, `NOTICE`, and `AUTHORS` at the repository root define the terms.
- Versions already published remain under Apache-2.0; future versions may be
  relicensed only with the consent of all copyright holders.
- Commercial licensing terms are obtained from the authors.

## ADR-004: Container-level type dispatch instead of dtype generics

**Status:** Implemented (superseded by the typed-element revision of
[`specs/TENSOR.md`](specs/TENSOR.md), October 2026; the container-level dispatch
described here remains in the source until that migration lands)

**Decision:** `Tensor` carries its numerical representation through the container-level
`Storage` enum, and the `DType` tag is derived from the storage variant on demand.
The tensor type is not generic over the element type, and storage is not a trait
object.

**Rationale:**

- The dtype variety of this project is decided at runtime: by the training mode, by
  the layer-descriptor `mode` field, and by the contents of exported artifacts.
  Generic tensors would not remove the runtime dispatch; they would relocate it to
  every consumer that must handle the full variant set anyway.
- Quantization-aware training holds latent `F32` master weights and derived packed
  tensors in the same structures; heterogeneous collections require one concrete
  tensor type.
- The dispatch cost is one branch per operation; the kernels inside each arm are
  monomorphic, so per-element paths pay no dispatch cost.
- The pattern follows the runtimes the specification cites (PyTorch ATen,
  huggingface/candle, tracel-ai/burn, whose `TensorData` carries the dtype at
  runtime). Runtime dtype tags are the convergence point of machine-learning
  runtimes; element-type generics are the domain of fixed-type numerical array
  libraries.

**Current consequences:**

- Adding a future dtype is additive: one `Storage` variant, one `DType` tag, one
  match arm per internal operation. Public signatures do not change.
- A reserved variant without an access path is a supported state: `I8` is retained
  in the variant space with its access deferred beyond v0.1.0, under the same rule
  as the reserved descriptor `mode = 3`.
- `dtype()` is derived from the storage; the struct carries no parallel tag, so no
  pairing can drift.
