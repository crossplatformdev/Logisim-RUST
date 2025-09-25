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
    pub const fn new(width: u32) -> Self {
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

    /// Get the mask for this bit width
    pub fn get_mask(self) -> u64 {
        if self.0 == 0 {
            0
        } else if self.0 >= 64 {
            u64::MAX
        } else {
            (1u64 << self.0) - 1
        }
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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
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
    /// High impedance state (tri-state)
    HighZ,
}

/// Signal enum with Unknown, Zero, One, HiZ as requested in requirements
/// This provides the core signal values for digital logic simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalState {
    /// Unknown or uninitialized state
    Unknown,
    /// Logic low (0)
    Zero,
    /// Logic high (1)
    One,
    /// High impedance state (tri-state)
    HiZ,
}

impl SignalState {
    /// Convert to the internal Value representation
    pub fn to_value(self) -> Value {
        match self {
            SignalState::Unknown => Value::Unknown,
            SignalState::Zero => Value::Low,
            SignalState::One => Value::High,
            SignalState::HiZ => Value::HighZ,
        }
    }

    /// Convert from the internal Value representation
    pub fn from_value(value: Value) -> Self {
        match value {
            Value::Unknown => SignalState::Unknown,
            Value::Low => SignalState::Zero,
            Value::High => SignalState::One,
            Value::HighZ => SignalState::HiZ,
            Value::Error => SignalState::Unknown, // Map error to unknown
        }
    }

    /// Logical AND operation
    pub fn and(self, other: SignalState) -> SignalState {
        SignalState::from_value(self.to_value().and(other.to_value()))
    }

    /// Logical OR operation  
    pub fn or(self, other: SignalState) -> SignalState {
        SignalState::from_value(self.to_value().or(other.to_value()))
    }

    /// Logical NOT operation
    pub fn not(self) -> SignalState {
        SignalState::from_value(self.to_value().not())
    }
}

impl Value {
    /// Convert to boolean if possible
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Value::High => Some(true),
            Value::Low => Some(false),
            Value::Unknown | Value::Error | Value::HighZ => None,
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

    /// Logical XOR operation
    pub fn xor(self, other: Value) -> Value {
        match (self, other) {
            (Value::High, Value::Low) | (Value::Low, Value::High) => Value::High,
            (Value::High, Value::High) | (Value::Low, Value::Low) => Value::Low,
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
            Value::HighZ => Value::HighZ,
        }
    }

    /// Create a Value from a long integer with specified width (simplified for single-bit)
    pub fn from_long(value: i64, _width: BusWidth) -> Value {
        if value & 1 != 0 {
            Value::High
        } else {
            Value::Low
        }
    }

    /// Create a Value from individual bits (simplified - just returns first bit)
    pub fn from_bits(bits: &[Value]) -> Value {
        bits.first().copied().unwrap_or(Value::Low)
    }

    /// Convert to long integer value (simplified for single-bit)
    pub fn to_long_value(&self) -> i64 {
        match self {
            Value::High => 1,
            Value::Low => 0,
            _ => 0,
        }
    }

    /// Get a specific bit from a value (simplified)
    pub fn get_bit(&self, index: usize) -> Value {
        if index == 0 {
            *self
        } else {
            Value::Low
        }
    }

    /// Check if all bits in this value are fully defined (same as is_definite for single bits)
    pub fn is_fully_defined(&self) -> bool {
        self.is_definite()
    }

    /// Get the width of this value (always 1 for single-bit values)
    pub fn width(&self) -> BusWidth {
        BusWidth(1)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::High => write!(f, "1"),
            Value::Low => write!(f, "0"),
            Value::Unknown => write!(f, "X"),
            Value::Error => write!(f, "E"),
            Value::HighZ => write!(f, "Z"),
        }
    }
}

impl std::ops::Not for Value {
    type Output = Value;

    fn not(self) -> Value {
        self.not()
    }
}

/// Represents a signal with a value and timestamp
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signal {
    /// The value carried by this signal
    value: Value,
    /// When this signal was last updated
    timestamp: Timestamp,
}

impl Signal {
    /// Create a new signal with a value and timestamp
    pub fn new(value: Value, timestamp: Timestamp) -> Self {
        Signal { value, timestamp }
    }

    /// Create a signal with the current timestamp (0)
    pub fn new_now(value: Value) -> Self {
        Signal {
            value,
            timestamp: Timestamp(0),
        }
    }

    /// Get the value of this signal
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Get the timestamp of this signal
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Set a new value and timestamp
    pub fn set(&mut self, value: Value, timestamp: Timestamp) {
        self.value = value;
        self.timestamp = timestamp;
    }

    /// Get the width of this signal
    pub fn width(&self) -> BusWidth {
        self.value.width()
    }

    /// Create an unknown signal of given width
    pub fn unknown(_width: BusWidth) -> Self {
        Signal::new(Value::Unknown, Timestamp(0))
    }

