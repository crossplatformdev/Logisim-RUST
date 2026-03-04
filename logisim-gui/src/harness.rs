//! Programmatic GUI-driving harness for headless integration tests.
//!
//! The harness exposes a [`GuiHarness`] that holds the same [`AppState`] and
//! [`CircuitCanvas`] used at runtime, but processes events through the exact
//! same interaction methods (no egui frame is required). Tests can therefore
//! verify that the editor and simulation logic behave correctly end-to-end.

use crate::canvas::CircuitCanvas;
use crate::state::{AppState, Tool};
use egui::Pos2;
use logisim_core::{
    circuit::Circuit,
    component::{ComponentId, ComponentKind},
    history::UndoAction,
    value::{Bus, Value},
};

/// A fixed canvas origin used for all grid ↔ screen conversions inside the
/// harness (no real window exists so we anchor at (0,0)).
const ORIGIN: Pos2 = Pos2::new(0.0, 0.0);

// ── Event types ──────────────────────────────────────────────────────────────

/// A synthetic mouse or keyboard event that can be replayed against the GUI.
#[derive(Clone, Debug)]
pub enum SyntheticEvent {
    /// Left-click at the given grid coordinates on the canvas.
    ClickAtGrid { gx: i32, gy: i32 },
    /// Ctrl+left-click at the given grid coordinates (additive selection toggle).
    CtrlClickAtGrid { gx: i32, gy: i32 },
    /// Begin a component drag from the given grid coordinates.
    DragStartAtGrid { gx: i32, gy: i32 },
    /// Continue a drag to a new grid coordinate.
    DragToGrid { gx: i32, gy: i32 },
    /// Release the drag at the current position.
    DragEnd,
    /// A named keyboard action.
    KeyAction(KeyAction),
    /// Switch the active tool.
    SetTool(Tool),
    /// Switch the active circuit by name.
    SetActiveCircuit(String),
    /// Advance the simulation by one clock tick.
    Step,
    /// Toggle simulation run/stop.
    ToggleRun,
}

/// Named keyboard actions replicated by the harness without an egui context.
#[derive(Clone, Debug)]
pub enum KeyAction {
    Undo,
    Redo,
    DeleteSelected,
    New,
    Escape,
    ToggleSimulation,
}

// ── GuiHarness ───────────────────────────────────────────────────────────────

/// Headless GUI harness.
///
/// Holds the full application state (`AppState`) and canvas interaction state
/// (`CircuitCanvas`) and allows tests to drive the editor and simulator through
/// the same code paths as a real user.
pub struct GuiHarness {
    /// Public so tests can inspect state directly.
    pub state: AppState,
    canvas: CircuitCanvas,
    /// Screen position of the most recent drag point (for `DragEnd`).
    pub drag_pos: Option<Pos2>,
}

impl GuiHarness {
    /// Create a new harness with a blank project that has a single "main" circuit.
    pub fn new() -> Self {
        let mut state = AppState::new();
        let main = Circuit::new("main");
        state.project.add_circuit(main);
        state.active_circuit = "main".to_string();
        state.sync_simulator();
        GuiHarness {
            state,
            canvas: CircuitCanvas::new(),
            drag_pos: None,
        }
    }

    /// Convert a grid coordinate to a screen position (using the harness origin).
    fn grid_to_screen(&self, gx: i32, gy: i32) -> Pos2 {
        self.state.grid_to_screen(gx, gy, ORIGIN)
    }

    // ── Dispatch ─────────────────────────────────────────────────────────

    /// Dispatch a single synthetic event through the same logic paths the real
    /// UI uses.
    pub fn dispatch(&mut self, event: SyntheticEvent) {
        match event {
            SyntheticEvent::ClickAtGrid { gx, gy } => {
                let pos = self.grid_to_screen(gx, gy);
                self.canvas.on_click(pos, ORIGIN, &mut self.state, false);
            }

            SyntheticEvent::CtrlClickAtGrid { gx, gy } => {
                let pos = self.grid_to_screen(gx, gy);
                self.canvas.on_click(pos, ORIGIN, &mut self.state, true);
            }

            SyntheticEvent::DragStartAtGrid { gx, gy } => {
                let pos = self.grid_to_screen(gx, gy);
                self.canvas.on_drag_start(pos, ORIGIN, &mut self.state);
                self.drag_pos = Some(pos);
            }

            SyntheticEvent::DragToGrid { gx, gy } => {
                let pos = self.grid_to_screen(gx, gy);
                self.canvas.on_drag_move(pos, ORIGIN, &mut self.state);
                self.drag_pos = Some(pos);
            }

            SyntheticEvent::DragEnd => {
                self.canvas.on_drag_end(&mut self.state);
                self.drag_pos = None;
            }

            SyntheticEvent::KeyAction(action) => {
                self.apply_key_action(action);
            }

            SyntheticEvent::SetTool(tool) => {
                self.state.tool = tool;
            }

            SyntheticEvent::SetActiveCircuit(name) => {
                self.state.active_circuit = name;
                self.state.selected.clear();
            }

            SyntheticEvent::Step => {
                let name = self.state.active_circuit.clone();
                let _ = self.state.simulator.tick(&name);
            }

            SyntheticEvent::ToggleRun => {
                self.state.running = !self.state.running;
            }
        }
    }

