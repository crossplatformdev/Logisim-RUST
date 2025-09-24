/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! BitWidth - represents the width of a data bus in bits
//!
//! Rust port of BitWidth.java (enhanced version of existing BusWidth)

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Maximum width for values - must match Java implementation
pub const MAX_WIDTH: u32 = 64;
/// Minimum width for values
pub const MIN_WIDTH: u32 = 1;

/// Represents the width of a bus in bits
///
/// This extends the existing BusWidth concept with full Java compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BitWidth(u32);

impl BitWidth {
    /// The unknown width (0 bits)
    pub const UNKNOWN: BitWidth = BitWidth(0);

    /// Single bit width
    pub const ONE: BitWidth = BitWidth(1);

    /// Create a new BitWidth
    pub fn create(width: u32) -> Result<BitWidth, String> {
        if width > MAX_WIDTH {
            Err(format!("width {} must be at most {}", width, MAX_WIDTH))
        } else {
            Ok(BitWidth(width))
        }
    }

    /// Create a new BitWidth, panicking on invalid width
    pub fn new(width: u32) -> BitWidth {
        Self::create(width).expect("Invalid bit width")
    }

    /// Get the width as a u32
    pub fn get_width(self) -> u32 {
        self.0
    }

    /// Check if this is a single-bit signal
    pub fn is_single_bit(self) -> bool {
        self.0 == 1
    }

    /// Check if this is unknown width
    pub fn is_unknown(self) -> bool {
        self.0 == 0
    }

    /// Get the mask for this bit width
    pub fn get_mask(self) -> u64 {
        if self.0 == 0 {
            0
        } else if self.0 == MAX_WIDTH {
            u64::MAX
        } else {
            (1u64 << self.0) - 1
        }
    }

    /// Parse a BitWidth from a string
    /// Handles formats like "8", "/8", etc.
    pub fn parse(s: &str) -> Result<BitWidth, String> {
        if s.is_empty() {
            return Err("Width string cannot be empty".to_string());
        }

        let trimmed = if s.starts_with('/') { &s[1..] } else { s };

        match trimmed.parse::<u32>() {
            Ok(width) => Self::create(width),
            Err(_) => Err(format!("Invalid width format: '{}'", s)),
        }
    }

    /// Common bit widths for convenience
    pub const fn bit_width_1() -> BitWidth {
        BitWidth(1)
    }
    pub const fn bit_width_2() -> BitWidth {
        BitWidth(2)
    }
    pub const fn bit_width_3() -> BitWidth {
        BitWidth(3)
    }
    pub const fn bit_width_4() -> BitWidth {
        BitWidth(4)
    }
    pub const fn bit_width_5() -> BitWidth {
        BitWidth(5)
    }
    pub const fn bit_width_6() -> BitWidth {
        BitWidth(6)
    }
    pub const fn bit_width_7() -> BitWidth {
        BitWidth(7)
    }
    pub const fn bit_width_8() -> BitWidth {
        BitWidth(8)
    }
    pub const fn bit_width_16() -> BitWidth {
        BitWidth(16)
    }
    pub const fn bit_width_24() -> BitWidth {
        BitWidth(24)
    }
    pub const fn bit_width_32() -> BitWidth {
        BitWidth(32)
    }
    pub const fn bit_width_64() -> BitWidth {
        BitWidth(64)
    }

    /// Get common bit widths for UI dropdowns
    pub fn get_common_widths() -> Vec<BitWidth> {
        vec![
            Self::bit_width_1(),
            Self::bit_width_2(),
            Self::bit_width_3(),
            Self::bit_width_4(),
            Self::bit_width_5(),
            Self::bit_width_6(),
            Self::bit_width_7(),
            Self::bit_width_8(),
            Self::bit_width_16(),
            Self::bit_width_24(),
            Self::bit_width_32(),
            Self::bit_width_64(),
        ]
    }

