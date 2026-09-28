# tars-ml — Registro de Decisões (ADRs)

Cada ADR registra **uma decisão, o porquê e as consequências**. Regras de escrita:
decisão em uma frase; justificativas em bullets; nada de prosa.

---

## ADR-001 — Q8.24 como formato numérico âncora

**Decisão.** Todo o projeto usa ponto fixo **Q8.24** (1 sinal + 7 inteiros + 24 fracionários)
como formato de referência obrigatório de cada versão.

**Por quê.**
- Multiplicadores de ponto flutuante (IEEE 754) custam área e energia demais na NPU.
- 24 bits fracionários ≈ mantissa do `f32` (23 bits) → perda imperceptível vs. Rust.
- Faixa ±128 evita overflow em pesos/biases/ativações (Q4.28 estoura, Q16.16 desperdiça bits).

**Consequências.** Acumulador estendido de 48 bits na NPU para somas longas (ver
[QUANTIZATION.md](QUANTIZATION.md)).

---

## ADR-002 — Monorepo: Rust e SystemVerilog juntos

**Decisão.** Engine Rust e NPU em SystemVerilog vivem **no mesmo repositório**.

**Por quê.**
- Mudança no formato `.mem` do Rust quebraria a NPU imediatamente — no monorepo o CI pega no
  mesmo commit.
- Co-simulação (treino → export → paridade) em um único pipeline.

---

## ADR-003 — Treino Consciente da Precisão (QAT), nunca PTQ

**Decisão.** O modelo **treina simulando a aritmética do hardware** (QAT), com
Straight-Through Estimator no backward pass; quantização pós-treino (PTQ) não é usada.

**Por quê.** PTQ degrada acurácia, principalmente em redes pequenas e ternárias.
Com QAT os pesos nascem adaptados à baixa precisão.

---

## ADR-004 — Datapath selecionado em tempo de síntese via `parameter MODE`

**Decisão.** Um único módulo parametrizado (`npu_core #(.MODE(...))`) cobre Q8.24,
INT8 e Ternário usando `generate` — **não** arquivos separados (`npu_q8.sv`, `npu_int8.sv`, ...).

**Por quê.** Três datapaths distintos, zero duplicação de código.

---

## ADR-005 — Marco intermediário v0.5 (fechar o loop antes de escalar)

**Decisão.** Entre o dot product da v0 e o array sistólico da v1 existe a **v0.5**:
a rede XOR inteira rodando na NPU, camada por camada, com paridade zero-erro.

**Por quê.** O salto v0→v1 era grande demais; fechar o ciclo completo
Rust→`.mem`→NPU→paridade primeiro reduz risco.

---

## ADR-006 — v7 re-escopada para Sparsidade

**Decisão.** v7 = **Sparsidade & Eficiência Energética**: indução de pesos nulos (Sparsity QAT)
+ Zero-Value Skipping e empacotamento denso (16 pesos ternários/palavra) na NPU.

**Por quê.** Com a quantização distribuída ao longo de todo o roadmap, a v7 perdeu
seu propósito original ("CfC quantizada").

---

## ADR-007 — Contrato `.mem` em texto hexadecimal

**Decisão.** Exportação de pesos em arquivos de texto hex legíveis nativamente pelo
`$readmemh`, com cabeçalho de topologia. Spec: [MEM_FORMAT.md](MEM_FORMAT.md).

**Por quê.** Formato universal, zero parser customizado, comentários `//` ignorados
pelo próprio Verilog.

---

## ADR-008 — Visualização desacoplada do core

**Decisão.** O visualizador (`tars-viz`) é ferramenta externa; regra de dependência
**unidirecional**: visualizador depende do core, o core **nunca** depende do visualizador.

**Por quê.** Web/JSON/WebSocket dentro do `tars-core` quebraria `no_std` e a pureza
do runtime TinyML. O core deve sobreviver à deleção total do visualizador
(*Teste da Deleção Total*).

**Consequências.** Crates separadas: `tars-viz-protocol` (tipos agnósticos),
`tars-viz-server`, `tars-viz-web`. O core instala limpo via `cargo add tars`.

---

## ADR-009 — NPU em camadas: blocos comuns estáveis + engines especializados

> Decisão central de arquitetura de hardware. Vale para toda a evolução v0 → v8.

**Decisão.** A NPU é uma **composição**, não um bloco único:

```text
                    TARS NPU (top-level)
                       │
           ┌───────────┼───────────┐
           ▼           ▼           ▼
        linear      conv         cfc          ← engines (o que muda)
        engine      engine      engine
           │           │           │
           └─────┬─────┴─────┬─────┘
                 ▼           ▼
              MAC unit     SFU             ← blocos comuns (o que fica)
              memória / saturação / quantização
```

1. **Blocos comuns** — `mac_unit`, aritmética/saturação, memória, SFU, interfaces.
   Estabilizam e são reaproveitados por **todas** as versões.
2. **Engines especializados** — `dot → linear → conv → ode → cfc`. Cada um nasce
   apenas quando o roadmap pede, **sempre reusando os blocos comuns** (o CfC não
   reimplementa MAC, memória nem Sigmoid; acrescenta só estado `h`, `Δt` e `exp(-x)`).

**A regra para qualquer nova demanda de hardware:**

> **Isso muda apenas configuração/dimensões/precisão — ou introduz uma operação
> fisicamente nova?**

| Mudança | Tratamento | Exemplos |
|---|---|---|
| Configuração | **Parametrizar** o mesmo engine (`#(.MODE, .IN_FEATURES, ...)`) | 2→6 vira 128→64; Q8.24 vira Ternário |
| Operação nova | **Novo engine** reaproveitando os blocos comuns | dense→conv (line buffers, janelas 3x3); dense→ODE (estado, realimentação, RK4); ODE→CfC (3 matriciais paralelos + `exp(-x)`) |

**Alternativas rejeitadas.**
- *Copiar a NPU a cada versão* (`npu_v0.sv` → `npu_v1.sv` → ...): um bug corrigido no MAC
  precisa ser re-corrigido em todas as cópias. Duplicação.
- *"NPU universal" monolítica* (`if dense ... if conv ... if cfc ...`): as interfaces reais de
  CNN/CfC ainda são desconhecidas — complexidade prematura.

**Consequências.**
- Analogia direta com o software: `Sequential { Vec<AnyModule> }` ↔ `controller + engines +
  blocos comuns`. Mesma separação de responsabilidades.
- Estrutura-alvo `npu/{common/, engines/, controller.sv, tars_npu.sv}` — mas **criar arquivos
  só quando a abstração surgir naturalmente**. Hoje `main.sv` + `tb.sv` bastam.
- Versões antigas são preservadas pelo **Git** (`git tag v0.1.0`), nunca por
  `npu_v0_old.sv` / `npu_v0_final2.sv`.
- **Próximo passo imediato**: nenhuma NPU nova. Evoluir a NPU atual (dot product de 4
  elementos) no primeiro bloco reutilizável — um `dot_engine` genérico. `linear_engine` só depois.
