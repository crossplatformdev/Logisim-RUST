//! Signal and value types for the simulation.
//!
//! This module defines the fundamental types for representing digital signals,
//! including single-bit values, multi-bit buses, and timing information.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the width of a bus in bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BusWidth(pub u32);

impl BusWidth {
    /// Create a new bus width
    pub fn new(width: u32) -> Self {
        BusWidth(width)
    }

    /// Get the width as a u32
    pub fn as_u32(self) -> u32 {
        self.0
    }

    /// Check if this is a single-bit signal
    pub fn is_single_bit(self) -> bool {
        self.0 == 1
    }
}

impl From<u32> for BusWidth {
    fn from(width: u32) -> Self {
        BusWidth(width)
    }
}

impl fmt::Display for BusWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Timestamp for simulation events
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Create a new timestamp
    pub fn new(time: u64) -> Self {
        Timestamp(time)
    }

    /// Get the timestamp as u64
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Add a delay to this timestamp
    pub fn add_delay(self, delay: u64) -> Self {
        Timestamp(self.0 + delay)
    }
}

impl From<u64> for Timestamp {
    fn from(time: u64) -> Self {
        Timestamp(time)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Possible values for a digital signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    /// Logic high (1)
    High,
    /// Logic low (0)
    Low,
    /// Unknown or uninitialized state
    Unknown,
    /// Error state (e.g., contention)
    Error,
}

impl Value {
    /// Convert to boolean if possible
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Value::High => Some(true),
            Value::Low => Some(false),
            Value::Unknown | Value::Error => None,
        }
    }

    /// Convert from boolean
    pub fn from_bool(value: bool) -> Self {
        if value {
            Value::High
        } else {
            Value::Low
        }
    }

    /// Check if this is a definite value (not unknown or error)
    pub fn is_definite(self) -> bool {
        matches!(self, Value::High | Value::Low)
    }

    /// Logical AND operation
    pub fn and(self, other: Value) -> Value {
        match (self, other) {
            (Value::High, Value::High) => Value::High,
            (Value::Low, _) | (_, Value::Low) => Value::Low,
            (Value::Error, _) | (_, Value::Error) => Value::Error,
            _ => Value::Unknown,
        }
    }

    /// Logical OR operation
    pub fn or(self, other: Value) -> Value {
        match (self, other) {
            (Value::High, _) | (_, Value::High) => Value::High,
            (Value::Low, Value::Low) => Value::Low,
            (Value::Error, _) | (_, Value::Error) => Value::Error,
            _ => Value::Unknown,
        }
    }

    /// Logical NOT operation
    pub fn not(self) -> Value {
        match self {
            Value::High => Value::Low,
            Value::Low => Value::High,
            Value::Unknown => Value::Unknown,
            Value::Error => Value::Error,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::High => write!(f, "1"),
            Value::Low => write!(f, "0"),
            Value::Unknown => write!(f, "X"),
            Value::Error => write!(f, "E"),
        }
    }
}

/// Represents a single signal or a bus of signals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signal {
    /// The value(s) carried by this signal
    values: Vec<Value>,
    /// The width of this signal
    width: BusWidth,
}

impl Signal {
    /// Create a new single-bit signal
    pub fn new_single(value: Value) -> Self {
        Signal {
            values: vec![value],
            width: BusWidth(1),
        }
    }

    /// Create a new multi-bit signal (bus)
    pub fn new_bus(values: Vec<Value>) -> Self {
        let width = BusWidth(values.len() as u32);
        Signal { values, width }
    }

    /// Create a signal with a specific width, all bits set to the same value
    pub fn new_uniform(width: BusWidth, value: Value) -> Self {
        Signal {
            values: vec![value; width.as_u32() as usize],
            width,
        }
    }

    /// Create an unknown signal of given width
    pub fn unknown(width: BusWidth) -> Self {
        Self::new_uniform(width, Value::Unknown)
    }

    /// Get the width of this signal
    pub fn width(&self) -> BusWidth {
        self.width
    }

    /// Get a specific bit value
    pub fn get_bit(&self, index: usize) -> Option<Value> {
        self.values.get(index).copied()
    }

    /// Set a specific bit value
    pub fn set_bit(&mut self, index: usize, value: Value) -> Result<(), &'static str> {
        if index >= self.values.len() {
            return Err("Bit index out of range");
        }
        self.values[index] = value;
        Ok(())
    }

    /// Get all values as a slice
    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// Check if this is a single-bit signal
    pub fn is_single_bit(&self) -> bool {
        self.width.is_single_bit()
    }

    /// Convert to a single Value if this is a single-bit signal
    pub fn as_single(&self) -> Option<Value> {
        if self.is_single_bit() {
            self.values.first().copied()
        } else {
            None
        }
    }

    /// Convert bus to integer value if all bits are definite
    pub fn to_u64(&self) -> Option<u64> {
        let mut result = 0u64;
        for (i, &value) in self.values.iter().enumerate() {
            match value {
                Value::High => result |= 1u64 << i,
                Value::Low => {}, // bit remains 0
                _ => return None, // unknown or error
            }
        }
        Some(result)
    }

    /// Create signal from integer value
    pub fn from_u64(value: u64, width: BusWidth) -> Self {
        let mut values = Vec::with_capacity(width.as_u32() as usize);
        for i in 0..width.as_u32() {
            let bit = (value >> i) & 1;
            values.push(if bit == 1 { Value::High } else { Value::Low });
        }
        Signal { values, width }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_single_bit() {
            write!(f, "{}", self.values[0])
        } else {
            for value in self.values.iter().rev() {
                write!(f, "{}", value)?;
            }
            Ok(())
        }
    }
}

/// Type alias for Bus - same as Signal but emphasizes multi-bit nature
pub type Bus = Signal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_operations() {
        assert_eq!(Value::High.and(Value::High), Value::High);
        assert_eq!(Value::High.and(Value::Low), Value::Low);
        assert_eq!(Value::Low.and(Value::High), Value::Low);
        assert_eq!(Value::Low.and(Value::Low), Value::Low);

        assert_eq!(Value::High.or(Value::High), Value::High);
        assert_eq!(Value::High.or(Value::Low), Value::High);
        assert_eq!(Value::Low.or(Value::High), Value::High);
        assert_eq!(Value::Low.or(Value::Low), Value::Low);

        assert_eq!(Value::High.not(), Value::Low);
        assert_eq!(Value::Low.not(), Value::High);
    }

    #[test]
    fn test_signal_creation() {
        let sig = Signal::new_single(Value::High);
        assert!(sig.is_single_bit());
        assert_eq!(sig.as_single(), Some(Value::High));

        let bus = Signal::new_bus(vec![Value::Low, Value::High, Value::Low]);
        assert_eq!(bus.width(), BusWidth(3));
        assert_eq!(bus.get_bit(1), Some(Value::High));
    }

    #[test]
    fn test_signal_conversion() {
        let sig = Signal::from_u64(5, BusWidth(4)); // 0101
        assert_eq!(sig.get_bit(0), Some(Value::High)); // LSB
        assert_eq!(sig.get_bit(1), Some(Value::Low));
        assert_eq!(sig.get_bit(2), Some(Value::High));
        assert_eq!(sig.get_bit(3), Some(Value::Low));

        assert_eq!(sig.to_u64(), Some(5));
    }
}