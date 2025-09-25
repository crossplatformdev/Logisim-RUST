/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Priority Encoder component implementation
//!
//! A priority encoder outputs the binary index of the highest priority active input.
//! It scans the inputs from highest to lowest priority and outputs the index of the first active input found.

use crate::{
    comp::{Component, ComponentId, Pin, UpdateResult},
    data::{Bounds, Direction},
    signal::{BusWidth, Signal, Timestamp, Value},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Priority Encoder component for priority-based encoding
///
/// A priority encoder takes multiple input lines and outputs the binary index of the
/// highest priority active input. Input 0 has the lowest priority, and the highest
/// numbered input has the highest priority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityEncoder {
    /// Unique component identifier
    id: ComponentId,
    /// Component pins (inputs, output, and optional group signal/enable outputs)
    pins: HashMap<String, Pin>,
    /// Number of input lines
    num_inputs: u8,
    /// Number of output bits (log2 of num_inputs)
    output_bits: u8,
    /// Component facing direction
    facing: Direction,
    /// Whether to use three-state outputs
    tristate: bool,
    /// Whether component has enable input
    enable_input: bool,
    /// Whether component has group signal output (indicates any input is active)
    group_signal: bool,
    /// Whether component has enable output (indicates valid output)
    enable_output: bool,
    /// Component bounds for rendering
    bounds: Bounds,
}

impl PriorityEncoder {
    /// Create a new priority encoder with the given ID
    pub fn new(id: ComponentId) -> Self {
        let mut encoder = PriorityEncoder {
            id,
            pins: HashMap::new(),
            num_inputs: 4, // Default to 4 inputs
            output_bits: 2, // log2(4) = 2 bits output
            facing: Direction::East,
            tristate: false,
            enable_input: false,
            group_signal: true,  // Enable group signal by default
            enable_output: true, // Enable enable output by default
            bounds: Bounds::create(0, 0, 40, 50),
        };
        encoder.update_pins();
        encoder
    }

    /// Create a new priority encoder with specified parameters
    pub fn with_config(
        id: ComponentId,
        num_inputs: u8,
        facing: Direction,
    ) -> Self {
        let num_inputs = num_inputs.clamp(2, 32); // Reasonable range
        let output_bits = (num_inputs as f32).log2().ceil() as u8;
        
        let mut encoder = PriorityEncoder {
            id,
            pins: HashMap::new(),
            num_inputs,
            output_bits,
            facing,
            tristate: false,
            enable_input: false,
            group_signal: true,
            enable_output: true,
            bounds: Bounds::create(0, 0, 40, 50),
        };
        encoder.update_pins();
        encoder
    }

    /// Update pin configuration based on current settings
    fn update_pins(&mut self) {
        self.pins.clear();
        
        // Add input pins
        for i in 0..self.num_inputs {
            let pin_name = format!("input_{}", i);
            let pin = Pin::new_input(format!("Input {}", i), BusWidth(1)); // Each input is single bit
            self.pins.insert(pin_name, pin);
        }
        
        // Add output (binary encoded result)
        let output_pin = Pin::new_output("Output", BusWidth(self.output_bits as u32));
        self.pins.insert("output".to_string(), output_pin);
        
        // Add group signal output if enabled
        if self.group_signal {
            let group_pin = Pin::new_output("Group Signal", BusWidth(1));
            self.pins.insert("group_signal".to_string(), group_pin);
        }
        
        // Add enable output if enabled
        if self.enable_output {
            let enable_out_pin = Pin::new_output("Enable Out", BusWidth(1));
            self.pins.insert("enable_out".to_string(), enable_out_pin);
        }
        
        // Add enable input if enabled
        if self.enable_input {
            let enable_in_pin = Pin::new_input("Enable In", BusWidth(1));
            self.pins.insert("enable_in".to_string(), enable_in_pin);
        }
    }

    /// Set the number of inputs
    pub fn set_num_inputs(&mut self, num_inputs: u8) {
        let num_inputs = num_inputs.clamp(2, 32);
        if num_inputs != self.num_inputs {
            self.num_inputs = num_inputs;
            self.output_bits = (num_inputs as f32).log2().ceil() as u8;
            self.update_pins();
        }
    }

    /// Get the number of inputs
    pub fn num_inputs(&self) -> u8 {
        self.num_inputs
    }

    /// Get the number of output bits
    pub fn output_bits(&self) -> u8 {
        self.output_bits
    }

    /// Set the facing direction
    pub fn set_facing(&mut self, facing: Direction) {
        if facing != self.facing {
            self.facing = facing;
            self.update_pins();
        }
    }

    /// Get the facing direction
    pub fn facing(&self) -> Direction {
        self.facing
    }

    /// Set tristate output mode
    pub fn set_tristate(&mut self, tristate: bool) {
        self.tristate = tristate;
    }

    /// Check if tristate output is enabled
    pub fn tristate(&self) -> bool {
        self.tristate
    }

