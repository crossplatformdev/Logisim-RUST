# Resources / Assets Agent

## Role

Own upstream asset integration: icons, artwork, localization property files,
and board definitions.

## Responsibilities

- Import upstream icons from `src/main/resources/resources/logisim/` of v4.1.0.
- Place all static assets under `assets/` and track them in git.
- Integrate localization (`*.properties`) files from upstream into the locale
  module so user-visible strings are sourced from resource bundles, not
  hard-coded in Rust.
- Import board definition XML files to `assets/boards/`.
- Update `docs/PARITY_MATRIX.md` rows for asset/locale parity.

## Asset layout

```
assets/
  icons/          ← toolbar and palette icons (PNG/GIF from upstream)
  boards/         ← FPGA board definition XML files
  locale/         ← *.properties localization files (en, de, pt, nl, ja, ...)
```

## Localization rules

- All user-visible strings that upstream externalises into `*.properties` must
  be loaded from the equivalent resource file rather than hard-coded in Rust.
- Default locale: `en`.  Additional locales are a secondary priority.
- Missing translation keys must fall back to English, not panic.
- The runtime locale must be selectable at startup without recompilation.

## Asset checklist (pre-commit)

- [ ] New strings use a resource key, not a hard-coded literal.
- [ ] Added assets are present in `assets/` and tracked in git.
- [ ] `cargo build --workspace` succeeds (no panic on asset load).
- [ ] `cargo clippy -p logisim-gui -- -D warnings` clean.
- [ ] `docs/PARITY_MATRIX.md` updated.

## Instructions file

See `.github/instructions/resources.instructions.md`.
