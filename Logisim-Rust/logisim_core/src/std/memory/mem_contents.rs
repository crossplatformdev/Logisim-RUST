/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Memory contents implementation
//!
//! This module implements the memory storage model equivalent to MemContents.java.
//! It provides paginated memory storage with event notification.

use std::collections::HashMap;
use std::sync::{Arc, Weak, Mutex};

/// Listener for memory content changes
pub trait HexModelListener: Send + Sync {
    /// Called when bytes in memory are changed
    fn bytes_changed(&mut self, start_addr: i64, num_bytes: i32, old_values: &[i64]);
}

/// A page of memory data
#[derive(Clone, Debug)]
pub struct MemPage {
    data: Vec<i64>,
    width: i32,
}

impl MemPage {
    pub fn new(size: usize, width: i32, randomize: bool) -> Self {
        let mut data = vec![0; size];
        if randomize {
            // TODO: Implement randomization based on preferences
            // For now, just initialize to zeros
        }
        Self { data, width }
    }

    pub fn get(&self, offset: usize) -> i64 {
        self.data.get(offset).copied().unwrap_or(0)
    }

    pub fn get_range(&self, start: usize, length: usize) -> Vec<i64> {
        let end = (start + length).min(self.data.len());
        self.data[start..end].to_vec()
    }

    pub fn set(&mut self, offset: usize, value: i64) {
        if offset < self.data.len() {
            self.data[offset] = value;
        }
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}

/// Memory contents with paginated storage
#[derive(Clone)]
pub struct MemContents {
    width: i32,
    addr_bits: i32,
    mask: i64,
    pages: HashMap<usize, MemPage>,
    randomize: bool,
    listeners: Arc<Mutex<Vec<Weak<Mutex<dyn HexModelListener>>>>>,
}

impl MemContents {
    const PAGE_SIZE_BITS: i32 = 12;
    const PAGE_SIZE: usize = 1 << Self::PAGE_SIZE_BITS;
    const PAGE_MASK: usize = Self::PAGE_SIZE - 1;