    // ── Keyboard action helpers ───────────────────────────────────────────

    fn apply_key_action(&mut self, action: KeyAction) {
        match action {
            KeyAction::Undo => {
                if self.state.history.undo(&mut self.state.project) {
                    self.state.modified = true;
                    self.state.sync_simulator();
                    self.state.status = "Undo".to_string();
                }
            }
            KeyAction::Redo => {
                if self.state.history.redo(&mut self.state.project) {
                    self.state.modified = true;
                    self.state.sync_simulator();
                    self.state.status = "Redo".to_string();
                }
            }
            KeyAction::DeleteSelected => {
                let name = self.state.active_circuit.clone();
                if let Some(circuit) = self.state.project.circuits.get(&name) {
                    let to_remove: Vec<_> = self
                        .state
                        .selected
                        .iter()
                        .filter_map(|id| circuit.components.get(id).map(|c| (*id, c.clone())))
                        .collect();
                    if !to_remove.is_empty() {
                        let actions: Vec<_> = to_remove
                            .iter()
                            .map(|(id, comp)| UndoAction::RemoveComponent {
                                circuit_name: name.clone(),
                                id: *id,
                                component: comp.clone(),
                            })
                            .collect();
                        if actions.len() == 1 {
                            self.state.history.push(actions.into_iter().next().unwrap());
                        } else {
                            self.state.history.push(UndoAction::Batch(actions));
                        }
                    }
                }
                if let Some(circuit) = self.state.project.circuits.get_mut(&name) {
                    for id in self.state.selected.drain(..) {
                        circuit.remove_component(id);
                    }
                }
                self.state.modified = true;
                self.state.sync_simulator();
            }
            KeyAction::New => {
                let mut state = AppState::new();
                let main = Circuit::new("main");
                state.project.add_circuit(main);
                state.active_circuit = "main".to_string();
                state.sync_simulator();
                self.state = state;
            }
            KeyAction::Escape => {
                self.state.tool = Tool::Select;
                self.state.wire_start = None;
            }
            KeyAction::ToggleSimulation => {
                self.state.running = !self.state.running;
            }
        }
    }

    // ── Query helpers ─────────────────────────────────────────────────────

    /// Number of components in the active circuit.
    pub fn component_count(&self) -> usize {
        let name = &self.state.active_circuit;
        self.state
            .project
            .circuits
            .get(name)
            .map_or(0, |c| c.components.len())
    }

    /// Number of wires in the active circuit.
    pub fn wire_count(&self) -> usize {
        let name = &self.state.active_circuit;
        self.state
            .project
            .circuits
            .get(name)
            .map_or(0, |c| c.wires.len())
    }

    /// Read bit 0 of the value driven on the pin of component `id`.
    pub fn read_pin_bit0(&self, id: ComponentId) -> Option<Value> {
        let name = &self.state.active_circuit;
        self.state
            .simulator
            .read_pin(name, id)
            .map(|bus| bus.get(0))
    }

    /// Set the driven value of an input pin.
    pub fn set_pin(&mut self, id: ComponentId, bus: Bus) {
        let name = self.state.active_circuit.clone();
        self.state.simulator.set_pin_value(&name, id, bus);
        let _ = self.state.simulator.propagate(&name);
    }

    /// Propagate the simulation once (without advancing the clock).
    pub fn propagate(&mut self) {
        let name = self.state.active_circuit.clone();
        let _ = self.state.simulator.propagate(&name);
    }
}

impl Default for GuiHarness {
    fn default() -> Self {
        Self::new()
    }
}

