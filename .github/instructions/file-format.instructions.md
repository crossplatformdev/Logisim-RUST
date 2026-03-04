---
applyTo: "logisim-file/**"
---

# File-Format Instructions

## Upstream reference

`.circ` files are XML. The canonical schema is defined by Logisim-Evolution v4.1.0 and can be
inspected in any `.circ` file produced by the upstream Java application.

Key upstream source files:
- `com.cburch.logisim.file.XmlReader`
- `com.cburch.logisim.file.XmlWriter`

## Required behavior

### Parsing (`logisim-file/src/parser.rs`)

- Every `<comp lib="N" name="…">` element must map to a `ComponentKind` variant.
- Library numbers: `"0"` → Wiring, `"1"` → Gates, `"2"` → Plexers, `"3"` → Arithmetic,
  `"4"` → Memory, `"5"` → I/O.
- Unknown component names must return `Err(ParseError::UnknownComponent)` — never silently drop.
- `<wire>` elements must parse `x1`, `y1`, `x2`, `y2` as `i32` grid coordinates.
- `<circuit name="…">` elements become `Circuit` entries in `Project.circuits`.

### Writing (`logisim-file/src/writer.rs`)

- Every `ComponentKind` variant must have a corresponding write arm.
- Round-trip fidelity: `parse(write(project)) == project` for all components in scope.

### Attributes

- Component `loc`, `name` (label), `facing`, bit-width, and other attributes must round-trip.

## Checklist before committing

- [ ] `cargo test -p logisim-file` passes.
- [ ] Round-trip test covers the changed component.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo clippy -p logisim-file -- -D warnings` clean.
- [ ] `docs/PARITY_MATRIX.md` updated.
