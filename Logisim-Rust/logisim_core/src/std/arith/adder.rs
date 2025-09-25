/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Adder Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Adder`

use crate::component::{Component, ComponentId, Pin, Propagator, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multi-bit Adder component
/// 
/// Performs binary addition on two multi-bit inputs with carry-in and carry-out.
/// Supports configurable bit width and handles overflow correctly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adder {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
}

impl Adder {
    /// Create a new 8-bit adder (default width)
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, BusWidth(8))
    }
    
    /// Create a new adder with specified bit width
    pub fn new_with_width(id: ComponentId, bit_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        
        // Input pins
        pins.insert("A".to_string(), Pin::new_input("A", bit_width));
        pins.insert("B".to_string(), Pin::new_input("B", bit_width));
        pins.insert("Carry_In".to_string(), Pin::new_input("Carry_In", BusWidth(1)));
        
        // Output pins
        pins.insert("Sum".to_string(), Pin::new_output("Sum", bit_width));
        pins.insert("Carry_Out".to_string(), Pin::new_output("Carry_Out", BusWidth(1)));

        Adder { id, pins, bit_width }
    }
    
    /// Compute the sum with carry propagation
    /// Returns (sum, carry_out)
    fn compute_sum(bit_width: BusWidth, value_a: &Value, value_b: &Value, carry_in: &Value) -> (Value, Value) {
        let width = bit_width.0;
        
        // Handle special cases for carry_in
        let carry_in = match carry_in {
            Value::Unknown | Value::HighZ => Value::Low,
            _ => carry_in.clone(),
        };
        
        if value_a.is_fully_defined() && value_b.is_fully_defined() && carry_in.is_fully_defined() {
            // Fast path for fully defined values
            if width == 64 {
                // Special handling for 64-bit to avoid overflow
                let a_val = value_a.to_long_value();
                let b_val = value_b.to_long_value();
                let c_val = carry_in.to_long_value();
                
                let mask = !(1u64 << 63);
                let a_sign = (a_val as u64) >> 63 != 0;
                let b_sign = (b_val as u64) >> 63 != 0;
                
                // Calculate carry out from sign bits
                let masked_a = (a_val as u64) & mask;
                let masked_b = (b_val as u64) & mask;
                let c_in_sign = ((masked_a + masked_b + c_val as u64) >> 63) != 0;
                
                let carry_out = (a_sign && b_sign) || (a_sign && c_in_sign) || (b_sign && c_in_sign);
                
                let sum = a_val.wrapping_add(b_val).wrapping_add(c_val);
                
                (
                    Value::from_long(sum, bit_width),
                    if carry_out { Value::High } else { Value::Low }
                )
            } else {
                // Standard addition for widths < 64
                let sum = value_a.to_long_value() + value_b.to_long_value() + carry_in.to_long_value();
                let carry_out = ((sum >> width) & 1) != 0;
                
                (
                    Value::from_long(sum, bit_width),
                    if carry_out { Value::High } else { Value::Low }
                )
            }
        } else {
            // Bit-by-bit computation for undefined values
            let mut result_bits = vec![Value::Error; width];
            let mut carry = carry_in.clone();
            
            for i in 0..width {
                if matches!(carry, Value::Error) {
                    result_bits[i] = Value::Error;
                } else if matches!(carry, Value::Unknown) {
                    result_bits[i] = Value::Unknown;
                } else {
                    let bit_a = value_a.get_bit(i);
                    let bit_b = value_b.get_bit(i);
                    
                    if matches!(bit_a, Value::Error) || matches!(bit_b, Value::Error) {
                        result_bits[i] = Value::Error;
                        carry = Value::Error;
                    } else if matches!(bit_a, Value::Unknown) || matches!(bit_b, Value::Unknown) {
                        result_bits[i] = Value::Unknown;
                        carry = Value::Unknown;
                    } else {
                        // Full adder logic for this bit
                        let a_bit = matches!(bit_a, Value::High);
                        let b_bit = matches!(bit_b, Value::High);
                        let c_bit = matches!(carry, Value::High);
                        
                        let sum_bit = a_bit ^ b_bit ^ c_bit;
                        let carry_bit = (a_bit && b_bit) || (a_bit && c_bit) || (b_bit && c_bit);
                        
                        result_bits[i] = if sum_bit { Value::High } else { Value::Low };
                        carry = if carry_bit { Value::High } else { Value::Low };
                    }
                }
            }
            
            (Value::from_bits(&result_bits), carry)
        }
    }
    
    /// Get the current bit width
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
    
    /// Set the bit width (updates pin widths accordingly)
    pub fn set_bit_width(&mut self, width: BusWidth) {
        self.bit_width = width;
        if let Some(pin) = self.pins.get_mut("A") {
            pin.set_width(width);
        }
        if let Some(pin) = self.pins.get_mut("B") {
            pin.set_width(width);
        }
        if let Some(pin) = self.pins.get_mut("Sum") {
            pin.set_width(width);
        }
    }
}

impl Component for Adder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Adder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Get input values
        let value_a = self.pins.get("A").map(|p| p.signal().value()).unwrap_or(Value::Unknown);
        let value_b = self.pins.get("B").map(|p| p.signal().value()).unwrap_or(Value::Unknown);
        let carry_in = self.pins.get("Carry_In").map(|p| p.signal().value()).unwrap_or(Value::Low);
        
        // Compute sum and carry out
        let (sum, carry_out) = Self::compute_sum(self.bit_width, &value_a, &value_b, &carry_in);
        
        // Update output pins
        let mut changed = false;
        if let Some(sum_pin) = self.pins.get_mut("Sum") {
            if sum_pin.signal().value() != sum {
                sum_pin.set_signal(Signal::new(sum, _current_time));
                changed = true;
            }
        }
        
        if let Some(carry_pin) = self.pins.get_mut("Carry_Out") {
            if carry_pin.signal().value() != carry_out {
                carry_pin.set_signal(Signal::new(carry_out, _current_time));
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
        // Reset all pins to their default states
        for pin in self.pins.values_mut() {
            pin.reset();
        }
    }
}

impl Propagator for Adder {
    fn propagate(&mut self, current_time: Timestamp) {
        // Calculate propagation delay based on bit width
        let delay = (self.bit_width.0 + 2) * 1; // PER_DELAY = 1 from Java
        let propagation_time = current_time + delay as u64;
        
        // Perform the update at the calculated time
        self.update(propagation_time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder_creation() {
        let adder = Adder::new(ComponentId(1));
        assert_eq!(adder.id(), ComponentId(1));
        assert_eq!(adder.name(), "Adder");
        assert_eq!(adder.bit_width(), BusWidth(8));
        assert_eq!(adder.pins().len(), 5);
    }

    #[test]
    fn test_adder_with_custom_width() {
        let adder = Adder::new_with_width(ComponentId(1), BusWidth(16));
        assert_eq!(adder.bit_width(), BusWidth(16));
    }

    #[test]
    fn test_compute_sum_simple() {
        let a = Value::from_long(5, BusWidth(8));
        let b = Value::from_long(3, BusWidth(8));
        let cin = Value::Low;
        
        let (sum, cout) = Adder::compute_sum(BusWidth(8), &a, &b, &cin);
        
        assert_eq!(sum.to_long_value(), 8);
        assert_eq!(cout, Value::Low);
    }

    #[test]
    fn test_compute_sum_with_carry() {
        let a = Value::from_long(255, BusWidth(8));  // Max value for 8-bit
        let b = Value::from_long(1, BusWidth(8));
        let cin = Value::Low;
        
        let (sum, cout) = Adder::compute_sum(BusWidth(8), &a, &b, &cin);
        
        assert_eq!(sum.to_long_value(), 0);  // Overflow wraps to 0
        assert_eq!(cout, Value::High);       // Carry out is set
    }

    #[test]
    fn test_compute_sum_with_carry_in() {
        let a = Value::from_long(5, BusWidth(8));
        let b = Value::from_long(3, BusWidth(8));
        let cin = Value::High;
        
        let (sum, cout) = Adder::compute_sum(BusWidth(8), &a, &b, &cin);
        
        assert_eq!(sum.to_long_value(), 9);  // 5 + 3 + 1 = 9
        assert_eq!(cout, Value::Low);
    }

    #[test]
    fn test_bit_width_change() {
        let mut adder = Adder::new(ComponentId(1));
        
        adder.set_bit_width(BusWidth(16));
        assert_eq!(adder.bit_width(), BusWidth(16));
        
        // Check that pin widths were updated
        assert_eq!(adder.pins().get("A").unwrap().width(), BusWidth(16));
        assert_eq!(adder.pins().get("B").unwrap().width(), BusWidth(16));
        assert_eq!(adder.pins().get("Sum").unwrap().width(), BusWidth(16));
        assert_eq!(adder.pins().get("Carry_In").unwrap().width(), BusWidth(1));
        assert_eq!(adder.pins().get("Carry_Out").unwrap().width(), BusWidth(1));
    }

    #[test]
    fn test_component_reset() {
        let mut adder = Adder::new(ComponentId(1));
        adder.reset();
        // Should not panic and should reset all pins
        assert_eq!(adder.pins().len(), 5);
    }
}