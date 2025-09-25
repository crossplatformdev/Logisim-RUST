/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Bit Selector component implementation
//!
//! A bit selector extracts a range of bits from an input bus and outputs them.
//! It can select a single bit or a range of bits from a wider bus.

use crate::{
    comp::{Component, ComponentId, Pin, UpdateResult},
    data::{Bounds, Direction},
    signal::{BusWidth, Signal, Timestamp, Value},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bit Selector component for bit extraction
///
/// A bit selector takes a multi-bit input and outputs a selected range of bits.
/// The selection can be a single bit or multiple contiguous bits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitSelector {
    /// Unique component identifier
    id: ComponentId,
    /// Component pins (input and output)
    pins: HashMap<String, Pin>,
    /// Width of the input bus
    input_width: BusWidth,
    /// Width of the output (number of bits to select)
    output_width: BusWidth,
    /// Starting bit index for selection (0-based, from LSB)
    start_bit: u8,
    /// Component facing direction
    facing: Direction,
    /// Component bounds for rendering
    bounds: Bounds,
}

impl BitSelector {
    /// Create a new bit selector with the given ID
    pub fn new(id: ComponentId) -> Self {
        let mut selector = BitSelector {
            id,
            pins: HashMap::new(),
            input_width: BusWidth(8), // Default 8-bit input
            output_width: BusWidth(1), // Default 1-bit output
            start_bit: 0, // Start from LSB
            facing: Direction::East,
            bounds: Bounds::create(0, 0, 40, 30),
        };
        selector.update_pins();
        selector
    }

    /// Create a new bit selector with specified parameters
    pub fn with_config(
        id: ComponentId,
        input_width: BusWidth,
        output_width: BusWidth,
        start_bit: u8,
        facing: Direction,
    ) -> Self {
        let mut selector = BitSelector {
            id,
            pins: HashMap::new(),
            input_width,
            output_width,
            start_bit,
            facing,
            bounds: Bounds::create(0, 0, 40, 30),
        };
        selector.update_pins();
        selector
    }

    /// Update pin configuration based on current settings
    fn update_pins(&mut self) {
        self.pins.clear();
        
        // Add input pin
        let input_pin = Pin::new_input("Input", self.input_width);
        self.pins.insert("input".to_string(), input_pin);
        
        // Add output pin
        let output_pin = Pin::new_output("Output", self.output_width);
        self.pins.insert("output".to_string(), output_pin);
    }

    /// Set the input width
    pub fn set_input_width(&mut self, width: BusWidth) {
        if width != self.input_width {
            self.input_width = width;
            self.validate_parameters();
            self.update_pins();
        }
    }

    /// Get the input width
    pub fn input_width(&self) -> BusWidth {
        self.input_width
    }

    /// Set the output width
    pub fn set_output_width(&mut self, width: BusWidth) {
        if width != self.output_width {
            self.output_width = width;
            self.validate_parameters();
            self.update_pins();
        }
    }

    /// Get the output width
    pub fn output_width(&self) -> BusWidth {
        self.output_width
    }

    /// Set the starting bit index
    pub fn set_start_bit(&mut self, start_bit: u8) {
        if start_bit != self.start_bit {
            self.start_bit = start_bit;
            self.validate_parameters();
        }
    }

    /// Get the starting bit index
    pub fn start_bit(&self) -> u8 {
        self.start_bit
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

    /// Validate that the parameters are consistent and adjust if necessary
    fn validate_parameters(&mut self) {
        // Ensure start_bit is within input range
        if self.start_bit >= self.input_width.0 as u8 {
            self.start_bit = 0;
        }
        
        // Ensure output width doesn't exceed available bits from start_bit
        let max_output_width = self.input_width.0 - self.start_bit as u32;
        if self.output_width.0 > max_output_width {
            self.output_width = BusWidth(max_output_width);
        }
        
        // Ensure output width is at least 1 bit
        if self.output_width.0 == 0 {
            self.output_width = BusWidth(1);
        }
    }

    /// Get the ending bit index (inclusive)
    pub fn end_bit(&self) -> u8 {
        (self.start_bit + self.output_width.0 as u8 - 1).min(self.input_width.0 as u8 - 1)
    }

    /// Extract bits from a value based on current configuration
    fn extract_bits(&self, input_value: &Value) -> Value {
        // Simplified implementation for single-bit values
        match input_value {
            Value::Low => Value::Low,
            Value::High => {
                // For now, just pass through the value
                // TODO: Implement proper multi-bit extraction
                *input_value
            }
            Value::HighZ => Value::HighZ,
            Value::Error => Value::Error,
            Value::Unknown => Value::Unknown,
        }
    }
}

impl Component for BitSelector {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Bit Selector"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Get input signal
        let input_value = if let Some(input_pin) = self.pins.get("input") {
            *input_pin.signal.value()
        } else {
            return UpdateResult::NoChange;
        };

        // Extract the selected bits
        let output_value = self.extract_bits(input_value);

