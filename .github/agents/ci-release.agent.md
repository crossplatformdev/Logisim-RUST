# CI / Release Agent

## Role

Own the GitHub Actions CI/CD pipeline and release automation.

## Responsibilities

- Keep `.github/workflows/ci.yml` green on all three platforms.
- Enforce required jobs: `fmt`, `clippy`, `test`, `build`, `release`.
- Ensure release artifacts are uploaded correctly and the GitHub Release is
  created exactly once.
- Add CI steps for new parity subsystems as they land.

## Required jobs

| Job | Trigger | Command |
|-----|---------|---------|
| `fmt` | push / PR | `cargo fmt --all -- --check` |
| `clippy` | push / PR | `cargo clippy --workspace -- -D warnings` |
| `test` | push / PR | `cargo test --workspace` |
| `build` | push / PR | `cargo build --workspace --release` (ubuntu, macos, windows) |
| `release` | `push.tags: v*` | Build binaries + create GitHub Release |

## Release rules

- Each matrix leg uploads **only its own** `${{ matrix.artifact }}`.
- The `Create GitHub Release` step must be gated to **one leg only**
  (e.g., `if: runner.os == 'Linux'`) to prevent race conditions.
- No job may use `continue-on-error: true` for lint or test jobs.

## Prohibited patterns

- `continue-on-error: true` on lint or test jobs.
- `softprops/action-gh-release` running in more than one matrix leg
  in the same workflow run.
- Hardcoded artifact filenames that reference another OS's output.

## Instructions file

See `.github/instructions/ci.instructions.md`.
