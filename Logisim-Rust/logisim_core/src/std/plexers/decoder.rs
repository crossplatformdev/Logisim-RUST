/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Decoder component implementation
//!
//! A decoder is an address decoder that activates one output based on a binary input address.
//! For an n-bit input, it has 2^n outputs where only one output is active (high) at a time.

use crate::{
    component::{Component, ComponentId, Pin, Propagator, UpdateResult},
    data::{BitWidth, Bounds, Direction, Location},
    signal::{BusWidth, Signal, Timestamp, Value},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Decoder component for address decoding
///
/// A decoder takes an n-bit address input and activates one of 2^n outputs.
/// Only the output corresponding to the input address is set to high; all others are low.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decoder {
    /// Unique component identifier
    id: ComponentId,
    /// Component pins (input, outputs, and optional enable)
    pins: HashMap<String, Pin>,
    /// Number of input address bits (determines number of outputs)
    input_bits: u8,
    /// Component facing direction
    facing: Direction,
    /// Whether to use three-state outputs
    tristate: bool,
    /// Whether component has enable input
    enable: bool,
    /// Component bounds for rendering
    bounds: Bounds,
}

impl Decoder {
    /// Create a new decoder with the given ID
    pub fn new(id: ComponentId) -> Self {
        let mut decoder = Decoder {
            id,
            pins: HashMap::new(),
            input_bits: 2, // Default to 2-bit input (4 outputs)
            facing: Direction::East,
            tristate: false,
            enable: false,
            bounds: Bounds::new(0, 0, 40, 50),
        };
        decoder.update_pins();
        decoder
    }

    /// Create a new decoder with specified parameters
    pub fn with_config(
        id: ComponentId,
        input_bits: u8,
        facing: Direction,
    ) -> Self {
        let mut decoder = Decoder {
            id,
            pins: HashMap::new(),
            input_bits: input_bits.clamp(1, 8),
            facing,
            tristate: false,
            enable: false,
            bounds: Bounds::new(0, 0, 40, 50),
        };
        decoder.update_pins();
        decoder
    }

    /// Update pin configuration based on current settings
    fn update_pins(&mut self) {
        self.pins.clear();
        
        let num_outputs = 1 << self.input_bits; // 2^input_bits
        
        // Add address input
        let input_pin = Pin::new(
            "Address".to_string(),
            BusWidth(self.input_bits as u32),
            crate::component::PinDirection::Input,
            Location::new(0, 25),
        );
        self.pins.insert("address".to_string(), input_pin);
        
        // Add outputs (one for each possible address)
        for i in 0..num_outputs {
            let pin_name = format!("output_{}", i);
            let pin = Pin::new(
                format!("Output {}", i),
                BusWidth(1), // Each output is single bit
                crate::component::PinDirection::Output,
                Location::new(40, 5 + i * 5),
            );
            self.pins.insert(pin_name, pin);
        }
        
        // Add enable input if enabled
        if self.enable {
            let enable_pin = Pin::new(
                "Enable".to_string(),
                BusWidth(1),
                crate::component::PinDirection::Input,
                Location::new(20, 0),
            );
            self.pins.insert("enable".to_string(), enable_pin);
        }
    }

    /// Set the number of input bits
    pub fn set_input_bits(&mut self, bits: u8) {
        if bits != self.input_bits && bits >= 1 && bits <= 8 {
            self.input_bits = bits;
            self.update_pins();
        }
    }

    /// Get the number of input bits
    pub fn input_bits(&self) -> u8 {
        self.input_bits
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
    pub fn set_enable(&mut self, enable: bool) {
        if enable != self.enable {
            self.enable = enable;
            self.update_pins();
        }
    }

    /// Check if enable input is present
    pub fn enable(&self) -> bool {
        self.enable
    }

    /// Calculate the number of outputs based on input bits
    pub fn num_outputs(&self) -> usize {
        1 << self.input_bits
    }
}

impl Component for Decoder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Decoder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Get address input signal
        let address_value = if let Some(address_pin) = self.pins.get("address") {
            match &address_pin.signal {
                Some(signal) => signal.value(),
                None => return UpdateResult::NoChange, // No address signal
            }
        } else {
            return UpdateResult::NoChange;
        };

