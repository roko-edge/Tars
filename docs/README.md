# tars-ml — Documentação (Índice & Princípios)

`tars-ml` é uma **biblioteca TinyML de Co-Design Hardware/Software**: treine redes pequenas
em Rust (Edition 2024, zero dependências de ML além do std e `rand`), e faça o deploy em
uma NPU parametrizada em SystemVerilog ou em microcontroladores via runtime em ponto fixo (`no_std`).

---

## Ordem de leitura recomendada

| # | Documento | O que contém |
|---|---|---|
| 1 | [STATUS.md](STATUS.md) | **Onde estamos agora** — estado real do código, lacunas v0 e próximo passo |
| 2 | [ARCHITECTURE.md](ARCHITECTURE.md) | Visão geral, os 3 pilares, NPU em camadas, visualização |
| 3 | [ROADMAP_NPU.md](ROADMAP_NPU.md) | Evolução v0 → v8 + critérios de conclusão (DoD) por versão |
| 4 | [DECISIONS.md](DECISIONS.md) | Registro de decisões (ADRs: Q8.24, QAT, NPU em camadas...) |
| 5 | [QUANTIZATION.md](QUANTIZATION.md) | Q8.24, INT8, Ternário, política de acumulador/saturação e SFU |
| 6 | [MEM_FORMAT.md](MEM_FORMAT.md) | Especificação `.mem` (contrato de interface Rust ↔ SystemVerilog) |

> 💡 **Perdeu o fio da meada?** Comece sempre pelo [STATUS.md](STATUS.md).

---

## Princípios inegociáveis

1. **Código 100% humano** — IAs nunca escrevem/editam código-fonte ([AGENTS.md](../AGENTS.md)).
2. **Zero bibliotecas de ML** — álgebra, otimizadores e ativações feitos à mão.
3. **Treino QAT** — o modelo aprende na precisão do hardware (Q8.24 / INT8 / Ternário).
4. **Paridade zero-erro** — o que o Rust calcula, a NPU reproduz bit a bit.
5. **NPU em camadas (ADR-009)** — blocos comuns reutilizáveis + engines especializados;
   nunca copiar a NPU nem criar um monólito.
6. **Observabilidade desacoplada (ADR-008)** — o visualizador depende do core; o core
   nunca depende do visualizador.

---

## Glossário

| Termo | Significado |
|---|---|
| **ADR** | Architectural Decision Record — registro de decisão de arquitetura |
| **CfC** | Closed-form Continuous-time Network — rede de tempo contínuo com inferência O(1) |
| **DoD** | Definition of Done — critérios objetivos de conclusão de versão |
| **GEMM** | General Matrix Multiply |
| **LTC** | Liquid Time-Constant Network — rede bio-inspirada de tempo contínuo |
| **LOD** | Level of Detail — níveis de detalhe do visualizador (1: modelo → 5: bit) |
| **MAC** | Multiply-Accumulate — operação $acc \leftarrow acc + (a \times b)$ |
| **NPU** | Neural Processing Unit |
| **QAT** | Quantization-Aware Training — treino simulando a quantização no forward |
| **Q8.24** | Fixed-point 32b: 1s + 7i + 24f (resolução $\approx 5.96 \times 10^{-8}$) |
| **SFU** | Special Function Unit — bloco de ativações em HW (ReLU, Sigmoid) |
| **STE** | Straight-Through Estimator — aproximação de gradiente por não-linearidade |
