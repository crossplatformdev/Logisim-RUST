/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Binary to BCD Converter Component
//!
//! This module implements a binary to BCD (Binary Coded Decimal) converter component
//! that converts binary input values to multiple BCD outputs based on the decimal
//! representation of the input value.

use crate::component::{Component, ComponentId, Pin, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Binary to BCD Converter
///
/// Converts binary input values (4-13 bits) to multiple BCD outputs representing
/// the decimal digits of the input value. Each BCD output is 4 bits wide and
/// represents one decimal digit.
///
/// ## Ports
///
/// - **Input**: Binary value (configurable width: 4-13 bits)
/// - **Outputs**: BCD digits (4 bits each), number depends on input bit width
///
/// ## Example
///
/// For a 9-bit input (max value 511):
/// - Input: binary 123 (0b01111011)
/// - Output 1: BCD 3 (0b0011) - ones digit
/// - Output 2: BCD 2 (0b0010) - tens digit  
/// - Output 3: BCD 1 (0b0001) - hundreds digit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinToBcd {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: u32,
}

impl BinToBcd {
    /// Unique identifier of the tool, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "Binary_to_BCD_converter";

    /// Propagation delay in time units
    pub const PROPAGATION_DELAY: u64 = 1;

    /// Default input bit width
    const DEFAULT_BIT_WIDTH: u32 = 9;

    /// Creates a new Binary to BCD converter with default bit width
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, Self::DEFAULT_BIT_WIDTH)
    }

    /// Creates a new Binary to BCD converter with specified bit width
    pub fn new_with_width(id: ComponentId, bit_width: u32) -> Self {
        let bit_width = bit_width.clamp(4, 13);
        let num_bcd_outputs = Self::calculate_bcd_outputs(bit_width);
        
        let mut pins = HashMap::new();
        
        // Input pin
        pins.insert("BIN_IN".to_string(), Pin::new_input("BIN_IN", BusWidth(bit_width)));
        
        // BCD output pins (from most significant to least significant)
        for i in 0..num_bcd_outputs {
            let power = num_bcd_outputs - 1 - i;
            let decimal_value = 10_u32.pow(power as u32);
            let pin_name = format!("BCD_{}", decimal_value);
            pins.insert(pin_name.clone(), Pin::new_output(&pin_name, BusWidth(4)));
        }

        BinToBcd { id, pins, bit_width }
    }

    /// Calculate number of BCD output ports needed based on bit width
    fn calculate_bcd_outputs(bit_width: u32) -> usize {
        let max_value = (1u32 << bit_width) - 1;
        let max_decimal = max_value as f64;
        (max_decimal.log10().floor() as usize) + 1
    }

    /// Convert binary value to BCD digits
    fn binary_to_bcd_digits(&self, mut binary_value: u32) -> Vec<u8> {
        let num_outputs = Self::calculate_bcd_outputs(self.bit_width);
        let mut digits = Vec::with_capacity(num_outputs);
        
        // Extract digits from least significant to most significant
        for _ in 0..num_outputs {
            digits.push((binary_value % 10) as u8);
            binary_value /= 10;
        }
        
        // Reverse to get most significant digit first
        digits.reverse();
        digits
    }

    /// Convert BCD digit to 4-bit signal
    fn digit_to_signal(&self, digit: u8) -> Signal {
        let values = vec![
            if (digit & 1) != 0 { Value::High } else { Value::Low },
            if (digit & 2) != 0 { Value::High } else { Value::Low },
            if (digit & 4) != 0 { Value::High } else { Value::Low },
            if (digit & 8) != 0 { Value::High } else { Value::Low },
        ];
        Signal::new_bus(values)
    }

    /// Get the bit width of this converter
    pub fn get_bit_width(&self) -> u32 {
        self.bit_width
    }

    /// Get the number of BCD outputs
    pub fn get_num_bcd_outputs(&self) -> usize {
        Self::calculate_bcd_outputs(self.bit_width)
    }
}

impl Component for BinToBcd {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        Self::ID
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, timestamp: Timestamp) -> UpdateResult {
        self.propagate(timestamp)
    }

    fn reset(&mut self) {
        // Reset all outputs to unknown
        for pin in self.pins.values_mut() {
            if pin.is_output() {
                pin.set_signal(Signal::unknown(pin.get_width()));
            }
        }
    }
}

