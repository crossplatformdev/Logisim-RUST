//! Undo/Redo command history for circuit editing.
//!
//! Every user action that mutates a circuit is represented as an [`UndoAction`].
//! The [`UndoHistory`] keeps two stacks — undo and redo — and applies or
//! reverses actions against a [`Project`].

use crate::circuit::Wire;
use crate::component::{Component, ComponentId, ComponentKind, Facing};
use crate::project::Project;

// ── UndoAction ────────────────────────────────────────────────────────────────

/// A single reversible edit to a circuit.
#[derive(Clone, Debug)]
pub enum UndoAction {
    /// A component was added.
    AddComponent {
        circuit_name: String,
        id: ComponentId,
        component: Component,
    },
    /// A component was removed.
    RemoveComponent {
        circuit_name: String,
        id: ComponentId,
        component: Component,
    },
    /// A wire segment was added.
    AddWire { circuit_name: String, wire: Wire },
    /// A wire segment was removed.
    RemoveWire { circuit_name: String, wire: Wire },
    /// A component was moved from one grid position to another.
    MoveComponent {
        circuit_name: String,
        id: ComponentId,
        old_x: i32,
        old_y: i32,
        new_x: i32,
        new_y: i32,
    },
    /// A component's label was changed.
    ChangeLabel {
        circuit_name: String,
        id: ComponentId,
        old_label: String,
        new_label: String,
    },
    /// A component's facing direction was changed.
    ChangeFacing {
        circuit_name: String,
        id: ComponentId,
        old_facing: Facing,
        new_facing: Facing,
    },
    /// A component's kind (and thus its kind-specific attributes) was changed.
    ChangeKind {
        circuit_name: String,
        id: ComponentId,
        old_kind: ComponentKind,
        new_kind: ComponentKind,
    },
    /// A batch of actions that should be treated as one undo step.
    Batch(Vec<UndoAction>),
}

impl UndoAction {
    /// Returns the inverse of this action (what needs to happen to undo it).
    pub fn inverse(&self) -> Self {
        match self.clone() {
            UndoAction::AddComponent {
                circuit_name,
                id,
                component,
            } => UndoAction::RemoveComponent {
                circuit_name,
                id,
                component,
            },
            UndoAction::RemoveComponent {
                circuit_name,
                id,
                component,
            } => UndoAction::AddComponent {
                circuit_name,
                id,
                component,
            },
            UndoAction::AddWire { circuit_name, wire } => {
                UndoAction::RemoveWire { circuit_name, wire }
            }
            UndoAction::RemoveWire { circuit_name, wire } => {
                UndoAction::AddWire { circuit_name, wire }
            }
            UndoAction::MoveComponent {
                circuit_name,
                id,
                old_x,
                old_y,
                new_x,
                new_y,
            } => UndoAction::MoveComponent {
                circuit_name,
                id,
                old_x: new_x,
                old_y: new_y,
                new_x: old_x,
                new_y: old_y,
            },
            UndoAction::ChangeLabel {
                circuit_name,
                id,
                old_label,
                new_label,
            } => UndoAction::ChangeLabel {
                circuit_name,
                id,
                old_label: new_label,
                new_label: old_label,
            },
            UndoAction::ChangeFacing {
                circuit_name,
                id,
                old_facing,
                new_facing,
            } => UndoAction::ChangeFacing {
                circuit_name,
                id,
                old_facing: new_facing,
                new_facing: old_facing,
            },
            UndoAction::ChangeKind {
                circuit_name,
                id,
                old_kind,
                new_kind,
            } => UndoAction::ChangeKind {
                circuit_name,
                id,
                old_kind: new_kind,
                new_kind: old_kind,
            },
            UndoAction::Batch(actions) => {
                // Reverse the order so sub-actions undo correctly.
                let inversed: Vec<_> = actions.iter().rev().map(|a| a.inverse()).collect();
                UndoAction::Batch(inversed)
            }
        }
    }

    /// Apply this action to the project, mutating circuits accordingly.
    pub fn apply(&self, project: &mut Project) {
        match self {
            UndoAction::AddComponent {
                circuit_name,
                id,
                component,
            } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    circuit.insert_component_with_id(*id, component.clone());
                }
            }
            UndoAction::RemoveComponent {
                circuit_name, id, ..
            } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    circuit.components.remove(id);
                }
            }
            UndoAction::AddWire { circuit_name, wire } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    circuit.add_wire(wire.from.x, wire.from.y, wire.to.x, wire.to.y);
                }
            }
            UndoAction::RemoveWire { circuit_name, wire } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    circuit.remove_wire(wire.from.x, wire.from.y, wire.to.x, wire.to.y);
                }
            }
            UndoAction::MoveComponent {
                circuit_name,
                id,
                new_x,
                new_y,
                ..
            } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    if let Some(comp) = circuit.components.get_mut(id) {
                        comp.x = *new_x;
                        comp.y = *new_y;
                    }
                }
            }
            UndoAction::ChangeLabel {
                circuit_name,
                id,
                new_label,
                ..
            } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    if let Some(comp) = circuit.components.get_mut(id) {
                        comp.label = new_label.clone();
                    }
                }
            }
            UndoAction::ChangeFacing {
                circuit_name,
                id,
                new_facing,
                ..
            } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    if let Some(comp) = circuit.components.get_mut(id) {
                        comp.facing = *new_facing;
                    }
                }
            }
            UndoAction::ChangeKind {
                circuit_name,
                id,
                new_kind,
                ..
            } => {
                if let Some(circuit) = project.circuits.get_mut(circuit_name) {
                    if let Some(comp) = circuit.components.get_mut(id) {
                        comp.kind = new_kind.clone();
                    }
                }
            }
            UndoAction::Batch(actions) => {
                for action in actions {
                    action.apply(project);
                }
            }
        }
    }
}

