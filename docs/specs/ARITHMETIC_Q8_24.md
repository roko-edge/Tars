# Specification: Q8.24 Fixed-Point Arithmetic and Bit-Exact Parity

This document specifies the shared numeric contract between the Rust training runtime
and the SystemVerilog NPU. Compliance is mandatory for bit-exact hardware parity.

> **V0 Scope:** The full v0.1.0 release requirements, including MNIST benchmark targets,
> ternary quantization, and research logging protocol, are defined in
> [`docs/specs/V0_CONTRACT.md`](V0_CONTRACT.md).

---

## 1. Numeric Format Definition

| Property | Value |
|---|---|
| Total width | 32 bits |
| Sign representation | Two's complement, signed |
| Integer bits (incl. sign) | 8 |
| Fractional bits | 24 |
| Value interpretation | $v = \frac{w}{2^{24}}$ where $w$ is the signed 32-bit word |

Valid representable range: $[-128.0, +127.999999940\ldots]$.

---

## 2. Conversion Contract (Rust, `src/export.rs`)

### 2.1 Encoding (float to fixed)

The canonical encoding function in the Rust runtime must satisfy:

$$w = \text{clip}_{\text{int32}} \left( \text{round}(x \cdot 2^{24}) \right)$$

where:

- `round` is round-half-to-even (ties to even), matching IEEE-754 default rounding.
- `clip_int32` clamps the value to $[-2^{31}, 2^{31} - 1]$ to prevent silent overflow.

### 2.2 Current Deviation (Normative Warning)

The existing `to_q8_24` implementation performs **truncation**:

```rust
pub fn to_q8_24(x: f32) -> u32 {
    ((x * 16_777_216.0) as i32) as u32
}
```

The `as` cast truncates toward zero. Until both the exporter and the NPU arithmetic
specification agree on rounding mode, **truncation remains the normative behavior**
for v1 of this contract. Any change requires updating this specification, `src/export.rs`,
and `npu/src/npu.sv` / `npu/src/pe.sv` in the same revision (Engineering Invariant 4 of the ROADMAP).

---

## 3. Arithmetic Contract (SystemVerilog, `npu/src/`)

### 3.1 Multiplication

The product of two Q8.24 values $a \cdot b$ requires $8 + 24 + 8 + 24 = 64$ bits (Q16.48):

```systemverilog
logic signed [63:0] raw_mult;
assign raw_mult = 64'(a) * 64'(b);  // Q16.48
```

To return the result to Q8.24, the fractional length must be reduced from 48 to 24 bits.
Under the **truncation** regime:

```systemverilog
assign mult = raw_mult[55:24];  // Truncates 24 low-order fractional bits
```

### 3.2 Accumulation

Accumulation of $N$ products may exceed 32 bits. The accumulator must be widened:

$$\text{acc}_{64} = \sum_{i=0}^{N-1} \text{mult}_i$$

The NPU must implement saturation after the accumulation completes:

$$\text{acc}_{32} = \begin{cases}
+2^{31}-1 & \text{if } \text{acc}_{64} > +2^{31}-1 \\
-2^{31} & \text{if } \text{acc}_{64} < -2^{31} \\
\text{acc}_{64} & \text{otherwise}
\end{cases}$$

### 3.3 Current Deviation (Normative Warning)

The current RTL maintains the accumulator in 32 bits without saturation. This deviation is
a known limitation of the prototype and is scheduled for correction in Phase 2 of the
ROADMAP (see `docs/roadmap/ROADMAP.md`).

---

## 4. Parity Test Protocol

Bit-exact parity between Rust golden inference and NPU simulation is verified through
golden vector exchange:

```text
Rust Runtime                              NPU Simulation
────────────                              ──────────────
1. train model (seeded PRNG)
2. run inference on dataset
3. quantize activations & weights
   (to_q8_24, truncation)
4. write:
   ├── activations.mem                   ├── $readmemh(activations)
   ├── weights.mem                       ├── $readmemh(weights)
   ├── bias.mem                          ├── $readmemh(bias)
   └── expected.mem  ─────────────────►  ├── compute (dot product + bias)
                                          └── assert result == expected
                                          (PASS/FAIL + $fatal on mismatch)
```

The testbench must terminate with a non-zero exit status on any mismatch to support
automated CI integration (Make target: `make test`).