    /// Create new memory contents
    pub fn create(addr_bits: i32, width: i32, randomize: bool) -> Self {
        let mask = if width >= 64 { -1 } else { (1i64 << width) - 1 };
        
        Self {
            width,
            addr_bits,
            mask,
            pages: HashMap::new(),
            randomize,
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a listener for memory changes
    pub fn add_hex_model_listener(&mut self, listener: Arc<Mutex<dyn HexModelListener>>) {
        if let Ok(mut listeners) = self.listeners.lock() {
            listeners.push(Arc::downgrade(&listener));
        }
    }

    /// Remove a listener
    pub fn remove_hex_model_listener(&mut self, listener: &Arc<Mutex<dyn HexModelListener>>) {
        if let Ok(mut listeners) = self.listeners.lock() {
            listeners.retain(|weak| {
                weak.upgrade().map_or(false, |strong| !Arc::ptr_eq(&strong, listener))
            });
        }
    }

    /// Clear all memory contents
    pub fn clear(&mut self) {
        let page_indices: Vec<usize> = self.pages.keys().copied().collect();
        for page_index in page_indices {
            self.clear_page(page_index);
        }
    }

    /// Conditionally clear memory (based on preferences)
    /// TODO: Implement preference-based clearing
    pub fn cond_clear(&mut self) {
        // For now, just clear everything
        self.clear();
    }

    /// Clear a specific page
    fn clear_page(&mut self, page_index: usize) {
        if let Some(page) = self.pages.get(&page_index) {
            let old_values = page.get_range(0, page.length());
            let changed = old_values.iter().any(|&val| (val & self.mask) != 0);
            
            if changed {
                self.pages.remove(&page_index);
                let start_addr = (page_index << Self::PAGE_SIZE_BITS) as i64;
                self.fire_bytes_changed(start_addr, old_values.len() as i32, &old_values);
            }
        }
    }

    /// Get value at address
    pub fn get(&self, addr: i64) -> i64 {
        if addr < 0 {
            return 0;
        }

        let page_index = (addr as usize) >> Self::PAGE_SIZE_BITS;
        let offset = (addr as usize) & Self::PAGE_MASK;

        self.pages.get(&page_index)
            .map_or(0, |page| page.get(offset) & self.mask)
    }

    /// Set value at address
    pub fn set(&mut self, addr: i64, value: i64) {
        if addr < 0 {
            return;
        }

        let page_index = (addr as usize) >> Self::PAGE_SIZE_BITS;
        let offset = (addr as usize) & Self::PAGE_MASK;
        let masked_value = value & self.mask;

        // Get or create page
        if !self.pages.contains_key(&page_index) {
            if masked_value == 0 {
                return; // No need to create page for zero value
            }
            self.pages.insert(page_index, MemPage::new(Self::PAGE_SIZE, self.width, self.randomize));
        }

        if let Some(page) = self.pages.get_mut(&page_index) {
            let old_value = page.get(offset);
            if old_value != masked_value {
                page.set(offset, masked_value);
                self.fire_bytes_changed(addr, 1, &[old_value]);
            }
        }
    }

    /// Fill a range of memory with a value
    pub fn fill(&mut self, start: i64, len: i64, value: i64) {
        for addr in start..(start + len) {
            self.set(addr, value);
        }
    }

    /// Get the width of memory values
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Get the value width (same as width)
    pub fn get_value_width(&self) -> i32 {
        self.width
    }

    /// Get the logarithm of memory length
    pub fn get_log_length(&self) -> i32 {
        self.addr_bits
    }

    /// Get first non-zero offset
    pub fn get_first_offset(&self) -> i64 {
        self.pages.keys()
            .min()
            .map(|&page_index| (page_index << Self::PAGE_SIZE_BITS) as i64)
            .unwrap_or(0)
    }

    /// Get last non-zero offset
    pub fn get_last_offset(&self) -> i64 {
        self.pages.keys()
            .max()
            .map(|&page_index| ((page_index + 1) << Self::PAGE_SIZE_BITS - 1) as i64)
            .unwrap_or(0)
    }

    /// Check if memory is completely clear
    pub fn is_clear(&self) -> bool {
        self.pages.is_empty()
    }

    /// Fire bytes changed event to listeners
    fn fire_bytes_changed(&self, start_addr: i64, num_bytes: i32, old_values: &[i64]) {
        if let Ok(listeners) = self.listeners.lock() {
            let mut to_remove = Vec::new();
            for (index, weak_listener) in listeners.iter().enumerate() {
                if let Some(listener) = weak_listener.upgrade() {
                    if let Ok(mut listener) = listener.lock() {
                        listener.bytes_changed(start_addr, num_bytes, old_values);
                    }
                } else {
                    to_remove.push(index);
                }
            }
            // Clean up dead references (would need mutable access)
        }
    }
}

/// Sub-component for memory contents (equivalent to MemContentsSub.java)
pub struct MemContentsSub;

impl MemContentsSub {
    /// Create a memory page
    pub fn create_page(size: usize, width: i32, randomize: bool) -> MemPage {
        MemPage::new(size, width, randomize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct TestListener {
        events: Vec<(i64, i32, Vec<i64>)>,
    }

    impl TestListener {
        fn new() -> Self {
            Self { events: Vec::new() }
        }
    }

    impl HexModelListener for TestListener {
        fn bytes_changed(&mut self, start_addr: i64, num_bytes: i32, old_values: &[i64]) {
            self.events.push((start_addr, num_bytes, old_values.to_vec()));
        }
    }

    #[test]
    fn test_mem_contents_creation() {
        let mem = MemContents::create(10, 8, false);
        assert_eq!(mem.get_width(), 8);
        assert_eq!(mem.get_log_length(), 10);
        assert!(mem.is_clear());
    }

    #[test]
    fn test_mem_contents_basic_operations() {
        let mut mem = MemContents::create(10, 8, false);
        
        // Test set/get
        mem.set(100, 0xFF);
        assert_eq!(mem.get(100), 0xFF);
        assert!(!mem.is_clear());
        
        // Test masking
        mem.set(101, 0x1FF); // Should be masked to 0xFF for 8-bit width
        assert_eq!(mem.get(101), 0xFF);
    }

    #[test]
    fn test_mem_contents_clear() {
        let mut mem = MemContents::create(10, 8, false);
        mem.set(100, 0xFF);
        assert!(!mem.is_clear());
        
        mem.clear();
        assert!(mem.is_clear());
        assert_eq!(mem.get(100), 0);
    }

    #[test]
    fn test_mem_contents_fill() {
        let mut mem = MemContents::create(10, 8, false);
        mem.fill(100, 5, 0xAA);
        
        for addr in 100..105 {
            assert_eq!(mem.get(addr), 0xAA);
        }
    }

    #[test]
    fn test_mem_page() {
        let mut page = MemPage::new(100, 8, false);
        page.set(50, 0xFF);
        assert_eq!(page.get(50), 0xFF);
        assert_eq!(page.length(), 100);
        
        let range = page.get_range(49, 3);
        assert_eq!(range, vec![0, 0xFF, 0]);
    }
}