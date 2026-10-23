# AGENTS.md - AI Agent Policy

## 1. Scope and Human Ownership

Core library architecture, NPU designs, and source code implementations are strictly owned by the human maintainer.

AI agents may read protected files to understand system context and contracts, but must not modify, delete, rename, overwrite, or change their permissions.

This policy file (`AGENTS.md`) is also strictly protected and must not be altered by agents.

## 2. File Classification and Access Matrix

File permissions are governed by repository domain and file type rather than general Git tracking status:

### Strictly Protected (Read-Only)
- **Directories:** `lib/` and `npu/` (including all subdirectories and files within them).
- **Source & Hardware Files:** All Rust (`.rs`) and SystemVerilog (`.sv`, `.svh`) files across the entire repository, as well as any other implementation source code or test suites.
- **Policy Files:** Root and nested `AGENTS.md` files.

### Editable (Write-Permitted Within Task Scope)
- **Build & Infrastructure:** `Makefile`, build scripts, CI/CD pipelines, container definitions, and infrastructure configuration files.
- **Documentation:** Files within `docs/`, provided all edits strictly adhere to Sections 5 and 6.
- **Untracked / Temporary Files:** Allowed only when explicitly requested by the maintainer and outside protected paths.

Agents must verify the target path and file extension before making any change. If a file's classification is uncertain, the agent must stop and ask the maintainer.

## 3. Permitted and Prohibited Actions

Agents may inspect the entire repository, analyze code in `lib/` and `npu/`, and report findings without editing protected files.

Agents must not use `Makefile` targets, build scripts, code generators, symlinks, or alternate paths to indirectly overwrite, format, or mutate `.rs`, `.sv`, `lib/`, or `npu/` files.

Every operation must respect the protection of its actual targets, including indirect writes and generated output.

Write access to infrastructure and build files does not authorize exposing secrets, weakening security controls, or modifying host system configurations outside the repository.

Hardware programming, FPGA/NPU flashing, privileged changes, and external side effects require explicit maintainer authorization.

## 4. Git and Commit Authorship

Agents must not stage files, create commits, alter `.gitignore` to bypass path restrictions, or publish changes.
Commit messages are written exclusively by the human author.
Agents may review spelling or grammar only upon explicit request.

## 5. Documentation Standards

Agents are authorized to create and update documentation within `docs/`, provided they strictly follow these rules:

- Use a formal, objective technical tone without emojis, greetings, praise, or conversational filler.
- Specify purpose, contracts, architecture, and rationale.
- Do not include complete implementation bodies or ready-made algorithms in documentation.
- Ensure each documentation directory provides a `README.md` entry point.
- Prefer consolidated specifications over fragmented documents.

## 6. Documentation Organization

All documentation modifications must respect the following directory structure:

- `docs/`: user documentation for implemented library features.
- `docs/engineering/`: architecture, contracts, and technical proposals.
- `docs/project/`: planning, status, research, and communication.

## 7. Local Instructions

Nested `AGENTS.md` files may impose additional restrictions within specific subdirectories.
They must not relax the protection of `lib/`, `npu/`, `.rs`, or `.sv` files.
