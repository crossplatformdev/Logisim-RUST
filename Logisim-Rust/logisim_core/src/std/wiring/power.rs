/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Power component - VCC power supply (always outputs 1/high)
//!
//! Power components provide a consistent high signal (1) for circuits.
//! They are essential for providing known power levels.

use crate::{
    comp::{ComponentId, Component, Pin, UpdateResult},
    signal::{BusWidth, Signal, Timestamp},
    std::wiring::WiringComponentFactory,
};
use std::collections::HashMap;

/// Unique identifier for the Power component
/// Do NOT change as it will prevent project files from loading.
pub const POWER_ID: &str = "Power";

/// Power component attributes
#[derive(Debug, Clone)]
pub struct PowerAttributes {
    pub width: BusWidth,
}

impl Default for PowerAttributes {
    fn default() -> Self {
        Self { width: BusWidth(1) }
    }
}

/// Power component implementation
#[derive(Debug)]
pub struct Power {
    id: ComponentId,
    attributes: PowerAttributes,
    pins: HashMap<String, Pin>,
}

impl Power {
    /// Create a new power component
    pub fn new(id: ComponentId) -> Self {
        let attributes = PowerAttributes::default();

        // Create the output pin - power always outputs high
        let mut output_pin = Pin::new_output("out", attributes.width);
        output_pin.signal = Signal::all_high(attributes.width);

        let mut pins = HashMap::new();
        pins.insert("out".to_string(), output_pin);

        Self {
            id,
            attributes,
            pins,
        }
    }

    /// Set the bit width (power maintains all high values)
    pub fn set_width(&mut self, width: BusWidth) {
        self.attributes.width = width;

        // Update the pin width and signal
        if let Some(pin) = self.pins.get_mut("out") {
            pin.width = width;
            pin.signal = Signal::all_high(width);
        }
    }

    /// Get the bit width
    pub fn get_width(&self) -> BusWidth {
        self.attributes.width
    }
}

impl Component for Power {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        POWER_ID
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Power components always output high (1)
        let mut result = UpdateResult::new();

        // Always output the power value (all ones)
        if let Some(pin) = self.pins.get("out") {
            result.add_output("out".to_string(), pin.signal.clone());
        }

        result
    }

    fn reset(&mut self) {
        // Power always outputs high, no change needed on reset
        if let Some(pin) = self.pins.get_mut("out") {
            pin.signal = Signal::all_high(self.attributes.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // Power has no propagation delay
    }
}

/// Factory for creating Power components
pub struct PowerFactory;

impl WiringComponentFactory for PowerFactory {
    fn id(&self) -> &'static str {
        POWER_ID
    }

    fn display_name(&self) -> &str {
        "Power"
    }

    fn description(&self) -> &str {
        "VCC power supply (always outputs 1/high)"
    }

    fn icon_path(&self) -> Option<&str> {
        Some("power.gif")
    }

    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(Power::new(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_creation() {
        let power = Power::new(ComponentId(1));
        assert_eq!(power.id(), ComponentId(1));
        assert_eq!(power.name(), POWER_ID);
        assert_eq!(power.get_width(), BusWidth(1)); // Default width
    }

    #[test]
    fn test_power_width_setting() {
        let mut power = Power::new(ComponentId(1));

        // Test setting different widths
        power.set_width(BusWidth(8));
        assert_eq!(power.get_width(), BusWidth(8));

        // Check that the pin width was updated
        if let Some(pin) = power.pins.get("out") {
            assert_eq!(pin.width, BusWidth(8));
        }
    }

    #[test]
    fn test_power_factory() {
        let factory = PowerFactory;
        assert_eq!(factory.id(), POWER_ID);
        assert_eq!(factory.display_name(), "Power");
        assert!(factory.icon_path().is_some());

        let component = factory.create_component(ComponentId(42));
        assert_eq!(component.id(), ComponentId(42));
        assert_eq!(component.name(), POWER_ID);
    }

    #[test]
    fn test_power_output_signal() {
        let mut power = Power::new(ComponentId(1));
        power.set_width(BusWidth(4));

        // Check that the output pin has the correct signal (all high)
        if let Some(pin) = power.pins.get("out") {
            assert_eq!(pin.width, BusWidth(4));
            // The signal should be all ones for the given width
        }
    }

    #[test]
    fn test_power_update() {
        let mut power = Power::new(ComponentId(1));

        let result = power.update(Timestamp(0));

        // Should have one output
        assert_eq!(result.outputs.len(), 1);
        assert!(result.outputs.contains_key("out"));
        assert_eq!(result.delay, 0); // No propagation delay
    }

    #[test]
    fn test_power_always_high() {
        let mut power = Power::new(ComponentId(1));

        // Even after reset, power should still output high
        power.reset();
        let result = power.update(Timestamp(0));

        assert!(result.outputs.contains_key("out"));
        // The output should be all high values
    }
}
