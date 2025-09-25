/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Digital Oscilloscope Component
//!
//! Rust port of `com.cburch.logisim.std.io.extra.DigitalOscilloscope`
//!
//! Multi-channel signal visualization component for monitoring digital signals.

use crate::{
    component::{Component, ComponentId},
    data::{Attribute, BitWidth, Bounds, Direction, Value},
    signal::Signal,
    util::StringGetter,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trigger type for oscilloscope
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TriggerType {
    None,
    Rising,
    Falling,
    Both,
}

/// Digital oscilloscope component state data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OscilloscopeData {
    /// Number of input channels
    pub input_count: usize,
    /// Number of states to display
    pub state_count: usize,
    /// Trigger configuration
    pub trigger_type: TriggerType,
    /// Whether to show clock lines
    pub show_clock: bool,
    /// Display color
    pub color: (u8, u8, u8), // RGB
    /// Signal history for each channel
    pub signal_history: Vec<Vec<bool>>,
    /// Current position in history buffer
    pub history_position: usize,
}

impl OscilloscopeData {
    /// Create new oscilloscope data
    pub fn new(input_count: usize, state_count: usize) -> Self {
        let mut signal_history = Vec::new();
        for _ in 0..input_count {
            signal_history.push(vec![false; state_count]);
        }

        Self {
            input_count,
            state_count,
            trigger_type: TriggerType::Rising,
            show_clock: true,
            color: (0, 208, 208), // Cyan
            signal_history,
            history_position: 0,
        }
    }

    /// Add new signal states to history
    pub fn add_states(&mut self, states: &[bool]) {
        if states.len() != self.input_count {
            return;
        }

        for (channel, &state) in states.iter().enumerate() {
            self.signal_history[channel][self.history_position] = state;
        }

        self.history_position = (self.history_position + 1) % self.state_count;
    }

    /// Get signal history for a channel
    pub fn get_channel_history(&self, channel: usize) -> Option<&Vec<bool>> {
        self.signal_history.get(channel)
    }

    /// Clear all signal history
    pub fn clear_history(&mut self) {
        for channel in &mut self.signal_history {
            channel.fill(false);
        }
        self.history_position = 0;
    }
}

/// Digital Oscilloscope component implementation
///
/// Displays digital signal waveforms over time for multiple input channels.
/// Provides trigger functionality and configurable display options.
#[derive(Debug, Clone)]
pub struct DigitalOscilloscope {
    /// Component identifier
    id: ComponentId,
    /// Current oscilloscope state
    data: OscilloscopeData,
    /// Component attributes
    attributes: HashMap<String, Attribute>,
}

impl DigitalOscilloscope {
    /// Create a new digital oscilloscope component
    pub fn new(id: ComponentId) -> Self {
        let mut attributes = HashMap::new();
        
        // Initialize default attributes
        attributes.insert(
            "inputs".to_string(),
            Attribute::Integer(3),
        );
        attributes.insert(
            "nState".to_string(),
            Attribute::Integer(10),
        );
        attributes.insert(
            "frontlines".to_string(),
            Attribute::String("rising".to_string()),
        );
        attributes.insert(
            "showclock".to_string(),
            Attribute::Boolean(true),
        );
        attributes.insert(
            "color".to_string(),
            Attribute::Color((0, 208, 208)),
        );
        attributes.insert(
            "label".to_string(),
            Attribute::String("".to_string()),
        );

        Self {
            id,
            data: OscilloscopeData::new(3, 10),
            attributes,
        }
    }

    /// Get the current oscilloscope data
    pub fn get_data(&self) -> &OscilloscopeData {
        &self.data
    }

    /// Get mutable reference to oscilloscope data
    pub fn get_data_mut(&mut self) -> &mut OscilloscopeData {
        &mut self.data
    }

    /// Parse trigger type from string
    fn parse_trigger_type(s: &str) -> TriggerType {
        match s {
            "no" => TriggerType::None,
            "rising" => TriggerType::Rising,
            "falling" => TriggerType::Falling,
            "both" => TriggerType::Both,
            _ => TriggerType::Rising,
        }
    }

    /// Update component configuration based on attributes
    fn update_configuration(&mut self) {
        let input_count = self.get_attribute("inputs")
            .and_then(|attr| attr.as_integer())
            .map(|&i| i.max(1).min(32) as usize)
            .unwrap_or(3);

        let state_count = self.get_attribute("nState")
            .and_then(|attr| attr.as_integer())
            .map(|&i| i.max(4).min(35) as usize)
            .unwrap_or(10);

        // Recreate data if configuration changed
        if input_count != self.data.input_count || state_count != self.data.state_count {
            self.data = OscilloscopeData::new(input_count, state_count);
        }

        // Update trigger type
        if let Some(trigger_str) = self.get_attribute("frontlines")
            .and_then(|attr| attr.as_string()) {
            self.data.trigger_type = Self::parse_trigger_type(trigger_str);
        }

        // Update show clock
        if let Some(&show_clock) = self.get_attribute("showclock")
            .and_then(|attr| attr.as_boolean()) {
            self.data.show_clock = show_clock;
        }

        // Update color
        if let Some(&color) = self.get_attribute("color")
            .and_then(|attr| attr.as_color()) {
            self.data.color = color;
        }
    }