// ── Integration tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use logisim_core::value::BitWidth;

    // ── Helper ────────────────────────────────────────────────────────────────

    /// Build a 1-bit AND-gate schematic entirely through the harness.
    ///
    /// Layout (ports on the AND gate use offsets `(0, i)` from its position):
    /// ```
    ///  Pin A (0,0)  ──── AND (3,0):in0  ──out(3,2)──  Pin OUT (6,2)
    ///  Pin B (0,1)  ──── AND (3,0):in1
    /// ```
    ///
    /// Returns `(id_a, id_b, id_and, id_out)`.
    fn build_and_circuit(
        h: &mut GuiHarness,
    ) -> (ComponentId, ComponentId, ComponentId, ComponentId) {
        // Place Pin A at (0,0), Pin B at (0,1).
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 0 }); // Pin A
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 1 }); // Pin B

        // AND gate at (3,0): in0=(3,0), in1=(3,1), out=(3,2).
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(
            ComponentKind::AndGate {
                inputs: 2,
                width: BitWidth::ONE,
                negate_inputs: vec![false, false],
                negate_output: false,
            },
        )));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 0 });

        // Output Pin at (6,2).
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: true,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 6, gy: 2 });

        // Retrieve IDs by position.
        let name = h.state.active_circuit.clone();
        let circuit = h.state.project.circuits.get(&name).unwrap();
        let find = |x: i32, y: i32| {
            circuit
                .components
                .iter()
                .find(|(_, c)| c.x == x && c.y == y)
                .map(|(id, _)| *id)
                .expect("component not found")
        };
        let id_a = find(0, 0);
        let id_b = find(0, 1);
        let id_and = find(3, 0);
        let id_out = find(6, 2);

        // Draw wires that correctly align with gate port positions.
        h.dispatch(SyntheticEvent::SetTool(Tool::Wire));
        // A(0,0) → AND.in0(3,0): horizontal wire, same row.
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 0 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 0 });
        // B(0,1) → AND.in1(3,1): horizontal wire, same row.
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 1 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 1 });
        // AND.out(3,2) → OUT(6,2): horizontal wire, same row.
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 2 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 6, gy: 2 });

        h.dispatch(SyntheticEvent::SetTool(Tool::Select));
        h.propagate();

        (id_a, id_b, id_and, id_out)
    }

    // ── Test: place components through harness ────────────────────────────────

    #[test]
    fn test_place_components_via_harness() {
        let mut h = GuiHarness::new();
        assert_eq!(h.component_count(), 0);

        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 5, gy: 5 });
        assert_eq!(h.component_count(), 1);

        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 10, gy: 5 });
        assert_eq!(h.component_count(), 2);
    }

    // ── Test: draw wires through harness ─────────────────────────────────────

    #[test]
    fn test_draw_wires_via_harness() {
        let mut h = GuiHarness::new();

        h.dispatch(SyntheticEvent::SetTool(Tool::Wire));
        // First click sets the start point.
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 0 });
        assert_eq!(h.wire_count(), 0, "no wire yet — just set start point");

        // Second click on same row → one horizontal wire.
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 5, gy: 0 });
        assert_eq!(h.wire_count(), 1);

        // L-shaped wire using non-overlapping points: creates 2 new segments.
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 1, gy: 1 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 6, gy: 4 });
        assert_eq!(h.wire_count(), 3);
    }

    // ── Test: undo/redo through harness ──────────────────────────────────────

    #[test]
    fn test_undo_redo_via_harness() {
        let mut h = GuiHarness::new();

        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 5, gy: 5 });
        assert_eq!(h.component_count(), 1);

        // Undo removes the component.
        h.dispatch(SyntheticEvent::KeyAction(KeyAction::Undo));
        assert_eq!(h.component_count(), 0);

        // Redo re-adds it.
        h.dispatch(SyntheticEvent::KeyAction(KeyAction::Redo));
        assert_eq!(h.component_count(), 1);
    }

    // ── Test: component drag-to-move ─────────────────────────────────────────

    #[test]
    fn test_drag_to_move_via_harness() {
        let mut h = GuiHarness::new();

        // Place a pin at grid (5, 5).
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 5, gy: 5 });

        h.dispatch(SyntheticEvent::SetTool(Tool::Select));

        // Drag from (5,5) → (10,10).
        h.dispatch(SyntheticEvent::DragStartAtGrid { gx: 5, gy: 5 });
        h.dispatch(SyntheticEvent::DragToGrid { gx: 10, gy: 10 });
        h.dispatch(SyntheticEvent::DragEnd);

        // Component should now be at (10,10).
        let name = h.state.active_circuit.clone();
        let circuit = h.state.project.circuits.get(&name).unwrap();
        let comp = circuit.components.values().next().unwrap();
        assert_eq!((comp.x, comp.y), (10, 10));

        // Undo should restore to (5,5).
        h.dispatch(SyntheticEvent::KeyAction(KeyAction::Undo));
        let circuit = h.state.project.circuits.get(&name).unwrap();
        let comp = circuit.components.values().next().unwrap();
        assert_eq!((comp.x, comp.y), (5, 5));
    }

    // ── Test: AND-gate simulation ─────────────────────────────────────────────

    #[test]
    fn test_and_gate_simulation_via_harness() {
        let mut h = GuiHarness::new();
        let (id_a, id_b, _id_and, id_out) = build_and_circuit(&mut h);

        // Both inputs LOW → OUT should be LOW (False or Unknown after fresh propagate).
        h.set_pin(id_a, Bus::from_u64(0, 1));
        h.set_pin(id_b, Bus::from_u64(0, 1));
        h.propagate();
        let out_0_0 = h.read_pin_bit0(id_out).unwrap_or(Value::Unknown);
        assert_ne!(out_0_0, Value::True, "AND(0,0) must not be True");

        // A=1, B=0 → OUT should be LOW.
        h.set_pin(id_a, Bus::from_u64(1, 1));
        h.set_pin(id_b, Bus::from_u64(0, 1));
        h.propagate();
        let out_1_0 = h.read_pin_bit0(id_out).unwrap_or(Value::Unknown);
        assert_ne!(out_1_0, Value::True, "AND(1,0) must not be True");

        // A=1, B=1 → OUT should be HIGH.
        h.set_pin(id_a, Bus::from_u64(1, 1));
        h.set_pin(id_b, Bus::from_u64(1, 1));
        h.propagate();
        let out_1_1 = h.read_pin_bit0(id_out).unwrap_or(Value::Unknown);
        assert_eq!(out_1_1, Value::True, "AND(1,1) must be True");
    }

    // ── Test: delete selected through harness ─────────────────────────────────

    #[test]
    fn test_delete_selected_via_harness() {
        let mut h = GuiHarness::new();

        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 5, gy: 5 });
        assert_eq!(h.component_count(), 1);

        // Select the component by clicking on it.
        h.dispatch(SyntheticEvent::SetTool(Tool::Select));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 5, gy: 5 });
        assert_eq!(h.state.selected.len(), 1, "component should be selected");

        // Delete it.
        h.dispatch(SyntheticEvent::KeyAction(KeyAction::DeleteSelected));
        assert_eq!(h.component_count(), 0);

        // Undo restores it.
        h.dispatch(SyntheticEvent::KeyAction(KeyAction::Undo));
        assert_eq!(h.component_count(), 1);
    }

    // ── Test: OR-gate simulation ──────────────────────────────────────────────

    #[test]
    fn test_or_gate_via_harness() {
        let mut h = GuiHarness::new();

        // Layout: Pin A(0,0), Pin B(0,1), OR gate(3,0), Pin OUT(6,2).
        // OR gate ports: in0=(3,0), in1=(3,1), out=(3,2).
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 0 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 1 });

        h.dispatch(SyntheticEvent::SetTool(Tool::Place(
            ComponentKind::OrGate {
                inputs: 2,
                width: BitWidth::ONE,
                negate_inputs: vec![false, false],
                negate_output: false,
            },
        )));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 0 });

        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: true,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 6, gy: 2 });

        let name = h.state.active_circuit.clone();
        let circuit = h.state.project.circuits.get(&name).unwrap();
        let find = |x: i32, y: i32| {
            circuit
                .components
                .iter()
                .find(|(_, c)| c.x == x && c.y == y)
                .map(|(id, _)| *id)
                .unwrap()
        };
        let id_a = find(0, 0);
        let id_b = find(0, 1);
        let id_out = find(6, 2);

        // Connect with horizontal wires aligned to port positions.
        h.dispatch(SyntheticEvent::SetTool(Tool::Wire));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 0 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 0 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 0, gy: 1 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 1 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 2 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 6, gy: 2 });

        h.set_pin(id_a, Bus::from_u64(0, 1));
        h.set_pin(id_b, Bus::from_u64(0, 1));
        h.propagate();
        assert_ne!(
            h.read_pin_bit0(id_out).unwrap_or(Value::Unknown),
            Value::True,
            "OR(0,0) must not be True"
        );

        h.set_pin(id_a, Bus::from_u64(1, 1));
        h.set_pin(id_b, Bus::from_u64(0, 1));
        h.propagate();
        assert_eq!(
            h.read_pin_bit0(id_out).unwrap_or(Value::Unknown),
            Value::True,
            "OR(1,0) must be True"
        );
    }

    // ── Test: new project clears state ───────────────────────────────────────

    #[test]
    fn test_new_project_clears_state() {
        let mut h = GuiHarness::new();
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 5, gy: 5 });
        assert_eq!(h.component_count(), 1);

        h.dispatch(SyntheticEvent::KeyAction(KeyAction::New));
        assert_eq!(
            h.component_count(),
            0,
            "new project must clear all components"
        );
        assert_eq!(
            h.state.tool,
            Tool::Select,
            "tool must reset to Select on new project"
        );
    }

    // ── Test: remaining event variants ───────────────────────────────────────

    #[test]
    fn test_step_togglerun_setactivecircuit_escape_togglesimulation() {
        let mut h = GuiHarness::new();

        // ToggleRun / ToggleSimulation
        assert!(!h.state.running);
        h.dispatch(SyntheticEvent::ToggleRun);
        assert!(h.state.running);
        h.dispatch(SyntheticEvent::KeyAction(KeyAction::ToggleSimulation));
        assert!(!h.state.running);

        // Step advances clock_tick by 1.
        let name = h.state.active_circuit.clone();
        let before = h.state.simulator.state(&name).map_or(0, |s| s.clock_tick);
        h.dispatch(SyntheticEvent::Step);
        let after = h.state.simulator.state(&name).map_or(0, |s| s.clock_tick);
        assert_eq!(after, before + 1, "Step must advance clock_tick");

        // SetActiveCircuit switches the active circuit.
        h.state.project.add_circuit(Circuit::new("second_circuit"));
        h.dispatch(SyntheticEvent::SetActiveCircuit(
            "second_circuit".to_string(),
        ));
        assert_eq!(h.state.active_circuit, "second_circuit");
        assert!(
            h.state.selected.is_empty(),
            "selection must clear on switch"
        );

        // Escape resets wire_start and switches tool back to Select.
        h.dispatch(SyntheticEvent::SetTool(Tool::Wire));
        h.state.wire_start = Some((5, 5));
        h.dispatch(SyntheticEvent::KeyAction(KeyAction::Escape));
        assert_eq!(h.state.tool, Tool::Select);
        assert!(h.state.wire_start.is_none());
    }

    // ── Test: rubber-band selection ───────────────────────────────────────────

    #[test]
    fn test_rubber_band_selection() {
        let mut h = GuiHarness::new();

        // Place two pins at different grid positions.
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 3 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 15, gy: 15 });
        assert_eq!(h.component_count(), 2);

        // Switch to Select, start a rubber-band drag that covers only (3,3).
        h.dispatch(SyntheticEvent::SetTool(Tool::Select));
        h.dispatch(SyntheticEvent::DragStartAtGrid { gx: 0, gy: 0 }); // canvas bg → rubber band
        h.dispatch(SyntheticEvent::DragToGrid { gx: 7, gy: 7 });
        h.dispatch(SyntheticEvent::DragEnd);

        // Only the pin at (3,3) should be selected.
        assert_eq!(
            h.state.selected.len(),
            1,
            "rubber-band must select only the covered component"
        );
    }

    // ── Test: Ctrl+click additive selection ──────────────────────────────────────

    #[test]
    fn test_ctrl_click_additive_selection() {
        let mut h = GuiHarness::new();

        // Place two input pins at different positions.
        h.dispatch(SyntheticEvent::SetTool(Tool::Place(ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        })));
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 3 });
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 10, gy: 10 });
        assert_eq!(h.component_count(), 2);

        h.dispatch(SyntheticEvent::SetTool(Tool::Select));

        // Normal click selects only (3,3).
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 3, gy: 3 });
        assert_eq!(h.state.selected.len(), 1, "single click selects one");

        // Ctrl+click on (10,10) adds it to the selection.
        h.dispatch(SyntheticEvent::CtrlClickAtGrid { gx: 10, gy: 10 });
        assert_eq!(h.state.selected.len(), 2, "ctrl+click adds to selection");

        // Ctrl+click on (3,3) again removes it.
        h.dispatch(SyntheticEvent::CtrlClickAtGrid { gx: 3, gy: 3 });
        assert_eq!(h.state.selected.len(), 1, "ctrl+click toggles off");

        // Normal click on empty canvas clears selection.
        h.dispatch(SyntheticEvent::ClickAtGrid { gx: 50, gy: 50 });
        assert_eq!(h.state.selected.len(), 0, "click on empty clears selection");
    }
}
