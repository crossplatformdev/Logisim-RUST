/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Hex Model - Data model for hex editor
//!
//! Rust port of HexModel.java and HexModelListener.java

use std::sync::{Arc, Weak};

/// Event fired when hex model data changes
#[derive(Debug, Clone)]
pub struct HexModelEvent {
    pub source_id: usize,
    pub event_type: HexModelEventType,
}

#[derive(Debug, Clone)]
pub enum HexModelEventType {
    /// Bytes changed at specific addresses
    BytesChanged {
        start: u64,
        num_bytes: u64,
        old_values: Vec<u64>,
    },
    /// Metadata changed (size, width, etc.)
    MetainfoChanged,
}

/// Trait for listening to hex model changes
pub trait HexModelListener: Send + Sync {
    /// Called when bytes in the model change
    fn bytes_changed(&mut self, source: &dyn HexModel, start: u64, num_bytes: u64, old_values: &[u64]);
    
    /// Called when model metadata changes
    fn metainfo_changed(&mut self, source: &dyn HexModel);
}

/// Main trait for hex data models
///
/// This trait defines the interface for hex data storage and manipulation,
/// equivalent to the Java HexModel interface.
pub trait HexModel: Send + Sync {
    /// Register a listener for changes to the values
    fn add_hex_model_listener(&mut self, listener: Box<dyn HexModelListener>);
    
    /// Fill a series of values with the same value
    fn fill(&mut self, start: u64, length: u64, value: u64);
    
    /// Return the value at the given address
    fn get(&self, address: u64) -> u64;
    
    /// Return the offset of the initial value to be displayed
    fn get_first_offset(&self) -> u64;
    
    /// Return the number of values to be displayed
    fn get_last_offset(&self) -> u64;
    
    /// Return number of bits in each value
    fn get_value_width(&self) -> u32;
    
    /// Unregister a listener for changes to the values
    fn remove_hex_model_listener(&mut self, listener_id: usize);
    
    /// Change the value at the given address
    fn set(&mut self, address: u64, value: u64);
    
    /// Change a series of values at the given addresses
    fn set_range(&mut self, start: u64, values: &[u64]);
    
    /// Get a unique identifier for this model
    fn get_id(&self) -> usize;
}

/// Simple in-memory implementation of HexModel
pub struct MemoryHexModel {
    id: usize,
    data: Vec<u64>,
    first_offset: u64,
    value_width: u32,
    listeners: Vec<Box<dyn HexModelListener>>,
    next_listener_id: usize,
}

impl MemoryHexModel {
    /// Create a new memory hex model
    pub fn new(size: usize, value_width: u32) -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
        
