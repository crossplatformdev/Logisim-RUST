/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Dense Logic Circuit
//!
//! Equivalent to Java DenseLogicCircuit.java
//! Provides dense logic circuit representation.

use std::collections::HashMap;

/// Dense Logic Circuit
/// 
/// Represents a dense logic circuit with optimized storage.
/// Equivalent to Java DenseLogicCircuit.
#[derive(Debug, Clone)]
pub struct DenseLogicCircuit {
    inputs: Vec<String>,
    outputs: Vec<String>,
    logic_table: HashMap<String, Vec<bool>>,
}

impl DenseLogicCircuit {
    /// Create a new dense logic circuit
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            logic_table: HashMap::new(),
        }
    }

    /// Add an input to the circuit
    pub fn add_input(&mut self, name: String) {
        self.inputs.push(name);
    }

    /// Add an output to the circuit
    pub fn add_output(&mut self, name: String) {
        self.outputs.push(name);
    }

    /// Get inputs
    pub fn get_inputs(&self) -> &[String] {
        &self.inputs
    }

    /// Get outputs
    pub fn get_outputs(&self) -> &[String] {
        &self.outputs
    }

    /// Set logic table for an output
    pub fn set_logic_table(&mut self, output: String, table: Vec<bool>) {
        self.logic_table.insert(output, table);
    }

    /// Get logic table for an output
    pub fn get_logic_table(&self, output: &str) -> Option<&Vec<bool>> {
        self.logic_table.get(output)
    }
}

impl Default for DenseLogicCircuit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense_logic_circuit() {
        let mut circuit = DenseLogicCircuit::new();
        circuit.add_input("a".to_string());
        circuit.add_input("b".to_string());
        circuit.add_output("y".to_string());
        
        assert_eq!(circuit.get_inputs().len(), 2);
        assert_eq!(circuit.get_outputs().len(), 1);
        
        circuit.set_logic_table("y".to_string(), vec![false, false, false, true]);
        assert!(circuit.get_logic_table("y").is_some());
    }
}