/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Gates Library
//!
//! Rust port of `com.cburch.logisim.std.gates.GatesLibrary`

use super::*;
use crate::comp::ComponentId;

/// Gates Library - collection of all logic gate components
///
/// This library provides access to all the standard logic gates including
/// basic gates (AND, OR, NOT, etc.), specialized gates (buffers, parity),
/// and programmable logic (PLA).
pub struct GatesLibrary {
    id: String,
}

impl GatesLibrary {
    /// Unique identifier for the gates library
    pub const ID: &'static str = "Gates";

    /// Create a new gates library
    pub fn new() -> Self {
        GatesLibrary {
            id: Self::ID.to_string(),
        }
    }

    /// Get display name for the library
    pub fn display_name() -> &'static str {
        "Gates"
    }

    /// Create an AND gate
    pub fn create_and_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(AndGate::new(id))
    }

    /// Create an OR gate
    pub fn create_or_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(OrGate::new(id))
    }

    /// Create a NOT gate
    pub fn create_not_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(NotGate::new(id))
    }

    /// Create a NAND gate
    pub fn create_nand_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(NandGate::new(id))
    }

    /// Create a NOR gate
    pub fn create_nor_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(NorGate::new(id))
    }

    /// Create an XOR gate
    pub fn create_xor_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(XorGate::new(id))
    }

    /// Create an XNOR gate
    pub fn create_xnor_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(XnorGate::new(id))
    }

    /// Create a buffer
    pub fn create_buffer(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(Buffer::new(id))
    }

    /// Create a controlled buffer
    pub fn create_controlled_buffer(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(ControlledBuffer::new(id))
    }

    /// Create an even parity gate
    pub fn create_even_parity_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(EvenParityGate::new(id))
    }

    /// Create an odd parity gate
    pub fn create_odd_parity_gate(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(OddParityGate::new(id))
    }

    /// Create a PLA
    pub fn create_pla(id: ComponentId) -> Box<dyn crate::comp::Component> {
        Box::new(Pla::new(id))
    }

    /// Get list of all available gate types
    pub fn get_gate_types() -> Vec<&'static str> {
        vec![
            "AND Gate",
            "OR Gate",
            "NOT Gate",
            "NAND Gate",
            "NOR Gate",
            "XOR Gate",
            "XNOR Gate",
            "Buffer",
            "Controlled Buffer",
            "Even Parity",
            "Odd Parity",
            "PLA",
        ]
    }

    /// Create a gate by type name
    pub fn create_gate_by_name(
        name: &str,
        id: ComponentId,
    ) -> Option<Box<dyn crate::comp::Component>> {
        match name {
            "AND Gate" => Some(Self::create_and_gate(id)),
            "OR Gate" => Some(Self::create_or_gate(id)),
            "NOT Gate" => Some(Self::create_not_gate(id)),
            "NAND Gate" => Some(Self::create_nand_gate(id)),
            "NOR Gate" => Some(Self::create_nor_gate(id)),
            "XOR Gate" => Some(Self::create_xor_gate(id)),
            "XNOR Gate" => Some(Self::create_xnor_gate(id)),
            "Buffer" => Some(Self::create_buffer(id)),
            "Controlled Buffer" => Some(Self::create_controlled_buffer(id)),
            "Even Parity" => Some(Self::create_even_parity_gate(id)),
            "Odd Parity" => Some(Self::create_odd_parity_gate(id)),
            "PLA" => Some(Self::create_pla(id)),
            _ => None,
        }
    }
}

impl Default for GatesLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gates_library_creation() {
        let library = GatesLibrary::new();
        assert_eq!(library.id, "Gates");
    }

    #[test]
    fn test_gate_creation() {
        let and_gate = GatesLibrary::create_and_gate(ComponentId(1));
        assert_eq!(and_gate.name(), "AND");

        let or_gate = GatesLibrary::create_or_gate(ComponentId(2));
        assert_eq!(or_gate.name(), "OR");

        let not_gate = GatesLibrary::create_not_gate(ComponentId(3));
        assert_eq!(not_gate.name(), "NOT");
    }

    #[test]
    fn test_gate_creation_by_name() {
        let and_gate = GatesLibrary::create_gate_by_name("AND Gate", ComponentId(1));
        assert!(and_gate.is_some());
        assert_eq!(and_gate.unwrap().name(), "AND");

        let invalid_gate = GatesLibrary::create_gate_by_name("Invalid Gate", ComponentId(1));
        assert!(invalid_gate.is_none());
    }

    #[test]
    fn test_gate_types_list() {
        let types = GatesLibrary::get_gate_types();
        assert!(types.contains(&"AND Gate"));
        assert!(types.contains(&"OR Gate"));
        assert!(types.contains(&"NOT Gate"));
        assert_eq!(types.len(), 12); // All gate types
    }
}
