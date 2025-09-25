/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Counter data state management
//!
//! Represents the state of a counter component.
//! Equivalent to Java's com.cburch.gray.CounterData class.

use crate::data::BitWidth;
use crate::signal::Value;

/// Trait for instance data that can be stored with component instances.
pub trait InstanceData: Send + Sync {
    fn clone_box(&self) -> Box<dyn InstanceData>;
}

/// Placeholder for instance state management.
pub struct InstanceState;

/// Represents the state of a counter.
///
/// This is equivalent to Java's CounterData class.
#[derive(Debug, Clone)]
pub struct CounterData {
    /// The last clock input value observed.
    last_clock: Option<Value>,
    /// The current value emitted by the counter.
    value: Value,
}

impl CounterData {
    /// Constructs a state with the given values.
    pub fn new(last_clock: Option<Value>, value: Value) -> Self {
        Self { last_clock, value }
    }

    /// Retrieves the state associated with this counter in the circuit state,
    /// generating the state if necessary.
    ///
    /// This is equivalent to Java's CounterData.get() method.
    pub fn get(_state: &mut InstanceState, _width: BitWidth) -> CounterData {
        // For now, return a default implementation
        // In a full implementation, this would interact with the circuit state
        CounterData::new(None, Value::Low)
    }

    /// Returns the current value emitted by the counter.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Updates the current value emitted by the counter.
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    /// Updates the last clock observed, returning true if triggered.
    ///
    /// Returns true if this represents a rising edge trigger (false to true).
    pub fn update_clock(&mut self, value: Value) -> bool {
        let old = self.last_clock;
        self.last_clock = Some(value);

        // Check for rising edge: old was false/low, new is true/high
        match (old, value) {
            (Some(Value::Low), Value::High) => true,
            (None, Value::High) => true,
            _ => false,
        }
    }
}

impl InstanceData for CounterData {
    fn clone_box(&self) -> Box<dyn InstanceData> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_data_creation() {
        let data = CounterData::new(None, Value::Low);

        assert_eq!(data.value(), &Value::Low);
        assert_eq!(data.last_clock, None);
    }

    #[test]
    fn test_set_value() {
        let mut data = CounterData::new(None, Value::Low);
        data.set_value(Value::High);

        assert_eq!(data.value(), &Value::High);
    }

    #[test]
    fn test_clock_trigger_detection() {
        let mut data = CounterData::new(None, Value::Low);

        // First high should trigger
        assert!(data.update_clock(Value::High));

        // High to high should not trigger
        assert!(!data.update_clock(Value::High));

        // High to low should not trigger
        assert!(!data.update_clock(Value::Low));

        // Low to high should trigger
        assert!(data.update_clock(Value::High));
    }
}
