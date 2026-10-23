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
