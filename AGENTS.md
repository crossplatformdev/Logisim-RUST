# Agent Execution Rules — Logisim-RUST

This file defines binding execution rules for all coding agents (GitHub Copilot, Copilot Chat,
Codex, and any future automated agent) operating in this repository.

## Mission

Produce a behaviorally faithful Rust rewrite of **Logisim-Evolution v4.1.0**.
Upstream reference: <https://github.com/logisim-evolution/logisim-evolution/releases/tag/v4.1.0>

## Non-negotiable rules

1. **No stubs.** Every function must do real work. Delete or skip a feature rather than stub it.
2. **No TODO comments** that defer required upstream behavior.
3. **No placeholder implementations** disguised as progress.
4. **No documentation-only mitigation.** If a feature is missing, implement it; do not describe the gap.
5. **No scope objections.** "This is too large" is not an acceptable response; reduce the parity gap
   with concrete commits instead.
6. **Run `cargo fmt --all` before every commit.** The workspace must pass `cargo fmt --all -- --check`.
7. **Run `cargo clippy -- -D warnings` before every commit.** Zero warnings required.
8. **All tests must pass** (`cargo test --workspace`) before committing.

## How agents must behave in this repo

For every task:

1. Read the relevant `.github/instructions/*.instructions.md` file for the subsystem.
2. Compare the Rust implementation against upstream v4.1.0 for the subsystem.
3. Identify the highest-impact parity gap.
4. Implement it in Rust with tests.
5. Run `cargo fmt --all`, `cargo clippy -- -D warnings`, `cargo test --workspace`.
6. Commit with a clear message.
7. Update `docs/PARITY_MATRIX.md` to reflect the new status.
8. Repeat until the gap is closed or the session ends.

## Priority order

1. Simulation semantics correctness
2. `.circ` file compatibility (parse + write round-trip)
3. Component / library coverage
4. GUI workflow fidelity
5. Undo/redo and editor completeness
6. Appearance / resource / localization parity
7. Chronogram / timing-diagram
8. HDL export
9. FPGA / board integration
10. CLI completeness

## What counts as "done" for a subsystem

A subsystem is done when:

- The Rust behavior matches upstream v4.1.0 in normal usage.
- At least one test exercises the new behavior.
- `docs/PARITY_MATRIX.md` shows the item as **implemented** and **tested**.
- `cargo fmt`, `cargo clippy`, and `cargo test` all pass.

## Workstream ownership

| Workstream | Crate(s) / Path(s) | Instruction file |
|------------|-------------------|-----------------|
| Simulation semantics | `logisim-core/src/simulation.rs` | `core.instructions.md` |
| Component library | `logisim-core/src/component.rs` | `core.instructions.md` |
| File format (parse/write) | `logisim-file/src/` | `file-format.instructions.md` |
| GUI / canvas / editor | `logisim-gui/src/` | `gui.instructions.md` |
| CI / release pipeline | `.github/workflows/` | `ci.instructions.md` |
| Resources / localization | `assets/`, `logisim-gui/src/locale.rs` | `resources.instructions.md` |
| Parity tracking | `docs/PARITY_MATRIX.md`, `docs/PARITY.md` | `parity.instructions.md` |

## Handoff rules

- Before starting work in a crate owned by another workstream, read that workstream's
  instruction file.
- If a change in workstream A requires a matching change in workstream B, make both changes
  in the same commit with a combined prefix (e.g. `core+file: …`).
- After finishing a subsystem, update `docs/PARITY_MATRIX.md` immediately — do not defer this
  to a cleanup commit.

## Escalation rules

- If progress in your workstream is blocked because a required API in another crate does not
  exist, **add the API first** (with a minimal but correct implementation) rather than
  creating a workaround that bypasses the correct abstraction.
- If a parity requirement conflicts with Rust safety or memory model constraints, document the
  exact conflict in a code comment and implement the closest safe equivalent.
- Never mark a feature `implemented` in the parity matrix to unblock yourself.  If the
  feature is not done, it is `partial` or `missing`.

## Commit message prefix conventions

| Prefix | Scope |
|--------|-------|
| `core:` | Simulation semantics, component logic |
| `file:` | Parser, writer, round-trip |
| `gui:` | Canvas, toolbar, dialogs, harness |
| `cli:` | CLI commands and output |
| `ci:` | Workflow YAML, release scripts |
| `docs:` | Parity matrix, architecture, README |
| `assets:` | Icons, localization files, board definitions |
| `test:` | Test-only changes (no production code) |
