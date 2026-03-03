# UX Interaction Fidelity Agent

## Workstream

UX / interaction semantics — ensuring every mouse, keyboard, and gesture interaction
matches the upstream Logisim-Evolution v4.1.0 behaviour exactly.

## Owned paths

- `logisim-gui/src/canvas.rs` — pointer events, drag, click, multi-select
- `logisim-gui/src/toolbar.rs` — keyboard shortcuts, tool switching
- `logisim-gui/src/state.rs` — selection state, tool state

## Upstream reference

Interaction semantics are defined by:
- `com.cburch.logisim.tools.SelectTool`
- `com.cburch.logisim.tools.WiringTool`
- `com.cburch.logisim.tools.PokeTool`
- `com.cburch.logisim.tools.AddTool`
- `com.cburch.logisim.comp.Component` hit-test methods

## Required interaction parity

### Selection tool

- **Left-click on empty canvas**: deselects all.
- **Left-click on component**: selects that component only.
- **Ctrl+left-click on component**: adds/removes component from selection (additive toggle).
- **Left-drag on empty canvas**: rubber-band rectangle selection (replaces selection).
- **Ctrl+left-drag**: rubber-band that *adds* to the existing selection.
- **Delete key**: removes all selected components.
- **Move selected**: drag any selected component to move all selected components together.

### Wire tool

- **First click**: anchors wire start point.
- **Second click**: completes L-shaped wire (horizontal then vertical).
- **Escape**: cancels in-progress wire.

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| Ctrl+Z | Undo |
| Ctrl+Y / Ctrl+Shift+Z | Redo |
| Delete / Backspace | Delete selected |
| Escape | Cancel current tool action |
| Ctrl+A | Select all |

## Checklist before committing

- [ ] `cargo test -p logisim-gui` passes.
- [ ] `cargo clippy -p logisim-gui -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `docs/PARITY_MATRIX.md` updated.
