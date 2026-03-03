//! Signal values for digital logic simulation.
//!
//! Logisim-Evolution uses a multi-valued logic system:
//! - `False` (0)
//! - `True` (1)
//! - `Unknown` (X) - uninitialised or indeterminate
//! - `Error` (E) - short circuit (multiple conflicting drivers)
//! - `HighZ` (Z) - high-impedance / undriven wire

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// A single-bit logic value using multi-valued logic.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize)]
pub enum Value {
    /// Logic 0 (driven low).
    False,
    /// Logic 1 (driven high).
    True,
    /// Unknown / uninitialised (X).
    #[default]
    Unknown,
    /// Error / short circuit (E).
    Error,
    /// High impedance / undriven (Z).
    HighZ,
}

impl Value {
    /// Returns `true` when the value is a definite logic 1.
    pub fn is_true(self) -> bool {
        self == Value::True
    }

    /// Returns `true` when the value is a definite logic 0.
    pub fn is_false(self) -> bool {
        self == Value::False
    }

    /// Returns `true` when the value is known (True or False).
    pub fn is_known(self) -> bool {
        matches!(self, Value::True | Value::False)
    }

    /// Returns `true` when the value is an error (short circuit).
    pub fn is_error(self) -> bool {
        self == Value::Error
    }

    /// Returns `true` when the value is high-impedance.
    pub fn is_high_z(self) -> bool {
        self == Value::HighZ
    }

    /// Resolve two drivers on the same wire (wired-OR-style resolution table).
    ///
    /// | self \ other | F | T | X | E | Z |
    /// |:---:|:---:|:---:|:---:|:---:|:---:|
    /// | F   | F | E | X | E | F |
    /// | T   | E | T | X | E | T |
    /// | X   | X | X | X | X | X |
    /// | E   | E | E | E | E | E |
    /// | Z   | F | T | X | E | Z |
    pub fn resolve(self, other: Value) -> Value {
        match (self, other) {
            // Same value → keep it
            (Value::False, Value::False) => Value::False,
            (Value::True, Value::True) => Value::True,
            (Value::Unknown, Value::Unknown) => Value::Unknown,
            (Value::Error, _) | (_, Value::Error) => Value::Error,
            // High-Z is transparent
            (Value::HighZ, v) | (v, Value::HighZ) => v,
            // True vs False → short circuit
            (Value::True, Value::False) | (Value::False, Value::True) => Value::Error,
            // Unknown with known/unknown → Unknown
            (Value::Unknown, _) | (_, Value::Unknown) => Value::Unknown,
        }
    }

    /// Numeric representation used in .circ file serialisation.
    pub fn to_char(self) -> char {
        match self {
            Value::False => '0',
            Value::True => '1',
            Value::Unknown => 'x',
            Value::Error => 'E',
            Value::HighZ => 'Z',
        }
    }

    /// Parse a character from a .circ file.
    pub fn from_char(c: char) -> Option<Value> {
        match c {
            '0' => Some(Value::False),
            '1' => Some(Value::True),
            'x' | 'X' | 'u' | 'U' => Some(Value::Unknown),
            'e' | 'E' => Some(Value::Error),
            'z' | 'Z' => Some(Value::HighZ),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Not for Value {
    type Output = Value;
    fn not(self) -> Value {
        match self {
            Value::False => Value::True,
            Value::True => Value::False,
            Value::Unknown => Value::Unknown,
            Value::Error => Value::Error,
            Value::HighZ => Value::Unknown,
        }
    }
}

impl BitAnd for Value {
    type Output = Value;
    fn bitand(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::False, _) | (_, Value::False) => Value::False,
            (Value::True, Value::True) => Value::True,
            (Value::Error, _) | (_, Value::Error) => Value::Error,
            _ => Value::Unknown,
        }
    }
}

impl BitOr for Value {
    type Output = Value;
    fn bitor(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::True, _) | (_, Value::True) => Value::True,
            (Value::False, Value::False) => Value::False,
            (Value::Error, _) | (_, Value::Error) => Value::Error,
            _ => Value::Unknown,
        }
    }
}

impl BitXor for Value {
    type Output = Value;
    fn bitxor(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::True, Value::False) | (Value::False, Value::True) => Value::True,
            (Value::True, Value::True) | (Value::False, Value::False) => Value::False,
            (Value::Error, _) | (_, Value::Error) => Value::Error,
            _ => Value::Unknown,
        }
    }
}

/// A multi-bit bus value, where each bit is independently a [`Value`].
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Bus {
    bits: Vec<Value>,
}

impl Bus {
    /// Create a new all-`Unknown` bus of the given width.
    pub fn unknown(width: usize) -> Self {
        Bus {
            bits: vec![Value::Unknown; width],
        }
    }

