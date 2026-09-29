# Architectural Decisions

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
- The current memory-file mismatch must be handled before a shared pipeline exists.