    /// Get all widths in a range for UI dropdowns
    pub fn get_range_widths(min: u32, max: u32) -> Vec<BitWidth> {
        let max = max.min(MAX_WIDTH);
        let min = min.max(MIN_WIDTH);

        if max - min > 12 {
            // Too many options for dropdown, return common ones in range
            Self::get_common_widths()
                .into_iter()
                .filter(|w| w.0 >= min && w.0 <= max)
                .collect()
        } else {
            (min..=max).map(|w| BitWidth(w)).collect()
        }
    }
}

impl Default for BitWidth {
    fn default() -> Self {
        Self::ONE
    }
}

impl From<u32> for BitWidth {
    fn from(width: u32) -> Self {
        Self::new(width)
    }
}

impl From<BitWidth> for u32 {
    fn from(width: BitWidth) -> u32 {
        width.0
    }
}

impl FromStr for BitWidth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl fmt::Display for BitWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u32> for BitWidth {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for BitWidth {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

// Conversion compatibility with existing BusWidth
impl From<crate::signal::BusWidth> for BitWidth {
    fn from(bus_width: crate::signal::BusWidth) -> Self {
        BitWidth(bus_width.as_u32())
    }
}

impl From<BitWidth> for crate::signal::BusWidth {
    fn from(bit_width: BitWidth) -> Self {
        crate::signal::BusWidth::new(bit_width.0)
    }
}

/// Attribute type for BitWidth selection in UI
#[derive(Debug, Clone)]
pub struct BitWidthAttribute {
    name: String,
    display_name: String,
    min_width: u32,
    max_width: u32,
    choices: Option<Vec<BitWidth>>,
}

impl BitWidthAttribute {
    /// Create a new BitWidth attribute with default choices
    pub fn new(name: String, display_name: String) -> Self {
        Self {
            name,
            display_name,
            min_width: MIN_WIDTH,
            max_width: MAX_WIDTH,
            choices: Some(BitWidth::get_common_widths()),
        }
    }

    /// Create a new BitWidth attribute with custom range
    pub fn new_with_range(name: String, display_name: String, min: u32, max: u32) -> Self {
        let choices = if max - min <= 12 {
            Some(BitWidth::get_range_widths(min, max))
        } else {
            None // Use text editor for large ranges
        };

        Self {
            name,
            display_name,
            min_width: min,
            max_width: max,
            choices,
        }
    }

    /// Get the available choices for this attribute
    pub fn get_choices(&self) -> Option<&[BitWidth]> {
        self.choices.as_deref()
    }

    /// Parse a value for this attribute
    pub fn parse(&self, value: &str) -> Result<BitWidth, String> {
        let width = BitWidth::parse(value)?;

        if width.0 < self.min_width {
            return Err(format!("bit width must be at least {}", self.min_width));
        }

        if width.0 > self.max_width {
            return Err(format!("bit width must be at most {}", self.max_width));
        }

        Ok(width)
    }

    /// Get the name of this attribute
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the display name of this attribute
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_width_creation() {
        let width = BitWidth::new(8);
        assert_eq!(width.get_width(), 8);
        assert!(!width.is_single_bit());
        assert!(!width.is_unknown());
    }

    #[test]
    fn test_bit_width_constants() {
        assert_eq!(BitWidth::UNKNOWN.get_width(), 0);
        assert_eq!(BitWidth::ONE.get_width(), 1);

        assert!(BitWidth::UNKNOWN.is_unknown());
        assert!(BitWidth::ONE.is_single_bit());
    }

    #[test]
    fn test_bit_width_create() {
        assert!(BitWidth::create(32).is_ok());
        assert!(BitWidth::create(MAX_WIDTH).is_ok());
        assert!(BitWidth::create(MAX_WIDTH + 1).is_err());
    }

