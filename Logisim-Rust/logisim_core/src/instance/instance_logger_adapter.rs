/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Logger Adapter
//!
//! Adapter between the instance logger system and external logging interfaces.

use crate::data::BitWidth;
use crate::instance::{InstanceLogger, InstanceState};
use crate::Value;

/// Adapter that bridges InstanceLogger implementations with external logging systems.
///
/// This is equivalent to Java's `InstanceLoggerAdapter` class.
pub struct InstanceLoggerAdapter {
    logger: Box<dyn InstanceLogger>,
}

impl InstanceLoggerAdapter {
    /// Creates a new logger adapter.
    pub fn new(logger: Box<dyn InstanceLogger>) -> Self {
        Self { logger }
    }

    /// Delegates to the wrapped logger.
    pub fn get_log_name(
        &self,
        state: &dyn InstanceState,
        option: &dyn std::any::Any,
    ) -> Option<String> {
        self.logger.get_log_name(state, option)
    }

    /// Delegates to the wrapped logger.
    pub fn get_bit_width(
        &self,
        state: &dyn InstanceState,
        option: &dyn std::any::Any,
    ) -> Option<BitWidth> {
        self.logger.get_bit_width(state, option)
    }

    /// Delegates to the wrapped logger.
    pub fn get_log_options(&self, state: &dyn InstanceState) -> Vec<Box<dyn std::any::Any>> {
        self.logger.get_log_options(state)
    }

    /// Delegates to the wrapped logger.
    pub fn get_log_value(
        &self,
        state: &dyn InstanceState,
        option: &dyn std::any::Any,
    ) -> Option<Value> {
        self.logger.get_log_value(state, option)
    }

    /// Delegates to the wrapped logger.
    pub fn is_input(&self, state: &dyn InstanceState, option: &dyn std::any::Any) -> bool {
        self.logger.is_input(state, option)
    }
}