    /// Get the component's display name
    pub fn display_name() -> StringGetter {
        StringGetter::new("DigitalOscilloscopeComponent")
    }

    /// Get the component's factory ID
    pub fn factory_id() -> &'static str {
        "Digital Oscilloscope"
    }
}

impl Component for DigitalOscilloscope {
    fn get_id(&self) -> ComponentId {
        self.id
    }

    fn get_type_name(&self) -> &'static str {
        "Digital Oscilloscope"
    }

    fn get_bounds(&self) -> Bounds {
        // Oscilloscope bounds: depends on number of inputs and states
        let width = (self.data.state_count * 10 + 40).max(100);
        let height = (self.data.input_count * 20 + 60).max(80);
        
        Bounds::new(-width as i32 / 2, -height as i32 / 2, width as i32, height as i32)
    }

    fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    fn set_attribute(&mut self, name: String, value: Attribute) {
        self.attributes.insert(name, value);
        self.update_configuration();
    }

    fn get_input_count(&self) -> usize {
        self.data.input_count
    }

    fn get_output_count(&self) -> usize {
        0
    }

    fn propagate(&mut self, inputs: &[Signal]) -> Vec<Signal> {
        // Convert input signals to boolean states
        let mut states = Vec::new();
        for input in inputs.iter().take(self.data.input_count) {
            states.push(input.value.is_high());
        }

        // Add states to history
        if !states.is_empty() {
            self.data.add_states(&states);
        }

        vec![] // No outputs
    }

    fn is_interactive(&self) -> bool {
        false // Display-only component
    }

    fn clone_component(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscilloscope_creation() {
        let scope = DigitalOscilloscope::new(ComponentId::new(1));
        assert_eq!(scope.get_id(), ComponentId::new(1));
        assert_eq!(scope.get_type_name(), "Digital Oscilloscope");
        assert!(!scope.is_interactive());
        assert_eq!(scope.get_input_count(), 3);
        assert_eq!(scope.get_output_count(), 0);
    }

    #[test]
    fn test_oscilloscope_data() {
        let mut data = OscilloscopeData::new(2, 5);
        assert_eq!(data.input_count, 2);
        assert_eq!(data.state_count, 5);
        assert_eq!(data.signal_history.len(), 2);
        assert_eq!(data.signal_history[0].len(), 5);

        // Test adding states
        data.add_states(&[true, false]);
        assert_eq!(data.history_position, 1);
        assert!(data.signal_history[0][0]);
        assert!(!data.signal_history[1][0]);
    }

    #[test]
    fn test_trigger_type_parsing() {
        assert_eq!(DigitalOscilloscope::parse_trigger_type("no"), TriggerType::None);
        assert_eq!(DigitalOscilloscope::parse_trigger_type("rising"), TriggerType::Rising);
        assert_eq!(DigitalOscilloscope::parse_trigger_type("falling"), TriggerType::Falling);
        assert_eq!(DigitalOscilloscope::parse_trigger_type("both"), TriggerType::Both);
        assert_eq!(DigitalOscilloscope::parse_trigger_type("invalid"), TriggerType::Rising);
    }

    #[test]
    fn test_oscilloscope_configuration() {
        let mut scope = DigitalOscilloscope::new(ComponentId::new(1));
        
        // Change input count
        scope.set_attribute("inputs".to_string(), Attribute::Integer(5));
        assert_eq!(scope.get_data().input_count, 5);
        assert_eq!(scope.get_input_count(), 5);
        
        // Change state count
        scope.set_attribute("nState".to_string(), Attribute::Integer(15));
        assert_eq!(scope.get_data().state_count, 15);
    }

    #[test]
    fn test_oscilloscope_propagation() {
        let mut scope = DigitalOscilloscope::new(ComponentId::new(1));
        
        let inputs = vec![
            Signal::new(Value::high(BitWidth::new(1))),
            Signal::new(Value::low(BitWidth::new(1))),
            Signal::new(Value::high(BitWidth::new(1))),
        ];
        
        let outputs = scope.propagate(&inputs);
        assert_eq!(outputs.len(), 0);
        
        // Check that history was updated
        assert_eq!(scope.get_data().history_position, 1);
        assert!(scope.get_data().signal_history[0][0]);
        assert!(!scope.get_data().signal_history[1][0]);
        assert!(scope.get_data().signal_history[2][0]);
    }

    #[test]
    fn test_oscilloscope_bounds() {
        let scope = DigitalOscilloscope::new(ComponentId::new(1));
        let bounds = scope.get_bounds();
        
        // Should have reasonable size based on inputs and states
        assert!(bounds.width >= 100);
        assert!(bounds.height >= 80);
    }

    #[test]
    fn test_history_wraparound() {
        let mut data = OscilloscopeData::new(1, 3);
        
        // Fill history buffer
        data.add_states(&[true]);  // position 0
        data.add_states(&[false]); // position 1  
        data.add_states(&[true]);  // position 2
        data.add_states(&[false]); // position 0 (wraparound)
        
        assert_eq!(data.history_position, 1);
        assert!(!data.signal_history[0][0]); // Should be overwritten
    }
}