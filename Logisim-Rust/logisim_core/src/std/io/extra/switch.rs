/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Switch Component
//!
//! Rust port of `com.cburch.logisim.std.io.extra.Switch`
//!
//! A toggle switch component that allows manual input control.
//! Users can click the switch to toggle between on/off states.

use crate::{
    comp::{Component, ComponentId, UpdateResult, Pin},
    data::{AttributeSet, Bounds, Location},
    signal::{BusWidth, Signal, Timestamp, Value},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for the Switch component
pub const SWITCH_ID: &str = "Switch";

/// Switch component state data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchData {
    /// Current state of the switch (true = on/active, false = off/inactive)
    pub active: bool,
}

impl SwitchData {
    /// Create new switch data with default inactive state
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Toggle the switch state
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// Set the switch state
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Get the current switch state
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for SwitchData {
    fn default() -> Self {
        Self::new()
    }
}

/// Switch component implementation
///
/// A toggle switch that can be manually controlled to provide input signals.
/// When active, it passes the input signal through to the output.
/// When inactive, it outputs an unknown/floating value.
#[derive(Debug, Clone)]
pub struct Switch {
    /// Component identifier
    id: ComponentId,
    /// Current switch state
    data: SwitchData,
    /// Component pins
    pins: HashMap<String, Pin>,
    /// Location of the component
    location: Option<Location>,
}

impl Switch {
    /// Create a new switch component
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();

        // Input pin for the signal to switch
        pins.insert(
            "input".to_string(),
            Pin::new_input("input", BusWidth::new(1)),
        );
        // Output pin for the switched signal
        pins.insert(
            "output".to_string(),
            Pin::new_output("output", BusWidth::new(1)),
        );

        Self {
            id,
            data: SwitchData::new(),
            pins,
            location: None,
        }
    }

    /// Get the current switch data
    pub fn get_data(&self) -> &SwitchData {
        &self.data
    }

    /// Get mutable reference to switch data
    pub fn get_data_mut(&mut self) -> &mut SwitchData {
        &mut self.data
    }

    /// Toggle the switch state
    pub fn toggle(&mut self) {
        self.data.toggle();
    }

    /// Handle mouse click to toggle switch
    pub fn handle_mouse_click(&mut self, _location: Location) {
        self.toggle();
    }

    /// Get the component's display name
    pub fn display_name() -> &'static str {
        "Switch"
    }

    /// Get the component's factory ID
    pub fn factory_id() -> &'static str {
        SWITCH_ID
    }

    /// Check if component is interactive
    pub fn is_interactive(&self) -> bool {
        true
    }
}

impl Component for Switch {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Switch"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        if let Some(input_pin) = self.get_pin("input") {
            let output_value = if self.data.is_active() {
                // Pass through input when switch is active
                input_pin.signal.as_single().unwrap_or(Value::Unknown)
            } else {
                // Output unknown/floating when inactive
                Value::Unknown
            };

            let output_signal = Signal::new_single(output_value);
            result.add_output("output".to_string(), output_signal);
            result.set_delay(1); // Minimal propagation delay
        }

        result
    }

    fn reset(&mut self) {
        self.data = SwitchData::new();
    }

    fn location(&self) -> Option<Location> {
        self.location
    }

    fn bounds(&self) -> Option<Bounds> {
        Some(Bounds::create(-20, -15, 20, 30))
    }

    fn attribute_set(&self) -> Option<&AttributeSet> {
        None // Simple implementation without attributes for now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_creation() {
        let switch = Switch::new(ComponentId::new(1));
        assert_eq!(switch.id(), ComponentId::new(1));
        assert_eq!(switch.name(), "Switch");
        assert!(!switch.get_data().is_active());
        assert!(switch.is_interactive());
    }

    #[test]
    fn test_switch_toggle() {
        let mut switch = Switch::new(ComponentId::new(1));

        // Initially inactive
        assert!(!switch.get_data().is_active());

        // Toggle to active
        switch.toggle();
        assert!(switch.get_data().is_active());

        // Toggle back to inactive
        switch.toggle();
        assert!(!switch.get_data().is_active());
    }

    #[test]
    fn test_switch_data_serialization() {
        let data = SwitchData::new();
        let json = serde_json::to_string(&data).unwrap();
        let deserialized: SwitchData = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_switch_bounds() {
        let switch = Switch::new(ComponentId::new(1));
        let bounds = switch.bounds();

        assert!(bounds.is_some());
        assert_eq!(bounds.unwrap(), Bounds::create(-20, -15, 20, 30));
    }

    #[test]
    fn test_switch_pins() {
        let switch = Switch::new(ComponentId::new(1));
        assert_eq!(switch.pins().len(), 2);
        assert!(switch.get_pin("input").is_some());
        assert!(switch.get_pin("output").is_some());
    }
}
