/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Measures - Layout calculations for hex editor
//!
//! Rust port of Measures.java

use super::hex_model::HexModel;

/// Manages layout calculations and measurements for the hex editor
pub struct Measures {
    // Character and cell dimensions
    header_chars: usize,
    cell_chars: usize,
    header_width: f32,
    spacer_width: f32,
    cell_width: f32,
    cell_height: f32,
    
    // Layout parameters
    cols: usize,
    base_x: f32,
    
    // Font metrics
    char_width: f32,
    line_height: f32,
    
    // Computed flags
    guessed: bool,
    
    // Cached preferred size
    preferred_width: f32,
    preferred_height: f32,
}

impl Measures {
    /// Create new measures instance
    pub fn new() -> Self {
        let mut measures = Self {
            header_chars: 4,
            cell_chars: 2,
            header_width: 0.0,
            spacer_width: 0.0,
            cell_width: 0.0,
            cell_height: 0.0,
            cols: 1,
            base_x: 0.0,
            char_width: 8.0,
            line_height: 16.0,
            guessed: true,
            preferred_width: 0.0,
            preferred_height: 0.0,
        };
        
        measures.compute_cell_size(None);
        measures
    }
    
    /// Recompute all measurements
    pub fn recompute(&mut self, model: Option<&dyn HexModel>) {
        self.compute_cell_size(model);
    }
    
    /// Ensure measurements are computed
    pub fn ensure_computed(&mut self, model: Option<&dyn HexModel>) {
        if self.guessed || self.cell_width <= 0.0 {
            self.compute_cell_size(model);
        }
    }
    
    /// Get base address for display
    pub fn get_base_address(&self, model: Option<&dyn HexModel>) -> u64 {
        match model {
            Some(model) => {
                let addr0 = model.get_first_offset();
                addr0 - (addr0 % self.cols as u64)
            }
            None => 0,
        }
    }
    
    /// Get base X coordinate
    pub fn get_base_x(&self) -> f32 {
        self.base_x
    }
    
    /// Get cell character count
    pub fn get_cell_chars(&self) -> usize {
        self.cell_chars
    }
    
    /// Get cell height
    pub fn get_cell_height(&self) -> f32 {
        self.cell_height
    }
    
    /// Get cell width
    pub fn get_cell_width(&self) -> f32 {
        self.cell_width
    }
    
    /// Get column count
    pub fn get_column_count(&self) -> usize {
        self.cols
    }
    
    /// Get label character count
    pub fn get_label_chars(&self) -> usize {
        self.header_chars
    }
    
    /// Get label width
    pub fn get_label_width(&self) -> f32 {
        self.header_width
    }
    
    /// Get values area width
    pub fn get_values_width(&self) -> f32 {
        ((self.cols - 1) / 4) as f32 * self.spacer_width + self.cols as f32 * self.cell_width
    }
    
    /// Get values area X coordinate
    pub fn get_values_x(&self) -> f32 {
        self.base_x + self.spacer_width
    }
    
    /// Convert screen coordinates to address
    pub fn to_address(&self, x: f32, y: f32, model: Option<&dyn HexModel>) -> Option<u64> {
        let model = model?;
        let addr0 = model.get_first_offset();
        let addr1 = model.get_last_offset();
        
        let base = self.get_base_address(Some(model)) + ((y / self.cell_height) as u64) * self.cols as u64;
        let mut offs = ((x - self.base_x) / (self.cell_width + (self.spacer_width + 2.0) / 4.0)) as i32;
        
        if offs < 0 {
            offs = 0;
        }
        if offs >= self.cols as i32 {
            offs = self.cols as i32 - 1;
        }
        
        let mut ret = base + offs as u64;
        if ret > addr1 {
            ret = addr1;
        }
        if ret < addr0 {
            ret = addr0;
        }
        
        Some(ret)
    }
    
    /// Convert address to X coordinate
    pub fn to_x(&self, addr: u64, model: Option<&dyn HexModel>) -> f32 {
        let col = (addr % self.cols as u64) as usize;
        self.base_x + (1 + (col / 4)) as f32 * self.spacer_width + col as f32 * self.cell_width
    }
    
    /// Convert address to Y coordinate
    pub fn to_y(&self, addr: u64, model: Option<&dyn HexModel>) -> f32 {
        let base_addr = self.get_base_address(model);
        let row = (addr.saturating_sub(base_addr)) / self.cols as u64;
        row as f32 * self.cell_height
    }
    
