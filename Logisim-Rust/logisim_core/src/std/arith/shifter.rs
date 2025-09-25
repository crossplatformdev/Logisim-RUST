/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Shifter Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Shifter`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shift operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShiftType {
    /// Logical left shift (fill with zeros on the right)
    LogicalLeft,
    /// Logical right shift (fill with zeros on the left)
    LogicalRight,
    /// Arithmetic right shift (sign-extend on the left)
    ArithmeticRight,
    /// Roll left (circular shift left)
    RollLeft,
    /// Roll right (circular shift right)
    RollRight,
}

/// Multi-bit Shifter component
/// 
/// Performs various types of bit shifting operations:
/// - Logical shifts (fill with zeros)
/// - Arithmetic right shift (sign extension)
/// - Circular shifts (roll operations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shifter {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
    shift_type: ShiftType,
}

impl Shifter {
    /// Create a new 8-bit logical left shifter (default)
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_settings(id, BusWidth(8), ShiftType::LogicalLeft)
    }
    
    /// Create a new shifter with specified bit width and shift type
    pub fn new_with_settings(id: ComponentId, bit_width: BusWidth, shift_type: ShiftType) -> Self {
        let mut pins = HashMap::new();
        
        // Calculate the width needed for the shift amount
        // log2(bit_width) bits are needed to represent all possible shift amounts
        let shift_width = BusWidth(((bit_width.0 as f64).log2().ceil() as u32).max(1));
        
        // Input pins (using legacy pin names for compatibility)
        pins.insert("Input".to_string(), Pin::new_input("Input", bit_width));
        pins.insert("Shift".to_string(), Pin::new_input("Shift", shift_width));
        
        // Output pin
        pins.insert("Output".to_string(), Pin::new_output("Output", bit_width));
        
        Shifter { id, pins, bit_width, shift_type }
    }
    
    /// Perform the shift operation
    fn shift_value(&self, data: &Value, distance: &Value) -> Value {
        // Handle error and unknown cases
        if !data.is_fully_defined() || !distance.is_fully_defined() {
            if matches!(data, Value::Error) || matches!(distance, Value::Error) {
                return Value::Error;
            } else {
                return Value::Unknown;
            }
        }
        
        let data_val = data.to_long_value() as u64;
        let shift_amount = distance.to_long_value() as u32;
        let width = self.bit_width.0;
        
        // Create bit mask for the data width
        let mask = if width >= 64 { u64::MAX } else { (1u64 << width) - 1 };
        let masked_data = data_val & mask;
        
        let result = match self.shift_type {
            ShiftType::LogicalLeft => {
                if shift_amount >= width {
                    0 // All bits shifted out
                } else {
                    (masked_data << shift_amount) & mask
                }
            }
            ShiftType::LogicalRight => {
                if shift_amount >= width {
                    0 // All bits shifted out
                } else {
                    masked_data >> shift_amount
                }
            }
            ShiftType::ArithmeticRight => {
                if shift_amount >= width {
                    // Sign extend completely
                    let sign_bit = masked_data & (1u64 << (width - 1));
                    if sign_bit != 0 { mask } else { 0 }
                } else {
                    let sign_bit = masked_data & (1u64 << (width - 1));
                    let shifted = masked_data >> shift_amount;
                    
                    if sign_bit != 0 {
                        // Negative number - fill with 1s
                        let fill_mask = mask << (width - shift_amount);
                        shifted | (fill_mask & mask)
                    } else {
                        // Positive number - normal right shift
                        shifted
                    }
                }
            }
            ShiftType::RollLeft => {
                let effective_shift = shift_amount % width;
                if effective_shift == 0 {
                    masked_data
                } else {
                    let left_part = (masked_data << effective_shift) & mask;
                    let right_part = masked_data >> (width - effective_shift);
                    (left_part | right_part) & mask
                }
            }
            ShiftType::RollRight => {
                let effective_shift = shift_amount % width;
                if effective_shift == 0 {
                    masked_data
                } else {
                    let right_part = masked_data >> effective_shift;
                    let left_part = (masked_data << (width - effective_shift)) & mask;
                    (left_part | right_part) & mask
                }
            }
        };
        
        Value::from_long(result as i64, self.bit_width)
    }
    
    /// Get the current bit width
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
    
    /// Get the current shift type
    pub fn shift_type(&self) -> ShiftType {
        self.shift_type
    }
    
    /// Set the bit width (updates pin widths accordingly)
    pub fn set_bit_width(&mut self, width: BusWidth) {
        self.bit_width = width;
        let shift_width = BusWidth(((width.0 as f64).log2().ceil() as u32).max(1));
        
        if let Some(pin) = self.pins.get_mut("Input") {
            pin.width = width; pin.signal = Signal::unknown(width);
        }
        if let Some(pin) = self.pins.get_mut("Shift") {
            pin.set_width(shift_width);
        }
        if let Some(pin) = self.pins.get_mut("Output") {
            pin.width = width; pin.signal = Signal::unknown(width);
        }
    }
    
    /// Set the shift type
    pub fn set_shift_type(&mut self, shift_type: ShiftType) {
        self.shift_type = shift_type;
    }
}

impl Component for Shifter {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Shifter"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        // Get input values
        let data = self.pins.get("Input").map(|p| p.get_signal().value()).unwrap_or(&Value::Unknown);
        let distance = self.pins.get("Shift").map(|p| p.get_signal().value()).unwrap_or(&Value::Unknown);
        
        // Perform shift operation
        let output = self.shift_value(&data, &distance);
        
        // Update output pin
        let mut changed = false;
        if let Some(pin) = self.pins.get_mut("Output") {
            if pin.get_signal().value() != &output {
                let _ = pin.set_signal(Signal::new(output, current_time));
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
    fn test_shifter_creation() {
        let shifter = Shifter::new(ComponentId(1));
        assert_eq!(shifter.id(), ComponentId(1));
        assert_eq!(shifter.name(), "Shifter");
        assert_eq!(shifter.bit_width(), BusWidth(8));
        assert_eq!(shifter.shift_type(), ShiftType::LogicalLeft);
        assert_eq!(shifter.pins().len(), 3);
    }

    #[test]
    fn test_logical_left_shift() {
        let shifter = Shifter::new_with_settings(ComponentId(1), BusWidth(8), ShiftType::LogicalLeft);
        
        // 0b10110101 << 2 = 0b01010100 (181 << 2 = 84 when masked to 8 bits)
        let result = shifter.shift_value(
            &Value::from_long(0b10110101, BusWidth(8)),
            &Value::from_long(2, BusWidth(3))
        );
        
        assert_eq!(result.to_long_value(), 0b01010100);
    }

    #[test]
    fn test_logical_right_shift() {
        let shifter = Shifter::new_with_settings(ComponentId(1), BusWidth(8), ShiftType::LogicalRight);
        
        // 0b10110101 >> 2 = 0b00101101 (181 >> 2 = 45)
        let result = shifter.shift_value(
            &Value::from_long(0b10110101, BusWidth(8)),
            &Value::from_long(2, BusWidth(3))
        );
        
        assert_eq!(result.to_long_value(), 0b00101101);
    }

    #[test]
    fn test_arithmetic_right_shift_negative() {
        let shifter = Shifter::new_with_settings(ComponentId(1), BusWidth(8), ShiftType::ArithmeticRight);
        
        // 0b10110101 >> 2 = 0b11101101 (negative number, sign extended)
        let result = shifter.shift_value(
            &Value::from_long(0b10110101, BusWidth(8)),
            &Value::from_long(2, BusWidth(3))
        );
        
        assert_eq!(result.to_long_value(), 0b11101101);
    }

    #[test]
    fn test_roll_left() {
        let shifter = Shifter::new_with_settings(ComponentId(1), BusWidth(8), ShiftType::RollLeft);
        
        // 0b10110101 roll left 2 = 0b11010110
        let result = shifter.shift_value(
            &Value::from_long(0b10110101, BusWidth(8)),
            &Value::from_long(2, BusWidth(3))
        );
        
        assert_eq!(result.to_long_value(), 0b11010110);
    }

    #[test]
    fn test_roll_right() {
        let shifter = Shifter::new_with_settings(ComponentId(1), BusWidth(8), ShiftType::RollRight);
        
        // 0b10110101 roll right 2 = 0b01101101
        let result = shifter.shift_value(
            &Value::from_long(0b10110101, BusWidth(8)),
            &Value::from_long(2, BusWidth(3))
        );
        
        assert_eq!(result.to_long_value(), 0b01101101);
    }

    #[test]
    fn test_component_reset() {
        let mut shifter = Shifter::new(ComponentId(1));
        shifter.reset();
        // Should not panic and should reset all pins
        assert_eq!(shifter.pins().len(), 3);
    }
}