    #[test]
    fn test_bit_width_mask() {
        assert_eq!(BitWidth::new(1).get_mask(), 0x1);
        assert_eq!(BitWidth::new(8).get_mask(), 0xFF);
        assert_eq!(BitWidth::new(16).get_mask(), 0xFFFF);
        assert_eq!(BitWidth::new(32).get_mask(), 0xFFFFFFFF);
        assert_eq!(BitWidth::new(64).get_mask(), 0xFFFFFFFFFFFFFFFF);
        assert_eq!(BitWidth::UNKNOWN.get_mask(), 0);
    }

    #[test]
    fn test_bit_width_parse() {
        assert_eq!(BitWidth::parse("8").unwrap().get_width(), 8);
        assert_eq!(BitWidth::parse("/16").unwrap().get_width(), 16);
        assert!(BitWidth::parse("").is_err());
        assert!(BitWidth::parse("invalid").is_err());
        assert!(BitWidth::parse("999").is_err());
    }

    #[test]
    fn test_bit_width_from_str() {
        assert_eq!("8".parse::<BitWidth>().unwrap().get_width(), 8);
        assert!("invalid".parse::<BitWidth>().is_err());
    }

    #[test]
    fn test_bit_width_display() {
        let width = BitWidth::new(8);
        assert_eq!(width.to_string(), "8");
    }

    #[test]
    fn test_bit_width_conversions() {
        let width = BitWidth::new(8);
        let as_u32: u32 = width.into();
        assert_eq!(as_u32, 8);

        let from_u32 = BitWidth::from(16u32);
        assert_eq!(from_u32.get_width(), 16);
    }

    #[test]
    fn test_bit_width_comparisons() {
        let width8 = BitWidth::new(8);
        let width16 = BitWidth::new(16);

        assert!(width8 < width16);
        assert!(width8 == 8u32);
        assert!(width8 < 16u32);
    }

    #[test]
    fn test_bit_width_common_widths() {
        let common = BitWidth::get_common_widths();
        assert!(!common.is_empty());
        assert!(common.contains(&BitWidth::ONE));
        assert!(common.contains(&BitWidth::bit_width_8()));
        assert!(common.contains(&BitWidth::bit_width_32()));
    }

    #[test]
    fn test_bit_width_range_widths() {
        let range = BitWidth::get_range_widths(1, 4);
        assert_eq!(range.len(), 4);
        assert_eq!(range[0].get_width(), 1);
        assert_eq!(range[3].get_width(), 4);

        // Large range should return filtered common widths
        let large_range = BitWidth::get_range_widths(1, 64);
        assert!(!large_range.is_empty());
        assert!(large_range.len() <= 12);
    }

    #[test]
    fn test_bus_width_compatibility() {
        let bus_width = crate::signal::BusWidth::new(8);
        let bit_width: BitWidth = bus_width.into();
        assert_eq!(bit_width.get_width(), 8);

        let back_to_bus: crate::signal::BusWidth = bit_width.into();
        assert_eq!(back_to_bus.as_u32(), 8);
    }

    #[test]
    fn test_bit_width_attribute() {
        let attr = BitWidthAttribute::new("width".to_string(), "Data Width".to_string());

        assert_eq!(attr.get_name(), "width");
        assert_eq!(attr.get_display_name(), "Data Width");
        assert!(attr.get_choices().is_some());

        let parsed = attr.parse("8").unwrap();
        assert_eq!(parsed.get_width(), 8);

        assert!(attr.parse("999").is_err());
    }

    #[test]
    fn test_bit_width_attribute_range() {
        let attr =
            BitWidthAttribute::new_with_range("width".to_string(), "Width".to_string(), 2, 8);

        assert!(attr.parse("1").is_err()); // Below min
        assert!(attr.parse("9").is_err()); // Above max
        assert!(attr.parse("4").is_ok()); // In range

        let choices = attr.get_choices().unwrap();
        assert_eq!(choices.len(), 7); // 2, 3, 4, 5, 6, 7, 8
    }
}
