/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Logger System
//!
//! This module provides the `InstanceLogger` trait for components that support
//! logging/monitoring capabilities. This is equivalent to Java's `InstanceLogger` class.

use crate::data::BitWidth;
use crate::{Value};
use crate::instance::InstanceState;

/// Trait for components that provide logging/monitoring capabilities.
///
/// This trait allows components to expose internal signals or state for
/// monitoring, waveform generation, and debugging purposes.
pub trait InstanceLogger {
    /// Returns the display name for a loggable option.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `option` - The logging option identifier
    ///
    /// # Returns
    ///
    /// Human-readable name for the option, or None if invalid.
    fn get_log_name(&self, state: &dyn InstanceState, option: &dyn std::any::Any) -> Option<String>;

    /// Returns the bit width for a loggable option.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `option` - The logging option identifier
    ///
    /// # Returns
    ///
    /// Bit width of the logged signal, or None if invalid.
    fn get_bit_width(&self, state: &dyn InstanceState, option: &dyn std::any::Any) -> Option<BitWidth>;

    /// Returns all available logging options for this component.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    ///
    /// # Returns
    ///
    /// Vector of option identifiers that can be logged.
    fn get_log_options(&self, state: &dyn InstanceState) -> Vec<Box<dyn std::any::Any>> {
        let _ = state;
        Vec::new()
    }

    /// Returns the current value for a loggable option.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `option` - The logging option identifier
    ///
    /// # Returns
    ///
    /// Current signal value, or None if invalid.
    fn get_log_value(&self, state: &dyn InstanceState, option: &dyn std::any::Any) -> Option<Value>;

    /// Checks if a logging option represents an input signal.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `option` - The logging option identifier
    ///
    /// # Returns
    ///
    /// True if the option is an input signal, false otherwise.
    fn is_input(&self, state: &dyn InstanceState, option: &dyn std::any::Any) -> bool {
        let _ = (state, option);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::BitWidth;
    
    struct MockLogger;
    
    impl InstanceLogger for MockLogger {
        fn get_log_name(&self, _state: &dyn InstanceState, _option: &dyn std::any::Any) -> Option<String> {
            Some("Mock Signal".to_string())
        }

        fn get_bit_width(&self, _state: &dyn InstanceState, _option: &dyn std::any::Any) -> Option<BitWidth> {
            Some(BitWidth::new(1))
        }

        fn get_log_value(&self, _state: &dyn InstanceState, _option: &dyn std::any::Any) -> Option<Value> {
            Some(Value::High)
        }
    }

    // Note: Cannot easily test without a concrete InstanceState implementation
    // Tests would be added when integrated with the full system
}