        // Set output
        if let Some(output_pin) = self.pins.get_mut("output") {
            let output_signal = Signal::new(self.output_width, output_value);
            output_pin.signal = Some(output_signal);
            UpdateResult::Changed
        } else {
            UpdateResult::NoChange
        }
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
    fn test_bit_selector_creation() {
        let selector = BitSelector::new(ComponentId(1));
        assert_eq!(selector.id(), ComponentId(1));
        assert_eq!(selector.name(), "Bit Selector");
        assert_eq!(selector.input_width(), BusWidth(8));
        assert_eq!(selector.output_width(), BusWidth(1));
        assert_eq!(selector.start_bit(), 0);
    }

    #[test]
    fn test_bit_selector_configuration() {
        let mut selector = BitSelector::new(ComponentId(1));
        
        // Test setting input width
        selector.set_input_width(BusWidth(16));
        assert_eq!(selector.input_width(), BusWidth(16));
        
        // Test setting output width
        selector.set_output_width(BusWidth(4));
        assert_eq!(selector.output_width(), BusWidth(4));
        
        // Test setting start bit
        selector.set_start_bit(2);
        assert_eq!(selector.start_bit(), 2);
        assert_eq!(selector.end_bit(), 5); // start_bit + output_width - 1
        
        // Test setting facing direction
        selector.set_facing(Direction::North);
        assert_eq!(selector.facing(), Direction::North);
    }

    #[test]
    fn test_bit_selector_parameter_validation() {
        let mut selector = BitSelector::new(ComponentId(1));
        selector.set_input_width(BusWidth(8));
        
        // Test start bit validation - should clamp to valid range
        selector.set_start_bit(10); // Beyond input width
        assert_eq!(selector.start_bit(), 0); // Should be reset to 0
        
        // Test output width validation
        selector.set_start_bit(6);
        selector.set_output_width(BusWidth(8)); // Too wide for remaining bits
        assert_eq!(selector.output_width(), BusWidth(2)); // Should be clamped to available bits
    }

    #[test]
    fn test_bit_selector_pins() {
        let selector = BitSelector::new(ComponentId(1));
        let pins = selector.pins();
        
        // Should have 1 input + 1 output = 2 pins
        assert_eq!(pins.len(), 2);
        assert!(pins.contains_key("input"));
        assert!(pins.contains_key("output"));
    }

    #[test]
    fn test_bit_selector_reset() {
        let mut selector = BitSelector::new(ComponentId(1));
        
        // Set some pin signals
        if let Some(pin) = selector.pins.get_mut("input") {
            pin.signal = Some(Signal::new(BusWidth(8), Value::Binary(vec![true, false, true, false, false, false, false, false])));
        }
        
        // Reset should clear all signals
        selector.reset();
        
        for pin in selector.pins().values() {
            assert!(pin.signal.is_none());
        }
    }

    #[test]
    fn test_bit_selector_propagation_delay() {
        let selector = BitSelector::new(ComponentId(1));
        assert_eq!(selector.propagation_delay(), 3);
    }

    #[test]
    fn test_bit_extraction() {
        let mut selector = BitSelector::new(ComponentId(1));
        selector.set_input_width(BusWidth(8));
        selector.set_output_width(BusWidth(4));
        selector.set_start_bit(2);
        
        // Test extracting bits from binary value
        let input_bits = vec![true, false, true, true, false, true, false, false]; // 0b00101101
        let input_value = Value::Binary(input_bits);
        let output_value = selector.extract_bits(&input_value);
        
        // Should extract bits 2-5: [true, true, false, true]
        match output_value {
            Value::High => {
                assert_eq!(bits.len(), 4);
                assert_eq!(bits, vec![true, true, false, true]);
            }
            _ => panic!("Expected binary output"),
        }
    }

    #[test]
    fn test_single_bit_extraction() {
        let mut selector = BitSelector::new(ComponentId(1));
        selector.set_input_width(BusWidth(8));
        selector.set_output_width(BusWidth(1));
        selector.set_start_bit(3);
        
        // Test extracting single bit
        let input_bits = vec![true, false, true, true, false, true, false, false]; // bit 3 is true
        let input_value = Value::Binary(input_bits);
        let output_value = selector.extract_bits(&input_value);
        
        assert_eq!(output_value, Value::High);
        
        // Test extracting single bit that is false
        selector.set_start_bit(1); // bit 1 is false
        let output_value = selector.extract_bits(&input_value);
        assert_eq!(output_value, Value::Low);
    }

    #[test] 
    fn test_error_propagation() {
        let selector = BitSelector::new(ComponentId(1));
        
        // Test error value propagation
        let output_value = selector.extract_bits(&Value::Error);
        assert_eq!(output_value, Value::Error);
        
        // Test high impedance propagation
        let output_value = selector.extract_bits(&Value::HighZ);
        assert_eq!(output_value, Value::HighZ);
    }
}