/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

pub use super::{HexModelListener, VecHexModel};

/// Interface for hex editor data model
/// 
/// Provides access to binary data that can be displayed and edited
/// in a hex editor interface with change notification support.
pub trait HexModel {
    /// Registers a listener for changes to the values
    fn add_hex_model_listener(&mut self, listener: Box<dyn HexModelListener>);

    /// Unregisters a listener for changes to the values
    fn remove_hex_model_listener(&mut self, listener_id: usize);

    /// Fills a series of values with the same value
    fn fill(&mut self, start: u64, length: u64, value: u64);

    /// Returns the value at the given address
    fn get(&self, address: u64) -> u64;

    /// Returns the offset of the initial value to be displayed
    fn get_first_offset(&self) -> u64;

    /// Returns the number of values to be displayed
    fn get_last_offset(&self) -> u64;

    /// Returns number of bits in each value
    fn get_value_width(&self) -> u32;

    /// Changes the value at the given address
    fn set(&mut self, address: u64, value: u64);

    /// Changes a series of values at the given addresses
    fn set_range(&mut self, start: u64, values: &[u64]);
}

/// Simple implementation of HexModel backed by a Vec
pub struct VecHexModel {
    data: Vec<u64>,
    listeners: Vec<Box<dyn HexModelListener>>,
    first_offset: u64,
    value_width: u32,
}

impl VecHexModel {
    /// Create a new VecHexModel with the specified size
    pub fn new(size: usize, value_width: u32) -> Self {
        Self {
            data: vec![0; size],
            listeners: Vec::new(),
            first_offset: 0,
            value_width,
        }
    }

    /// Create a new VecHexModel from existing data
    pub fn from_data(data: Vec<u64>, value_width: u32) -> Self {
        Self {
            data,
            listeners: Vec::new(),
            first_offset: 0,
            value_width,
        }
    }

    /// Set the first offset for display
    pub fn set_first_offset(&mut self, offset: u64) {
        self.first_offset = offset;
        self.notify_metainfo_changed();
    }

    /// Set the value width in bits
    pub fn set_value_width(&mut self, width: u32) {
        self.value_width = width;
        self.notify_metainfo_changed();
    }

    /// Resize the data buffer
    pub fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, 0);
        self.notify_metainfo_changed();
    }

    fn notify_bytes_changed(&mut self, start: u64, num_bytes: u64, old_values: &[u64]) {
        // Create a copy of listeners to avoid borrow issues
        let listeners: Vec<_> = self.listeners.iter().collect();
        for listener in listeners {
            listener.bytes_changed(start, num_bytes, old_values);
        }
    }

    fn notify_metainfo_changed(&mut self) {
        // Create a copy of listeners to avoid borrow issues
        let listeners: Vec<_> = self.listeners.iter().collect();
        for listener in listeners {
            listener.metainfo_changed();
        }
    }
}

impl HexModel for VecHexModel {
    fn add_hex_model_listener(&mut self, listener: Box<dyn HexModelListener>) {
        self.listeners.push(listener);
    }

    fn remove_hex_model_listener(&mut self, listener_id: usize) {
        if listener_id < self.listeners.len() {
            self.listeners.remove(listener_id);
        }
    }

    fn fill(&mut self, start: u64, length: u64, value: u64) {
        let start_idx = start as usize;
        let end_idx = ((start + length) as usize).min(self.data.len());
        
        if start_idx < self.data.len() && start_idx < end_idx {
            let old_values: Vec<u64> = self.data[start_idx..end_idx].to_vec();
            
            for i in start_idx..end_idx {
                self.data[i] = value;
            }
            
            self.notify_bytes_changed(start, length, &old_values);
        }
    }

    fn get(&self, address: u64) -> u64 {
        self.data.get(address as usize).copied().unwrap_or(0)
    }

    fn get_first_offset(&self) -> u64 {
        self.first_offset
    }

    fn get_last_offset(&self) -> u64 {
        self.first_offset + (self.data.len() as u64).saturating_sub(1)
    }

    fn get_value_width(&self) -> u32 {
        self.value_width
    }

    fn set(&mut self, address: u64, value: u64) {
        let addr_idx = address as usize;
        if let Some(cell) = self.data.get_mut(addr_idx) {
            let old_value = *cell;
            *cell = value;
            self.notify_bytes_changed(address, 1, &[old_value]);
        }
    }

    fn set_range(&mut self, start: u64, values: &[u64]) {
        let start_idx = start as usize;
        let end_idx = (start_idx + values.len()).min(self.data.len());
        
        if start_idx < self.data.len() && start_idx < end_idx {
            let old_values: Vec<u64> = self.data[start_idx..end_idx].to_vec();
            
            for (i, &value) in values.iter().enumerate() {
                if let Some(cell) = self.data.get_mut(start_idx + i) {
                    *cell = value;
                }
            }
            
            self.notify_bytes_changed(start, (end_idx - start_idx) as u64, &old_values);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener {
        bytes_changed_calls: std::cell::RefCell<Vec<(u64, u64)>>,
        metainfo_changed_calls: std::cell::RefCell<u32>,
    }

    impl TestListener {
        fn new() -> Self {
            Self {
                bytes_changed_calls: std::cell::RefCell::new(Vec::new()),
                metainfo_changed_calls: std::cell::RefCell::new(0),
            }
        }
    }

    impl HexModelListener for TestListener {
        fn bytes_changed(&self, start: u64, num_bytes: u64, _old_values: &[u64]) {
            self.bytes_changed_calls.borrow_mut().push((start, num_bytes));
        }

        fn metainfo_changed(&self) {
            *self.metainfo_changed_calls.borrow_mut() += 1;
        }
    }

    #[test]
    fn test_vec_hex_model_basic_operations() {
        let mut model = VecHexModel::new(16, 8);
        
        // Test initial state
        assert_eq!(model.get_first_offset(), 0);
        assert_eq!(model.get_last_offset(), 15);
        assert_eq!(model.get_value_width(), 8);
        assert_eq!(model.get(0), 0);
        
        // Test set/get
        model.set(5, 0xFF);
        assert_eq!(model.get(5), 0xFF);
        
        // Test fill
        model.fill(0, 4, 0xAA);
        for i in 0..4 {
            assert_eq!(model.get(i), 0xAA);
        }
        
        // Test range set
        model.set_range(10, &[0x11, 0x22, 0x33]);
        assert_eq!(model.get(10), 0x11);
        assert_eq!(model.get(11), 0x22);
        assert_eq!(model.get(12), 0x33);
    }

    #[test]
    fn test_bounds_checking() {
        let mut model = VecHexModel::new(4, 8);
        
        // Test out of bounds access
        assert_eq!(model.get(100), 0);
        
        // Test out of bounds set (should not panic)
        model.set(100, 0xFF);
        assert_eq!(model.get(100), 0);
        
        // Test partial range set beyond bounds
        model.set_range(2, &[0x11, 0x22, 0x33, 0x44]);
        assert_eq!(model.get(2), 0x11);
        assert_eq!(model.get(3), 0x22);
        assert_eq!(model.get(4), 0); // Out of bounds, should remain 0
    }

    #[test]
    fn test_configuration_changes() {
        let mut model = VecHexModel::new(8, 8);
        
        // Test changing first offset
        model.set_first_offset(100);
        assert_eq!(model.get_first_offset(), 100);
        assert_eq!(model.get_last_offset(), 107);
        
        // Test changing value width
        model.set_value_width(16);
        assert_eq!(model.get_value_width(), 16);
        
        // Test resizing
        model.resize(16);
        assert_eq!(model.get_last_offset(), 115); // 100 + 16 - 1
    }
}