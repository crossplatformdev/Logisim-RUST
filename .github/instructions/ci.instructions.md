---
applyTo: ".github/workflows/**"
---

# CI Instructions

## Required jobs

| Job | Trigger | Purpose |
|-----|---------|---------|
| `fmt` | push / PR | `cargo fmt --all -- --check` |
| `clippy` | push / PR | `cargo clippy --workspace -- -D warnings` |
| `test` | push / PR | `cargo test --workspace` |
| `build` | push / PR | `cargo build --workspace --release` on ubuntu, macos, windows |
| `release` | `push.tags: ['v*']` | Build release binaries and create GitHub Release |

## Rules

- All jobs must pass before a PR can be merged.
- The `release` job must run only once (not once per matrix OS). Use `if: matrix.os == 'ubuntu-latest'`
  to create the GitHub Release; all matrix legs upload artifacts.
- No job may use `continue-on-error: true` for lint or test jobs.
- Cross-platform builds must target `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`,
  `x86_64-pc-windows-msvc`.

## Adding new jobs

When adding a new parity subsystem that has integration-level behavior (e.g. `.circ`
round-trip, simulation truth-table regression), add a corresponding CI step that fails
the build if the behavior regresses.

## Checklist before committing CI changes

- [ ] Workflow YAML is valid (`yamllint` or GitHub Actions syntax check).
- [ ] Release job creates the GitHub Release from exactly one matrix leg.
- [ ] `push.tags: ['v*']` trigger is present on the main workflow.
