# Research Logs and Academic Publication Notes

This directory contains empirical research logs, mathematical derivations, hardware/software co-design trade-offs, and experimental benchmarks for TARS.

The contents of this directory serve as the primary source material for compiling academic publications on TARS (e.g., hardware-efficient NPU acceleration, ternary quantization loss bounds, and bit-exact co-simulation).

---

## Log Index

| ID | Title | Domain | Key Finding / Artifact |
|---|---|---|---|
| `LOG-000` | Research Logging Protocol & Paper Outline | Methodology | Template and target paper section mapping |

---

## Paper Structural Mapping

Logs in this directory map directly to the planned academic paper sections:

```text
Paper Section                      Corresponding Research Logs
─────────────────────────          ───────────────────────────────────────────
1. Introduction & Background       LOG-000 (Motivation, Co-Design Philosophy)
2. Software Tensor Runtime         LOG-001 (Zero-Copy Strided Tensor Engine)
3. Quantization & Ternary Models   LOG-002 (Q8.24 vs. Ternary Loss Bounds)
4. SystemVerilog NPU Architecture  LOG-003 (Layer Engine FSM & Accumulator Width)
5. Experimental Results (MNIST)    LOG-004 (Accuracy vs. Quantization Bit-Width)
6. Conclusion & Future Work        LOG-005 (Extension to CNNs and CfC Networks)
```

---

## Logging Guidelines

1. **Self-Contained Empirical Proofs:** Every log must contain exact reproducible commands, seeds, and quantitative tables.
2. **Mathematical Rigor:** Equations must be typeset in LaTeX math syntax ($y = f(Wx + b)$).
3. **Hardware Waveforms & RTL Metrics:** RTL changes must include cycle counts, LUT/register utilization estimates, or GTKWave waveform analysis.