impl BinToBcd {
    /// Convert signal to u32 value if possible
    fn signal_to_u32(&self, signal: &Signal) -> Option<u32> {
        if signal.width().as_u32() > 32 {
            return None;
        }
        
        let mut value = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            match bit {
                Value::High => value |= 1 << i,
                Value::Low => {},
                Value::Unknown | Value::Error => return None,
            }
        }
        Some(value)
    }

    /// Perform the propagation logic - convert binary input to BCD outputs
    fn propagate(&mut self, _timestamp: Timestamp) -> UpdateResult {
        let mut updates = Vec::new();
        
        if let Some(input_pin) = self.pins.get("BIN_IN") {
            let input_signal = input_pin.get_signal();
            
            // Check if input is valid
            if let Some(binary_value) = self.signal_to_u32(input_signal) {
                // Convert to BCD digits
                let bcd_digits = self.binary_to_bcd_digits(binary_value);
                let num_outputs = self.get_num_bcd_outputs();
                
                // Set each BCD output
                for (i, &digit) in bcd_digits.iter().enumerate() {
                    let power = num_outputs - 1 - i;
                    let decimal_value = 10_u32.pow(power as u32);
                    let pin_name = format!("BCD_{}", decimal_value);
                    
                    if let Some(output_pin) = self.pins.get_mut(&pin_name) {
                        let signal = self.digit_to_signal(digit);
                        output_pin.set_signal(signal);
                        updates.push((self.id, pin_name, Self::PROPAGATION_DELAY));
                    }
                }
            } else {
                // Invalid input, set all outputs to unknown
                let num_outputs = self.get_num_bcd_outputs();
                for i in 0..num_outputs {
                    let power = num_outputs - 1 - i;
                    let decimal_value = 10_u32.pow(power as u32);
                    let pin_name = format!("BCD_{}", decimal_value);
                    
                    if let Some(output_pin) = self.pins.get_mut(&pin_name) {
                        output_pin.set_signal(Signal::unknown(BusWidth(4)));
                        updates.push((self.id, pin_name, Self::PROPAGATION_DELAY));
                    }
                }
            }
        }
        
        UpdateResult { updates }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let converter = BinToBcd::new(ComponentId(1));
        assert_eq!(converter.id(), ComponentId(1));
        assert_eq!(converter.get_bit_width(), 9);
    }

    #[test]
    fn test_id_constant() {
        // Ensure ID never changes for .circ file compatibility
        assert_eq!(BinToBcd::ID, "Binary_to_BCD_converter");
    }

    #[test]
    fn test_bcd_outputs_calculation() {
        // 4 bits: max 15, needs 2 BCD digits
        assert_eq!(BinToBcd::calculate_bcd_outputs(4), 2);
        
        // 8 bits: max 255, needs 3 BCD digits  
        assert_eq!(BinToBcd::calculate_bcd_outputs(8), 3);
        
        // 10 bits: max 1023, needs 4 BCD digits
        assert_eq!(BinToBcd::calculate_bcd_outputs(10), 4);
    }

    #[test]
    fn test_binary_to_bcd_conversion() {
        let converter = BinToBcd::new_with_width(ComponentId(1), 8);
        
        let test_cases = vec![
            (0, vec![0, 0, 0]),
            (7, vec![0, 0, 7]),
            (10, vec![0, 1, 0]),
            (99, vec![0, 9, 9]),
            (123, vec![1, 2, 3]),
            (255, vec![2, 5, 5]),
        ];

        for (binary, expected_digits) in test_cases {
            let digits = converter.binary_to_bcd_digits(binary);
            assert_eq!(digits, expected_digits, "Failed for input {}", binary);
        }
    }

    #[test]
    fn test_digit_to_signal_conversion() {
        let converter = BinToBcd::new(ComponentId(1));
        
        let signal_0 = converter.digit_to_signal(0);
        assert_eq!(signal_0.values(), &[Value::Low, Value::Low, Value::Low, Value::Low]);
        
        let signal_5 = converter.digit_to_signal(5);
        assert_eq!(signal_5.values(), &[Value::High, Value::Low, Value::High, Value::Low]);
        
        let signal_9 = converter.digit_to_signal(9);
        assert_eq!(signal_9.values(), &[Value::High, Value::Low, Value::Low, Value::High]);
    }

    #[test]
    fn test_pin_configuration() {
        let converter = BinToBcd::new_with_width(ComponentId(1), 8);
        let pins = converter.pins();
        
        // Should have 1 input + 3 outputs for 8-bit input
        assert_eq!(pins.len(), 4);
        
        // Check input pin
        let input_pin = pins.get("BIN_IN").unwrap();
        assert!(input_pin.is_input());
        assert_eq!(input_pin.get_width(), BusWidth(8));
        
        // Check output pins
        assert!(pins.contains_key("BCD_100"));
        assert!(pins.contains_key("BCD_10"));
        assert!(pins.contains_key("BCD_1"));
        
        for pin_name in ["BCD_100", "BCD_10", "BCD_1"] {
            let pin = pins.get(pin_name).unwrap();
            assert!(pin.is_output());
            assert_eq!(pin.get_width(), BusWidth(4));
        }
    }

    #[test]
    fn test_component_reset() {
        let mut converter = BinToBcd::new(ComponentId(1));
        converter.reset();
        
        // All output pins should be unknown after reset
        for (name, pin) in converter.pins() {
            if pin.is_output() {
                assert!(name.starts_with("BCD_"));
                // Signal should be unknown (we can't easily test this without more infrastructure)
            }
        }
    }
}