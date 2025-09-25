/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Divider Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Divider`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multi-bit Divider component
/// 
/// Performs integer division of two n-bit inputs.
/// Produces both quotient and remainder outputs.
/// Division by zero produces error outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divider {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
}

impl Divider {
    /// Create a new 8-bit divider (default width)
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, BusWidth(8))
    }
    
    /// Create a new divider with specified bit width
    pub fn new_with_width(id: ComponentId, bit_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        
        // Input pins
        pins.insert("Dividend".to_string(), Pin::new_input("Dividend", bit_width));
        pins.insert("Divisor".to_string(), Pin::new_input("Divisor", bit_width));
        
        // Output pins (same width as inputs)
        pins.insert("Quotient".to_string(), Pin::new_output("Quotient", bit_width));
        pins.insert("Remainder".to_string(), Pin::new_output("Remainder", bit_width));
        
        Divider { id, pins, bit_width }
    }
    
    /// Get the current bit width
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
    
    /// Set the bit width (updates pin widths accordingly)
    pub fn set_bit_width(&mut self, width: BusWidth) {
        self.bit_width = width;
        
        for pin_name in &["Dividend", "Divisor", "Quotient", "Remainder"] {
            if let Some(pin) = self.pins.get_mut(*pin_name) {
                pin.width = width; pin.signal = Signal::unknown(width);
            }
        }
    }
}

impl Component for Divider {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Divider"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        // Get input values
        let dividend = self.pins.get("Dividend").map(|p| p.get_signal().value()).unwrap_or(Value::Unknown);
        let divisor = self.pins.get("Divisor").map(|p| p.get_signal().value()).unwrap_or(Value::Unknown);
        
        // Perform division
        let (quotient, remainder) = if dividend.is_fully_defined() && divisor.is_fully_defined() {
            let divisor_val = divisor.to_long_value();
            if divisor_val == 0 {
                // Division by zero
                (Value::Error, Value::Error)
            } else {
                let dividend_val = dividend.to_long_value();
                let div_result = dividend_val / divisor_val;
                let rem_result = dividend_val % divisor_val;
                (
                    Value::from_long(div_result, self.bit_width),
                    Value::from_long(rem_result, self.bit_width)
                )
            }
        } else {
            // Handle error/unknown inputs
            let error_val = if matches!(dividend, Value::Error) || matches!(divisor, Value::Error) {
                Value::Error
            } else {
                Value::Unknown
            };
            (error_val, error_val)
        };
        
        // Update output pins
        let mut changed = false;
        
        if let Some(pin) = self.pins.get_mut("Quotient") {
            if pin.get_signal().value() != &quotient {
                let _ = pin.set_signal(Signal::new(quotient, current_time));
                changed = true;
            }
        }
        
        if let Some(pin) = self.pins.get_mut("Remainder") {
            if pin.get_signal().value() != &remainder {
                let _ = pin.set_signal(Signal::new(remainder, current_time));
                changed = true;
            }
        }
        
        if changed {
            UpdateResult::changed()
        } else {
            UpdateResult::no_change()
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
    fn test_divider_creation() {
        let divider = Divider::new(ComponentId(1));
        assert_eq!(divider.id(), ComponentId(1));
        assert_eq!(divider.name(), "Divider");
        assert_eq!(divider.bit_width(), BusWidth(8));
        assert_eq!(divider.pins().len(), 4);
    }

    #[test]
    fn test_divider_with_custom_width() {
        let divider = Divider::new_with_width(ComponentId(1), BusWidth(16));
        assert_eq!(divider.bit_width(), BusWidth(16));
    }

    #[test]
    fn test_bit_width_change() {
        let mut divider = Divider::new(ComponentId(1));
        
        divider.set_bit_width(BusWidth(16));
        assert_eq!(divider.bit_width(), BusWidth(16));
        
        // Check that all pin widths were updated
        for pin_name in &["Dividend", "Divisor", "Quotient", "Remainder"] {
            assert_eq!(divider.pins().get(*pin_name).unwrap().width(), BusWidth(16));
        }
    }

    #[test]
    fn test_component_reset() {
        let mut divider = Divider::new(ComponentId(1));
        divider.reset();
        // Should not panic and should reset all pins
        assert_eq!(divider.pins().len(), 4);
    }
}