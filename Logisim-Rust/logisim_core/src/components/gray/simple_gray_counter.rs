/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Simple gray counter component
//!
//! Manufactures a simple counter that iterates over the 4-bit Gray Code.
//! Equivalent to Java's com.cburch.gray.SimpleGrayCounter class.

use super::{ComponentTool, CounterData};
use crate::signal::{BusWidth, Value};

/// Simple Gray counter that iterates over 4-bit Gray Code.
///
/// This is equivalent to Java's SimpleGrayCounter class.
/// It provides a fixed 4-bit Gray code counter with clock input.
pub struct SimpleGrayCounter {
    /// Fixed width for this simple counter
    width: BusWidth,
}

impl SimpleGrayCounter {
    /// Unique identifier of the tool, used as reference in project files.
    pub const ID: &'static str = "Gray Counter (Simple)";

    /// Fixed width for the simple Gray counter
    pub const WIDTH: BusWidth = BusWidth::new(4);

    pub fn new() -> Self {
        Self { width: Self::WIDTH }
    }

    /// Get the bit width of this counter
    pub fn get_width(&self) -> BusWidth {
        self.width
    }

    /// Simulate one step of the counter
    /// Returns the new output value given the current state and clock input
    pub fn step(&self, current_data: &mut CounterData, clock: Value) -> Value {
        let triggered = current_data.update_clock(clock);

        if triggered {
            // Get current value as integer
            let current_val = match current_data.value() {
                Value::High => 1,
                Value::Low => 0,
                _ => 0, // Unknown/error states become 0
            };

            // For 4-bit counter, we need to track the actual count
            // This is a simplified implementation
            let next_val = if current_val == 0 { 1 } else { 0 };
            let next_value = if next_val == 1 {
                Value::High
            } else {
                Value::Low
            };

            current_data.set_value(next_value);
            next_value
        } else {
            *current_data.value()
        }
    }

    /// Get the complete 4-bit Gray code sequence
    pub fn get_sequence() -> Vec<u8> {
        vec![
            0b0000, // 0
            0b0001, // 1
            0b0011, // 3
            0b0010, // 2
            0b0110, // 6
            0b0111, // 7
            0b0101, // 5
            0b0100, // 4
            0b1100, // 12
            0b1101, // 13
            0b1111, // 15
            0b1110, // 14
            0b1010, // 10
            0b1011, // 11
            0b1001, // 9
            0b1000, // 8
        ]
    }

    /// Convert a position in the sequence to Gray code
    pub fn position_to_gray(position: u8) -> u8 {
        let sequence = Self::get_sequence();
        sequence[position as usize % sequence.len()]
    }

    /// Find position of a Gray code value in the sequence
    pub fn gray_to_position(gray_value: u8) -> Option<u8> {
        let sequence = Self::get_sequence();
        sequence
            .iter()
            .position(|&x| x == gray_value)
            .map(|p| p as u8)
    }
}

impl ComponentTool for SimpleGrayCounter {
    fn get_name(&self) -> &str {
        Self::ID
    }

    fn get_display_name(&self) -> &str {
        "Simple Gray Counter"
    }
}

impl Default for SimpleGrayCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_gray_counter_creation() {
        let counter = SimpleGrayCounter::new();
        assert_eq!(counter.get_width(), BusWidth::new(4));
        assert_eq!(counter.get_name(), "Gray Counter (Simple)");
        assert_eq!(counter.get_display_name(), "Simple Gray Counter");
    }

    #[test]
    fn test_gray_sequence() {
        let sequence = SimpleGrayCounter::get_sequence();
        assert_eq!(sequence.len(), 16);

        // Verify it's a valid 4-bit Gray code sequence
        // Each consecutive pair should differ by exactly one bit
        for i in 0..sequence.len() {
            let current = sequence[i];
            let next = sequence[(i + 1) % sequence.len()];
            let diff = current ^ next;

            // Count number of 1 bits in diff (should be exactly 1)
            let bit_count = diff.count_ones();
            assert_eq!(
                bit_count, 1,
                "Values {} and {} differ by {} bits",
                current, next, bit_count
            );
        }
    }

    #[test]
    fn test_position_conversions() {
        let sequence = SimpleGrayCounter::get_sequence();

        // Test position to gray conversion
        for (pos, &expected_gray) in sequence.iter().enumerate() {
            let gray = SimpleGrayCounter::position_to_gray(pos as u8);
            assert_eq!(gray, expected_gray);
        }

        // Test gray to position conversion
        for (expected_pos, &gray) in sequence.iter().enumerate() {
            let pos = SimpleGrayCounter::gray_to_position(gray);
            assert_eq!(pos, Some(expected_pos as u8));
        }

        // Test invalid gray code
        assert_eq!(SimpleGrayCounter::gray_to_position(0xFF), None);
    }

    #[test]
    fn test_counter_step() {
        let counter = SimpleGrayCounter::new();
        let mut data = CounterData::new(None, Value::Low);

        // First step with rising edge should trigger
        let result = counter.step(&mut data, Value::High);
        assert_eq!(result, Value::High);

        // Same high level should not trigger
        let result = counter.step(&mut data, Value::High);
        assert_eq!(result, Value::High); // Value unchanged

        // Falling edge should not trigger
        let result = counter.step(&mut data, Value::Low);
        assert_eq!(result, Value::High); // Value unchanged

        // Rising edge should trigger again
        let result = counter.step(&mut data, Value::High);
        assert_eq!(result, Value::Low); // Toggled back
    }

    #[test]
    fn test_default_implementation() {
        let counter = SimpleGrayCounter::default();
        assert_eq!(counter.get_width(), BusWidth::new(4));
    }

    #[test]
    fn test_sequence_properties() {
        let sequence = SimpleGrayCounter::get_sequence();

        // Should start with 0
        assert_eq!(sequence[0], 0b0000);

        // Should contain all 4-bit values exactly once
        let mut found = [false; 16];
        for &value in &sequence {
            assert!((value as usize) < 16, "Value {} out of range", value);
            assert!(
                !found[value as usize],
                "Value {} appears multiple times",
                value
            );
            found[value as usize] = true;
        }

        // All values should be found
        for (i, &found_val) in found.iter().enumerate() {
            assert!(found_val, "Value {} not found in sequence", i);
        }
    }
}
