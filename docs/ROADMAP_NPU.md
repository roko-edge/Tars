# tars-ml — Roadmap v0 → v8 (Co-Design Hardware/Software)

Progressão incremental: fundamentos de álgebra linear em Rust (Edition 2024) até
NPUs dedicadas em SystemVerilog para redes de tempo contínuo (CfC).

---

## Como executar o roadmap

1. **Modo Âncora (`Q8_24`)**: caminho crítico obrigatório de **toda** versão.
   Nenhuma versão avança sem validação completa em Q8.24.
2. **Modos Estendidos (`INT8`, `TERNARY`)**: implementados em todas as versões,
   com DoD formal exigido apenas nos **marcos de destaque**: v0, v0.5, v1, v6 e v7.
3. **Regra do "Não Empacar"**: se travar na quantização/ternarização de uma versão,
   feche o marco em Q8.24, registre no [STATUS.md](STATUS.md) e avance.
4. **Hardware em camadas (ADR-009)**: cada versão adiciona apenas o **engine** da
   operação nova; MAC, SFU, memória e saturação são sempre reutilizados.

---

## Matriz de Precisão por Versão

| Versão | Q8.24 (Âncora) | INT8 QAT | Ternário QAT |
|---|---|---|---|
| v0: Neural Core | Matrix Q8.24 | Matrix INT8 | Mux 3:1 |
| v0.5: Loop Closure | XOR na NPU | XOR INT8 | XOR Ternária |
| v1: Dense MLP | Systolic Q8.24 | Systolic INT8 | Systolic Ternary |
| v2: Spatial CNN | LineBuffer Q8.24 | LineBuffer INT8 | LineBuffer Tern |
| v3: Multi-Ch CNN | DoubleBuffer Q8 | DoubleBuffer INT8 | DoubleBuf Tern |
| v4: Neural ODE | Integrator Q8.24 | Integrator INT8 | Integrator Tern |
| v5: LTC | Liquid Core Q8 | Liquid Core INT8 | Liquid Core Tern |
| v6: CfC | CFE Core Q8.24 | CFE Core INT8 | CFE Core Ternary |
| v7: Sparsidade | Packing & Skip Q8 | Zero-Skip INT8 | Zero-Skip Tern |
| v8: Streaming Edge | Event DMA Q8.24 | Event DMA INT8 | Event DMA Tern |

---

## Detalhamento e Critérios de Conclusão (DoD)

### v0 — Neural Core & Matrix Engine

> Álgebra linear contígua, backprop analítico, QAT e produto escalar na NPU.

- **Rust**: tensores/matrizes com memória contígua; multiplicação/soma/Hadamard/transposição;
  derivadas analíticas (`σ' = σ(1-σ)`, `ReLU'(x) = x>0 ? 1 : 0`); QAT (Q8.24, INT8, Ternário)
  com STE; `exporter.rs` gerando `.mem` ([MEM_FORMAT.md](MEM_FORMAT.md)).
- **NPU**: `npu_core #(.MODE)` com acumulador de 48 bits, saturação e SFU com ReLU.

- [ ] XOR (2→2→1) e regressão linear com MSE < 0.01 em Q8.24
- [ ] Exporter gera `.mem` válido nos 3 modos
- [ ] Dot product de 4 elementos no `tb.sv` com **zero divergência** vs. Rust nos 3 modos

### v0.5 — Fechamento do Loop de Co-Design

> Fechar o ciclo completo Rust→NPU numa rede inteira antes de escalar (ADR-005).

- **NPU**: testbench lê `model.mem` e guia a NPU camada a camada pela rede XOR.

- [ ] Inferência completa das 4 combinações do XOR em SystemVerilog
- [ ] Paridade **zero erro de bit** vs. `expected.mem` em Q8.24
- [ ] Ciclos de clock por inferência registrados no [STATUS.md](STATUS.md)

### v1 — Dense MLP & Array Sistólico

- **Rust**: `DenseLayer`; Softmax estável + CCE (gradiente `y_hat − y`);
  SGD com momentum; loader MNIST (IDX binário).
- **NPU**: `npu_systolic` — array 8x8 de PEs + SRAM de pesos por camada.

- [ ] MNIST Q8.24 > 95% · INT8 > 93% · Ternário > 90% *(hipóteses — recalibrar)*
- [ ] Paridade zero-erro em 100 amostras de teste