    /// Create a new all-`HighZ` bus of the given width (undriven / high-impedance).
    pub fn high_z(width: usize) -> Self {
        Bus {
            bits: vec![Value::HighZ; width],
        }
    }

    /// Create a bus from a single `Value` repeated across `width` bits.
    pub fn from_value(value: Value, width: usize) -> Self {
        Bus {
            bits: vec![value; width],
        }
    }

    /// Create a bus from an integer value (LSB first).
    pub fn from_u64(value: u64, width: usize) -> Self {
        let bits = (0..width)
            .map(|i| {
                if (value >> i) & 1 == 1 {
                    Value::True
                } else {
                    Value::False
                }
            })
            .collect();
        Bus { bits }
    }

    /// Convert to u64 if all bits are known; returns `None` on unknown/error bits.
    pub fn to_u64(&self) -> Option<u64> {
        let mut result = 0u64;
        for (i, &bit) in self.bits.iter().enumerate() {
            match bit {
                Value::True => result |= 1 << i,
                Value::False => {}
                _ => return None,
            }
        }
        Some(result)
    }

    /// Number of bits in this bus.
    pub fn width(&self) -> usize {
        self.bits.len()
    }

    /// Get a single bit.
    pub fn get(&self, index: usize) -> Value {
        self.bits.get(index).copied().unwrap_or(Value::Unknown)
    }

    /// Set a single bit.
    pub fn set(&mut self, index: usize, value: Value) {
        if index < self.bits.len() {
            self.bits[index] = value;
        }
    }

    /// Bitwise NOT.
    pub fn not(&self) -> Bus {
        Bus {
            bits: self.bits.iter().map(|&v| !v).collect(),
        }
    }

    /// Bitwise AND.
    pub fn and(&self, other: &Bus) -> Bus {
        let width = self.width().max(other.width());
        Bus {
            bits: (0..width).map(|i| self.get(i) & other.get(i)).collect(),
        }
    }

    /// Bitwise OR.
    pub fn or(&self, other: &Bus) -> Bus {
        let width = self.width().max(other.width());
        Bus {
            bits: (0..width).map(|i| self.get(i) | other.get(i)).collect(),
        }
    }

    /// Bitwise XOR.
    pub fn xor(&self, other: &Bus) -> Bus {
        let width = self.width().max(other.width());
        Bus {
            bits: (0..width).map(|i| self.get(i) ^ other.get(i)).collect(),
        }
    }

    /// Wire resolution across multiple drivers.
    pub fn resolve(&self, other: &Bus) -> Bus {
        let width = self.width().max(other.width());
        Bus {
            bits: (0..width)
                .map(|i| self.get(i).resolve(other.get(i)))
                .collect(),
        }
    }

    /// Returns `true` if all bits are `False`.
    pub fn is_all_false(&self) -> bool {
        self.bits.iter().all(|&v| v == Value::False)
    }

    /// Returns `true` if all bits are `True`.
    pub fn is_all_true(&self) -> bool {
        self.bits.iter().all(|&v| v == Value::True)
    }

    /// Returns `true` if any bit is `Error`.
    pub fn has_error(&self) -> bool {
        self.bits.contains(&Value::Error)
    }

    /// Returns `true` if all bits are known.
    pub fn is_fully_known(&self) -> bool {
        self.bits.iter().all(|&v| v.is_known())
    }

    /// Slice a sub-bus from `start` (inclusive) to `end` (exclusive).
    pub fn slice(&self, start: usize, end: usize) -> Bus {
        let bits = (start..end).map(|i| self.get(i)).collect();
        Bus { bits }
    }

    /// Concatenate two buses (other is the high bits).
    pub fn concat(&self, high: &Bus) -> Bus {
        let mut bits = self.bits.clone();
        bits.extend_from_slice(&high.bits);
        Bus { bits }
    }

    /// Format as a hex string for display (little-endian bit order).
    pub fn to_hex_string(&self) -> String {
        if let Some(v) = self.to_u64() {
            let nibbles = self.width().div_ceil(4);
            format!("{:0>width$X}", v, width = nibbles)
        } else {
            self.bits.iter().rev().map(|v| v.to_char()).collect()
        }
    }
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

/// Bit width of a port or signal.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, PartialOrd, Ord)]
pub struct BitWidth(pub u32);

impl BitWidth {
    pub const ONE: BitWidth = BitWidth(1);
    pub const TWO: BitWidth = BitWidth(2);
    pub const FOUR: BitWidth = BitWidth(4);
    pub const EIGHT: BitWidth = BitWidth(8);
    pub const SIXTEEN: BitWidth = BitWidth(16);
    pub const THIRTY_TWO: BitWidth = BitWidth(32);
    pub const SIXTY_FOUR: BitWidth = BitWidth(64);

