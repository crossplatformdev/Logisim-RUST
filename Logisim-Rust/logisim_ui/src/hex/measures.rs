/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

use super::HexModel;

/// Handles layout calculations and measurements for the hex editor
/// 
/// This struct manages the dimensions, positioning, and coordinate
/// conversions needed to display hex data in a grid layout.
pub struct Measures {
    header_chars: u32,
    cell_chars: u32,
    header_width: i32,
    spacer_width: i32,
    cell_width: i32,
    cell_height: i32,
    cols: u32,
    base_x: i32,
    value_width: u32,
    computed: bool,
}

impl Measures {
    /// Create a new Measures instance
    pub fn new(value_width: u32, cols: u32) -> Self {
        let mut measures = Self {
            header_chars: 4,
            cell_chars: 2,
            header_width: 0,
            spacer_width: 0,
            cell_width: 0,
            cell_height: 0,
            cols,
            base_x: 0,
            value_width,
            computed: false,
        };
        measures.compute_cell_size();
        measures
    }

    fn compute_cell_size(&mut self) {
        // Compute number of characters in headers and cells
        self.header_chars = if self.value_width == 0 {
            4
        } else {
            // Calculate hex digits needed for largest address
            let mut log_size = 0;
            let mut addr_end = 1u64 << 16; // Default assumption
            while addr_end > (1u64 << log_size) {
                log_size += 1;
            }
            (log_size + 3) / 4
        };

        self.cell_chars = if self.value_width == 0 {
            2
        } else {
            (self.value_width + 3) / 4
        };

        // Use default character sizes (in a real implementation, these would come from font metrics)
        let char_width = 8;
        let space_width = 6;
        let line_height = 16;

        // Update header and cell dimensions
        self.header_width = (self.header_chars * char_width as u32 + space_width as u32) as i32;
        self.spacer_width = space_width;
        self.cell_width = (self.cell_chars * char_width as u32 + space_width as u32) as i32;
        self.cell_height = line_height;

        self.computed = true;
        self.width_changed(800); // Default width assumption
    }

    /// Get the base address for display (aligned to column boundary)
    pub fn get_base_address(&self, first_offset: u64) -> u64 {
        first_offset - (first_offset % self.cols as u64)
    }

    /// Get the X coordinate for the base of the display
    pub fn get_base_x(&self) -> i32 {
        self.base_x
    }

    /// Get the number of characters per cell
    pub fn get_cell_chars(&self) -> u32 {
        self.cell_chars
    }

    /// Get the height of each cell in pixels
    pub fn get_cell_height(&self) -> i32 {
        self.cell_height
    }

    /// Get the width of each cell in pixels
    pub fn get_cell_width(&self) -> i32 {
        self.cell_width
    }

    /// Get the number of columns in the display
    pub fn get_column_count(&self) -> u32 {
        self.cols
    }

    /// Get the number of characters in address labels
    pub fn get_label_chars(&self) -> u32 {
        self.header_chars
    }

    /// Get the width of address labels
    pub fn get_label_width(&self) -> i32 {
        self.header_width
    }

    /// Get the width of the values area
    pub fn get_values_width(&self) -> i32 {
        ((self.cols - 1) / 4) as i32 * self.spacer_width + self.cols as i32 * self.cell_width
    }

    /// Get the X coordinate of the values area
    pub fn get_values_x(&self) -> i32 {
        self.base_x + self.spacer_width
    }

    /// Get the current value width in bits
    pub fn get_value_width(&self) -> u32 {
        self.value_width
    }

    /// Update measurements when model changes
    pub fn recompute(&mut self, value_width: u32) {
        self.value_width = value_width;
        self.compute_cell_size();
    }

    /// Convert screen coordinates to memory address
    pub fn to_address(&self, x: i32, y: i32, first_offset: u64, last_offset: u64) -> Option<u64> {
        if !self.computed {
            return None;
        }

        let base = self.get_base_address(first_offset) + ((y / self.cell_height) as u64 * self.cols as u64);
        let mut offs = (x - self.base_x) / (self.cell_width + (self.spacer_width + 2) / 4);
        if offs < 0 {
            offs = 0;
        }
        if offs >= self.cols as i32 {
            offs = self.cols as i32 - 1;
        }

        let mut ret = base + offs as u64;
        if ret > last_offset {
            ret = last_offset;
        }
        if ret < first_offset {
            ret = first_offset;
        }
        Some(ret)
    }

