/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL Circuit Component
//!
//! Equivalent to Java HdlCircuitComponent.java
//! Base class for HDL-based circuit components.

use crate::comp::{Component, Pin, UpdateResult};
use crate::hdl::HdlContent;
use crate::{ComponentId, Timestamp};
use std::collections::HashMap;

/// HDL Circuit Component
/// 
/// Base implementation for HDL-based circuit components that can be used
/// to represent both VHDL entities and BLIF circuits.
/// Equivalent to Java HdlCircuitComponent.
#[derive(Debug)]
pub struct HdlCircuitComponent {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    content_type: HdlCircuitType,
}

/// Types of HDL circuits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdlCircuitType {
    /// VHDL entity
    Vhdl,
    /// BLIF circuit
    Blif,
    /// Generic HDL circuit
    Generic,
}

impl HdlCircuitComponent {
    /// Create a new HDL circuit component
    pub fn new(id: ComponentId, circuit_type: HdlCircuitType) -> Self {
        Self {
            id,
            pins: HashMap::new(),
            content_type: circuit_type,
        }
    }

    /// Get the circuit type
    pub fn get_circuit_type(&self) -> HdlCircuitType {
        self.content_type
    }

    /// Add a pin to the component
    pub fn add_pin(&mut self, name: String, pin: Pin) {
        self.pins.insert(name, pin);
    }

    /// Remove a pin from the component
    pub fn remove_pin(&mut self, name: &str) -> Option<Pin> {
        self.pins.remove(name)
    }

    /// Get a pin by name
    pub fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    /// Get all pins
    pub fn get_pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    /// Clear all pins
    pub fn clear_pins(&mut self) {
        self.pins.clear();
    }

    /// Update pins from HDL content
    pub fn update_pins_from_content<T: HdlContent>(&mut self, content: &T) {
        self.pins.clear();
        
        // In a real implementation, this would parse the HDL content
        // and create appropriate pins based on the interface
        
        // For now, just create a placeholder pin structure
        if content.is_valid() {
            // This would be implemented based on actual parsing logic
        }
    }

    /// Get input pin count
    pub fn get_input_count(&self) -> usize {
        self.pins.values().filter(|pin| pin.is_input()).count()
    }

    /// Get output pin count  
    pub fn get_output_count(&self) -> usize {
        self.pins.values().filter(|pin| pin.is_output()).count()
    }

    /// Check if component has valid pin configuration
    pub fn has_valid_pins(&self) -> bool {
        !self.pins.is_empty() && 
        self.pins.values().all(|pin| pin.is_valid())
    }
}

impl Component for HdlCircuitComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        match self.content_type {
            HdlCircuitType::Vhdl => "VHDL Circuit",
            HdlCircuitType::Blif => "BLIF Circuit", 
            HdlCircuitType::Generic => "HDL Circuit",
        }
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, timestamp: Timestamp) -> UpdateResult {
        // HDL components would implement their simulation logic here
        // For now, just indicate no changes
        UpdateResult::NoChange
    }

    fn reset(&mut self) {
        // Reset all pins to their default state
        for pin in self.pins.values_mut() {
            pin.reset();
        }
    }
}

/// HDL Circuit factory for creating different types of HDL components
pub struct HdlCircuitFactory;

impl HdlCircuitFactory {
    /// Create a new VHDL circuit component
    pub fn create_vhdl_circuit(id: ComponentId) -> HdlCircuitComponent {
        HdlCircuitComponent::new(id, HdlCircuitType::Vhdl)
    }

    /// Create a new BLIF circuit component  
    pub fn create_blif_circuit(id: ComponentId) -> HdlCircuitComponent {
        HdlCircuitComponent::new(id, HdlCircuitType::Blif)
    }

    /// Create a generic HDL circuit component
    pub fn create_generic_circuit(id: ComponentId) -> HdlCircuitComponent {
        HdlCircuitComponent::new(id, HdlCircuitType::Generic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BusWidth;

    #[test]
    fn test_hdl_circuit_creation() {
        let component = HdlCircuitComponent::new(ComponentId::new(), HdlCircuitType::Vhdl);
        assert_eq!(component.get_circuit_type(), HdlCircuitType::Vhdl);
        assert_eq!(component.get_pins().len(), 0);
    }

    #[test]
    fn test_pin_management() {
        let mut component = HdlCircuitComponent::new(ComponentId::new(), HdlCircuitType::Blif);
        
        let pin = Pin::new_input("test_pin", BusWidth(1));
        component.add_pin("test".to_string(), pin);
        
        assert_eq!(component.get_pins().len(), 1);
        assert!(component.get_pin("test").is_some());
        assert!(component.get_pin("nonexistent").is_none());
        
        let removed = component.remove_pin("test");
        assert!(removed.is_some());
        assert_eq!(component.get_pins().len(), 0);
    }

    #[test]
    fn test_pin_counting() {
        let mut component = HdlCircuitComponent::new(ComponentId::new(), HdlCircuitType::Generic);
        
        component.add_pin("in1".to_string(), Pin::new_input("in1", BusWidth(1)));
        component.add_pin("in2".to_string(), Pin::new_input("in2", BusWidth(1)));
        component.add_pin("out1".to_string(), Pin::new_output("out1", BusWidth(1)));
        
        assert_eq!(component.get_input_count(), 2);
        assert_eq!(component.get_output_count(), 1);
    }

    #[test]
    fn test_factory_methods() {
        let vhdl = HdlCircuitFactory::create_vhdl_circuit(ComponentId::new());
        assert_eq!(vhdl.get_circuit_type(), HdlCircuitType::Vhdl);
        
        let blif = HdlCircuitFactory::create_blif_circuit(ComponentId::new());
        assert_eq!(blif.get_circuit_type(), HdlCircuitType::Blif);
        
        let generic = HdlCircuitFactory::create_generic_circuit(ComponentId::new());
        assert_eq!(generic.get_circuit_type(), HdlCircuitType::Generic);
    }
}