    pub fn new(bits: u32) -> Self {
        assert!((1..=64).contains(&bits), "BitWidth must be 1–64");
        BitWidth(bits)
    }

    pub fn get(self) -> u32 {
        self.0
    }

    pub fn unknown_bus(self) -> Bus {
        Bus::unknown(self.0 as usize)
    }

    pub fn false_bus(self) -> Bus {
        Bus::from_u64(0, self.0 as usize)
    }
}

impl Default for BitWidth {
    fn default() -> Self {
        BitWidth::ONE
    }
}

impl fmt::Display for BitWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_not() {
        assert_eq!(!Value::True, Value::False);
        assert_eq!(!Value::False, Value::True);
        assert_eq!(!Value::Unknown, Value::Unknown);
        assert_eq!(!Value::Error, Value::Error);
        assert_eq!(!Value::HighZ, Value::Unknown);
    }

    #[test]
    fn test_value_and() {
        assert_eq!(Value::True & Value::True, Value::True);
        assert_eq!(Value::True & Value::False, Value::False);
        assert_eq!(Value::False & Value::Unknown, Value::False);
        assert_eq!(Value::True & Value::Unknown, Value::Unknown);
        assert_eq!(Value::Error & Value::True, Value::Error);
    }

    #[test]
    fn test_value_or() {
        assert_eq!(Value::False | Value::False, Value::False);
        assert_eq!(Value::False | Value::True, Value::True);
        assert_eq!(Value::False | Value::Unknown, Value::Unknown);
        assert_eq!(Value::True | Value::Unknown, Value::True);
        assert_eq!(Value::Error | Value::False, Value::Error);
    }

    #[test]
    fn test_value_xor() {
        assert_eq!(Value::True ^ Value::True, Value::False);
        assert_eq!(Value::True ^ Value::False, Value::True);
        assert_eq!(Value::False ^ Value::False, Value::False);
        assert_eq!(Value::True ^ Value::Unknown, Value::Unknown);
    }

    #[test]
    fn test_value_resolve() {
        assert_eq!(Value::False.resolve(Value::False), Value::False);
        assert_eq!(Value::True.resolve(Value::True), Value::True);
        assert_eq!(Value::True.resolve(Value::False), Value::Error);
        assert_eq!(Value::False.resolve(Value::True), Value::Error);
        assert_eq!(Value::HighZ.resolve(Value::True), Value::True);
        assert_eq!(Value::HighZ.resolve(Value::False), Value::False);
        assert_eq!(Value::HighZ.resolve(Value::HighZ), Value::HighZ);
        assert_eq!(Value::Error.resolve(Value::True), Value::Error);
    }

    #[test]
    fn test_bus_from_u64() {
        let b = Bus::from_u64(0b1010, 4);
        assert_eq!(b.get(0), Value::False);
        assert_eq!(b.get(1), Value::True);
        assert_eq!(b.get(2), Value::False);
        assert_eq!(b.get(3), Value::True);
        assert_eq!(b.to_u64(), Some(0b1010));
    }

    #[test]
    fn test_bus_not() {
        let b = Bus::from_u64(0b1010, 4);
        let r = b.not();
        assert_eq!(r.to_u64(), Some(0b0101));
    }

    #[test]
    fn test_bus_and() {
        let a = Bus::from_u64(0b1100, 4);
        let b = Bus::from_u64(0b1010, 4);
        assert_eq!(a.and(&b).to_u64(), Some(0b1000));
    }

    #[test]
    fn test_bus_or() {
        let a = Bus::from_u64(0b1100, 4);
        let b = Bus::from_u64(0b1010, 4);
        assert_eq!(a.or(&b).to_u64(), Some(0b1110));
    }

    #[test]
    fn test_bus_xor() {
        let a = Bus::from_u64(0b1100, 4);
        let b = Bus::from_u64(0b1010, 4);
        assert_eq!(a.xor(&b).to_u64(), Some(0b0110));
    }

    #[test]
    fn test_bus_slice() {
        let b = Bus::from_u64(0b11001010, 8);
        let s = b.slice(2, 6);
        assert_eq!(s.width(), 4);
        // bits 2..6 of 0b11001010 = bits[2]=0,bits[3]=1,bits[4]=0,bits[5]=0
        assert_eq!(s.to_u64(), Some(0b0010));
    }

    #[test]
    fn test_bus_concat() {
        let lo = Bus::from_u64(0b1100, 4);
        let hi = Bus::from_u64(0b1010, 4);
        let cat = lo.concat(&hi);
        assert_eq!(cat.width(), 8);
        assert_eq!(cat.to_u64(), Some(0b10101100));
    }

    #[test]
    fn test_bitwidth_unknown_bus() {
        let w = BitWidth::FOUR;
        let b = w.unknown_bus();
        assert_eq!(b.width(), 4);
        assert!(b.to_u64().is_none());
    }
}
