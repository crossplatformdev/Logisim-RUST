/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Generic Interface Component
//!
//! Equivalent to Java GenericInterfaceComponent.java
//! Provides generic interface component implementation.

use crate::comp::{Component, Pin, UpdateResult};
use crate::{ComponentId, Timestamp};
use std::collections::HashMap;

/// Generic Interface Component
/// 
/// A generic interface component that can be used for various HDL interface types.
/// Equivalent to Java GenericInterfaceComponent.
#[derive(Debug)]
pub struct GenericInterfaceComponent {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    interface_type: String,
}

impl GenericInterfaceComponent {
    /// Create a new generic interface component
    pub fn new(id: ComponentId, interface_type: String) -> Self {
        Self {
            id,
            pins: HashMap::new(),
            interface_type,
        }
    }

    /// Get the interface type
    pub fn get_interface_type(&self) -> &str {
        &self.interface_type
    }

    /// Set the interface type
    pub fn set_interface_type(&mut self, interface_type: String) {
        self.interface_type = interface_type;
    }

    /// Add a pin to the interface
    pub fn add_pin(&mut self, name: String, pin: Pin) {
        self.pins.insert(name, pin);
    }

    /// Remove a pin from the interface
    pub fn remove_pin(&mut self, name: &str) -> Option<Pin> {
        self.pins.remove(name)
    }

    /// Get all pins in the interface
    pub fn get_interface_pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    /// Clear all pins
    pub fn clear_pins(&mut self) {
        self.pins.clear();
    }
}

impl Component for GenericInterfaceComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        &self.interface_type
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _timestamp: Timestamp) -> UpdateResult {
        // Generic interface components typically don't have simulation logic
        UpdateResult::new()
    }

    fn reset(&mut self) {
        // Reset all pins to their default state
        self.pins.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BusWidth;

    #[test]
    fn test_generic_interface_component() {
        let mut component = GenericInterfaceComponent::new(
            ComponentId::new(),
            "test_interface".to_string()
        );
        
        assert_eq!(component.get_interface_type(), "test_interface");
        assert_eq!(component.get_interface_pins().len(), 0);
        
        let pin = Pin::new_input("test_pin", BusWidth(1));
        component.add_pin("test".to_string(), pin);
        
        assert_eq!(component.get_interface_pins().len(), 1);
        assert!(component.get_pin_by_name("test").is_some());
    }

    #[test]
    fn test_interface_type_modification() {
        let mut component = GenericInterfaceComponent::new(
            ComponentId::new(),
            "original".to_string()
        );
        
        component.set_interface_type("modified".to_string());
        assert_eq!(component.get_interface_type(), "modified");
    }

    #[test]
    fn test_pin_management() {
        let mut component = GenericInterfaceComponent::new(
            ComponentId::new(),
            "test".to_string()
        );
        
        let pin1 = Pin::new_input("pin1", BusWidth(1));
        let pin2 = Pin::new_output("pin2", BusWidth(1));
        
        component.add_pin("p1".to_string(), pin1);
        component.add_pin("p2".to_string(), pin2);
        
        assert_eq!(component.get_pins().len(), 2);
        
        let removed = component.remove_pin("p1");
        assert!(removed.is_some());
        assert_eq!(component.get_pins().len(), 1);
        
        component.clear_pins();
        assert_eq!(component.get_pins().len(), 0);
    }
}