/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Pin component - Circuit I/O pins with bidirectional support
//!
//! Pins are the fundamental I/O components in Logisim circuits, allowing
//! communication between subcircuits and external interfaces.

use crate::{
    component::{Component, ComponentId, Pin as ComponentPin, PinDirection, UpdateResult},
    data::{BitWidth, Direction},
    signal::{BusWidth, Signal, Timestamp, Value},
    std::wiring::WiringComponentFactory,
};
use std::collections::HashMap;

/// Unique identifier for the Pin component
/// Do NOT change as it will prevent project files from loading.
pub const PIN_ID: &str = "Pin";

/// Pin type options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinType {
    Input,
    Output,
}

/// Pin behavior options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinBehavior {
    Simple,
    Tristate,
    PullDown,
    PullUp,
}

/// Pin component attributes
#[derive(Debug, Clone)]
pub struct PinAttributes {
    pub pin_type: PinType,
    pub behavior: PinBehavior,
    pub width: BusWidth,
    pub facing: Direction,
    pub initial_value: u64,
    pub label: String,
    pub label_location: Direction,
}

impl Default for PinAttributes {
    fn default() -> Self {
        Self {
            pin_type: PinType::Input,
            behavior: PinBehavior::Simple,
            width: BusWidth(1),
            facing: Direction::East,
            initial_value: 0,
            label: String::new(),
            label_location: Direction::West,
        }
    }
}

impl PinAttributes {
    /// Check if this pin is an output pin
    pub fn is_output(&self) -> bool {
        self.pin_type == PinType::Output
    }

    /// Check if this pin is an input pin
    pub fn is_input(&self) -> bool {
        self.pin_type == PinType::Input
    }
}

/// Internal state of a pin component
#[derive(Debug, Clone)]
pub struct PinState {
    /// The value the pin is trying to drive
    pub intended_value: Signal,
    /// The actual value on the pin (may differ due to conflicts)
    pub actual_value: Signal,
    /// Whether the pin is actively driving
    pub driving: bool,
}

impl Default for PinState {
    fn default() -> Self {
        Self {
            intended_value: Signal::new_single(Value::Unknown),
            actual_value: Signal::new_single(Value::Unknown),
            driving: false,
        }
    }
}

/// Pin component implementation
#[derive(Debug)]
pub struct Pin {
    id: ComponentId,
    attributes: PinAttributes,
    state: PinState,
    pins: HashMap<String, ComponentPin>,
}

impl Pin {
    /// Create a new pin component
    pub fn new(id: ComponentId) -> Self {
        let attributes = PinAttributes::default();
        let state = PinState::default();

        // Create the component pin based on pin type
        let pin_direction = if attributes.is_output() {
            PinDirection::Output
        } else {
            PinDirection::Input
        };

        let component_pin = match pin_direction {
            PinDirection::Input => ComponentPin::new_input("pin", attributes.width),
            PinDirection::Output => ComponentPin::new_output("pin", attributes.width),
            PinDirection::InOut => ComponentPin::new_inout("pin", attributes.width),
        };

        let mut pins = HashMap::new();
        pins.insert("pin".to_string(), component_pin);

        Self {
            id,
            attributes,
            state,
            pins,
        }
    }

    /// Set the intended value for this pin
    pub fn set_intended_value(&mut self, value: Signal) {
        self.state.intended_value = value.clone();
        if self.attributes.is_input() {
            self.state.driving = true;
            // Update the pin signal
            if let Some(pin) = self.pins.get_mut("pin") {
                pin.signal = value;
            }
        }
    }

    /// Get the current intended value
    pub fn get_intended_value(&self) -> &Signal {
        &self.state.intended_value
    }

    /// Get the actual value on the pin
    pub fn get_actual_value(&self) -> &Signal {
        &self.state.actual_value
    }
}

impl Component for Pin {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        PIN_ID
    }

    fn pins(&self) -> &HashMap<String, ComponentPin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, ComponentPin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Pin behavior is largely passive - it responds to external changes
        // TODO: Implement tristate logic, pull-up/pull-down behavior
        let mut result = UpdateResult::new();

        // For input pins, the intended value drives the output
        if self.attributes.is_input() && self.state.driving {
            if let Some(pin) = self.pins.get_mut("pin") {
                pin.signal = self.state.intended_value.clone();
                result.add_output("pin".to_string(), pin.signal.clone());
            }
        }

        result
    }

    fn reset(&mut self) {
        // Reset to initial state
        let initial_signal = Signal::from_u64(
            self.attributes.initial_value,
            self.state.intended_value.width(),
        );
        self.state.intended_value = initial_signal.clone();
        self.state.actual_value = initial_signal;
        self.state.driving = self.attributes.is_input();

        // Reset pin signal
        if let Some(pin) = self.pins.get_mut("pin") {
            pin.signal = self.state.intended_value.clone();
        }
    }
}

/// Factory for creating Pin components
pub struct PinFactory;

impl WiringComponentFactory for PinFactory {
    fn id(&self) -> &'static str {
        PIN_ID
    }

    fn display_name(&self) -> &str {
        "Pin"
    }

    fn description(&self) -> &str {
        "Circuit I/O pin with bidirectional support"
    }

    fn icon_path(&self) -> Option<&str> {
        Some("pin.gif")
    }

    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(Pin::new(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_creation() {
        let pin = Pin::new(ComponentId(1));
        assert_eq!(pin.id(), ComponentId(1));
        assert_eq!(pin.name(), PIN_ID);
        assert!(pin.attributes.is_input());
        assert!(!pin.attributes.is_output());
    }

    #[test]
    fn test_pin_attributes() {
        let mut attrs = PinAttributes::default();
        assert!(attrs.is_input());
        assert!(!attrs.is_output());

        attrs.pin_type = PinType::Output;
        assert!(!attrs.is_input());
        assert!(attrs.is_output());
    }

    #[test]
    fn test_pin_factory() {
        let factory = PinFactory;
        assert_eq!(factory.id(), PIN_ID);
        assert_eq!(factory.display_name(), "Pin");
        assert!(factory.icon_path().is_some());

        let component = factory.create_component(ComponentId(42));
        assert_eq!(component.id(), ComponentId(42));
        assert_eq!(component.name(), PIN_ID);
    }

    #[test]
    fn test_pin_state() {
        let mut pin = Pin::new(ComponentId(1));

        // Test setting intended value
        let test_signal = Signal::new_single(Value::High);
        pin.set_intended_value(test_signal.clone());
        assert_eq!(pin.get_intended_value(), &test_signal);
    }
}