    /// Set enable input
    pub fn set_enable_input(&mut self, enable: bool) {
        if enable != self.enable_input {
            self.enable_input = enable;
            self.update_pins();
        }
    }

    /// Check if enable input is present
    pub fn enable_input(&self) -> bool {
        self.enable_input
    }

    /// Set group signal output
    pub fn set_group_signal(&mut self, group_signal: bool) {
        if group_signal != self.group_signal {
            self.group_signal = group_signal;
            self.update_pins();
        }
    }

    /// Check if group signal output is present
    pub fn group_signal(&self) -> bool {
        self.group_signal
    }

    /// Set enable output
    pub fn set_enable_output(&mut self, enable_output: bool) {
        if enable_output != self.enable_output {
            self.enable_output = enable_output;
            self.update_pins();
        }
    }

    /// Check if enable output is present
    pub fn enable_output(&self) -> bool {
        self.enable_output
    }
}

impl Component for PriorityEncoder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Priority Encoder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Check if enabled (if enable input exists)
        if self.enable_input {
            if let Some(enable_pin) = self.pins.get("enable_in") {
                match &enable_pin.signal {
                    Some(signal) => {
                        if signal.value() == Value::Low {
                            // Component is disabled, set outputs appropriately
                            let disabled_value = if self.tristate {
                                Value::HighZ
                            } else {
                                Value::Low
                            };
                            
                            // Set main output
                            if let Some(output_pin) = self.pins.get_mut("output") {
                                let output_signal = Signal::new(BusWidth(self.output_bits as u32), disabled_value.clone());
                                output_pin.signal = Some(output_signal);
                            }
                            
                            // Set group signal
                            if let Some(group_pin) = self.pins.get_mut("group_signal") {
                                let group_signal = Signal::new(BusWidth(1), Value::Low);
                                group_pin.signal = Some(group_signal);
                            }
                            
                            // Set enable output
                            if let Some(enable_out_pin) = self.pins.get_mut("enable_out") {
                                let enable_signal = Signal::new(BusWidth(1), Value::Low);
                                enable_out_pin.signal = Some(enable_signal);
                            }
                            
                            return UpdateResult::Changed;
                        }
                    }
                    None => return UpdateResult::NoChange,
                }
            }
        }

        // Read all input signals
        let mut input_values = vec![false; self.num_inputs as usize];
        let mut any_input_active = false;
        let mut any_invalid = false;
        
        for i in 0..self.num_inputs {
            let input_pin_name = format!("input_{}", i);
            if let Some(input_pin) = self.pins.get(&input_pin_name) {
                match &input_pin.signal {
                    Some(signal) => {
                        match signal.value() {
                            Value::High => {
                                input_values[i as usize] = true;
                                any_input_active = true;
                            }
                            Value::Low => {
                                input_values[i as usize] = false;
                            }
                            Value::HighZ => {
                                input_values[i as usize] = false; // Treat as zero
                            }
                            Value::Error => {
                                any_invalid = true;
                            }
                            Value::High => {
                                // High value means input is active
                                input_values[i as usize] = true;
                                any_input_active = true;
                            }
                        }
                    }
                    None => {
                        input_values[i as usize] = false; // No signal = zero
                    }
                }
            }
        }

        if any_invalid {
            // Set outputs to error state
            if let Some(output_pin) = self.pins.get_mut("output") {
                let output_signal = Signal::new(BusWidth(self.output_bits as u32), Value::Error);
                output_pin.signal = Some(output_signal);
            }
            
            if let Some(group_pin) = self.pins.get_mut("group_signal") {
                let group_signal = Signal::new(BusWidth(1), Value::Error);
                group_pin.signal = Some(group_signal);
            }
            
            if let Some(enable_out_pin) = self.pins.get_mut("enable_out") {
                let enable_signal = Signal::new(BusWidth(1), Value::Error);
                enable_out_pin.signal = Some(enable_signal);
            }
            
            return UpdateResult::Changed;
        }

        // Find highest priority active input (scan from highest index to lowest)
        let mut encoded_output: Option<usize> = None;
        for i in (0..self.num_inputs as usize).rev() {
            if input_values[i] {
                encoded_output = Some(i);
                break;
            }
        }

        // Set main output
        if let Some(output_pin) = self.pins.get_mut("output") {
            let output_value = match encoded_output {
                Some(index) => {
                    // Convert index to binary representation
                    if self.output_bits == 1 {
                        if index == 0 { Value::Low } else { Value::High }
                    } else {
                        // Simplified: for multi-bit outputs, just return High for now
                        // TODO: Implement proper binary encoding
                        Value::High
                    }
                }
                None => {
                    // No active inputs
                    if self.tristate {
                        Value::HighZ
                    } else {
                        Value::Low
                    }
                }
            };
            
            let output_signal = Signal::new(BusWidth(self.output_bits as u32), output_value);
            output_pin.signal = Some(output_signal);
        }

        // Set group signal output (indicates if any input is active)
        if let Some(group_pin) = self.pins.get_mut("group_signal") {
            let group_value = if any_input_active { Value::High } else { Value::Low };
            let group_signal = Signal::new(BusWidth(1), group_value);
            group_pin.signal = Some(group_signal);
        }

        // Set enable output (indicates valid output - same as group signal for priority encoder)
        if let Some(enable_out_pin) = self.pins.get_mut("enable_out") {
            let enable_value = if any_input_active { Value::High } else { Value::Low };
            let enable_signal = Signal::new(BusWidth(1), enable_value);
            enable_out_pin.signal = Some(enable_signal);
        }

        UpdateResult::Changed
    }

    fn reset(&mut self) {
        // Clear all pin signals
        for pin in self.pins.values_mut() {
            pin.signal = None;
        }
    }

    fn propagation_delay(&self) -> u64 {
        super::plexers_library::PlexersLibrary::DELAY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_encoder_creation() {
        let encoder = PriorityEncoder::new(ComponentId(1));
        assert_eq!(encoder.id(), ComponentId(1));
        assert_eq!(encoder.name(), "Priority Encoder");
        assert_eq!(encoder.num_inputs(), 4);
        assert_eq!(encoder.output_bits(), 2);
    }

    #[test]
    fn test_priority_encoder_configuration() {
        let mut encoder = PriorityEncoder::new(ComponentId(1));
        
        // Test setting number of inputs
        encoder.set_num_inputs(8);
        assert_eq!(encoder.num_inputs(), 8);
        assert_eq!(encoder.output_bits(), 3); // log2(8) = 3
        
        // Test setting facing direction
        encoder.set_facing(Direction::North);
        assert_eq!(encoder.facing(), Direction::North);
        
        // Test enabling tristate
        encoder.set_tristate(true);
        assert!(encoder.tristate());
        
        // Test enabling enable input
        encoder.set_enable_input(true);
        assert!(encoder.enable_input());
        
        // Test disabling group signal
        encoder.set_group_signal(false);
        assert!(!encoder.group_signal());
        
        // Test disabling enable output
        encoder.set_enable_output(false);
        assert!(!encoder.enable_output());
    }

    #[test]
    fn test_priority_encoder_pins() {
        let encoder = PriorityEncoder::new(ComponentId(1));
        let pins = encoder.pins();
        
        // Should have 4 inputs + 1 output + 1 group + 1 enable_out = 7 pins
        assert_eq!(pins.len(), 7);
        assert!(pins.contains_key("input_0"));
        assert!(pins.contains_key("input_1"));
        assert!(pins.contains_key("input_2"));
        assert!(pins.contains_key("input_3"));
        assert!(pins.contains_key("output"));
        assert!(pins.contains_key("group_signal"));
        assert!(pins.contains_key("enable_out"));
        
        // Test with enable input
        let mut encoder = PriorityEncoder::new(ComponentId(2));
        encoder.set_enable_input(true);
        let pins = encoder.pins();
        
        // Should have one additional pin for enable input
        assert_eq!(pins.len(), 8);
        assert!(pins.contains_key("enable_in"));
    }

    #[test]
    fn test_priority_encoder_without_optional_outputs() {
        let mut encoder = PriorityEncoder::new(ComponentId(1));
        encoder.set_group_signal(false);
        encoder.set_enable_output(false);
        let pins = encoder.pins();
        
        // Should have 4 inputs + 1 output = 5 pins
        assert_eq!(pins.len(), 5);
        assert!(!pins.contains_key("group_signal"));
        assert!(!pins.contains_key("enable_out"));
    }

    #[test]
    fn test_priority_encoder_reset() {
        let mut encoder = PriorityEncoder::new(ComponentId(1));
        
        // Set some pin signals
        if let Some(pin) = encoder.pins.get_mut("input_0") {
            pin.signal = Some(Signal::new(BusWidth(1), Value::High));
        }
        
        // Reset should clear all signals
        encoder.reset();
        
        for pin in encoder.pins().values() {
            assert!(pin.signal.is_none());
        }
    }

    #[test]
    fn test_priority_encoder_propagation_delay() {
        let encoder = PriorityEncoder::new(ComponentId(1));
        assert_eq!(encoder.propagation_delay(), 3);
    }

    #[test]
    fn test_priority_encoder_output_bits_calculation() {
        let encoder = PriorityEncoder::with_config(ComponentId(1), 2, Direction::East);
        assert_eq!(encoder.output_bits(), 1); // log2(2) = 1
        
        let encoder = PriorityEncoder::with_config(ComponentId(2), 16, Direction::East);
        assert_eq!(encoder.output_bits(), 4); // log2(16) = 4
        
        let encoder = PriorityEncoder::with_config(ComponentId(3), 3, Direction::East);
        assert_eq!(encoder.output_bits(), 2); // ceil(log2(3)) = 2
    }
}