# File-Format Agent

## Role

Own `.circ` parse/write fidelity against Logisim-Evolution v4.1.0.

## Responsibilities

- Maintain `logisim-file/src/parser.rs` and `logisim-file/src/writer.rs`.
- Ensure every `ComponentKind` variant has both parse and write support.
- Enforce round-trip fidelity: `parse(write(project)) == project`.
- Add or update a round-trip test for every new component.

## Format rules

### Library numbers

| Library | `lib` attribute | `<lib desc=.../>` |
|---------|----------------|-------------------|
| Wiring  | `"0"`          | `#Wiring`         |
| Gates   | `"1"`          | `#Gates`          |
| Plexers | `"2"`          | `#Plexers`        |
| Arith   | `"3"`          | `#Arithmetic`     |
| Memory  | `"4"`          | `#Memory`         |
| I/O     | `"5"`          | `#I/O`            |
| TTL     | `"6"`          | `#TTL`            |
| User    | `"7"`          | `#user`           |

### Parser safety rules

- Never treat an unrecognised `lib` value as a `Subcircuit`.
- Return `Err(FileError::UnknownLibrary)` for unrecognised libraries.
- Return `Err(FileError::UnknownComponent)` for unrecognised component names
  within a known library.

### Writer rules

- Sort option keys and attribute keys before emission for deterministic output.
- Preserve `<appear>` XML verbatim via `appearance_xml: Option<String>`.
- Emit `<lib .../>` headers for every library used in the project.

## Instructions file

See `.github/instructions/file-format.instructions.md`.
