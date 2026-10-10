# AGENTS.md - AI Agent Policy

## 1. Scope and Human Ownership

Git-tracked files are maintained exclusively by humans.
AI agents may read them, but must not modify, delete, rename,
overwrite, or change their permissions.

This restriction applies to all tracked files, including source code,
configuration, scripts, tests, documentation, and this policy.

## 2. File Classification

A path is protected if it is tracked in the Git index or exists in
the current HEAD commit. Newly staged files are also protected.

Files already protected at the start of a task remain protected
throughout that task, even if their Git status changes.

Untracked files, including Git-ignored files, may be created or edited
only when the maintainer explicitly requests the specific work.

Agents must verify Git tracking status before making any change.
If classification is uncertain, they must stop and ask the maintainer.

## 3. Permitted and Prohibited Actions

Agents may inspect the repository and report findings without editing
protected files.

Agents must not untrack files, alter ignore rules, or use symlinks,
alternate paths, generators, or build commands to bypass protection.

Every operation must respect the protection of its actual targets,
including indirect writes and generated output.

Untracked status does not authorize unrelated changes, access to
secrets, or modification of system configuration.

Hardware programming, privileged changes, and external side effects
require explicit maintainer authorization.

## 4. Git and Commit Authorship

Agents must not stage files, create commits, or publish changes.
Commit messages are written by the human author.
Agents may review spelling or grammar only upon explicit request.

## 5. Documentation Standards

Any authorized documentation changes must use a formal technical tone,
without emojis, greetings, praise, or conversational filler.

Documentation must specify purpose, contracts, architecture, and
rationale, without complete implementation bodies or ready-made algorithms.

Each documentation directory must provide a README.md entry point.
Prefer consolidated specifications over fragmented documents.

## 6. Documentation Organization

- docs/: user documentation for implemented library features.
- docs/engineering/: architecture, contracts, and technical proposals.
- docs/project/: planning, status, research, and communication.

These conventions do not override Git-tracked file protection.

## 7. Local Instructions

Nested AGENTS.md files may impose additional restrictions.
They must not relax the protection of Git-tracked files.