    /// Convert memory address to X coordinate
    pub fn to_x(&self, addr: u64) -> i32 {
        let col = (addr % self.cols as u64) as i32;
        self.base_x + (1 + (col / 4)) * self.spacer_width + col * self.cell_width
    }

    /// Convert memory address to Y coordinate
    pub fn to_y(&self, addr: u64, base_address: u64) -> i32 {
        let row = (addr - base_address) / self.cols as u64;
        let ret = row * self.cell_height as u64;
        if ret < i32::MAX as u64 {
            ret as i32
        } else {
            i32::MAX
        }
    }

    /// Update layout when display width changes
    pub fn width_changed(&mut self, width: i32) {
        if !self.computed || self.cell_width <= 0 {
            self.cols = 16;
        } else {
            let ret = (width - self.header_width) / (self.cell_width + (self.spacer_width + 3) / 4);
            self.cols = if ret >= 16 {
                16
            } else if ret >= 8 {
                8
            } else {
                4
            };
        }

        let line_width = self.header_width + self.cols as i32 * self.cell_width + ((self.cols / 4) as i32 - 1) * self.spacer_width;
        let new_base = self.header_width + std::cmp::max(0, (width - line_width) / 2);
        self.base_x = new_base;
    }

    /// Check if measurements are computed
    pub fn is_computed(&self) -> bool {
        self.computed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measures_creation() {
        let measures = Measures::new(8, 16);
        
        assert_eq!(measures.get_value_width(), 8);
        assert_eq!(measures.get_column_count(), 16);
        assert!(measures.is_computed());
    }

    #[test]
    fn test_base_address_calculation() {
        let measures = Measures::new(8, 16);
        
        // Test alignment to column boundary
        assert_eq!(measures.get_base_address(0), 0);
        assert_eq!(measures.get_base_address(10), 0);
        assert_eq!(measures.get_base_address(16), 16);
        assert_eq!(measures.get_base_address(20), 16);
    }

    #[test]
    fn test_coordinate_conversions() {
        let measures = Measures::new(8, 16);
        
        // Test X coordinate calculation
        let x0 = measures.to_x(0);
        let x1 = measures.to_x(1);
        let x16 = measures.to_x(16);
        
        assert!(x1 > x0);
        assert!(x16 > x1);
        
        // Test Y coordinate calculation
        let y0 = measures.to_y(0, 0);
        let y16 = measures.to_y(16, 0);
        
        assert_eq!(y0, 0);
        assert!(y16 > y0);
    }

    #[test]
    fn test_address_from_coordinates() {
        let measures = Measures::new(8, 16);
        
        // Test conversion from screen coordinates to address
        let base_x = measures.get_base_x();
        let cell_width = measures.get_cell_width();
        let cell_height = measures.get_cell_height();
        
        // First cell should map to address 0
        if let Some(addr) = measures.to_address(base_x + cell_width / 2, cell_height / 2, 0, 255) {
            assert_eq!(addr, 0);
        }
        
        // Test bounds checking
        if let Some(addr) = measures.to_address(-100, 0, 0, 255) {
            assert_eq!(addr, 0); // Should clamp to first offset
        }
    }

    #[test]
    fn test_width_changes() {
        let mut measures = Measures::new(8, 16);
        
        // Test with different widths
        measures.width_changed(400);
        assert!(measures.get_column_count() >= 4);
        
        measures.width_changed(800);
        assert!(measures.get_column_count() >= 8);
        
        measures.width_changed(1200);
        assert!(measures.get_column_count() <= 16);
    }

    #[test]
    fn test_recompute() {
        let mut measures = Measures::new(8, 16);
        let original_cell_chars = measures.get_cell_chars();
        
        // Recompute with different value width
        measures.recompute(16);
        assert_eq!(measures.get_value_width(), 16);
        
        // Cell chars should change for wider values
        let new_cell_chars = measures.get_cell_chars();
        assert!(new_cell_chars >= original_cell_chars);
    }
}