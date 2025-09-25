/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Subtractor Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Subtractor`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::adder::Adder;

/// Multi-bit Subtractor component
/// 
/// Performs binary subtraction (A - B) on two multi-bit inputs with borrow-in and borrow-out.
/// Internally implemented using addition with complement (two's complement arithmetic).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtractor {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
}

impl Subtractor {
    /// Create a new 8-bit subtractor (default width)
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, BusWidth(8))
    }
    
    /// Create a new subtractor with specified bit width
    pub fn new_with_width(id: ComponentId, bit_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        
        // Input pins (A - B)
        pins.insert("A".to_string(), Pin::new_input("A", bit_width));          // Minuend
        pins.insert("B".to_string(), Pin::new_input("B", bit_width));          // Subtrahend
        pins.insert("Borrow_In".to_string(), Pin::new_input("Borrow_In", BusWidth(1)));
        
        // Output pins
        pins.insert("Difference".to_string(), Pin::new_output("Difference", bit_width));
        pins.insert("Borrow_Out".to_string(), Pin::new_output("Borrow_Out", BusWidth(1)));

        Subtractor { id, pins, bit_width }
    }
    
    /// Compute the difference using addition with complement
    /// Returns (difference, borrow_out)
    /// 
    /// Subtraction A - B is implemented as A + (~B) + (~borrow_in)
    /// The borrow_out is the complement of the carry_out from this addition
    fn compute_difference(
        bit_width: BusWidth, 
        value_a: &Value, 
        value_b: &Value, 
        borrow_in: &Value
    ) -> (Value, Value) {
        // Handle special cases for borrow_in
        let borrow_in = match borrow_in {
            Value::Unknown | Value::HighZ => Value::Low,
            _ => borrow_in.clone(),
        };
        
        // Compute A - B using A + (~B) + (~borrow_in)
        let not_b = value_b.not();
        let not_borrow_in = borrow_in.not();
        
        // Use the adder's computation logic
        let (sum, carry_out) = Adder::compute_sum(bit_width, value_a, &not_b, &not_borrow_in);
        
        // The result is the sum, and borrow_out is the complement of carry_out
        (sum, carry_out.not())
    }
    
    /// Get the current bit width
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
    
    /// Set the bit width (updates pin widths accordingly)
    pub fn set_bit_width(&mut self, width: BusWidth) {
        self.bit_width = width;
        if let Some(pin) = self.pins.get_mut("A") {
            pin.width = width; pin.signal = Signal::unknown(width);
        }
        if let Some(pin) = self.pins.get_mut("B") {
            pin.width = width; pin.signal = Signal::unknown(width);
        }
        if let Some(pin) = self.pins.get_mut("Difference") {
            pin.width = width; pin.signal = Signal::unknown(width);
        }
    }
}

impl Component for Subtractor {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Subtractor"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        // Get input values
        let value_a = self.pins.get("A").map(|p| p.get_signal().value()).unwrap_or(Value::Unknown);
        let value_b = self.pins.get("B").map(|p| p.get_signal().value()).unwrap_or(Value::Unknown);
        let borrow_in = self.pins.get("Borrow_In").map(|p| p.get_signal().value()).unwrap_or(Value::Low);
        
        // Compute difference and borrow out
        let (difference, borrow_out) = Self::compute_difference(
            self.bit_width, 
            &value_a, 
            &value_b, 
            &borrow_in
        );
        
        // Update output pins
        let mut changed = false;
        if let Some(diff_pin) = self.pins.get_mut("Difference") {
            if diff_pin.get_signal().value() != difference {
                diff_pin.set_signal(Signal::new(difference, current_time));
                changed = true;
            }
        }
        
        if let Some(borrow_pin) = self.pins.get_mut("Borrow_Out") {
            if borrow_pin.get_signal().value() != borrow_out {
                borrow_pin.set_signal(Signal::new(borrow_out, current_time));
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtractor_creation() {
        let subtractor = Subtractor::new(ComponentId(1));
        assert_eq!(subtractor.id(), ComponentId(1));
        assert_eq!(subtractor.name(), "Subtractor");
        assert_eq!(subtractor.bit_width(), BusWidth(8));
        assert_eq!(subtractor.pins().len(), 5);
    }

    #[test]
    fn test_subtractor_with_custom_width() {
        let subtractor = Subtractor::new_with_width(ComponentId(1), BusWidth(16));
        assert_eq!(subtractor.bit_width(), BusWidth(16));
    }

    #[test]
    fn test_compute_difference_simple() {
        let a = Value::from_long(10, BusWidth(8));
        let b = Value::from_long(3, BusWidth(8));
        let bin = Value::Low;
        
        let (diff, bout) = Subtractor::compute_difference(BusWidth(8), &a, &b, &bin);
        
        assert_eq!(diff.to_long_value(), 7);  // 10 - 3 = 7
        assert_eq!(bout, Value::Low);
    }

    #[test]
    fn test_compute_difference_with_borrow() {
        let a = Value::from_long(3, BusWidth(8));
        let b = Value::from_long(10, BusWidth(8));
        let bin = Value::Low;
        
        let (diff, bout) = Subtractor::compute_difference(BusWidth(8), &a, &b, &bin);
        
        // 3 - 10 = -7, which in 8-bit two's complement is 249 (256 - 7)
        assert_eq!(diff.to_long_value(), 249);
        assert_eq!(bout, Value::High);
    }

    #[test]
    fn test_compute_difference_with_borrow_in() {
        let a = Value::from_long(10, BusWidth(8));
        let b = Value::from_long(3, BusWidth(8));
        let bin = Value::High;
        
        let (diff, bout) = Subtractor::compute_difference(BusWidth(8), &a, &b, &bin);
        
        assert_eq!(diff.to_long_value(), 6);  // 10 - 3 - 1 = 6
        assert_eq!(bout, Value::Low);
    }

    #[test]
    fn test_compute_difference_zero() {
        let a = Value::from_long(5, BusWidth(8));
        let b = Value::from_long(5, BusWidth(8));
        let bin = Value::Low;
        
        let (diff, bout) = Subtractor::compute_difference(BusWidth(8), &a, &b, &bin);
        
        assert_eq!(diff.to_long_value(), 0);  // 5 - 5 = 0
        assert_eq!(bout, Value::Low);
    }

    #[test]
    fn test_bit_width_change() {
        let mut subtractor = Subtractor::new(ComponentId(1));
        
        subtractor.set_bit_width(BusWidth(16));
        assert_eq!(subtractor.bit_width(), BusWidth(16));
        
        // Check that pin widths were updated
        assert_eq!(subtractor.pins().get("A").unwrap().width(), BusWidth(16));
        assert_eq!(subtractor.pins().get("B").unwrap().width(), BusWidth(16));
        assert_eq!(subtractor.pins().get("Difference").unwrap().width(), BusWidth(16));
        assert_eq!(subtractor.pins().get("Borrow_In").unwrap().width(), BusWidth(1));
        assert_eq!(subtractor.pins().get("Borrow_Out").unwrap().width(), BusWidth(1));
    }

    #[test]
    fn test_component_reset() {
        let mut subtractor = Subtractor::new(ComponentId(1));
        subtractor.reset();
        // Should not panic and should reset all pins
        assert_eq!(subtractor.pins().len(), 5);
    }
}