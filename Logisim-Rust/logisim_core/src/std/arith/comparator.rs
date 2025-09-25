/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Comparator Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Comparator`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comparison mode for the comparator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonMode {
    /// Two's complement signed comparison
    Signed,
    /// Unsigned comparison
    Unsigned,
}

/// Multi-bit Comparator component
/// 
/// Compares two multi-bit inputs and produces three outputs:
/// - Greater Than (GT): high when A > B
/// - Equal (EQ): high when A = B  
/// - Less Than (LT): high when A < B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparator {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
    mode: ComparisonMode,
}

impl Comparator {
    /// Create a new 8-bit signed comparator (default)
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width_and_mode(id, BusWidth(8), ComparisonMode::Signed)
    }
    
    /// Create a new comparator with specified bit width and mode
    pub fn new_with_width_and_mode(id: ComponentId, bit_width: BusWidth, mode: ComparisonMode) -> Self {
        let mut pins = HashMap::new();
        
        // Input pins
        pins.insert("A".to_string(), Pin::new_input("A", bit_width));
        pins.insert("B".to_string(), Pin::new_input("B", bit_width));
        
        // Output pins
        pins.insert("Greater".to_string(), Pin::new_output("Greater", BusWidth(1)));
        pins.insert("Equal".to_string(), Pin::new_output("Equal", BusWidth(1)));
        pins.insert("Less".to_string(), Pin::new_output("Less", BusWidth(1)));
        
        Comparator { id, pins, bit_width, mode }
    }
    
    /// Compare two values based on the comparator mode
    fn compare_values(&self, value_a: &Value, value_b: &Value) -> (Value, Value, Value) {
        // Handle error and unknown cases first
        if !value_a.is_fully_defined() || !value_b.is_fully_defined() {
            let result_val = if matches!(value_a, Value::Error) || matches!(value_b, Value::Error) {
                Value::Error
            } else {
                Value::Unknown
            };
            return (result_val, result_val, result_val);
        }
        
        let a_val = value_a.to_long_value();
        let b_val = value_b.to_long_value();
        
        let (gt, eq, lt) = match self.mode {
            ComparisonMode::Signed => {
                // For signed comparison, interpret values as signed integers
                let a_signed = self.to_signed(a_val);
                let b_signed = self.to_signed(b_val);
                
                if a_signed > b_signed {
                    (Value::High, Value::Low, Value::Low)
                } else if a_signed == b_signed {
                    (Value::Low, Value::High, Value::Low)
                } else {
                    (Value::Low, Value::Low, Value::High)
                }
            }
            ComparisonMode::Unsigned => {
                // For unsigned comparison, use values directly
                let a_unsigned = a_val as u64;
                let b_unsigned = b_val as u64;
                
                if a_unsigned > b_unsigned {
                    (Value::High, Value::Low, Value::Low)
                } else if a_unsigned == b_unsigned {
                    (Value::Low, Value::High, Value::Low)
                } else {
                    (Value::Low, Value::Low, Value::High)
                }
            }
        };
        
        (gt, eq, lt)
    }
    
    /// Convert a value to signed based on bit width
    fn to_signed(&self, value: i64) -> i64 {
        let width = self.bit_width.0;
        if width >= 64 {
            value
        } else {
            let mask = (1i64 << width) - 1;
            let masked = value & mask;
            let sign_bit = 1i64 << (width - 1);
            
            if masked & sign_bit != 0 {
                // Negative number - sign extend
                masked | (!mask)
            } else {
                // Positive number
                masked
            }
        }
    }
    
    /// Get the current bit width
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
    
    /// Get the current comparison mode
    pub fn mode(&self) -> ComparisonMode {
        self.mode
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
    }
    
    /// Set the comparison mode
    pub fn set_mode(&mut self, mode: ComparisonMode) {
        self.mode = mode;
    }
}

impl Component for Comparator {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Comparator"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        // Get input values
        let value_a = self.pins.get("A").map(|p| p.get_signal().value()).unwrap_or(&Value::Unknown);
        let value_b = self.pins.get("B").map(|p| p.get_signal().value()).unwrap_or(&Value::Unknown);
        
        // Perform comparison
        let (gt, eq, lt) = self.compare_values(&value_a, &value_b);
        
        // Update output pins
        let mut changed = false;
        
        if let Some(gt_pin) = self.pins.get_mut("Greater") {
            if gt_pin.get_signal().value() != &gt {
                let _ = gt_pin.set_signal(Signal::new(gt, current_time));
                changed = true;
            }
        }
        
        if let Some(eq_pin) = self.pins.get_mut("Equal") {
            if eq_pin.get_signal().value() != &eq {
                let _ = eq_pin.set_signal(Signal::new(eq, current_time));
                changed = true;
            }
        }
        
        if let Some(lt_pin) = self.pins.get_mut("Less") {
            if lt_pin.get_signal().value() != &lt {
                let _ = lt_pin.set_signal(Signal::new(lt, current_time));
                changed = true;
            }
        }
        
        if changed {
            UpdateResult::with_outputs(outputs, 1)
        } else {
            UpdateResult::new()
        }
    }

    fn reset(&mut self) {
        // Reset all pins to their default states
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparator_creation() {
        let comparator = Comparator::new(ComponentId(1));
        assert_eq!(comparator.id(), ComponentId(1));
        assert_eq!(comparator.name(), "Comparator");
        assert_eq!(comparator.bit_width(), BusWidth(8));
        assert_eq!(comparator.mode(), ComparisonMode::Signed);
        assert_eq!(comparator.pins().len(), 5);
    }

    #[test]
    fn test_unsigned_comparison() {
        let comparator = Comparator::new_with_width_and_mode(
            ComponentId(1), 
            BusWidth(8), 
            ComparisonMode::Unsigned
        );
        
        // Test A > B
        let (gt, eq, lt) = comparator.compare_values(
            &Value::from_long(10, BusWidth(8)),
            &Value::from_long(5, BusWidth(8))
        );
        assert_eq!(gt, Value::High);
        assert_eq!(eq, Value::Low);
        assert_eq!(lt, Value::Low);
        
        // Test A = B
        let (gt, eq, lt) = comparator.compare_values(
            &Value::from_long(7, BusWidth(8)),
            &Value::from_long(7, BusWidth(8))
        );
        assert_eq!(gt, Value::Low);
        assert_eq!(eq, Value::High);
        assert_eq!(lt, Value::Low);
        
        // Test A < B
        let (gt, eq, lt) = comparator.compare_values(
            &Value::from_long(3, BusWidth(8)),
            &Value::from_long(8, BusWidth(8))
        );
        assert_eq!(gt, Value::Low);
        assert_eq!(eq, Value::Low);
        assert_eq!(lt, Value::High);
    }

    #[test]
    fn test_signed_comparison() {
        let comparator = Comparator::new_with_width_and_mode(
            ComponentId(1), 
            BusWidth(8), 
            ComparisonMode::Signed
        );
        
        // Test with negative numbers (255 as 8-bit signed = -1)
        let (gt, eq, lt) = comparator.compare_values(
            &Value::from_long(5, BusWidth(8)),
            &Value::from_long(255, BusWidth(8)) // -1 in two's complement
        );
        assert_eq!(gt, Value::High); // 5 > -1
        assert_eq!(eq, Value::Low);
        assert_eq!(lt, Value::Low);
    }

    #[test]
    fn test_error_handling() {
        let comparator = Comparator::new(ComponentId(1));
        
        let (gt, eq, lt) = comparator.compare_values(
            &Value::Error,
            &Value::from_long(5, BusWidth(8))
        );
        assert_eq!(gt, Value::Error);
        assert_eq!(eq, Value::Error);
        assert_eq!(lt, Value::Error);
    }

    #[test]
    fn test_unknown_handling() {
        let comparator = Comparator::new(ComponentId(1));
        
        let (gt, eq, lt) = comparator.compare_values(
            &Value::Unknown,
            &Value::from_long(5, BusWidth(8))
        );
        assert_eq!(gt, Value::Unknown);
        assert_eq!(eq, Value::Unknown);
        assert_eq!(lt, Value::Unknown);
    }

    #[test]
    fn test_bit_width_change() {
        let mut comparator = Comparator::new(ComponentId(1));
        
        comparator.set_bit_width(BusWidth(16));
        assert_eq!(comparator.bit_width(), BusWidth(16));
        
        // Check that pin widths were updated
        assert_eq!(comparator.pins().get("A").unwrap().width(), BusWidth(16));
        assert_eq!(comparator.pins().get("B").unwrap().width(), BusWidth(16));
    }

    #[test]
    fn test_mode_change() {
        let mut comparator = Comparator::new(ComponentId(1));
        
        comparator.set_mode(ComparisonMode::Unsigned);
        assert_eq!(comparator.mode(), ComparisonMode::Unsigned);
    }
}