//! Circuit data model.
//!
//! A [`Circuit`] holds a set of [`Component`]s and [`Wire`]s.
//! Components are addressed by [`ComponentId`]; wires are addressed by their
//! two endpoints.  The model is an exact structural representation matching the
//! Logisim-Evolution XML format.

use crate::component::{Component, ComponentId, ComponentKind, Facing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// ── CircuitId ─────────────────────────────────────────────────────────────────

/// A unique handle for a circuit within a project.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct CircuitId(pub String);

impl fmt::Display for CircuitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── Wire ──────────────────────────────────────────────────────────────────────

/// One endpoint of a wire: a grid coordinate.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct WireEnd {
    pub x: i32,
    pub y: i32,
}

impl WireEnd {
    pub fn new(x: i32, y: i32) -> Self {
        WireEnd { x, y }
    }
}

impl fmt::Display for WireEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A single wire segment between two grid points.
///
/// Logisim wires are always axis-aligned (horizontal or vertical).
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Wire {
    pub from: WireEnd,
    pub to: WireEnd,
}

impl Wire {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Wire {
            from: WireEnd::new(x1, y1),
            to: WireEnd::new(x2, y2),
        }
    }

    /// Returns `true` if this wire is horizontal (same Y coordinate).
    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    /// Returns `true` if this wire is vertical (same X coordinate).
    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    /// Returns `true` if the given point lies on this wire segment.
    pub fn contains(&self, x: i32, y: i32) -> bool {
        if self.is_horizontal() {
            y == self.from.y
                && x >= self.from.x.min(self.to.x)
                && x <= self.from.x.max(self.to.x)
        } else {
            x == self.from.x
                && y >= self.from.y.min(self.to.y)
                && y <= self.from.y.max(self.to.y)
        }
    }
}

// ── NetNode ───────────────────────────────────────────────────────────────────

/// Identifies one end of a connection: either a component port or a wire node.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum NetNode {
    /// A port on a specific component.
    Port {
        component: ComponentId,
        port_name: String,
    },
    /// A free wire junction point.
    Point { x: i32, y: i32 },
}

// ── Circuit ───────────────────────────────────────────────────────────────────

/// A single circuit sheet (analogous to one `.circ` file circuit element).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Circuit {
    /// Circuit name (used as an identifier when referenced as a subcircuit).
    pub name: String,
    /// All component instances.
    pub components: HashMap<ComponentId, Component>,
    /// All wire segments.
    pub wires: Vec<Wire>,
    /// Next free component ID.
    next_id: u64,
    /// Circuit-level attributes (appearance, description, etc.).
    pub attributes: HashMap<String, String>,
}

