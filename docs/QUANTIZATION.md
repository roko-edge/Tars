# tars-ml — Precisão Numérica & QAT

Spec compacta dos três modos numéricos, da política de acumulador/saturação e da SFU.

---

## QAT vs. PTQ

**PTQ** (quantizar só no fim) degrada acurácia, sobretudo em redes pequenas e ternárias.
O `tars-ml` usa **QAT**: o forward pass simula exatamente a aritmética discretizada do
hardware e o backward usa **Straight-Through Estimator** — `d(quant(w))/dw ≈ 1` — para
retropropagar por arredondamentos/limiares.

---

## Modos Numéricos

| Característica | Q8.24 (Âncora) | INT8 QAT | Ternário QAT |
|---|---|---|---|
| Papel | Referência de alta precisão | Balanceado | Compressão extrema |
| Representação | 1s + 7i + 24f (comp-2) | int8 com sinal | 2 bits: `01`=+1, `11`=-1, `00`=0 |
| Faixa | -128.0 a +127.99999994 | -128 a +127 | {-1, 0, +1} |
| Resolução | 1/2^24 ≈ 5.96e-8 | 1 inteiro | discreta (3 níveis) |
| Memória/peso | 4 bytes (base) | 1 byte (**4x menor**) | 2 bits (**16x menor**) |
| Aritmética NPU | Mult 32x32 + shift | Mult 8x8 | **Sem multiplicador** (Mux 3:1) |

### Q8.24

```rust
pub fn f32_para_q8_24(v: f32) -> i32 {
    (v.clamp(-128.0, 127.9999) * 16777216.0).round() as i32
}
pub fn q8_24_para_f32(v: i32) -> f32 {
    (v as f32) / 16777216.0
}
```

No QAT Q8.24, cada resultado do forward é truncado para 24 bits fracionários — o treino
não depende da precisão extra da CPU.

### INT8 QAT

- Datapath: `int8 × int8 → acc int32`.
- Escala por camada: `S = max(|w|)/127`; quantização `q = clamp(round(w/S), -128, 127)`.
- Forward: `w_quant = clamp(round(w/S), -128, 127) * S`.
- Requantização na NPU: multiplicação por escala de ponto fixo antes da próxima camada.

### Ternário QAT

- Limiarização com `Δ = 0.7 · média(|w|)` (atualizado por época):
  `w>Δ → +1 · |w|≤Δ → 0 · w<−Δ → −1`.
- STE: gradiente flui pelo peso latente contínuo, `≈1` dentro de `|w|≤1`.
- Hardware: o multiplicador vira um mux:

```systemverilog
case (w_code)
    2'b01:   mult_out =  activation; // +1
    2'b11:   mult_out = -activation; // -1
    default: mult_out =  '0;         //  0
endcase
```

---

## Acumulador & Saturação (contrato de paridade Rust ↔ NPU)

Para garantir **zero erro de bit**, Rust e SystemVerilog seguem as mesmas regras:

| Modo | Acumulador | Motivo |
|---|---|---|
| Q8.24 | **48 bits** | 32 bits estouram acima de ~128 produtos ≈ 1.0 |
| INT8 | 32 bits | suporta milhões de acúmulos |
| Ternário | 32 bits (Q8.24) | soma/subtração de ativações |

**Saturação** na volta para 32 bits: `> +127.9999… → 0x7FFFFFFF`; `< -128.0 → 0x80000000`.
O runtime Rust implementa a mesma regra bit a bit.

---

## SFU (Special Function Unit) — ativações em hardware

| Ativação | Implementação | Custo |
|---|---|---|
| ReLU | Comparador (negativo → 0) | Irrelevante |
| Sigmoid (Q8.24) | **PWL** de 16 segmentos em `[-8, +8]`; satura em 0/1 fora | LUT ingênua de 2^24 é inviável |
| Sigmoid (INT8) | LUT direta de **256 entradas** | 256 bytes de ROM |

---

## Esqueleto do módulo parametrizado

```systemverilog
module npu_core #(
    parameter string MODE = "Q8_24",  // "Q8_24", "INT8", "TERNARY"
    parameter int IN_FEATURES  = 4,
    parameter int OUT_FEATURES = 1
) (
    input  logic clk, rst, start,
    input  logic signed [31:0] in_data, in_weight,
    output logic signed [31:0] out_result,
    output logic done
);
    logic signed [47:0] acc;

    generate
        if (MODE == "TERNARY")      begin : g_ternary  /* Mux 3:1 + soma      */ end
        else if (MODE == "INT8")    begin : g_int8    /* Mult 8x8 + acc 32b   */ end
        else                        begin : g_q8_24   /* Mult 32x32 [55:24]   */ end
    endgenerate
endmodule
```