### v2 — Spatial CNN & Line-Buffer Engine

- **Rust**: `Conv2D`, `MaxPool2D`, `AvgPool2D`, `Flatten`; `im2col`+GEMM; Adam.
- **NPU**: `npu_cnn` — line buffer com shift registers para janelas KxK + max-pooling em HW.

- [ ] MNIST Q8.24 > 98%
- [ ] Convolução 28x28 sem acessos redundantes à memória externa

### v3 — Multi-channel CNN & Double Buffering

- **Rust**: conv multicanal `Cin→Cout`, `BatchNorm2D` (fusão na inferência), AdamW + cosine
  annealing; loader CIFAR-10.
- **NPU**: `npu_multichannel` — SRAM ping-pong (processa C enquanto carrega C+1).

- [ ] CIFAR-10 Q8.24 > 75%
- [ ] Fusão de BatchNorm nos pesos verificada no testbench

### v4 — Neural ODE & Integrador Temporal

- **Rust**: `dh/dt = f_θ(h,t)`; solvers Euler e RK4; backprop via adjoint state com QAT.
- **NPU**: `npu_ode` — pipeline reintroduzindo os 4 estágios do RK4 no núcleo matricial.

- [ ] Reconstrução de trajetória sintética (pêndulo/espiral) com MSE < 0.05
- [ ] Pipeline RK4 em ciclos determinísticos

### v5 — LTC (Liquid Time-Constant Core)

- **Rust**: célula LTC com condutâncias solúveis; AdamW com clipping `|grad| ≤ 1.0`.
- **NPU**: `npu_liquid` — solver exponencial não-linear para sigmoides/exponenciais.

- [ ] Série temporal irregular/gaps superando baseline RNN
- [ ] Registradores de feedback de estado sem corrupção

### v6 — CfC (Closed-Form Continuous-Time NPU)

- **Rust**: célula CfC `h(t) ≈ (f(x,h₀) ⊙ e^(-(A(x,h₀)+b)t)) + g(x,h₀)`; SiLU/Swish e Tanh com QAT.
- **NPU**: `npu_cfc` — Fast Closed-Form Engine: 3 sub-blocos matriciais paralelos +
  `e^(-x)` em ponto fixo (CORDIC ou PWL).

- [ ] Paridade de acurácia com LTC com inferência O(1) (sem passos de solver)
- [ ] `h(t)` em número fixo de ciclos determinísticos por amostra

### v7 — Sparsidade & Eficiência Energética

- **Rust**: Sparsity-Aware QAT forçando alta fração de pesos nulos; exportador com
  packing denso (16 pesos ternários de 2 bits por palavra de 32 bits).
- **NPU**: `npu_cfc_sparse` — **Zero-Value Skipping** (pula o ciclo quando peso = 0) +
  pack/unpack de bits na leitura da SRAM.

- [ ] CfC ternário esparso retém ≥ 95% da acurácia Q8.24
- [ ] Redução de ciclos proporcional à esparsidade, medida no `tb.sv`

### v8 — Streaming Edge & Standby Ativo

- **Rust**: pipeline orientado a eventos com **zero alocação dinâmica** (`no-malloc`/`no_std`).
- **NPU**: `npu_streaming` — Direct Sensor DMA (IMU/ECG), counter de `dt` automático para
  a CfC, **Wake-on-Event** (dorme mantendo estado na SRAM).

- [ ] Inferência compila sem warnings de alocação e roda em tempo real
- [ ] `tb.sv` demonstra *sleep → wake → inferência → sleep*

---

## Métricas obrigatórias a cada versão (benchmarking Co-Design)

1. **Paridade numérica**: 0 erros de bit Rust ↔ SystemVerilog.
2. **Memória**: bytes dos arquivos `.mem`.
3. **Ciclos**: clocks por inferência medidos no `tb.sv`.

---

## Referências

1. Hasani et al. *Closed-form continuous-time neural networks*. Nature MI, 2022.
2. Hasani et al. *Liquid Time-constant Networks*. AAAI, 2021.
3. Jacob et al. *Quantization and Training of Neural Networks for Efficient
   Integer-Arithmetic-Only Inference*. CVPR, 2018.
4. Li, Zhang, Liu. *Ternary Weight Networks*. arXiv:1605.04711, 2016.
5. Ma et al. *The Era of 1-bit LLMs*. Microsoft Research, 2024.
