/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Gray code incrementer component
//!
//! This component takes a multibit input and outputs the value that follows it in Gray Code.
//! For instance, given input 0100 the output is 1100.
//! Equivalent to Java's com.cburch.gray.GrayIncrementer class.

use super::ComponentTool;
use crate::signal::BusWidth;

/// Gray code incrementer component.
///
/// This component takes a multibit input and outputs the value that follows it in Gray Code.
/// This is equivalent to Java's GrayIncrementer class.
pub struct GrayIncrementer;

impl GrayIncrementer {
    /// Unique identifier of the tool, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "Gray Code Incrementer";

    /// Creates a new Gray code incrementer.
    pub fn new() -> Self {
        Self
    }

    /// Computes the next gray value in the sequence after prev.
    ///
    /// This static method implements the Gray code increment algorithm.
    /// It converts from Gray code to binary, increments, then converts back.
    pub fn next_gray(prev: u64, width: BusWidth) -> u64 {
        let mask = width.get_mask();

        // Convert Gray code to Binary
        let mut binary = prev & mask;
        let mut temp = binary;

        // Gray to Binary conversion
        let mut shift = 1;
        while temp != 0 {
            temp >>= 1;
            binary ^= temp;
            shift += 1;
            if shift >= 64 {
                break;
            }
        }

        // Increment the binary value
        binary = (binary + 1) & mask;

        // Convert back to Gray code
        Self::binary_to_gray(binary, width)
    }

    /// Convert binary to Gray code
    pub fn binary_to_gray(binary: u64, width: BusWidth) -> u64 {
        let mask = width.get_mask();
        ((binary >> 1) ^ binary) & mask
    }

    /// Convert Gray code to binary
    pub fn gray_to_binary(gray: u64, width: BusWidth) -> u64 {
        let mask = width.get_mask();
        let mut binary = gray & mask;
        let mut temp = binary;

        let mut shift = 1;
        while temp != 0 {
            temp >>= 1;
            binary ^= temp;
            shift += 1;
            if shift >= 64 {
                break;
            }
        }

        binary & mask
    }

    /// Get the standard Gray code sequence for a given width
    pub fn get_gray_sequence(width: BusWidth) -> Vec<u64> {
        let count = 1u64 << width.as_u32().min(16); // Limit to 16 bits for practical sequences
        let mut sequence = Vec::with_capacity(count as usize);

        for i in 0..count {
            sequence.push(Self::binary_to_gray(i, width));
        }

        sequence
    }
}

impl ComponentTool for GrayIncrementer {
    fn get_name(&self) -> &str {
        Self::ID
    }

    fn get_display_name(&self) -> &str {
        "Gray Incrementer"
    }
}

impl Default for GrayIncrementer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_incrementer_creation() {
        let incrementer = GrayIncrementer::new();
        assert_eq!(incrementer.get_name(), "Gray Code Incrementer");
        assert_eq!(incrementer.get_display_name(), "Gray Incrementer");
    }

    #[test]
    fn test_binary_to_gray_conversion() {
        let width = BusWidth::new(4);

        // Test known conversions
        assert_eq!(GrayIncrementer::binary_to_gray(0b0000, width), 0b0000);
        assert_eq!(GrayIncrementer::binary_to_gray(0b0001, width), 0b0001);
        assert_eq!(GrayIncrementer::binary_to_gray(0b0010, width), 0b0011);
        assert_eq!(GrayIncrementer::binary_to_gray(0b0011, width), 0b0010);
        assert_eq!(GrayIncrementer::binary_to_gray(0b0100, width), 0b0110);
    }

    #[test]
    fn test_gray_to_binary_conversion() {
        let width = BusWidth::new(4);

        // Test known conversions (reverse of binary to gray)
        assert_eq!(GrayIncrementer::gray_to_binary(0b0000, width), 0b0000);
        assert_eq!(GrayIncrementer::gray_to_binary(0b0001, width), 0b0001);
        assert_eq!(GrayIncrementer::gray_to_binary(0b0011, width), 0b0010);
        assert_eq!(GrayIncrementer::gray_to_binary(0b0010, width), 0b0011);
        assert_eq!(GrayIncrementer::gray_to_binary(0b0110, width), 0b0100);
    }

    #[test]
    fn test_next_gray_logic() {
        let width = BusWidth::new(4);

        // Test basic Gray code progression
        assert_eq!(GrayIncrementer::next_gray(0b0000, width), 0b0001);
        assert_eq!(GrayIncrementer::next_gray(0b0001, width), 0b0011);
        assert_eq!(GrayIncrementer::next_gray(0b0011, width), 0b0010);
        assert_eq!(GrayIncrementer::next_gray(0b0010, width), 0b0110);
        assert_eq!(GrayIncrementer::next_gray(0b0110, width), 0b0111);
    }

    #[test]
    fn test_gray_sequence() {
        let width = BusWidth::new(3);
        let sequence = GrayIncrementer::get_gray_sequence(width);

        // 3-bit Gray sequence should be: 000, 001, 011, 010, 110, 111, 101, 100
        let expected = vec![0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100];
        assert_eq!(sequence, expected);
    }

    #[test]
    fn test_roundtrip_conversion() {
        let width = BusWidth::new(4);

        // Test that binary -> gray -> binary gives original value
        for i in 0..16 {
            let gray = GrayIncrementer::binary_to_gray(i, width);
            let binary = GrayIncrementer::gray_to_binary(gray, width);
            assert_eq!(binary, i, "Roundtrip failed for value {}", i);
        }
    }

    #[test]
    fn test_default_implementation() {
        let incrementer = GrayIncrementer::default();
        assert_eq!(incrementer.get_name(), "Gray Code Incrementer");
    }
}
