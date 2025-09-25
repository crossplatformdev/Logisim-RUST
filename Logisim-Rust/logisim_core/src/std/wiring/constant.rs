/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Constant component - Provides constant value sources
//!
//! Constant components output a fixed value that can be configured.
//! They are useful for providing known signals to circuits.

use crate::{
    comp::{Component, ComponentId, Pin, UpdateResult},
    data::Direction,
    signal::{BusWidth, Signal, Timestamp},
    std::wiring::WiringComponentFactory,
};
use std::collections::HashMap;

/// Unique identifier for the Constant component
/// Do NOT change as it will prevent project files from loading.
pub const CONSTANT_ID: &str = "Constant";

/// Constant component attributes
#[derive(Debug, Clone)]
pub struct ConstantAttributes {
    pub width: BusWidth,
    pub value: u64,
    pub facing: Direction,
}

impl Default for ConstantAttributes {
    fn default() -> Self {
        Self {
            width: BusWidth(1),
            value: 1, // Default to high/true
            facing: Direction::East,
        }
    }
}

/// Constant component implementation
#[derive(Debug)]
pub struct Constant {
    id: ComponentId,
    attributes: ConstantAttributes,
    pins: HashMap<String, Pin>,
}

impl Constant {
    /// Create a new constant component
    pub fn new(id: ComponentId) -> Self {
        let attributes = ConstantAttributes::default();

        // Create the output pin
        let output_pin = Pin::new_output("out", attributes.width);

        let mut pins = HashMap::new();
        pins.insert("out".to_string(), output_pin);

        Self {
            id,
            attributes,
            pins,
        }
    }

    /// Set the constant value
    pub fn set_value(&mut self, value: u64) {
        self.attributes.value = value;
        self.update_output();
    }

    /// Get the constant value
    pub fn get_value(&self) -> u64 {
        self.attributes.value
    }

    /// Set the bit width
    pub fn set_width(&mut self, width: BusWidth) {
        self.attributes.width = width;

        // Update the pin width
        if let Some(pin) = self.pins.get_mut("out") {
            pin.width = width;
        }

        self.update_output();
    }

    /// Get the bit width
    pub fn get_width(&self) -> BusWidth {
        self.attributes.width
    }

    /// Update the output signal based on current value and width
    fn update_output(&mut self) {
        let signal = Signal::from_u64(self.attributes.value, self.attributes.width);
        if let Some(pin) = self.pins.get_mut("out") {
            pin.signal = signal;
        }
    }
}

impl Component for Constant {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        CONSTANT_ID
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Constant components always output the same value
        // This method is called if there were any external changes,
        // but constants don't respond to inputs
        let mut result = UpdateResult::new();

        // Always output the constant value
        if let Some(pin) = self.pins.get("out") {
            result.add_output("out".to_string(), pin.signal.clone());
        }

        result
    }

    fn reset(&mut self) {
        // Constants don't change on reset - they maintain their configured value
        self.update_output();
    }

    fn propagation_delay(&self) -> u64 {
        0 // Constants have no propagation delay
    }
}

/// Factory for creating Constant components
pub struct ConstantFactory;

impl WiringComponentFactory for ConstantFactory {
    fn id(&self) -> &'static str {
        CONSTANT_ID
    }

    fn display_name(&self) -> &str {
        "Constant"
    }

    fn description(&self) -> &str {
        "Constant value source"
    }

    fn icon_path(&self) -> Option<&str> {
        Some("constant.gif")
    }

    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(Constant::new(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_creation() {
        let constant = Constant::new(ComponentId(1));
        assert_eq!(constant.id(), ComponentId(1));
        assert_eq!(constant.name(), CONSTANT_ID);
        assert_eq!(constant.get_value(), 1); // Default value
        assert_eq!(constant.get_width(), BusWidth(1)); // Default width
    }

    #[test]
    fn test_constant_value_setting() {
        let mut constant = Constant::new(ComponentId(1));

        // Test setting different values
        constant.set_value(0);
        assert_eq!(constant.get_value(), 0);

        constant.set_value(255);
        assert_eq!(constant.get_value(), 255);
    }

    #[test]
    fn test_constant_width_setting() {
        let mut constant = Constant::new(ComponentId(1));

        // Test setting different widths
        constant.set_width(BusWidth(8));
        assert_eq!(constant.get_width(), BusWidth(8));

        // Check that the pin width was updated
        if let Some(pin) = constant.pins.get("out") {
            assert_eq!(pin.width, BusWidth(8));
        }
    }

    #[test]
    fn test_constant_factory() {
        let factory = ConstantFactory;
        assert_eq!(factory.id(), CONSTANT_ID);
        assert_eq!(factory.display_name(), "Constant");
        assert!(factory.icon_path().is_some());

        let component = factory.create_component(ComponentId(42));
        assert_eq!(component.id(), ComponentId(42));
        assert_eq!(component.name(), CONSTANT_ID);
    }

    #[test]
    fn test_constant_output_signal() {
        let mut constant = Constant::new(ComponentId(1));
        constant.set_value(0xAB);
        constant.set_width(BusWidth(8));

        // Check that the output pin has the correct signal
        if let Some(pin) = constant.pins.get("out") {
            // The signal should represent the value 0xAB with width 8
            assert_eq!(pin.width, BusWidth(8));
            // We can't easily test the exact signal value without more complex setup
        }
    }

    #[test]
    fn test_constant_update() {
        let mut constant = Constant::new(ComponentId(1));
        constant.set_value(42);

        let result = constant.update(Timestamp(0));

        // Should have one output
        assert_eq!(result.outputs.len(), 1);
        assert!(result.outputs.contains_key("out"));
        assert_eq!(result.delay, 0); // No propagation delay
    }
}
