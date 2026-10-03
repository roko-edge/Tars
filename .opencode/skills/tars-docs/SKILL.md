---
name: tars-docs
description: Write and maintain documentation for the TARS repository following its strict documentation style (no implementation code, no emojis, sober technical tone, hierarchical entry points). Use when creating or editing any .md file in docs/, README.md, or AGENTS.md.
---

# TARS Documentation Style Skill

## Context

TARS is a human-written ML library (Rust + SystemVerilog NPU) with educational/research
value. AI agents are never authors of code. Documentation is the only writable surface,
and it must never become a vehicle for smuggling implementations.

The authoritative policy lives in `AGENTS.md` at the repository root. This skill
operationalizes it for any documentation task.

## Hard Rules

1. **No implementation code in docs.**
   - Forbidden: full method bodies, `impl` blocks with logic, loops, algorithms,
     `todo!()` placeholders framed as code to copy.
   - Allowed: trait signatures, struct/type declarations without logic, port lists,
     interface contracts, pseudo-protocol descriptions, mathematical formulas.
   - Test: if a human could paste the block into the codebase and it would compile,
     it is too much. Reduce it to the signature or describe it in prose.

2. **Sober technical register.**
   - No emojis, no greetings, no praise, no conversational filler.
   - Write like the existing documents (`ARCHITECTURE.md`, `STATUS.md`,
     `DECISIONS.md`): declarative sentences, tables for structured facts,
     normative language ("must", "is implemented", "is not implemented").
   - Match the language of the file being edited (English for technical docs).

3. **Hierarchical structure with entry points.**
   - `docs/README.md` is the single navigation entry point.
   - Every directory under `docs/` opens with an index or README linking its files.
   - Prefer fewer, complete documents over many fragmented ones.

4. **Only `docs/`, `README.md`, and `AGENTS.md` are writable.**
   - Never touch `.rs`, `.sv`, `Cargo.toml`, `Makefile`, `flake.nix`, `.gitignore`.
   - Always work on a `docs/*` branch for the maintainer to review.

## Standard Document Skeleton

```markdown
# Title: Declarative and Specific

One-paragraph scope statement: what this document specifies or describes.

---

## 1. Section

Normative statements, tables, or equations.

---

## 2. Section

...
```

- Use `---` rules between major sections.
- Use tables for: component status, command status, terminology, navigation.
- Use LaTeX for math: $\mathcal{L}$, $W_{ij}$, $\sum$, etc.
- Use fenced `text` blocks only for directory trees and interface shapes, not
  implementations.

## Verification Checklist (run before finishing)

- [ ] Zero emojis anywhere in the file.
- [ ] No copy-pasteable implementation bodies (signatures only).
- [ ] Tone matches existing docs (no "Great!", no "Let's", no "I'll").
- [ ] Every new file is linked from its parent index/README.
- [ ] STATUS.md claims reflect actual code (verify by reading the source).
- [ ] No code files modified (check `git status` before reporting done).

## When Updating Existing Docs

- Read the current file first; preserve its structure and heading style.
- STATUS.md documents current reality: if code and doc disagree, fix the doc
  to match the code, and report the discrepancy to the maintainer.
- DECISIONS.md only records implemented decisions, never proposals.
- specs/ documents are normative contracts: precise, testable, versioned by
  section, no editorializing.