// ── UndoHistory ───────────────────────────────────────────────────────────────

/// A bounded undo/redo history stack.
pub struct UndoHistory {
    undo_stack: std::collections::VecDeque<UndoAction>,
    redo_stack: std::collections::VecDeque<UndoAction>,
    /// Maximum number of undo steps retained.
    max_size: usize,
}

impl UndoHistory {
    /// Create a new history with the given capacity.
    pub fn new(max_size: usize) -> Self {
        UndoHistory {
            undo_stack: std::collections::VecDeque::new(),
            redo_stack: std::collections::VecDeque::new(),
            max_size,
        }
    }

    /// Record a new action.  Clears the redo stack.
    pub fn push(&mut self, action: UndoAction) {
        self.redo_stack.clear();
        self.undo_stack.push_back(action);
        if self.undo_stack.len() > self.max_size {
            // Evict the oldest (front) entry in O(1).
            self.undo_stack.pop_front();
        }
    }

    /// Returns `true` if there is at least one action to undo.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns `true` if there is at least one action to redo.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Undo the most recent action, returning `true` on success.
    ///
    /// The caller is responsible for calling [`Project`] sync after this.
    pub fn undo(&mut self, project: &mut Project) -> bool {
        if let Some(action) = self.undo_stack.pop_back() {
            let inverse = action.inverse();
            inverse.apply(project);
            // Push the original action onto redo so it can be re-applied.
            self.redo_stack.push_back(action);
            true
        } else {
            false
        }
    }

    /// Redo the most recently undone action, returning `true` on success.
    pub fn redo(&mut self, project: &mut Project) -> bool {
        if let Some(action) = self.redo_stack.pop_back() {
            action.apply(project);
            // Push the original action back onto undo so it can be undone again.
            self.undo_stack.push_back(action);
            true
        } else {
            false
        }
    }

