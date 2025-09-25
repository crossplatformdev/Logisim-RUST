/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Negator Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Negator`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multi-bit Negator component
/// 
/// Performs two's complement negation on its input.
/// Output = -Input = (~Input) + 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Negator {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
}

impl Negator {
    /// Create a new 8-bit negator (default width)
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, BusWidth(8))
    }
    
    /// Create a new negator with specified bit width
    pub fn new_with_width(id: ComponentId, bit_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Input".to_string(), Pin::new_input("Input", bit_width));
        pins.insert("Output".to_string(), Pin::new_output("Output", bit_width));
        
        Negator { id, pins, bit_width }
    }
    
    /// Perform two's complement negation
    fn negate_value(&self, value: &Value) -> Value {
        if !value.is_fully_defined() {
            return *value; // Pass through errors and unknowns
        }
        
        let input_val = value.to_long_value();
        
        // Perform two's complement negation: ~input + 1
        let width = self.bit_width.0;
        if width >= 64 {
            // For 64-bit, use wrapping arithmetic
            let negated = input_val.wrapping_neg();
            Value::from_long(negated, self.bit_width)
        } else {
            // For smaller widths, mask to prevent overflow
            let mask = (1u64 << width) - 1;
            let masked_input = (input_val as u64) & mask;
            let negated = ((!masked_input).wrapping_add(1)) & mask;
            Value::from_long(negated as i64, self.bit_width)
        }
    }
    
    /// Get the current bit width
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
    
    /// Set the bit width (updates pin widths accordingly)
    pub fn set_bit_width(&mut self, width: BusWidth) {
        self.bit_width = width;
        if let Some(pin) = self.pins.get_mut("Input") {
            pin.width = width; pin.signal = Signal::unknown(width);
        }
        if let Some(pin) = self.pins.get_mut("Output") {
            pin.width = width; pin.signal = Signal::unknown(width);
        }
    }
}

impl Component for Negator {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Negator"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        // Get input value
        let input = self.pins.get("Input").map(|p| p.get_signal().value()).unwrap_or(Value::Unknown);
        
        // Compute negated output
        let output = self.negate_value(&input);
        
        // Update output pin
        let mut changed = false;
        if let Some(pin) = self.pins.get_mut("Output") {
            if pin.get_signal().value() != &output {
                let _ = pin.set_signal(Signal::new(output, current_time));
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
    fn test_negator_creation() {
        let negator = Negator::new(ComponentId(1));
        assert_eq!(negator.id(), ComponentId(1));
        assert_eq!(negator.name(), "Negator");
        assert_eq!(negator.bit_width(), BusWidth(8));
        assert_eq!(negator.pins().len(), 2);
    }

    #[test]
    fn test_negator_with_custom_width() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(16));
        assert_eq!(negator.bit_width(), BusWidth(16));
    }

    #[test]
    fn test_negate_positive() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(8));
        
        // Test negating 5 -> should become -5 (251 in 8-bit two's complement)
        let result = negator.negate_value(&Value::from_long(5, BusWidth(8)));
        assert_eq!(result.to_long_value(), 251); // 256 - 5 = 251
    }

    #[test]
    fn test_negate_negative() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(8));
        
        // Test negating -5 (251 in 8-bit two's complement) -> should become 5
        let result = negator.negate_value(&Value::from_long(251, BusWidth(8)));
        assert_eq!(result.to_long_value(), 5);
    }

    #[test]
    fn test_negate_zero() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(8));
        
        // Test negating 0 -> should remain 0
        let result = negator.negate_value(&Value::from_long(0, BusWidth(8)));
        assert_eq!(result.to_long_value(), 0);
    }

    #[test]
    fn test_negate_max_positive() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(8));
        
        // Test negating 127 (max positive 8-bit) -> should become -127 (129)
        let result = negator.negate_value(&Value::from_long(127, BusWidth(8)));
        assert_eq!(result.to_long_value(), 129); // 256 - 127 = 129
    }

    #[test]
    fn test_negate_min_negative() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(8));
        
        // Test negating -128 (128 in 8-bit two's complement) -> should become -128 (overflow case)
        let result = negator.negate_value(&Value::from_long(128, BusWidth(8)));
        assert_eq!(result.to_long_value(), 128); // -(-128) overflows back to -128
    }

    #[test]
    fn test_error_passthrough() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(8));
        
        let result = negator.negate_value(&Value::Error);
        assert_eq!(result, Value::Error);
    }

    #[test]
    fn test_unknown_passthrough() {
        let negator = Negator::new_with_width(ComponentId(1), BusWidth(8));
        
        let result = negator.negate_value(&Value::Unknown);
        assert_eq!(result, Value::Unknown);
    }

    #[test]
    fn test_bit_width_change() {
        let mut negator = Negator::new(ComponentId(1));
        
        negator.set_bit_width(BusWidth(16));
        assert_eq!(negator.bit_width(), BusWidth(16));
        
        // Check that pin widths were updated
        assert_eq!(negator.pins().get("Input").unwrap().width(), BusWidth(16));
        assert_eq!(negator.pins().get("Output").unwrap().width(), BusWidth(16));
    }

    #[test]
    fn test_component_reset() {
        let mut negator = Negator::new(ComponentId(1));
        negator.reset();
        // Should not panic and should reset all pins
        assert_eq!(negator.pins().len(), 2);
    }
}