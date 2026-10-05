# TARS User Documentation

This directory documents resources that are implemented and available to users of
the TARS Rust package and SystemVerilog prototype.

Project plans and internal engineering contracts are maintained separately. Their
presence in this repository does not imply that a feature is available.

---

## Navigation by Task

| Objective | Document | Scope |
|---|---|---|
| Prepare the environment and run TARS | [`GETTING_STARTED.md`](GETTING_STARTED.md) | Requirements, commands, current behavior, and operational limitations |
| Run the OR or XOR configurations | [`EXPERIMENTS.md`](EXPERIMENTS.md) | Experiment factories, training flow, outputs, and manual selection |
| Inspect the current public surface | [`API.md`](API.md) | Implemented modules, operations, and stability boundaries |
| Simulate the NPU prototype | [`../npu/README.md`](../npu/README.md) | Hardware environment, Make targets, and current simulation behavior |

---

## Other Repository Records

| Area | Entry Point | Purpose |
|---|---|---|
| Internal engineering | [`engineering/README.md`](engineering/README.md) | Architecture, implemented decisions, technical contracts, and proposals |
| Project operations | [`project/README.md`](project/README.md) | Status, roadmap, release targets, and research process |

Internal engineering and project records are not user documentation. A capability is
considered available only when it is described in this user section and agrees with
the current source tree.