        Self {
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            data: vec![0; size],
            first_offset: 0,
            value_width,
            listeners: Vec::new(),
            next_listener_id: 1,
        }
    }
    
    /// Create a new model with data
    pub fn new_with_data(data: Vec<u64>, first_offset: u64, value_width: u32) -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
        
        Self {
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            data,
            first_offset,
            value_width,
            listeners: Vec::new(),
            next_listener_id: 1,
        }
    }
    
    /// Set the first offset
    pub fn set_first_offset(&mut self, offset: u64) {
        if self.first_offset != offset {
            self.first_offset = offset;
            self.notify_metainfo_changed();
        }
    }
    
    /// Resize the model
    pub fn resize(&mut self, new_size: usize) {
        if self.data.len() != new_size {
            self.data.resize(new_size, 0);
            self.notify_metainfo_changed();
        }
    }
    
    /// Load data from bytes
    pub fn load_from_bytes(&mut self, bytes: &[u8]) {
        let values_per_byte = match self.value_width {
            1..=8 => 8 / self.value_width,
            9..=16 => 2,
            17..=32 => 1,
            33..=64 => 1,
            _ => 1,
        } as usize;
        
        self.data.clear();
        
        match self.value_width {
            1..=8 => {
                let mask = (1u64 << self.value_width) - 1;
                for &byte in bytes {
                    for i in 0..values_per_byte {
                        let shift = i * self.value_width;
                        if shift < 8 {
                            let value = (byte as u64 >> shift) & mask;
                            self.data.push(value);
                        }
                    }
                }
            }
            9..=16 => {
                let mask = (1u64 << self.value_width) - 1;
                for chunk in bytes.chunks(2) {
                    let mut word = chunk[0] as u64;
                    if chunk.len() > 1 {
                        word |= (chunk[1] as u64) << 8;
                    }
                    self.data.push(word & mask);
                }
            }
            17..=32 => {
                let mask = (1u64 << self.value_width) - 1;
                for chunk in bytes.chunks(4) {
                    let mut word = 0u64;
                    for (i, &byte) in chunk.iter().enumerate() {
                        word |= (byte as u64) << (i * 8);
                    }
                    self.data.push(word & mask);
                }
            }
            33..=64 => {
                for chunk in bytes.chunks(8) {
                    let mut word = 0u64;
                    for (i, &byte) in chunk.iter().enumerate() {
                        word |= (byte as u64) << (i * 8);
                    }
                    self.data.push(word);
                }
            }
            _ => {
                // Invalid width, treat as bytes
                self.data = bytes.iter().map(|&b| b as u64).collect();
            }
        }
        
        self.notify_metainfo_changed();
    }
    
    /// Export to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        match self.value_width {
            1..=8 => {
                let values_per_byte = 8 / self.value_width;
                for chunk in self.data.chunks(values_per_byte) {
                    let mut byte = 0u8;
                    for (i, &value) in chunk.iter().enumerate() {
                        byte |= (value as u8) << (i * self.value_width);
                    }
                    bytes.push(byte);
                }
            }
            9..=16 => {
                for &value in &self.data {
                    bytes.push(value as u8);
                    bytes.push((value >> 8) as u8);
                }
            }
            17..=32 => {
                for &value in &self.data {
                    bytes.push(value as u8);
                    bytes.push((value >> 8) as u8);
                    bytes.push((value >> 16) as u8);
                    bytes.push((value >> 24) as u8);
                }
            }
            33..=64 => {
                for &value in &self.data {
                    for i in 0..8 {
                        bytes.push((value >> (i * 8)) as u8);
                    }
                }
            }
            _ => {
                // Invalid width, treat as bytes
                bytes = self.data.iter().map(|&v| v as u8).collect();
            }
        }
        
        bytes
    }
    
    fn notify_bytes_changed(&mut self, start: u64, num_bytes: u64, old_values: Vec<u64>) {
        // Clone listeners to avoid borrow checker issues
        let listeners_ptr = &mut self.listeners as *mut Vec<Box<dyn HexModelListener>>;
        unsafe {
            for listener in (*listeners_ptr).iter_mut() {
                listener.bytes_changed(self, start, num_bytes, &old_values);
            }
        }
    }
    
    fn notify_metainfo_changed(&mut self) {
        // Clone listeners to avoid borrow checker issues
        let listeners_ptr = &mut self.listeners as *mut Vec<Box<dyn HexModelListener>>;
        unsafe {
            for listener in (*listeners_ptr).iter_mut() {
                listener.metainfo_changed(self);
            }
        }
    }
}

impl HexModel for MemoryHexModel {
    fn add_hex_model_listener(&mut self, listener: Box<dyn HexModelListener>) {
        self.listeners.push(listener);
    }
    
    fn fill(&mut self, start: u64, length: u64, value: u64) {
        let start_idx = (start - self.first_offset) as usize;
        let end_idx = ((start + length - self.first_offset) as usize).min(self.data.len());
        
        if start_idx < self.data.len() {
            let old_values: Vec<u64> = self.data[start_idx..end_idx].to_vec();
            
            for i in start_idx..end_idx {
                self.data[i] = value;
            }
            
            self.notify_bytes_changed(start, length, old_values);
        }
    }
    
    fn get(&self, address: u64) -> u64 {
        let idx = (address - self.first_offset) as usize;
        if idx < self.data.len() {
            self.data[idx]
        } else {
            0
        }
    }
    
    fn get_first_offset(&self) -> u64 {
        self.first_offset
    }
    
    fn get_last_offset(&self) -> u64 {
        self.first_offset + self.data.len() as u64 - 1
    }
    
    fn get_value_width(&self) -> u32 {
        self.value_width
    }
    
