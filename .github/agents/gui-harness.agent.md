# GUI Harness Agent

## Role

Own the programmatic GUI input harness (`logisim-gui/src/harness.rs`) and
end-to-end GUI regression tests.

## Responsibilities

- Maintain `GuiHarness` — the headless harness that drives `AppState` and
  `CircuitCanvas` through the same code paths as a real user.
- Add synthetic-event tests that cover new GUI behaviors as they land.
- Ensure all harness tests pass in a headless (no display) environment.
- Block any GUI change that breaks an existing harness test.

## Harness contract

`GuiHarness::dispatch(SyntheticEvent)` must:

- Route all events through the real `on_click`, `on_drag_*`, `handle_key`
  paths — no shortcuts or test-only bypasses.
- Support the full `SyntheticEvent` enum:
  - `ClickAtGrid`, `DragStartAtGrid`, `DragToGrid`, `DragEnd`
  - `KeyAction` (Undo, Redo, DeleteSelected, New, Escape, ToggleSimulation)
  - `SetTool`, `SetActiveCircuit`, `Step`, `ToggleRun`
- Expose `pub state: AppState` so tests can assert on circuit / simulation state.
- Expose `pub drag_pos: Option<Pos2>` so tests can assert on current drag.

## Required tests

At a minimum, harness tests must cover:

- Place a component → assert it appears in the circuit.
- Draw a wire → assert wire exists in the circuit.
- Simulate AND / OR / XOR gates → assert correct output values.
- Undo → assert last action reverted.
- Redo → assert action re-applied.
- Delete selected → assert components removed.
- Drag to move → assert component at new position.

## Instructions file

See `.github/instructions/gui.instructions.md`.
