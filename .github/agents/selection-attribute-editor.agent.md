# Selection + Attribute Editor Agent

## Workstream

Component selection and attribute editing — ensuring that:
1. Every component can be selected with mouse and keyboard.
2. The attribute panel shows and allows editing of every property that upstream exposes.
3. Attribute edits are reflected immediately in the circuit, persist to `.circ` files,
   and are undo-able.

## Owned paths

- `logisim-gui/src/attr_panel.rs` — attribute table display and editing
- `logisim-gui/src/canvas.rs` — selection hit-test, multi-select
- `logisim-gui/src/state.rs` — `selected` field, selection helpers
- `logisim-core/src/component.rs` — `ComponentKind` attribute fields
- `logisim-file/src/parser.rs` — parse component attributes from `.circ`
- `logisim-file/src/writer.rs` — write component attributes to `.circ`
- `logisim-core/src/history.rs` — `UndoAction::ChangeAttribute`

## Upstream reference

- `com.cburch.logisim.gui.main.AttrTableModel`
- `com.cburch.logisim.comp.Component.getAttributeSet()`
- `com.cburch.logisim.data.Attribute`
- `com.cburch.logisim.data.AttributeSet`

## Required parity

### Selection

- Single-click selects; Ctrl+click toggles additive.
- Rubber-band box selection.
- Select-all (Ctrl+A).
- Delete key removes all selected.

### Attribute panel

For each selected component the panel must show every attribute that upstream shows,
including at minimum:

| Attribute | Upstream key | Component |
|-----------|-------------|-----------|
| Facing | `facing` | All |
| Label | `label` | All |
| Label Font | `labelfont` | All |
| Data Bits | `width` | Pin, Gates |
| Number of Inputs | `inputs` | Multi-input Gates |
| Negate inputs | `negateN` | Gates |
| Three-state? | `tristate` | Buffer/NOT |
| Address Bits | `addrWidth` | RAM/ROM |
| Data Bits | `dataWidth` | RAM/ROM |
| Sync? | `sync` | Register/FlipFlop |

Attribute edits must:
1. Update the component's field in `project.circuits`.
2. Push an `UndoAction::ChangeAttribute` to the undo history.
3. Mark `state.modified = true`.
4. Call `state.sync_simulator()`.

## Checklist before committing

- [ ] `cargo test -p logisim-core` passes.
- [ ] `cargo test -p logisim-gui` passes.
- [ ] `cargo clippy --workspace -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `docs/PARITY_MATRIX.md` updated.
