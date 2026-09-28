# tars-ml

Biblioteca **TinyML de Co-Design Hardware/Software** construída do zero: redes neurais
pequenas treinadas em Rust (Edition 2024, zero dependências de ML), executadas em uma
NPU parametrizada em SystemVerilog ou em microcontroladores via runtime em ponto fixo.

---

## Ambiente

Com Nix (recomendado — inclui ferramentas de hardware):

```bash
nix develop             # Rust + ferramentas de hardware + suporte
nix develop .#hardware  # apenas iverilog / verilator / gtkwave
```

Sem Nix: instale [rustup](https://rustup.rs). Nota: fora do ambiente Nix, `cargo build`
falha em `plotters`, que exige a biblioteca de sistema `fontconfig`.

## Compilar e executar (Rust)

```bash
cargo build
cargo run --bin main           # treino de demonstração
cargo run --bin visualization  # protótipo de visualização
cargo test
```

## NPU (SystemVerilog)

```bash
cd npu
make sim     # compila com iverilog (-g2012) e executa no terminal
make wave    # abre as ondas geradas no GTKWave
make lint    # análise estática com Verilator
make clean   # limpa a pasta build/
```

## Documentação

Índice completo em [docs/README.md](docs/README.md). Principais:

| Documento | Conteúdo |
|---|---|
| [docs/STATUS.md](docs/STATUS.md) | Estado atual do código e próximos passos |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Arquitetura e os 3 pilares |
| [docs/ROADMAP_NPU.md](docs/ROADMAP_NPU.md) | Evolução v0 a v8 com critérios de conclusão |
| [docs/DECISIONS.md](docs/DECISIONS.md) | Registro de decisões (ADRs) |

## Princípios

1. **Código 100% humano** — IAs não escrevem código neste repositório ([AGENTS.md](AGENTS.md)).
2. **Zero bibliotecas de ML** — álgebra, otimizadores e ativações feitos à mão.
3. **Treino na precisão alvo (QAT)** — Q8.24, INT8 e Ternário, nunca pós-treino.
4. **Paridade zero-erro** — o que o Rust calcula, a NPU reproduz bit a bit.
