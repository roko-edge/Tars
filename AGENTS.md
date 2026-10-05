# AGENTS.md — Política de Agentes e IA neste Repositório

## 1. Regra Inviolável (Propriedade Humana do Código)

**Qualquer e todo agente (ou modelo de IA) neste repositório NÃO PODE encostar no código.**

Isso significa que nenhum agente pode:

- Criar, editar, renomear ou deletar **qualquer arquivo de código-fonte** (`.rs`, `.sv`, `.c`, `.cpp`, `.h`), incluindo `main.rs` ou `main.sv`;
- Modificar arquivos de build ou configuração do projeto (`Cargo.toml`, `Makefile`, `flake.nix`, `.gitignore`);
- Aplicar refatorações, "correções", formatação automática ou reescritas de código, **mesmo que detecte bugs, warnings ou más práticas**;
- Gerar patches, commits ou pull requests que alterem código;
- Alterar o AGENTS.md.

---

## 2. O que um Agente PODE Fazer

- **Ler** todo o repositório livremente (código, build, histórico, issues);
- **Escrever e editar apenas documentação** (`.md`, `.txt` em `docs/`) — e somente quando o mantenedor pedir explicitamente;
- Trabalhar **sempre em uma branch separada** (ex.: `docs/*`), para que o mantenedor revise e aprove antes de qualquer merge;
- Reportar problemas encontrados no código **por escrito** (issue, relatório, comentário em revisão), sem corrigi-los por conta própria.

---

## 3. Diretrizes Estritas de Estilo da Documentação

Ao criar ou modificar arquivos em `docs/`, os agentes devem obrigatoriamente seguir estas regras:

1. **Proibido Código de Implementação (Zero Copy-Paste Code):**
   - Nunca incluir corpos completos de funções/métodos (`impl`, laços `for`, algoritmos prontos) dentro dos arquivos Markdown.
   - Incluir apenas assinaturas abstratas de traits, definições estruturais de tipos, equações matemáticas e diagramas conceituais.
   - O papel da documentação é especificar **o que** deve ser feito, **por que** e **quais os contratos**, deixando a escrita do código exclusivamente para os humanos.

2. **Linguagem Técnica Sobria (Sem Emojis ou Fluff):**
   - Manter o tom formal, direto e acadêmico/de engenharia em inglês ou português.
   - **Proibido usar emojis**, saudações informais, elogios ou frases conversacionais de modelos de linguagem.
   - Manter tabelas claras de navegação e formatação matemática padrão em LaTeX ($\mathcal{L}$, $W_{ij}$).

3. **Hierarquia Concisa e Pontos de Entrada:**
   - Cada diretório de documentação deve ter um arquivo `README.md` atuando como ponto de entrada com navegação por tarefa e por integrante do time.
   - Evitar proliferação de arquivos fragmentados; agrupar especificações relacionadas em documentos normativos diretos.

---

## 4. Mensagens de Commit (Autoria Humana do Histórico)

**Commit messages são escritas pelo autor humano da mudança.**
Agentes de IA não devem gerar mensagens de commit finais para substituição da reflexão do autor; podem, no máximo, revisar ortografia ou gramática a pedido explícito do mantenedor. O histórico de commits é o registro permanente da intenção humana e da tomada de decisão dos membros do projeto.

---

## 5. Propriedade do Código

Todo o código de `tars-ml` é escrito **exclusivamente por humanos**.
Agentes de IA são ferramentas de consulta, pesquisa e documentação — nunca autores ou editores do código.

Se um agente precisa apontar um problema, o fluxo é:

1. Documentar o problema (o quê, onde, por quê);
2. Aguardar o humano corrigir manualmente;
3. Nunca "adiantar" a correção.

## 6. padrão de documentação

1. O diretório docs/ está dividido em três pontos de acesso:

    docs/
        Documentação direcionada ao usuário — ensina a usar os recursos implementados na biblioteca.

    docs/engineering/
        Registro de arquitetura, contratos e propostas técnicas internas.

    docs/project/
        Registra planejamento, status, pesquisa e comunicação.

    A regra é que agentes, ao escreverem documentação, devem seguir essa divisão à risca.

---

*Esta política é deliberada: `tars-ml` é uma biblioteca de machine learning construída do zero com propósito educacional/de pesquisa. O valor do projeto está no aprendizado humano profundo — o código deve refletir decisão humana em cada linha.*
