/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Clock component - Clock signal generation
//!
//! Clock components generate periodic square wave signals that can be used
//! to drive sequential logic circuits.

use crate::{
    component::{ClockEdge, Component, ComponentId, Pin as ComponentPin, UpdateResult},
    data::Direction,
    signal::{BusWidth, Signal, Timestamp, Value},
    std::wiring::WiringComponentFactory,
};
use std::collections::HashMap;

/// Unique identifier for the Clock component
/// Do NOT change as it will prevent project files from loading.
pub const CLOCK_ID: &str = "Clock";

/// Clock component attributes
#[derive(Debug, Clone)]
pub struct ClockAttributes {
    pub facing: Direction,
    pub high_duration: u64, // Duration in time units that clock stays high
    pub low_duration: u64,  // Duration in time units that clock stays low
}

impl Default for ClockAttributes {
    fn default() -> Self {
        Self {
            facing: Direction::East,
            high_duration: 1,
            low_duration: 1,
        }
    }
}

/// Internal state of a clock component
#[derive(Debug, Clone)]
pub struct ClockState {
    /// Current output value
    pub current_value: Value,
    /// Time when the next transition should occur
    pub next_transition_time: u64,
    /// Whether the clock is running
    pub running: bool,
}

impl Default for ClockState {
    fn default() -> Self {
        Self {
            current_value: Value::Low,
            next_transition_time: 1, // Start with low duration for first transition
            running: true,
        }
    }
}

/// Clock component implementation
#[derive(Debug)]
pub struct Clock {
    id: ComponentId,
    attributes: ClockAttributes,
    state: ClockState,
    pins: HashMap<String, ComponentPin>,
}

impl Clock {
    /// Create a new clock component
    pub fn new(id: ComponentId) -> Self {
        let attributes = ClockAttributes::default();
        let mut state = ClockState::default();

        // Set initial next transition time based on current state (low) and low duration
        state.next_transition_time = attributes.low_duration;

        // Create the output pin - always 1-bit wide
        let mut output_pin = ComponentPin::new_output("out", BusWidth(1));
        output_pin.signal = Signal::new_single(state.current_value);

        let mut pins = HashMap::new();
        pins.insert("out".to_string(), output_pin);

        Self {
            id,
            attributes,
            state,
            pins,
        }
    }

    /// Set the high duration (time clock stays high)
    pub fn set_high_duration(&mut self, duration: u64) {
        self.attributes.high_duration = duration;
    }

    /// Set the low duration (time clock stays low)
    pub fn set_low_duration(&mut self, duration: u64) {
        self.attributes.low_duration = duration;
    }

    /// Get the high duration
    pub fn get_high_duration(&self) -> u64 {
        self.attributes.high_duration
    }

    /// Get the low duration
    pub fn get_low_duration(&self) -> u64 {
        self.attributes.low_duration
    }

    /// Toggle the clock output and schedule next transition
    fn toggle_output(&mut self, current_time: u64) -> bool {
        let old_value = self.state.current_value;

        // Toggle the value
        self.state.current_value = match self.state.current_value {
            Value::High => Value::Low,
            Value::Low => Value::High,
            _ => Value::Low, // Default to low for unknown/error states
        };

        // Calculate next transition time
        let duration = match self.state.current_value {
            Value::High => self.attributes.high_duration,
            Value::Low => self.attributes.low_duration,
            _ => self.attributes.low_duration,
        };

        self.state.next_transition_time = current_time + duration;

        // Update the output pin
        if let Some(pin) = self.pins.get_mut("out") {
            pin.signal = Signal::new_single(self.state.current_value);
        }

        // Return true if the value actually changed
        old_value != self.state.current_value
    }

    /// Get the current clock value
    pub fn get_current_value(&self) -> Value {
        self.state.current_value.clone()
    }

    /// Check if it's time for the next transition
    pub fn should_transition(&self, current_time: u64) -> bool {
        self.state.running && current_time >= self.state.next_transition_time
    }

    /// Start/stop the clock
    pub fn set_running(&mut self, running: bool) {
        self.state.running = running;
    }

    /// Check if the clock is running
    pub fn is_running(&self) -> bool {
        self.state.running
    }
}

