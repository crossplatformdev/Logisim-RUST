//! Project model: a collection of named circuits.

use crate::circuit::Circuit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Logisim-Evolution project: the top-level container for circuits and
/// project-level settings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    /// Project name.
    pub name: String,
    /// All circuits in this project, keyed by circuit name.
    pub circuits: HashMap<String, Circuit>,
    /// The name of the "main" (top-level) circuit.
    pub main_circuit: Option<String>,
    /// Project-level tool/option settings.
    pub options: HashMap<String, String>,
    /// Ordered list of circuit names (for deterministic display order).
    pub circuit_order: Vec<String>,
}

impl Project {
    /// Create a new empty project.
    pub fn new(name: impl Into<String>) -> Self {
        Project {
            name: name.into(),
            circuits: HashMap::new(),
            main_circuit: None,
            options: HashMap::new(),
            circuit_order: Vec::new(),
        }
    }

    /// Add a circuit to the project.
    pub fn add_circuit(&mut self, circuit: Circuit) {
        let name = circuit.name.clone();
        if self.main_circuit.is_none() {
            self.main_circuit = Some(name.clone());
        }
        if !self.circuit_order.contains(&name) {
            self.circuit_order.push(name.clone());
        }
        self.circuits.insert(name, circuit);
    }

    /// Get the main circuit name, defaulting to the first circuit.
    pub fn main_circuit_name(&self) -> Option<&str> {
        self.main_circuit
            .as_deref()
            .or_else(|| self.circuit_order.first().map(|s| s.as_str()))
    }

    /// Remove a circuit by name.
    pub fn remove_circuit(&mut self, name: &str) -> bool {
        self.circuit_order.retain(|n| n != name);
        if self.main_circuit.as_deref() == Some(name) {
            self.main_circuit = self.circuit_order.first().cloned();
        }
        self.circuits.remove(name).is_some()
    }

    /// Return circuits in their defined order.
    pub fn ordered_circuits(&self) -> Vec<&Circuit> {
        self.circuit_order
            .iter()
            .filter_map(|n| self.circuits.get(n))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;

    #[test]
    fn test_project_add_circuit() {
        let mut p = Project::new("test");
        p.add_circuit(Circuit::new("main"));
        p.add_circuit(Circuit::new("sub"));
        assert_eq!(p.circuits.len(), 2);
        assert_eq!(p.main_circuit_name(), Some("main"));
    }

    #[test]
    fn test_project_remove_circuit() {
        let mut p = Project::new("test");
        p.add_circuit(Circuit::new("main"));
        p.add_circuit(Circuit::new("sub"));
        assert!(p.remove_circuit("main"));
        assert_eq!(p.circuits.len(), 1);
        assert_eq!(p.main_circuit_name(), Some("sub"));
    }

    #[test]
    fn test_ordered_circuits() {
        let mut p = Project::new("test");
        p.add_circuit(Circuit::new("a"));
        p.add_circuit(Circuit::new("b"));
        p.add_circuit(Circuit::new("c"));
        let names: Vec<_> = p.ordered_circuits().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }
}
