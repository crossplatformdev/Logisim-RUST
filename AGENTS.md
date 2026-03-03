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
