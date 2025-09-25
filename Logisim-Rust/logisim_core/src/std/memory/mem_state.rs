/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Memory state management
//!
//! This module implements memory state tracking equivalent to MemState.java.
//! It manages the runtime state of memory components during simulation.

use crate::std::memory::mem_contents::MemContents;
use crate::instance::InstanceState;
use crate::Value;
use std::sync::{Arc, Mutex};

/// Memory state for tracking memory contents during simulation
#[derive(Clone)]
pub struct MemState {
    contents: Arc<Mutex<MemContents>>,
    current_addr: i64,
    addr_bits: i32,
    data_bits: i32,
}

impl MemState {
    /// Create new memory state
    pub fn new(addr_bits: i32, data_bits: i32) -> Self {
        let contents = MemContents::create(addr_bits, data_bits, false);
        Self {
            contents: Arc::new(Mutex::new(contents)),
            current_addr: 0,
            addr_bits,
            data_bits,
        }
    }

    /// Create memory state with randomized contents
    pub fn new_randomized(addr_bits: i32, data_bits: i32) -> Self {
        let contents = MemContents::create(addr_bits, data_bits, true);
        Self {
            contents: Arc::new(Mutex::new(contents)),
            current_addr: 0,
            addr_bits,
            data_bits,
        }
    }

    /// Get the memory contents
    pub fn get_contents(&self) -> Arc<Mutex<MemContents>> {
        Arc::clone(&self.contents)
    }

    /// Set the memory contents
    pub fn set_contents(&mut self, new_contents: MemContents) {
        if let Ok(mut contents) = self.contents.lock() {
            *contents = new_contents;
        }
    }

    /// Get current address
    pub fn get_current_addr(&self) -> i64 {
        self.current_addr
    }

    /// Set current address
    pub fn set_current_addr(&mut self, addr: i64) {
        self.current_addr = addr;
    }

    /// Get address bit width
    pub fn get_addr_bits(&self) -> i32 {
        self.addr_bits
    }

    /// Get data bit width
    pub fn get_data_bits(&self) -> i32 {
        self.data_bits
    }

    /// Read value from memory at specified address
    pub fn read(&self, addr: i64) -> Value {
        if let Ok(contents) = self.contents.lock() {
            let raw_value = contents.get(addr);
            // Use from_long to create Value from raw data  
            let bus_width = crate::signal::BusWidth::new(self.data_bits as u32);
            Value::from_long(raw_value, bus_width)
        } else {
            // Return error value if can't access contents
            let bus_width = crate::signal::BusWidth::new(self.data_bits as u32);
            Value::from_long(-1, bus_width)
        }
    }

    /// Write value to memory at specified address  
    pub fn write(&mut self, addr: i64, value: Value) {
        if let Ok(mut contents) = self.contents.lock() {
            if value.is_fully_defined() {
                let raw_value = value.to_long_value();
                contents.set(addr, raw_value);
            }
        }
    }

    /// Clear memory contents
    pub fn clear(&mut self) {
        if let Ok(mut contents) = self.contents.lock() {
            contents.clear();
        }
    }

    /// Check if memory is clear
    pub fn is_clear(&self) -> bool {
        if let Ok(contents) = self.contents.lock() {
            contents.is_clear()
        } else {
            true
        }
    }

    /// Get state from instance state
    pub fn get(state: &dyn InstanceState) -> Option<MemState> {
        // TODO: Implement proper instance state integration
        // This would interface with the component's instance data
        None
    }

    /// Set state in instance state
    pub fn set(state: &mut dyn InstanceState, mem_state: MemState) {
        // TODO: Implement proper instance state integration
        // This would store the memory state in the instance data
    }
}

impl Default for MemState {
    fn default() -> Self {
        Self::new(8, 8) // Default to 8-bit address and data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_state_creation() {
        let state = MemState::new(10, 8);
        assert_eq!(state.get_addr_bits(), 10);
        assert_eq!(state.get_data_bits(), 8);
        assert_eq!(state.get_current_addr(), 0);
        assert!(state.is_clear());
    }

    #[test]
    fn test_mem_state_read_write() {
        let mut state = MemState::new(10, 8);
        let value = Value::create_known(BitWidth::create(8), 0xFF);
        
        state.write(100, value);
        let read_value = state.read(100);
        
        assert!(read_value.is_fully_defined());
        assert_eq!(read_value.to_int_value(), 0xFF);
        assert!(!state.is_clear());
    }

    #[test]
    fn test_mem_state_current_addr() {
        let mut state = MemState::new(10, 8);
        state.set_current_addr(256);
        assert_eq!(state.get_current_addr(), 256);
    }

    #[test]
    fn test_mem_state_clear() {
        let mut state = MemState::new(10, 8);
        let value = Value::create_known(BitWidth::create(8), 0xAA);
        
        state.write(50, value);
        assert!(!state.is_clear());
        
        state.clear();
        assert!(state.is_clear());
        
        let read_value = state.read(50);
        assert_eq!(read_value.to_int_value(), 0);
    }

    #[test]
    fn test_mem_state_contents_access() {
        let state = MemState::new(10, 8);
        let contents = state.get_contents();
        
        // Test that we can access the contents through the Arc<Mutex<>>
        if let Ok(contents_guard) = contents.lock() {
            assert_eq!(contents_guard.get_width(), 8);
            assert_eq!(contents_guard.get_log_length(), 10);
        }
    }

    #[test]
    fn test_mem_state_clone() {
        let mut state1 = MemState::new(10, 8);
        let value = Value::create_known(BitWidth::create(8), 0x42);
        state1.write(123, value);
        
        let state2 = state1.clone();
        let read_value = state2.read(123);
        assert_eq!(read_value.to_int_value(), 0x42);
    }
}