impl Circuit {
    /// Create an empty circuit with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Circuit {
            name: name.into(),
            components: HashMap::new(),
            wires: Vec::new(),
            next_id: 1,
            attributes: HashMap::new(),
        }
    }

    /// Add a component and return its newly assigned ID.
    pub fn add_component(&mut self, kind: ComponentKind, x: i32, y: i32) -> ComponentId {
        let id = ComponentId(self.next_id);
        self.next_id += 1;
        self.components
            .insert(id, Component::new(id, kind, x, y));
        id
    }

    /// Add a component with a label.
    pub fn add_component_with_label(
        &mut self,
        kind: ComponentKind,
        x: i32,
        y: i32,
        label: impl Into<String>,
    ) -> ComponentId {
        let id = self.add_component(kind, x, y);
        self.components.get_mut(&id).unwrap().label = label.into();
        id
    }

    /// Remove a component by ID.  Returns `true` if it existed.
    pub fn remove_component(&mut self, id: ComponentId) -> bool {
        self.components.remove(&id).is_some()
    }

    /// Get an immutable reference to a component.
    pub fn get_component(&self, id: ComponentId) -> Option<&Component> {
        self.components.get(&id)
    }

    /// Get a mutable reference to a component.
    pub fn get_component_mut(&mut self, id: ComponentId) -> Option<&mut Component> {
        self.components.get_mut(&id)
    }

    /// Add a wire segment.
    pub fn add_wire(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let wire = Wire::new(x1, y1, x2, y2);
        if !self.wires.contains(&wire) {
            self.wires.push(wire);
        }
    }

    /// Remove a wire segment.  Returns `true` if it existed.
    pub fn remove_wire(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let wire = Wire::new(x1, y1, x2, y2);
        if let Some(pos) = self.wires.iter().position(|w| w == &wire) {
            self.wires.remove(pos);
            true
        } else {
            false
        }
    }

    /// Find all components whose port lies at the given grid coordinate.
    pub fn components_at_point(&self, x: i32, y: i32) -> Vec<(ComponentId, String)> {
        let mut result = Vec::new();
        for (id, comp) in &self.components {
            for (port_name, pos) in comp.all_port_positions() {
                if pos == (x, y) {
                    result.push((*id, port_name));
                }
            }
        }
        result
    }

    /// Find all wires touching the given grid point.
    pub fn wires_at_point(&self, x: i32, y: i32) -> Vec<&Wire> {
        self.wires.iter().filter(|w| w.contains(x, y)).collect()
    }

    /// Compute the connected "net" sets using union-find over grid points.
    ///
    /// Returns a map from each point to its canonical representative point.
    pub fn compute_nets(&self) -> HashMap<(i32, i32), (i32, i32)> {
        // Collect all grid points that are mentioned by wires.
        let mut parent: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        let find = |parent: &mut HashMap<(i32, i32), (i32, i32)>, mut x: (i32, i32)| -> (i32, i32) {
            loop {
                let p = *parent.entry(x).or_insert(x);
                if p == x {
                    return x;
                }
                let gp = *parent.entry(p).or_insert(p);
                parent.insert(x, gp);
                x = gp;
            }
        };

        // Union all points that share a wire.
        for wire in &self.wires {
            let p1 = (wire.from.x, wire.from.y);
            let p2 = (wire.to.x, wire.to.y);
            // Union all intermediate points on axis-aligned segments.
            if wire.is_horizontal() {
                let y = wire.from.y;
                let x_min = wire.from.x.min(wire.to.x);
                let x_max = wire.from.x.max(wire.to.x);
                for x in x_min..=x_max {
                    let pt = (x, y);
                    let root_p1 = find(&mut parent, p1);
                    let root_pt = find(&mut parent, pt);
                    if root_p1 != root_pt {
                        parent.insert(root_pt, root_p1);
                    }
                }
                // p2 was visited in the loop above; no extra union needed.
            } else if wire.is_vertical() {
                let x = wire.from.x;
                let y_min = wire.from.y.min(wire.to.y);
                let y_max = wire.from.y.max(wire.to.y);
                for y in y_min..=y_max {
                    let pt = (x, y);
                    let root_p1 = find(&mut parent, p1);
                    let root_pt = find(&mut parent, pt);
                    if root_p1 != root_pt {
                        parent.insert(root_pt, root_p1);
                    }
                }
                // p2 was visited in the loop above; no extra union needed.
            } else {
                // Diagonal wire: only union the two endpoints.
                let root_p1 = find(&mut parent, p1);
                let root_p2 = find(&mut parent, p2);
                if root_p1 != root_p2 {
                    parent.insert(root_p2, root_p1);
                }
            }
        }

        // Path-compress every entry.
        let keys: Vec<(i32, i32)> = parent.keys().copied().collect();
        for k in keys {
            find(&mut parent, k);
        }
        parent
    }

    /// Return all input pins in this circuit (output=false pins).
    pub fn input_pins(&self) -> Vec<&Component> {
        self.components
            .values()
            .filter(|c| matches!(c.kind, ComponentKind::Pin { is_output: false, .. }))
            .collect()
    }

    /// Return all output pins in this circuit (output=true pins).
    pub fn output_pins(&self) -> Vec<&Component> {
        self.components
            .values()
            .filter(|c| matches!(c.kind, ComponentKind::Pin { is_output: true, .. }))
            .collect()
    }

    /// Set the facing of a component.
    pub fn set_facing(&mut self, id: ComponentId, facing: Facing) {
        if let Some(comp) = self.components.get_mut(&id) {
            comp.facing = facing;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::BitWidth;

    fn make_circuit() -> Circuit {
        let mut c = Circuit::new("main");
        // Add an AND gate
        c.add_component(
            ComponentKind::AndGate {
                inputs: 2,
                width: BitWidth::ONE,
                negate_inputs: vec![false, false],
                negate_output: false,
            },
            10,
            10,
        );
        // Add two input pins
        c.add_component(ComponentKind::Pin { is_output: false, width: BitWidth::ONE }, 0, 10);
        c.add_component(ComponentKind::Pin { is_output: false, width: BitWidth::ONE }, 0, 11);
        // Add one output pin
        c.add_component(ComponentKind::Pin { is_output: true, width: BitWidth::ONE }, 20, 10);
        // Wire them up
        c.add_wire(0, 10, 10, 10);
        c.add_wire(0, 11, 10, 11);
        c.add_wire(12, 10, 20, 10);
        c
    }

    #[test]
    fn test_add_remove_component() {
        let mut c = Circuit::new("test");
        let id = c.add_component(ComponentKind::Clock, 0, 0);
        assert!(c.get_component(id).is_some());
        assert!(c.remove_component(id));
        assert!(c.get_component(id).is_none());
    }

    #[test]
    fn test_wire_containment() {
        let w = Wire::new(0, 0, 10, 0);
        assert!(w.is_horizontal());
        assert!(w.contains(5, 0));
        assert!(!w.contains(5, 1));
        assert!(!w.contains(11, 0));
    }

    #[test]
    fn test_circuit_input_output_pins() {
        let c = make_circuit();
        assert_eq!(c.input_pins().len(), 2);
        assert_eq!(c.output_pins().len(), 1);
    }

    #[test]
    fn test_compute_nets_simple() {
        let mut c = Circuit::new("test");
        c.add_wire(0, 0, 10, 0);
        c.add_wire(10, 0, 20, 0);
        let nets = c.compute_nets();
        // All points on a single connected segment should share the same root.
        let root0 = nets[&(0, 0)];
        let root10 = nets[&(10, 0)];
        let root20 = nets[&(20, 0)];
        assert_eq!(root0, root10);
        assert_eq!(root10, root20);
    }

    #[test]
    fn test_no_duplicate_wires() {
        let mut c = Circuit::new("test");
        c.add_wire(0, 0, 10, 0);
        c.add_wire(0, 0, 10, 0);
        assert_eq!(c.wires.len(), 1);
    }

    #[test]
    fn test_components_at_point() {
        let c = make_circuit();
        // input pin at (0,10) has output port at (0,10)
        let at = c.components_at_point(0, 10);
        assert!(!at.is_empty());
    }
}