    /// Check if this signal is at a definite value
    pub fn is_definite(&self) -> bool {
        self.value.is_fully_defined()
    }

    /// Create a single-bit signal (convenience method)
    pub fn new_single(value: Value) -> Self {
        Signal {
            value,
            timestamp: Timestamp(0),
        }
    }

    /// Get the value as a single bit (if this is single-bit signal)
    pub fn as_single(&self) -> Option<Value> {
        Some(self.value)
    }

    /// Check if this is a single-bit signal
    pub fn is_single_bit(&self) -> bool {
        true // For now, all signals are single-bit
    }

    /// Get a specific bit from this signal
    pub fn get_bit(&self, _index: u32) -> Option<Value> {
        Some(self.value) // For single-bit signals, always return the value
    }

    /// Create a signal from a u64 value
    pub fn from_u64(value: u64, _width: BusWidth) -> Self {
        let signal_value = if value == 0 { Value::Low } else { Value::High };
        Signal::new_single(signal_value)
    }

    /// Create a bus signal from multiple values
    pub fn new_bus(values: Vec<Value>) -> Self {
        // For now, just use the first value as a single-bit signal
        let value = values.first().copied().unwrap_or(Value::Unknown);
        Signal::new_single(value)
    }

    /// Create a signal with all bits low
    pub fn all_low(_width: BusWidth) -> Self {
        Signal::new_single(Value::Low)
    }

    /// Create a signal with all bits high
    pub fn all_high(_width: BusWidth) -> Self {
        Signal::new_single(Value::High)
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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

        assert_eq!(!Value::High, Value::Low);
        assert_eq!(!Value::Low, Value::High);
    }

    #[test]
    fn test_signal_enum_basic() {
        // Test SignalState enum variants
        assert_eq!(SignalState::One.to_value(), Value::High);
        assert_eq!(SignalState::Zero.to_value(), Value::Low);
        assert_eq!(SignalState::Unknown.to_value(), Value::Unknown);
        assert_eq!(SignalState::HiZ.to_value(), Value::HighZ);
    }

    #[test]
    fn test_signal_conversion() {
        // Test conversions between SignalState and Value
        assert_eq!(SignalState::from_value(Value::High), SignalState::One);
        assert_eq!(SignalState::from_value(Value::Low), SignalState::Zero);
        assert_eq!(SignalState::from_value(Value::Unknown), SignalState::Unknown);
        assert_eq!(SignalState::from_value(Value::HighZ), SignalState::HiZ);
        assert_eq!(SignalState::from_value(Value::Error), SignalState::Unknown);
    }

    #[test]
    fn test_signal_logic_operations() {
        // Test SignalState enum logic operations
        assert_eq!(SignalState::One.and(SignalState::One), SignalState::One);
        assert_eq!(SignalState::One.and(SignalState::Zero), SignalState::Zero);
        assert_eq!(SignalState::Zero.and(SignalState::One), SignalState::Zero);
        assert_eq!(SignalState::Zero.and(SignalState::Zero), SignalState::Zero);

        assert_eq!(SignalState::One.or(SignalState::One), SignalState::One);
        assert_eq!(SignalState::One.or(SignalState::Zero), SignalState::One);
        assert_eq!(SignalState::Zero.or(SignalState::One), SignalState::One);
        assert_eq!(SignalState::Zero.or(SignalState::Zero), SignalState::Zero);

        assert_eq!(SignalState::One.not(), SignalState::Zero);
        assert_eq!(SignalState::Zero.not(), SignalState::One);
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
    fn test_signal_conversion_old() {
        let sig = Signal::from_u64(5, BusWidth(4)); // 0101
        assert_eq!(sig.get_bit(0), Some(Value::High)); // LSB
        assert_eq!(sig.get_bit(1), Some(Value::Low));
        assert_eq!(sig.get_bit(2), Some(Value::High));
        assert_eq!(sig.get_bit(3), Some(Value::Low));

        assert_eq!(sig.to_u64(), Some(5));
    }

    #[test]
    fn test_id_assignment() {
        use crate::netlist::{NodeId, NetId};
        use crate::comp::ComponentId;
        
        // Test strong-typed ID creation and display
        let node_id = NodeId::new(42);
        assert_eq!(node_id.as_u64(), 42);
        assert_eq!(format!("{}", node_id), "N42");

        let net_id = NetId::new(123);
        assert_eq!(net_id.as_u64(), 123);
        assert_eq!(format!("{}", net_id), "Net123");

        let component_id = ComponentId::new(456);
        assert_eq!(component_id.as_u64(), 456);
        assert_eq!(format!("{}", component_id), "C456");

        let bus_width = BusWidth::new(8);
        assert_eq!(bus_width.as_u32(), 8);
        assert_eq!(format!("{}", bus_width), "8");

        let timestamp = Timestamp::new(1000);
        assert_eq!(timestamp.as_u64(), 1000);
        assert_eq!(format!("{}", timestamp), "1000");
    }
}