    /// Clear all history (e.g. after loading a new file).
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for UndoHistory {
    fn default() -> Self {
        UndoHistory::new(200)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;
    use crate::component::ComponentKind;
    use crate::value::BitWidth;

    fn make_project() -> Project {
        let mut p = Project::new("test");
        p.add_circuit(Circuit::new("main"));
        p
    }

    #[test]
    fn test_undo_add_component() {
        let mut p = make_project();
        let mut hist = UndoHistory::new(10);

        // Add a component via action.
        let kind = ComponentKind::Pin {
            is_output: false,
            width: BitWidth::ONE,
        };
        let id = {
            let c = p.circuits.get_mut("main").unwrap();
            c.add_component(kind.clone(), 5, 5)
        };
        let comp = p.circuits["main"].components[&id].clone();
        hist.push(UndoAction::AddComponent {
            circuit_name: "main".to_string(),
            id,
            component: comp,
        });

        assert!(p.circuits["main"].components.contains_key(&id));
        assert!(hist.can_undo());

        hist.undo(&mut p);
        assert!(!p.circuits["main"].components.contains_key(&id));
        assert!(hist.can_redo());

        hist.redo(&mut p);
        assert!(p.circuits["main"].components.contains_key(&id));
    }

    #[test]
    fn test_undo_add_wire() {
        let mut p = make_project();
        let mut hist = UndoHistory::new(10);

        p.circuits.get_mut("main").unwrap().add_wire(0, 0, 10, 0);
        hist.push(UndoAction::AddWire {
            circuit_name: "main".to_string(),
            wire: Wire::new(0, 0, 10, 0),
        });

        assert_eq!(p.circuits["main"].wires.len(), 1);
        hist.undo(&mut p);
        assert_eq!(p.circuits["main"].wires.len(), 0);
        hist.redo(&mut p);
        assert_eq!(p.circuits["main"].wires.len(), 1);
    }

    #[test]
    fn test_undo_move_component() {
        let mut p = make_project();
        let mut hist = UndoHistory::new(10);

        let id = {
            let c = p.circuits.get_mut("main").unwrap();
            c.add_component(ComponentKind::Clock, 0, 0)
        };
        // Move component to (5, 5).
        {
            let c = p.circuits.get_mut("main").unwrap();
            hist.push(UndoAction::MoveComponent {
                circuit_name: "main".to_string(),
                id,
                old_x: 0,
                old_y: 0,
                new_x: 5,
                new_y: 5,
            });
            c.components.get_mut(&id).unwrap().x = 5;
            c.components.get_mut(&id).unwrap().y = 5;
        }
        assert_eq!(p.circuits["main"].components[&id].x, 5);

        hist.undo(&mut p);
        assert_eq!(p.circuits["main"].components[&id].x, 0);

        hist.redo(&mut p);
        assert_eq!(p.circuits["main"].components[&id].x, 5);
    }

    #[test]
    fn test_max_size_evicts_oldest() {
        let mut hist = UndoHistory::new(3);

        for _ in 0..5 {
            hist.push(UndoAction::AddWire {
                circuit_name: "main".to_string(),
                wire: Wire::new(0, 0, 1, 0),
            });
        }
        // Only 3 entries retained.
        assert_eq!(hist.undo_stack.len(), 3);
    }

    #[test]
    fn test_redo_cleared_on_new_action() {
        let mut p = make_project();
        let mut hist = UndoHistory::new(10);

        p.circuits.get_mut("main").unwrap().add_wire(0, 0, 10, 0);
        hist.push(UndoAction::AddWire {
            circuit_name: "main".to_string(),
            wire: Wire::new(0, 0, 10, 0),
        });
        hist.undo(&mut p);
        assert!(hist.can_redo());

        // New action clears redo stack.
        hist.push(UndoAction::AddWire {
            circuit_name: "main".to_string(),
            wire: Wire::new(5, 5, 15, 5),
        });
        assert!(!hist.can_redo());
    }

    #[test]
    fn test_undo_change_label() {
        let mut p = make_project();
        let mut hist = UndoHistory::new(10);

        let id = {
            let c = p.circuits.get_mut("main").unwrap();
            c.add_component(ComponentKind::Clock, 0, 0)
        };

        // Change label from "" to "clk".
        hist.push(UndoAction::ChangeLabel {
            circuit_name: "main".to_string(),
            id,
            old_label: String::new(),
            new_label: "clk".to_string(),
        });
        p.circuits
            .get_mut("main")
            .unwrap()
            .components
            .get_mut(&id)
            .unwrap()
            .label = "clk".to_string();

        assert_eq!(p.circuits["main"].components[&id].label, "clk");

        hist.undo(&mut p);
        assert_eq!(p.circuits["main"].components[&id].label, "");

        hist.redo(&mut p);
        assert_eq!(p.circuits["main"].components[&id].label, "clk");
    }

    #[test]
    fn test_undo_change_facing() {
        use crate::component::Facing;

        let mut p = make_project();
        let mut hist = UndoHistory::new(10);

        let id = {
            let c = p.circuits.get_mut("main").unwrap();
            c.add_component(ComponentKind::Clock, 0, 0)
        };

        // Default facing is East; change to North via action.apply().
        let action = UndoAction::ChangeFacing {
            circuit_name: "main".to_string(),
            id,
            old_facing: Facing::East,
            new_facing: Facing::North,
        };
        action.apply(&mut p);
        hist.push(action);

        assert_eq!(p.circuits["main"].components[&id].facing, Facing::North);

        hist.undo(&mut p);
        assert_eq!(p.circuits["main"].components[&id].facing, Facing::East);

        hist.redo(&mut p);
        assert_eq!(p.circuits["main"].components[&id].facing, Facing::North);
    }

    #[test]
    fn test_undo_change_kind() {
        use crate::value::BitWidth;

        let mut p = make_project();
        let mut hist = UndoHistory::new(10);

        // Add a 2-input AND gate.
        let id = {
            let c = p.circuits.get_mut("main").unwrap();
            c.add_component(
                ComponentKind::AndGate {
                    inputs: 2,
                    width: BitWidth::ONE,
                    negate_inputs: vec![false, false],
                    negate_output: false,
                },
                0,
                0,
            )
        };

        let old_kind = p.circuits["main"].components[&id].kind.clone();
        let new_kind = ComponentKind::AndGate {
            inputs: 3,
            width: BitWidth::ONE,
            negate_inputs: vec![false, false, false],
            negate_output: false,
        };

        // Change input count from 2 to 3.
        let action = UndoAction::ChangeKind {
            circuit_name: "main".to_string(),
            id,
            old_kind,
            new_kind,
        };
        action.apply(&mut p);
        hist.push(action);

        assert!(matches!(
            p.circuits["main"].components[&id].kind,
            ComponentKind::AndGate { inputs: 3, .. }
        ));

        hist.undo(&mut p);
        assert!(matches!(
            p.circuits["main"].components[&id].kind,
            ComponentKind::AndGate { inputs: 2, .. }
        ));

        hist.redo(&mut p);
        assert!(matches!(
            p.circuits["main"].components[&id].kind,
            ComponentKind::AndGate { inputs: 3, .. }
        ));
    }
}
