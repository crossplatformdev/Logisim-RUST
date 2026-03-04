---
applyTo: "logisim-gui/**"
---

# GUI Instructions

## Upstream reference

GUI behavior is defined by Logisim-Evolution v4.1.0 (`com.cburch.logisim.gui.main`).
The egui-based Rust implementation must match the user-visible behavior of the upstream Swing UI.

## Required behavior

### Canvas

- Grid: 10 px per grid unit at zoom 1×; toggleable.
- Pan: middle-mouse-button drag or right-drag (upstream behavior).
- Zoom: scroll wheel; Ctrl+scroll; zoom buttons in toolbar.
- Wire placement: click-drag in Wire tool; snaps to grid; T-junctions auto-connect.
- Component placement: click in Place tool; ghost preview follows cursor.
- Selection: rubber-band in Select tool; Ctrl+click to add/remove from selection.
- Move: drag selected components; grid-snapped; commits `MoveComponent` undo action.
- Delete: Delete key removes selected components and wires.

### Simulation overlay

- Wires coloured by live value: blue = 0, green = 1, red = error, grey = unknown/highZ.
- Poke tool: left-click toggles input pin value; updates simulation immediately.

### Undo/Redo

- Ctrl+Z = undo; Ctrl+Y or Ctrl+Shift+Z = redo.
- Edit menu shows Undo/Redo with enabled/disabled state.

### File operations

- File → New, Open, Save, Save As.
- Native OS file dialogs via `rfd`.

## Checklist before committing

- [ ] `cargo build -p logisim-gui` succeeds (headless build OK).
- [ ] `cargo clippy -p logisim-gui -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `docs/PARITY_MATRIX.md` updated.