    /// Handle width change
    pub fn width_changed(&mut self, width: f32, model: Option<&dyn HexModel>) -> bool {
        let old_cols = self.cols;
        let old_base_x = self.base_x;
        
        let available_width = if self.guessed || self.cell_width <= 0.0 {
            self.cols = 16;
            self.preferred_width
        } else {
            let ret = ((width - self.header_width) / (self.cell_width + (self.spacer_width + 3.0) / 4.0)) as usize;
            self.cols = if ret >= 16 {
                16
            } else if ret >= 8 {
                8
            } else {
                4
            };
            width
        };
        
        let line_width = self.header_width + self.cols as f32 * self.cell_width + ((self.cols / 4) as f32 - 1.0) * self.spacer_width;
        let new_base = self.header_width + (available_width - line_width).max(0.0) / 2.0;
        
        let changed = self.base_x != new_base || self.cols != old_cols;
        self.base_x = new_base;
        
        if self.cols != old_cols {
            self.recompute(model);
        }
        
        changed
    }
    
    /// Get preferred size
    pub fn get_preferred_size(&self) -> (f32, f32) {
        (self.preferred_width, self.preferred_height)
    }
    
    /// Compute cell sizes and layout
    fn compute_cell_size(&mut self, model: Option<&dyn HexModel>) {
        // Compute number of characters in headers and cells
        match model {
            Some(model) => {
                let mut log_size = 0;
                let mut addr_end = model.get_last_offset();
                while addr_end > (1u64 << log_size) {
                    log_size += 1;
                }
                self.header_chars = ((log_size + 3) / 4) as usize;
                self.cell_chars = ((model.get_value_width() + 3) / 4) as usize;
            }
            None => {
                self.header_chars = 4;
                self.cell_chars = 2;
            }
        }
        
        // Use default font metrics (could be made configurable)
        self.char_width = 8.0;
        let space_width = 6.0;
        self.line_height = 16.0;
        self.guessed = true; // We're guessing font metrics
        
        // Update header and cell dimensions
        self.header_width = self.header_chars as f32 * self.char_width + space_width;
        self.spacer_width = space_width;
        self.cell_width = self.cell_chars as f32 * self.char_width + space_width;
        self.cell_height = self.line_height;
        
        // Compute preferred size
        self.preferred_width = self.header_width + self.cols as f32 * self.cell_width + (self.cols / 4) as f32 * self.spacer_width;
        
        match model {
            Some(model) => {
                let addr0 = self.get_base_address(Some(model));
                let addr1 = model.get_last_offset();
                let rows = ((addr1 - addr0 + 1) + self.cols as u64 - 1) / self.cols as u64;
                self.preferred_height = rows as f32 * self.cell_height;
            }
            None => {
                self.preferred_height = 16.0 * self.cell_height;
            }
        }
    }
}

impl Default for Measures {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::hex_model::MemoryHexModel;
    
    #[test]
    fn test_measures_creation() {
        let measures = Measures::new();
        assert!(measures.get_cell_height() > 0.0);
        assert!(measures.get_cell_width() > 0.0);
        assert_eq!(measures.get_column_count(), 1);
    }
    
    #[test]
    fn test_address_conversion() {
        let mut measures = Measures::new();
        let model = MemoryHexModel::new(256, 8);
        
        measures.recompute(Some(&model));
        
        // Test coordinate conversion
        let addr = 0x10;
        let x = measures.to_x(addr, Some(&model));
        let y = measures.to_y(addr, Some(&model));
        
        assert!(x >= 0.0);
        assert!(y >= 0.0);
        
        // Test round-trip conversion (approximate due to floating point)
        if let Some(converted_addr) = measures.to_address(x, y, Some(&model)) {
            assert_eq!(converted_addr, addr);
        }
    }
    
    #[test]
    fn test_width_change() {
        let mut measures = Measures::new();
        let model = MemoryHexModel::new(256, 8);
        
        let changed = measures.width_changed(800.0, Some(&model));
        assert!(changed); // Should change from initial state
        
        let cols_after_resize = measures.get_column_count();
        assert!(cols_after_resize >= 4);
    }
    
    #[test]
    fn test_base_address() {
        let measures = Measures::new();
        let mut model = MemoryHexModel::new(256, 8);
        model.set_first_offset(0x1007);
        
        let base = measures.get_base_address(Some(&model));
        // Base address should be aligned to column boundary
        assert_eq!(base % measures.get_column_count() as u64, 0);
        assert!(base <= 0x1007);
    }
}