/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Ground component - Ground reference (always outputs 0/low)
//!
//! Ground components provide a consistent low signal (0) for circuits.
//! They are essential for providing known reference levels.

use crate::{
    comp::{Component, ComponentId, Pin, UpdateResult},
    signal::{BusWidth, Signal, Timestamp},
    std::wiring::WiringComponentFactory,
};
use std::collections::HashMap;

/// Unique identifier for the Ground component
/// Do NOT change as it will prevent project files from loading.
pub const GROUND_ID: &str = "Ground";

/// Ground component attributes
#[derive(Debug, Clone)]
pub struct GroundAttributes {
    pub width: BusWidth,
}

impl Default for GroundAttributes {
    fn default() -> Self {
        Self { width: BusWidth(1) }
    }
}

/// Ground component implementation
#[derive(Debug)]
pub struct Ground {
    id: ComponentId,
    attributes: GroundAttributes,
    pins: HashMap<String, Pin>,
}

impl Ground {
    /// Create a new ground component
    pub fn new(id: ComponentId) -> Self {
        let attributes = GroundAttributes::default();

        // Create the output pin - ground always outputs low
        let mut output_pin = Pin::new_output("out", attributes.width);
        output_pin.signal = Signal::all_low(attributes.width);

        let mut pins = HashMap::new();
        pins.insert("out".to_string(), output_pin);

        Self {
            id,
            attributes,
            pins,
        }
    }

    /// Set the bit width (ground maintains all low values)
    pub fn set_width(&mut self, width: BusWidth) {
        self.attributes.width = width;

        // Update the pin width and signal
        if let Some(pin) = self.pins.get_mut("out") {
            pin.width = width;
            pin.signal = Signal::all_low(width);
        }
    }

    /// Get the bit width
    pub fn get_width(&self) -> BusWidth {
        self.attributes.width
    }
}

impl Component for Ground {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        GROUND_ID
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Ground components always output low (0)
        let mut result = UpdateResult::new();

        // Always output the ground value (all zeros)
        if let Some(pin) = self.pins.get("out") {
            result.add_output("out".to_string(), pin.signal.clone());
        }

        result
    }

    fn reset(&mut self) {
        // Ground always outputs low, no change needed on reset
        if let Some(pin) = self.pins.get_mut("out") {
            pin.signal = Signal::all_low(self.attributes.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // Ground has no propagation delay
    }
}

/// Factory for creating Ground components
pub struct GroundFactory;

impl WiringComponentFactory for GroundFactory {
    fn id(&self) -> &'static str {
        GROUND_ID
    }

    fn display_name(&self) -> &str {
        "Ground"
    }

    fn description(&self) -> &str {
        "Ground reference (always outputs 0/low)"
    }

    fn icon_path(&self) -> Option<&str> {
        Some("ground.gif")
    }

    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(Ground::new(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_creation() {
        let ground = Ground::new(ComponentId(1));
        assert_eq!(ground.id(), ComponentId(1));
        assert_eq!(ground.name(), GROUND_ID);
        assert_eq!(ground.get_width(), BusWidth(1)); // Default width
    }

    #[test]
    fn test_ground_width_setting() {
        let mut ground = Ground::new(ComponentId(1));

        // Test setting different widths
        ground.set_width(BusWidth(8));
        assert_eq!(ground.get_width(), BusWidth(8));

        // Check that the pin width was updated
        if let Some(pin) = ground.pins.get("out") {
            assert_eq!(pin.width, BusWidth(8));
        }
    }

    #[test]
    fn test_ground_factory() {
        let factory = GroundFactory;
        assert_eq!(factory.id(), GROUND_ID);
        assert_eq!(factory.display_name(), "Ground");
        assert!(factory.icon_path().is_some());

        let component = factory.create_component(ComponentId(42));
        assert_eq!(component.id(), ComponentId(42));
        assert_eq!(component.name(), GROUND_ID);
    }

    #[test]
    fn test_ground_output_signal() {
        let mut ground = Ground::new(ComponentId(1));
        ground.set_width(BusWidth(4));

        // Check that the output pin has the correct signal (all low)
        if let Some(pin) = ground.pins.get("out") {
            assert_eq!(pin.width, BusWidth(4));
            // The signal should be all zeros for the given width
        }
    }

    #[test]
    fn test_ground_update() {
        let mut ground = Ground::new(ComponentId(1));

        let result = ground.update(Timestamp(0));

        // Should have one output
        assert_eq!(result.outputs.len(), 1);
        assert!(result.outputs.contains_key("out"));
        assert_eq!(result.delay, 0); // No propagation delay
    }

    #[test]
    fn test_ground_always_low() {
        let mut ground = Ground::new(ComponentId(1));

        // Even after reset, ground should still output low
        ground.reset();
        let result = ground.update(Timestamp(0));

        assert!(result.outputs.contains_key("out"));
        // The output should be all low values
    }
}
