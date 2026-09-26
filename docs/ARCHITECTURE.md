# tars-ml — Arquitetura de Co-Design Hardware/Software

`tars-ml` é uma biblioteca **TinyML de Co-Design Hardware/Software** construída do zero
para rodar redes neurais pequenas em hardware ultra-restrito (MCUs, FPGAs, ASICs),
onde PyTorch/GPUs não entram.

---

## 🏛️ Visão Geral em 3 Pilares

```text
 ┌──────────────────────────────────────────────────────────────────────────┐
 │                        tars-ml (Plataforma TinyML)                       │
 ├────────────────────────────┬────────────────────────────┬────────────────┤
 │ PILAR 1: Engine Rust (2024)│ PILAR 2: IP Core NPU       │ PILAR 3:       │
 │ (Treino, QAT & Runtime)    │ (SystemVerilog)            │ Observabilidade│
 │                            │                            │                │
 │ • Zero dependências de ML  │ • Em camadas (ADR-009)     │ • Ferramenta   │
 │ • Treino QAT nativo       │ • Parametrizado            │   externa      │
 │   (Q8.24, INT8, Ternário)  │   (`parameter MODE`)       │   (`tars-viz`) │
 │ • Runtime `no_std` portátil│ • Zero-erro de paridade vs.│ • Métricas,    │
 │   para microcontroladores  │   Rust (.mem contract)     │   grafos e LOD │
 └────────────────────────────┴────────────────────────────┴────────────────┘
```

---

## 1. Engine de Treino & Runtime Rust (`tars-core`)

- **Zero dependências de ML**: apenas Rust (Edition 2024) e a crate `rand` para pesos iniciais.
- **Treino QAT (Quantization-Aware Training)**: o modelo já aprende nas limitações
  aritméticas do hardware target. Três modos: **Q8.24** (âncora 32b fixed-point),
  **INT8 QAT** e **Ternário QAT** (-1, 0, +1 com STE).
- **Abstração modular**: `Sequential` com `Vec<AnyModule>` — composição livre de camadas
  `Linear`, ativações (`Relu`, `Sigmoid`), etc.
- **Runtime Portátil**: compila para qualquer MCU (ARM Cortex-M, RISC-V, ESP32)
  em ponto fixo sem depender da NPU física.

---

## 2. NPU em camadas em SystemVerilog (ADR-009)

A NPU **não** é uma casca fixa, nem um monólito de `if/else`, nem uma cópia por versão:

```text
                    TARS NPU (top-level / controller)
                       │
           ┌───────────┼───────────┐
           ▼           ▼           ▼
        linear      conv         cfc          ← engines especializados
        engine      engine      engine          (novas operações físicas)
           │           │           │
           └─────┬─────┴─────┬─────┘
                 ▼           ▼
              MAC unit     SFU             ← blocos comuns reutilizáveis
              memória / saturação / QAT        (estabilizam no tempo)
```

- **Blocos comuns**: `mac_unit`, registradores de saturação/acumulação,
  memória, SFU (ativações em hardware: ReLU combinacional, Sigmoid PWL/LUT).
- **Engines especializados**: entram conforme o roadmap pede
  (`dot → linear → conv → ode → cfc`), reutilizando 100% dos blocos comuns.
- **Parametrização por modo**: `npu_core #(.MODE("Q8_24" | "INT8" | "TERNARY"))` adapta o
  datapath via `generate` (32x32 mult, 8x8 mult, Mux 3:1).

---

## 3. Observabilidade & Co-Design Benchmarking (`tars-viz`)

A observabilidade roda como **ferramenta externa/opcional** (ADR-008):
- $\boxed{\text{visualizer depende do core; core NUNCA depende do visualizer}}$.
- O `tars-core` permanece limpo para `no_std` (sem HTTP, JSON ou dependências web).
- **Métricas do Co-Design**:
  - **Acurácia (%)**: Q8.24 vs. INT8 vs. Ternário.
  - **Memória (Bytes)**: footprint do arquivo `.mem`.
  - **Hardware**: LUTs/DSPs e ciclos de clock por inferência no testbench (`tb.sv`).
  - **Paridade**: zero divergência de bits entre Rust e SystemVerilog.

---

## 🎯 Aplicações-Alvo (TinyML Edge)

1. **Healthcare Embarcado**: monitores de ECG/EEG e próteses operando em miliwatts.
2. **Robótica & Drones**: controle em tempo real lendo IMU com latência sub-milissegundo.
3. **Indústria 4.0**: manutenção preditiva via análise acústica/vibração 100% offline.
4. **Cubesats & Aeroespacial**: navegação e atitude com orçamento de energia crítico.

---

## 🚫 O que o `tars-ml` NÃO é

- Não é concorrente do PyTorch/TensorFlow para LLMs ou modelos de servidor.
- Não é software isolado nem hardware isolado — é a união dos dois via contrato `.mem`.
- Não usa caixas pretas: toda a matemática e os circuitos são abertos do bit ao algoritmo.
