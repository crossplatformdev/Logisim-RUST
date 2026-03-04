# Resources / Localization / Assets Instructions

## Upstream reference

Upstream assets live under `src/main/resources/` in Logisim-Evolution v4.1.0.
Key directories:

- `resources/logisim/` — icons and toolbar artwork (`.gif`, `.png`)
- `resources/logisim/strings/` — localization property files (`*.properties`)
- `resources/logisim/boards/` — FPGA board definition XML files

## Required behavior

### Localization

- All user-visible strings that upstream externalises into `*.properties` files must be
  sourced from the equivalent resource file rather than hard-coded in Rust.
- The default locale is `en` (English).  Support for additional upstream locales (`de`, `pt`,
  `nl`, `ja`, etc.) is a secondary priority.
- The runtime locale must be selectable at startup (CLI flag or environment variable) without
  recompilation.
- Missing translation keys must fall back to the English string rather than panicking or
  displaying a raw key name.

### Icons and artwork

- All toolbar icons, component palette icons, and splash assets reused from upstream must be
  present in the repository under `assets/` and referenced from the build.
- Do **not** embed upstream SVG/PNG/GIF files inline in Rust source; keep them as separate
  static assets loaded at runtime.
- If an upstream icon cannot be reused (license or format incompatibility), provide a
  functionally equivalent replacement and document the substitution.

### Board definitions

- FPGA board definition XML files required for the board/FPGA workflows must be present under
  `assets/boards/` and parseable by the board-definition parser.
- Upstream board XML format must be preserved for interoperability.

## Checklist before committing

- [ ] New strings use a resource key rather than a hard-coded literal.
- [ ] Added assets are present in `assets/` and tracked in `git`.
- [ ] `cargo build --workspace` succeeds (asset loading does not panic at startup).
- [ ] `cargo clippy -p logisim-gui -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `docs/PARITY_MATRIX.md` updated for any newly integrated asset/locale.