    fn remove_hex_model_listener(&mut self, listener_id: usize) {
        // In this simple implementation, we'll just clear all listeners
        // A more sophisticated implementation would track listener IDs
        if listener_id == 0 {
            self.listeners.clear();
        }
    }
    
    fn set(&mut self, address: u64, value: u64) {
        let idx = (address - self.first_offset) as usize;
        if idx < self.data.len() {
            let old_value = self.data[idx];
            self.data[idx] = value;
            self.notify_bytes_changed(address, 1, vec![old_value]);
        }
    }
    
    fn set_range(&mut self, start: u64, values: &[u64]) {
        let start_idx = (start - self.first_offset) as usize;
        let end_idx = (start_idx + values.len()).min(self.data.len());
        
        if start_idx < self.data.len() {
            let old_values: Vec<u64> = self.data[start_idx..end_idx].to_vec();
            
            for (i, &value) in values.iter().enumerate() {
                if start_idx + i < self.data.len() {
                    self.data[start_idx + i] = value;
                }
            }
            
            self.notify_bytes_changed(start, values.len() as u64, old_values);
        }
    }
    
    fn get_id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestListener {
        bytes_changed_count: usize,
        metainfo_changed_count: usize,
    }
    
    impl TestListener {
        fn new() -> Self {
            Self {
                bytes_changed_count: 0,
                metainfo_changed_count: 0,
            }
        }
    }
    
    impl HexModelListener for TestListener {
        fn bytes_changed(&mut self, _source: &dyn HexModel, _start: u64, _num_bytes: u64, _old_values: &[u64]) {
            self.bytes_changed_count += 1;
        }
        
        fn metainfo_changed(&mut self, _source: &dyn HexModel) {
            self.metainfo_changed_count += 1;
        }
    }
    
    #[test]
    fn test_memory_hex_model_creation() {
        let model = MemoryHexModel::new(16, 8);
        assert_eq!(model.get_first_offset(), 0);
        assert_eq!(model.get_last_offset(), 15);
        assert_eq!(model.get_value_width(), 8);
        assert_eq!(model.get(0), 0);
    }
    
    #[test]
    fn test_hex_model_set_get() {
        let mut model = MemoryHexModel::new(16, 8);
        
        model.set(5, 0xFF);
        assert_eq!(model.get(5), 0xFF);
        assert_eq!(model.get(4), 0);
        assert_eq!(model.get(6), 0);
    }
    
    #[test]
    fn test_hex_model_fill() {
        let mut model = MemoryHexModel::new(16, 8);
        
        model.fill(2, 4, 0xAA);
        
        assert_eq!(model.get(1), 0);
        assert_eq!(model.get(2), 0xAA);
        assert_eq!(model.get(3), 0xAA);
        assert_eq!(model.get(4), 0xAA);
        assert_eq!(model.get(5), 0xAA);
        assert_eq!(model.get(6), 0);
    }
    
    #[test]
    fn test_hex_model_set_range() {
        let mut model = MemoryHexModel::new(16, 8);
        
        let values = vec![0x11, 0x22, 0x33, 0x44];
        model.set_range(3, &values);
        
        assert_eq!(model.get(2), 0);
        assert_eq!(model.get(3), 0x11);
        assert_eq!(model.get(4), 0x22);
        assert_eq!(model.get(5), 0x33);
        assert_eq!(model.get(6), 0x44);
        assert_eq!(model.get(7), 0);
    }
    
    #[test]
    fn test_load_from_bytes() {
        let mut model = MemoryHexModel::new(0, 8);
        let bytes = vec![0x12, 0x34, 0x56, 0x78];
        
        model.load_from_bytes(&bytes);
        
        assert_eq!(model.get_last_offset(), 3);
        assert_eq!(model.get(0), 0x12);
        assert_eq!(model.get(1), 0x34);
        assert_eq!(model.get(2), 0x56);
        assert_eq!(model.get(3), 0x78);
    }
    
    #[test]
    fn test_to_bytes() {
        let data = vec![0x12, 0x34, 0x56, 0x78];
        let model = MemoryHexModel::new_with_data(data, 0, 8);
        
        let bytes = model.to_bytes();
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }
}