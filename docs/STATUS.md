# tars-ml — Status Atual & Próximos Passos

Bússola do projeto: o que existe de fato no código, o que falta para fechar a **v0**,
e o próximo passo mínimo. *Última atualização*: 26/09/2026 (último commit: `164aa61`).

---

## Onde estamos

**Pré-v0 (Neural Core)** — o esqueleto da rede neural em Rust está de pé e o treino
básico converge, mas ainda no "modo didático": gradiente numérico, float32 puro.
Nada do que diferencia o projeto (QAT, exporter, paridade NPU) existe em código ainda.

---

## Software (Rust, `src/`)

| Componente | Estado | Observação |
|---|---|---|
| Rede multicamada | ✅ | `Sequential` com builder (`.linear()`, `.relu()`, `.sigmoid()`), `AnyModule` |
| Forward pass | ✅ | Trait `Module` (`Linear`, `Activation`) |
| Ativações | ✅ | Sigmoid (`math.rs`), ReLU |
| Custo MSE | ✅ | `cost()` em `lib.rs` |
| Otimizador | ✅ | Apenas BGD |
| Gradiente | ⚠️ | **Diferenças finitas** (`num_grad`, h=1e-3) — não analítico |
| Matrizes contíguas | ❌ | `Linear` usa `Vec<Vec<f32>>` (layout não-linear) |
| Testes | ❌ | Zero `#[test]` |

- `bin/main.rs`: treino de demonstração em dataset sintético de regressão (2→6→2).
- `view/plot.rs`: protótipo inicial com petgraph, não integrado ao modelo.
- Inicialização de pesos: `rand::random()` uniforme (sem He/Xavier) — aceitável por ora.

## Hardware (`npu/`)

| Componente | Estado | Observação |
|---|---|---|
| Dot product 4 elementos | ✅ | `main.sv`: FSM `start`/`done`, registrador de bias |
| Testbench | ✅ | `tb.sv` funcional |
| Acumulador 48b + saturação | ❌ | Acumulador atual é de 32 bits |
| SFU (ReLU/Sigmoid) | ❌ | Não existe |
| `parameter MODE` | ❌ | Só existe a variante binária |

- `Makefile` referencia `main2.sv`/`tb2.sv` (variante ternária) que **não existem no
  repositório** → alvos `sim2`, `test`, `wave2`, `lint` quebram.
- `npu/README.md` instrui `cd simple` (pasta inexistente).

---

## Lacunas para fechar a v0

| # | Item | Bloqueia |
|---|---|---|
| 1 | **Backprop analítico** (derivadas `σ' = σ(1-σ)`, `ReLU'`; backward pelo `Sequential`) | Tudo — QAT, XOR, escala de treino. Hoje cada parâmetro custa 2 forward completos |
| 2 | Testes automatizados (`cargo test`) | Refatorações seguras |
| 3 | QAT Q8.24 (depois INT8/Ternário) com STE | Matriz tripla — núcleo do projeto |
| 4 | `exporter.rs` → `.mem` | Loop co-design (v0.5) |
| 5 | NPU: acumulador 48b, saturação, SFU ReLU, `MODE` | Paridade 3 modos |
| 6 | Validação formal XOR 2→2→1 (MSE < 0.01) | DoD v0 |

---

## Próximos passos (ordem sugerida)

1. **Backprop analítico** — substituir `num_grad`. Maior alavancação do projeto.
2. **XOR 2→2→1** em float com backprop analítico → fecha o critério de convergência da v0.
3. **QAT Q8.24** com STE → depois INT8 e Ternário.
4. **`exporter.rs`** no formato [MEM_FORMAT.md](MEM_FORMAT.md).
5. **NPU**: evoluir o dot product atual num **`dot_engine` genérico** (ADR-009) — sem
   acumulador 48b/saturação/SFU antes disso.
6. **v0.5**: `tb.sv` executa a rede XOR inteira camada a camada, paridade zero-erro.

---

## Pendências de ambiente (correção manual humana)

1. **`npu/Makefile`**: remover/adaptar alvos que dependem de `main2.sv`/`tb2.sv` ausentes.
2. **`npu/README.md`**: remover `cd simple`; documentar alvos reais do Makefile.
3. **`.clang-format`** na raiz: legado da era C++ — decidir remoção.
4. **`README.txt`** (raiz): apontar para `docs/README.md` e o fluxo Cargo.
5. **Build fora do `nix develop`**: `plotters` exige `fontconfig` do sistema
   (via pkg-config). Dentro do shell do flake funciona.
