/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Dense Logic Circuit Builder
//!
//! Equivalent to Java DenseLogicCircuitBuilder.java
//! Provides building functionality for dense logic circuits.

use super::DenseLogicCircuit;

/// Dense Logic Circuit Builder
/// 
/// Builder for constructing dense logic circuits.
/// Equivalent to Java DenseLogicCircuitBuilder.
#[derive(Debug)]
pub struct DenseLogicCircuitBuilder {
    circuit: DenseLogicCircuit,
}

impl DenseLogicCircuitBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            circuit: DenseLogicCircuit::new(),
        }
    }

    /// Add an input to the circuit being built
    pub fn with_input(mut self, name: String) -> Self {
        self.circuit.add_input(name);
        self
    }

    /// Add an output to the circuit being built
    pub fn with_output(mut self, name: String) -> Self {
        self.circuit.add_output(name);
        self
    }

    /// Add a logic table for an output
    pub fn with_logic_table(mut self, output: String, table: Vec<bool>) -> Self {
        self.circuit.set_logic_table(output, table);
        self
    }

    /// Build the final circuit
    pub fn build(self) -> DenseLogicCircuit {
        self.circuit
    }

    /// Get a reference to the circuit being built
    pub fn circuit(&self) -> &DenseLogicCircuit {
        &self.circuit
    }

    /// Get a mutable reference to the circuit being built
    pub fn circuit_mut(&mut self) -> &mut DenseLogicCircuit {
        &mut self.circuit
    }
}

impl Default for DenseLogicCircuitBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let circuit = DenseLogicCircuitBuilder::new()
            .with_input("a".to_string())
            .with_input("b".to_string())
            .with_output("y".to_string())
            .with_logic_table("y".to_string(), vec![false, false, false, true])
            .build();
        
        assert_eq!(circuit.get_inputs().len(), 2);
        assert_eq!(circuit.get_outputs().len(), 1);
        assert!(circuit.get_logic_table("y").is_some());
    }

    #[test]
    fn test_builder_modifications() {
        let mut builder = DenseLogicCircuitBuilder::new();
        builder.circuit_mut().add_input("test".to_string());
        
        let circuit = builder.build();
        assert_eq!(circuit.get_inputs().len(), 1);
        assert_eq!(circuit.get_inputs()[0], "test");
    }
}