impl Component for Clock {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        CLOCK_ID
    }

    fn pins(&self) -> &HashMap<String, ComponentPin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, ComponentPin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // Check if it's time to toggle
        if self.should_transition(current_time.0) {
            if self.toggle_output(current_time.0) {
                // Value changed, output the new signal
                if let Some(pin) = self.pins.get("out") {
                    result.add_output("out".to_string(), pin.signal.clone());
                }

                // Set the delay until next transition
                result.set_delay(match self.state.current_value {
                    Value::High => self.attributes.high_duration,
                    Value::Low => self.attributes.low_duration,
                    _ => self.attributes.low_duration,
                });
            }
        }

        result
    }

    fn reset(&mut self) {
        // Reset to initial state
        self.state.current_value = Value::Low;
        self.state.next_transition_time = self.attributes.low_duration;
        self.state.running = true;

        // Update pin signal
        if let Some(pin) = self.pins.get_mut("out") {
            pin.signal = Signal::new_single(self.state.current_value);
        }
    }

    fn is_sequential(&self) -> bool {
        true // Clock is a sequential component
    }

    fn propagation_delay(&self) -> u64 {
        0 // Clock drives its output directly
    }
}

/// Factory for creating Clock components
pub struct ClockFactory;

impl WiringComponentFactory for ClockFactory {
    fn id(&self) -> &'static str {
        CLOCK_ID
    }

    fn display_name(&self) -> &str {
        "Clock"
    }

    fn description(&self) -> &str {
        "Clock signal generation"
    }

    fn icon_path(&self) -> Option<&str> {
        Some("clock.gif")
    }

    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(Clock::new(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_creation() {
        let clock = Clock::new(ComponentId(1));
        assert_eq!(clock.id(), ComponentId(1));
        assert_eq!(clock.name(), CLOCK_ID);
        assert_eq!(clock.get_current_value(), Value::Low);
        assert!(clock.is_running());
        assert!(clock.is_sequential());
    }

    #[test]
    fn test_clock_duration_setting() {
        let mut clock = Clock::new(ComponentId(1));

        clock.set_high_duration(10);
        clock.set_low_duration(5);

        assert_eq!(clock.get_high_duration(), 10);
        assert_eq!(clock.get_low_duration(), 5);
    }

    #[test]
    fn test_clock_factory() {
        let factory = ClockFactory;
        assert_eq!(factory.id(), CLOCK_ID);
        assert_eq!(factory.display_name(), "Clock");
        assert!(factory.icon_path().is_some());

        let component = factory.create_component(ComponentId(42));
        assert_eq!(component.id(), ComponentId(42));
        assert_eq!(component.name(), CLOCK_ID);
    }

    #[test]
    fn test_clock_transitions() {
        let mut clock = Clock::new(ComponentId(1));
        clock.set_high_duration(2);
        clock.set_low_duration(3);

        // Initial state should be low
        assert_eq!(clock.get_current_value(), Value::Low);

        // Should transition at time 1 (low_duration)
        assert!(clock.should_transition(1));
        assert!(!clock.should_transition(0));

        // Toggle to high
        let changed = clock.toggle_output(1);
        assert!(changed);
        assert_eq!(clock.get_current_value(), Value::High);

        // Next transition should be at time 3 (1 + high_duration of 2)
        assert!(clock.should_transition(3));
        assert!(!clock.should_transition(2));
    }

    #[test]
    fn test_clock_control() {
        let mut clock = Clock::new(ComponentId(1));

        // Initially running
        assert!(clock.is_running());

        // Stop the clock
        clock.set_running(false);
        assert!(!clock.is_running());
        assert!(!clock.should_transition(1000)); // Should not transition when stopped

        // Start the clock again
        clock.set_running(true);
        assert!(clock.is_running());
    }

    #[test]
    fn test_clock_reset() {
        let mut clock = Clock::new(ComponentId(1));

        // Change state
        clock.toggle_output(0);
        clock.set_running(false);

        // Reset
        clock.reset();

        // Should be back to initial state
        assert_eq!(clock.get_current_value(), Value::Low);
        assert!(clock.is_running());
    }

    #[test]
    fn test_clock_update() {
        let mut clock = Clock::new(ComponentId(1));
        clock.set_low_duration(1); // Set to match default

        // Before transition time
        let result = clock.update(Timestamp(0));
        assert_eq!(result.outputs.len(), 0); // No change

        // At transition time
        let result = clock.update(Timestamp(1));
        assert_eq!(result.outputs.len(), 1); // Should have output
        assert!(result.outputs.contains_key("out"));
    }
}
