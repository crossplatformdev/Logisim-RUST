---
applyTo: "logisim-core/**"
---

# Core / Simulation Instructions

## Upstream reference

Simulation semantics live in `com.cburch.logisim.circuit` and `com.cburch.logisim.comp` in
Logisim-Evolution v4.1.0. The canonical truth-table rules are defined in
`com.cburch.logisim.data.Value`.

## Required behavior

- **Value set**: False (0), True (1), Unknown (X), Error (multi-driver conflict), HighZ (Z).
- **Propagation**: iterative, convergence-based; oscillation detected after 1 000 iterations.
- **Multi-driver**: two driven signals produce Error; one driven + one HighZ yields the driven value.
- **Subcircuits**: recursively evaluated; pin values flow in/out through I/O pins.
- **Clock tick**: advances all clock components simultaneously before the next propagation pass.

## Component contract

Every `ComponentKind` variant must implement:

- `evaluate(&self, inputs: &[Value], width: BitWidth) -> Vec<Value>` — pure, no side-effects.
- Write support in `logisim-file/src/writer.rs`.
- Parse support in `logisim-file/src/parser.rs`.
- At least one unit test in `logisim-core/src/simulation.rs` or a dedicated test module.

## Checklist before committing

- [ ] `cargo test -p logisim-core` passes.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo clippy -p logisim-core -- -D warnings` clean.
- [ ] `docs/PARITY_MATRIX.md` updated.
