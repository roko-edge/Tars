# tars-ml — Formato `.mem` (contrato Rust ↔ SystemVerilog)

Contrato de interface entre o **exporter Rust** e a **NPU/testbench** via `$readmemh`.
Palavras de 32 bits em hex, uma por linha; comentários `//` são ignorados pelo Verilog.

---

## Arquivos por modelo treinado

| Arquivo | Conteúdo | Consumidor |
|---|---|---|
| `model.mem` | Cabeçalho de topologia + pesos/biases por camada | `npu_core.sv` |
| `input.mem` | Vetores de entrada de teste | `tb.sv` |
| `expected.mem` | Saídas calculadas pelo Rust (validação de paridade) | `tb.sv` |

---

## Layout do `model.mem`

```text
// tars-ml model export v1
// MODE: Q8_24
// TOPOLOGY: 2 -> 2 -> 1
00000002    // Palavra 0: N de camadas com pesos
00000002    // Palavra 1: in_features  da camada 0
00000002    // Palavra 2: out_features da camada 0
00000001    // Palavra 3: out_features da camada 1
// --- CAMADA 0: PESOS (2x2, row-major) ---
00800000    // W[0][0] = +0.5
FF800000    // W[0][1] = -0.5
00C00000    // W[1][0] = +0.75
FF400000    // W[1][1] = -0.75
// --- CAMADA 0: BIASES (2) ---
00100000    // B[0] = +0.0625
00000000    // B[1] = 0.0
// --- CAMADA 1: PESOS (1x2) ---
01000000    // W[0][0] = +1.0
FF000000    // W[0][1] = -1.0
// --- CAMADA 1: BIASES (1) ---
00050000    // B[0] = +0.0195
```

---

## Codificação por modo

v0–v6: cada parâmetro ocupa **uma palavra inteira de 32 bits** (simplicidade de carga).
O packing denso (16 ternários/palavra) entra na **v7**.

| Modo | Regra | Exemplos |
|---|---|---|
| `Q8_24` | int32 em comp-2, 8 dígitos hex | `+1.0→01000000` · `-1.0→FF000000` · `+0.5→00800000` · `-0.5→FF800000` |
| `INT8` | byte nos 8 bits inferiores, 24 superiores zerados | `+127→0000007F` · `-128→00000080` · `0→00000000` |
| `TERNARY` | código de 2 bits nos bits inferiores | `+1→00000001` · `-1→00000003` · `0→00000000` (`10` reservado → 0) |

---

## Validação de paridade

```text
// input.mem — XOR [1.0, 0.0] em Q8.24
01000000    // In[0] = +1.0
00000000    // In[1] = 0.0

// expected.mem — saída do Rust
00FC0000    // Out[0] ≈ +0.984
```

```systemverilog
module tb;
    logic signed [31:0] model_ram [0:1023];
    logic signed [31:0] test_input [0:15];
    logic signed [31:0] expected_out [0:15];

    initial begin
        $readmemh("build/model.mem",   model_ram);
        $readmemh("build/input.mem",   test_input);
        $readmemh("build/expected.mem", expected_out);
        $display("Modelo carregado. Camadas: %0d", model_ram[0]);
    end
endmodule
```