        // Check if enabled (if enable pin exists)
        if self.enable {
            if let Some(enable_pin) = self.pins.get("enable") {
                match &enable_pin.signal {
                    Some(signal) => {
                        if signal.value() == Value::Zero {
                            // Component is disabled, set all outputs to zero or high impedance
                            let disabled_value = if self.tristate {
                                Value::HighImpedance
                            } else {
                                Value::Zero
                            };
                            
                            for i in 0..self.num_outputs() {
                                let output_pin_name = format!("output_{}", i);
                                if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                                    let output_signal = Signal::new(BusWidth(1), disabled_value.clone());
                                    output_pin.signal = Some(output_signal);
                                }
                            }
                            return UpdateResult::Changed;
                        }
                    }
                    None => return UpdateResult::NoChange,
                }
            }
        }

        // Convert address value to index
        let address_index = match address_value {
            Value::Zero => 0,
            Value::One => {
                if self.input_bits == 1 {
                    1
                } else {
                    // For multi-bit inputs, Value::One represents the value 1
                    1
                }
            }
            Value::Binary(bits) => {
                // Convert binary representation to index
                bits.iter().enumerate().fold(0usize, |acc, (i, &bit)| {
                    acc + if bit { 1 << i } else { 0 }
                })
            }
            Value::HighImpedance | Value::Error => {
                // Invalid address signal - set all outputs to error or zero
                let error_value = if self.tristate {
                    Value::HighImpedance
                } else {
                    Value::Zero
                };
                
                for i in 0..self.num_outputs() {
                    let output_pin_name = format!("output_{}", i);
                    if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                        let output_signal = Signal::new(BusWidth(1), error_value.clone());
                        output_pin.signal = Some(output_signal);
                    }
                }
                return UpdateResult::Changed;
            }
        };

        // Ensure address index is within valid range
        if address_index >= self.num_outputs() {
            // Invalid address - set all outputs to zero
            for i in 0..self.num_outputs() {
                let output_pin_name = format!("output_{}", i);
                if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                    let output_signal = Signal::new(BusWidth(1), Value::Zero);
                    output_pin.signal = Some(output_signal);
                }
            }
            return UpdateResult::Changed;
        }

        // Set outputs: addressed output gets high, others get low
        let mut changed = false;
        for i in 0..self.num_outputs() {
            let output_pin_name = format!("output_{}", i);
            if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                let output_value = if i == address_index {
                    Value::One // Active output
                } else {
                    Value::Zero // Inactive outputs
                };
                
                let output_signal = Signal::new(BusWidth(1), output_value);
                output_pin.signal = Some(output_signal);
                changed = true;
            }
        }

        if changed {
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

impl Propagator for Decoder {
    fn propagate(&mut self, timestamp: Timestamp) -> UpdateResult {
        self.update(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let decoder = Decoder::new(ComponentId(1));
        assert_eq!(decoder.id(), ComponentId(1));
        assert_eq!(decoder.name(), "Decoder");
        assert_eq!(decoder.input_bits(), 2);
        assert_eq!(decoder.num_outputs(), 4);
    }

    #[test]
    fn test_decoder_configuration() {
        let mut decoder = Decoder::new(ComponentId(1));
        
        // Test setting input bits
        decoder.set_input_bits(3);
        assert_eq!(decoder.input_bits(), 3);
        assert_eq!(decoder.num_outputs(), 8);
        
        // Test setting facing direction
        decoder.set_facing(Direction::North);
        assert_eq!(decoder.facing(), Direction::North);
        
        // Test enabling tristate
        decoder.set_tristate(true);
        assert!(decoder.tristate());
        
        // Test enabling enable input
        decoder.set_enable(true);
        assert!(decoder.enable());
    }

    #[test]
    fn test_decoder_pins() {
        let decoder = Decoder::new(ComponentId(1));
        let pins = decoder.pins();
        
        // Should have 1 address input + 4 outputs = 5 pins
        assert_eq!(pins.len(), 5);
        assert!(pins.contains_key("address"));
        assert!(pins.contains_key("output_0"));
        assert!(pins.contains_key("output_1"));
        assert!(pins.contains_key("output_2"));
        assert!(pins.contains_key("output_3"));
        
        // Test with more input bits
        let mut decoder = Decoder::new(ComponentId(2));
        decoder.set_input_bits(3);
        let pins = decoder.pins();
        
        // Should have 1 address input + 8 outputs = 9 pins
        assert_eq!(pins.len(), 9);
        for i in 0..8 {
            assert!(pins.contains_key(&format!("output_{}", i)));
        }
    }

    #[test]
    fn test_decoder_with_enable() {
        let mut decoder = Decoder::new(ComponentId(1));
        decoder.set_enable(true);
        let pins = decoder.pins();
        
        // Should have 1 address input + 4 outputs + 1 enable = 6 pins
        assert_eq!(pins.len(), 6);
        assert!(pins.contains_key("enable"));
    }

    #[test]
    fn test_decoder_reset() {
        let mut decoder = Decoder::new(ComponentId(1));
        
        // Set some pin signals
        if let Some(pin) = decoder.pins.get_mut("address") {
            pin.signal = Some(Signal::new(BusWidth(2), Value::Binary(vec![true, false])));
        }
        
        // Reset should clear all signals
        decoder.reset();
        
        for pin in decoder.pins().values() {
            assert!(pin.signal.is_none());
        }
    }

    #[test]
    fn test_decoder_propagation_delay() {
        let decoder = Decoder::new(ComponentId(1));
        assert_eq!(decoder.propagation_delay(), 3);
    }

    #[test]
    fn test_decoder_single_bit_input() {
        let mut decoder = Decoder::new(ComponentId(1));
        decoder.set_input_bits(1);
        assert_eq!(decoder.num_outputs(), 2);
        
        let pins = decoder.pins();
        assert_eq!(pins.len(), 3); // 1 address input + 2 outputs
        assert!(pins.contains_key("output_0"));
        assert!(pins.contains_key("output_1"));
    }
}