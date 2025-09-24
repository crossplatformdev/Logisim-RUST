/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

use super::{HexModel, HexModelListener, Caret, Highlighter, Measures, HighlightHandle, Color};
use std::cell::RefCell;
use std::rc::Rc;

/// Main hex editor component that displays and allows editing of binary data
/// 
/// The HexEditor combines a data model, cursor management, highlighting,
/// and layout calculations to provide a complete hex editing interface.
pub struct HexEditor {
    model: Option<Rc<RefCell<dyn HexModel>>>,
    caret: Caret,
    highlighter: Rc<RefCell<Highlighter>>,
    measures: Rc<Measures>,
    background_color: Color,
    foreground_color: Color,
    listeners: Vec<Box<dyn HexModelListener>>,
}

impl HexEditor {
    /// Create a new hex editor with the given model
    pub fn new(model: Option<Rc<RefCell<dyn HexModel>>>) -> Self {
        let value_width = if let Some(ref model) = model {
            model.borrow().get_value_width()
        } else {
            8
        };

        let measures = Rc::new(Measures::new(value_width, 16));
        let highlighter = Rc::new(RefCell::new(Highlighter::new()));
        let mut caret = Caret::new(model.clone());
        
        caret.set_measures(measures.clone());
        caret.set_highlighter(highlighter.clone());

        Self {
            model,
            caret,
            highlighter,
            measures,
            background_color: (255, 255, 255), // White
            foreground_color: (0, 0, 0),       // Black
            listeners: Vec::new(),
        }
    }

    /// Get the current data model
    pub fn get_model(&self) -> Option<Rc<RefCell<dyn HexModel>>> {
        self.model.clone()
    }

    /// Set a new data model
    pub fn set_model(&mut self, model: Option<Rc<RefCell<dyn HexModel>>>) {
        // Clear existing state
        self.highlighter.borrow_mut().clear();
        self.caret.set_dot(None, false);

        // Update model
        self.model = model.clone();

        // Update measurements if model changed
        if let Some(ref model) = model {
            let value_width = model.borrow().get_value_width();
            self.measures = Rc::new(Measures::new(value_width, self.measures.get_column_count()));
            self.caret.set_measures(self.measures.clone());
        }
    }

    /// Get the caret (cursor and selection manager)
    pub fn get_caret(&mut self) -> &mut Caret {
        &mut self.caret
    }

    /// Get the measures (layout calculations)
    pub fn get_measures(&self) -> &Measures {
        &self.measures
    }

    /// Get the highlighter
    pub fn get_highlighter(&self) -> Rc<RefCell<Highlighter>> {
        self.highlighter.clone()
    }

    /// Add a highlight for the given address range
    /// 
    /// Returns a handle that can be used to remove the highlight later.
    pub fn add_highlight(&self, start: u64, end: u64, color: Color) -> Option<HighlightHandle> {
        self.highlighter.borrow_mut().add(start, end, color)
    }

    /// Remove a highlight by handle
    pub fn remove_highlight(&self, handle: HighlightHandle) -> bool {
        self.highlighter.borrow_mut().remove(handle)
    }

    /// Delete the selected range (fill with zeros)
    pub fn delete_selection(&mut self) {
        if let Some((start, end)) = self.caret.get_selection() {
            if let Some(ref model) = self.model {
                let length = end - start + 1;
                model.borrow_mut().fill(start, length, 0);
            }
        }
    }

    /// Check if there is an active selection
    pub fn selection_exists(&self) -> bool {
        self.caret.has_selection()
    }

    /// Select all data
    pub fn select_all(&mut self) {
        self.caret.select_all();
    }

    /// Set the background color
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Set the foreground color
    pub fn set_foreground_color(&mut self, color: Color) {
        self.foreground_color = color;
    }

    /// Get the background color
    pub fn get_background_color(&self) -> Color {
        self.background_color
    }

    /// Get the foreground color
    pub fn get_foreground_color(&self) -> Color {
        self.foreground_color
    }

    /// Handle key input for hex editing
    /// 
    /// # Arguments
    /// * `key_char` - The character that was typed
    /// * `ctrl_pressed` - Whether Ctrl key is held
    /// * `shift_pressed` - Whether Shift key is held
    /// 
    /// Returns true if the key was handled.
    pub fn handle_key_input(&mut self, key_char: char, ctrl_pressed: bool, shift_pressed: bool) -> bool {
        match key_char {
            // Navigation keys
            ' ' => {
                if ctrl_pressed {
                    self.caret.page_down(shift_pressed, 10); // Assume 10 visible rows
                } else {
                    self.caret.move_right(shift_pressed);
                }
                true
            }
            '\n' => {
                if ctrl_pressed {
                    self.caret.move_up(shift_pressed);
                } else {
                    self.caret.move_down(shift_pressed);
                }
                true
            }
            '\u{0008}' => { // Backspace
                self.caret.move_left(shift_pressed);
                true
            }
            '\u{007f}' => { // Delete
                if ctrl_pressed {
                    self.caret.page_up(shift_pressed, 10); // Assume 10 visible rows
                } else {
                    self.delete_selection();
                }
                true
            }
            // Hex digit input
            c if c.is_ascii_hexdigit() => {
                if let Some(digit) = c.to_digit(16) {
                    self.input_hex_digit(digit as u8);
                }
                true
            }
            _ => false,
        }
    }

    /// Handle special key presses (arrow keys, etc.)
    /// 
    /// # Arguments
    /// * `key_code` - The key code (using common key code constants)
    /// * `shift_pressed` - Whether Shift key is held
    /// 
    /// Returns true if the key was handled.
    pub fn handle_key_press(&mut self, key_code: u32, shift_pressed: bool) -> bool {
        match key_code {
            37 => { // Left arrow
                self.caret.move_left(shift_pressed);
                true
            }
            38 => { // Up arrow
                self.caret.move_up(shift_pressed);
                true
            }
            39 => { // Right arrow
                self.caret.move_right(shift_pressed);
                true
            }
            40 => { // Down arrow
                self.caret.move_down(shift_pressed);
                true
            }
            36 => { // Home
                self.caret.move_to_line_start(shift_pressed);
                true
            }
            35 => { // End
                self.caret.move_to_line_end(shift_pressed);
                true
            }
            33 => { // Page Up
                self.caret.page_up(shift_pressed, 10); // Assume 10 visible rows
                true
            }
            34 => { // Page Down
                self.caret.page_down(shift_pressed, 10); // Assume 10 visible rows
                true
            }
            _ => false,
        }
    }

    /// Handle mouse click
    /// 
    /// # Arguments
    /// * `x` - X coordinate of the click
    /// * `y` - Y coordinate of the click
    /// * `shift_pressed` - Whether Shift key is held
    pub fn handle_mouse_click(&mut self, x: i32, y: i32, shift_pressed: bool) {
        self.caret.set_cursor_from_coordinates(x, y, shift_pressed);
    }

    /// Handle mouse drag
    /// 
    /// # Arguments
    /// * `x` - X coordinate of the drag
    /// * `y` - Y coordinate of the drag
    pub fn handle_mouse_drag(&mut self, x: i32, y: i32) {
        self.caret.set_cursor_from_coordinates(x, y, true);
    }

    /// Input a hex digit at the current cursor position
    fn input_hex_digit(&mut self, digit: u8) {
        if let (Some(cursor), Some(ref model)) = (self.caret.get_dot(), &self.model) {
            let mut model_ref = model.borrow_mut();
            let current_value = model_ref.get(cursor);
            let new_value = (current_value * 16 + digit as u64) & self.get_value_mask();
            
            model_ref.set(cursor, new_value);
            
            // Move cursor to next position
            drop(model_ref); // Release borrow
            self.caret.move_right(false);
        }
    }

    /// Get the value mask based on the model's value width
    fn get_value_mask(&self) -> u64 {
        if let Some(ref model) = self.model {
            let width = model.borrow().get_value_width();
            if width >= 64 {
                u64::MAX
            } else {
                (1u64 << width) - 1
            }
        } else {
            0xFF // Default to 8-bit mask
        }
    }

    /// Format a value as hex string with appropriate width
    pub fn format_hex(&self, value: u64, chars: u32) -> String {
        format!("{:0width$x}", value, width = chars as usize)
    }

    /// Get visible address range for rendering
    /// 
    /// # Arguments
    /// * `clip_y` - Top of visible area
    /// * `clip_height` - Height of visible area
    /// 
    /// Returns (start_address, end_address) or None if no model
    pub fn get_visible_range(&self, clip_y: i32, clip_height: i32) -> Option<(u64, u64)> {
        if let Some(ref model) = self.model {
            let model_ref = model.borrow();
            let first_offset = model_ref.get_first_offset();
            let last_offset = model_ref.get_last_offset();
            let base_address = self.measures.get_base_address(first_offset);
            
            // Calculate visible address range from screen coordinates
            let cols = self.measures.get_column_count() as u64;
            let cell_height = self.measures.get_cell_height();
            
            let start_row = (clip_y / cell_height) as u64;
            let end_row = ((clip_y + clip_height) / cell_height + 1) as u64;
            
            let start_addr = base_address + start_row * cols;
            let end_addr = std::cmp::min(base_address + end_row * cols, last_offset + 1);
            
            Some((std::cmp::max(start_addr, first_offset), end_addr))
        } else {
            None
        }
    }

    /// Get rendering information for the visible range
    /// 
    /// Returns a vector of (address, x, y, hex_string, is_cursor) tuples
    /// for rendering the hex display.
    pub fn get_render_info(&self, clip_y: i32, clip_height: i32) -> Vec<(u64, i32, i32, String, bool)> {
        let mut render_info = Vec::new();
        
        if let Some(ref model) = self.model {
            let model_ref = model.borrow();
            
            if let Some((start_addr, end_addr)) = self.get_visible_range(clip_y, clip_height) {
                let first_offset = model_ref.get_first_offset();
                let last_offset = model_ref.get_last_offset();
                let base_address = self.measures.get_base_address(first_offset);
                let cell_chars = self.measures.get_cell_chars();
                let cursor_pos = self.caret.get_dot();
                
                for addr in start_addr..end_addr {
                    if addr >= first_offset && addr <= last_offset {
                        let value = model_ref.get(addr);
                        let hex_string = self.format_hex(value, cell_chars);
                        let x = self.measures.to_x(addr);
                        let y = self.measures.to_y(addr, base_address);
                        let is_cursor = cursor_pos == Some(addr);
                        
                        render_info.push((addr, x, y, hex_string, is_cursor));
                    }
                }
            }
        }
        
        render_info
    }

    /// Get address label information for rendering
    /// 
    /// Returns a vector of (base_address, x, y, label_string) tuples
    pub fn get_label_info(&self, clip_y: i32, clip_height: i32) -> Vec<(u64, i32, i32, String)> {
        let mut label_info = Vec::new();
        
        if let Some(ref model) = self.model {
            let model_ref = model.borrow();
            
            if let Some((start_addr, end_addr)) = self.get_visible_range(clip_y, clip_height) {
                let first_offset = model_ref.get_first_offset();
                let base_address = self.measures.get_base_address(first_offset);
                let cols = self.measures.get_column_count() as u64;
                let label_chars = self.measures.get_label_chars();
                let label_width = self.measures.get_label_width();
                let base_x = self.measures.get_base_x();
                
                let start_row = (start_addr - base_address) / cols;
                let end_row = (end_addr - base_address + cols - 1) / cols;
                
                for row in start_row..end_row {
                    let row_addr = base_address + row * cols;
                    let label = self.format_hex(row_addr, label_chars);
                    let x = base_x - label_width + label_width / 2; // Center the label
                    let y = self.measures.to_y(row_addr, base_address);
                    
                    label_info.push((row_addr, x, y, label));
                }
            }
        }
        
        label_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::VecHexModel;

    #[test]
    fn test_hex_editor_creation() {
        let model = Rc::new(RefCell::new(VecHexModel::new(256, 8)));
        let editor = HexEditor::new(Some(model.clone()));
        
        assert!(editor.get_model().is_some());
        assert!(!editor.selection_exists());
        assert_eq!(editor.get_background_color(), (255, 255, 255));
        assert_eq!(editor.get_foreground_color(), (0, 0, 0));
    }

    #[test]
    fn test_model_management() {
        let mut editor = HexEditor::new(None);
        assert!(editor.get_model().is_none());
        
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        editor.set_model(Some(model.clone()));
        assert!(editor.get_model().is_some());
        
        editor.set_model(None);
        assert!(editor.get_model().is_none());
    }

    #[test]
    fn test_highlight_operations() {
        let model = Rc::new(RefCell::new(VecHexModel::new(256, 8)));
        let editor = HexEditor::new(Some(model));
        
        // Test add highlight
        let handle = editor.add_highlight(0, 10, (255, 255, 0));
        assert!(handle.is_some());
        
        // Test remove highlight
        if let Some(h) = handle {
            assert!(editor.remove_highlight(h));
        }
    }

    #[test]
    fn test_selection_operations() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut editor = HexEditor::new(Some(model));
        
        // Initially no selection
        assert!(!editor.selection_exists());
        
        // Create selection through caret
        editor.get_caret().set_dot(Some(10), false);
        editor.get_caret().set_dot(Some(20), true);
        assert!(editor.selection_exists());
        
        // Test select all
        editor.select_all();
        assert!(editor.selection_exists());
        
        // Test delete selection
        editor.delete_selection();
        // Selection should still exist but data should be zeroed
        assert!(editor.selection_exists());
    }

    #[test]
    fn test_key_input_handling() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut editor = HexEditor::new(Some(model.clone()));
        
        // Set cursor position
        editor.get_caret().set_dot(Some(10), false);
        
        // Test hex digit input
        assert!(editor.handle_key_input('F', false, false));
        
        // Check that value was updated
        let new_value = model.borrow().get(10);
        assert_eq!(new_value, 15); // 'F' in hex
        
        // Test navigation keys
        assert!(editor.handle_key_input(' ', false, false)); // Space (move right)
        assert!(editor.handle_key_input('\n', false, false)); // Enter (move down)
        assert!(editor.handle_key_input('\u{0008}', false, false)); // Backspace (move left)
        
        // Test unhandled key
        assert!(!editor.handle_key_input('Z', false, false));
    }

    #[test]
    fn test_key_press_handling() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut editor = HexEditor::new(Some(model));
        
        // Set cursor position
        editor.get_caret().set_dot(Some(10), false);
        
        // Test arrow keys
        assert!(editor.handle_key_press(37, false)); // Left
        assert!(editor.handle_key_press(38, false)); // Up
        assert!(editor.handle_key_press(39, false)); // Right
        assert!(editor.handle_key_press(40, false)); // Down
        
        // Test special keys
        assert!(editor.handle_key_press(36, false)); // Home
        assert!(editor.handle_key_press(35, false)); // End
        assert!(editor.handle_key_press(33, false)); // Page Up
        assert!(editor.handle_key_press(34, false)); // Page Down
        
        // Test unhandled key
        assert!(!editor.handle_key_press(999, false));
    }

    #[test]
    fn test_mouse_handling() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut editor = HexEditor::new(Some(model));
        
        // Test mouse click
        editor.handle_mouse_click(100, 50, false);
        
        // Test mouse drag
        editor.handle_mouse_drag(150, 75);
        
        // Should have created a selection
        assert!(editor.selection_exists());
    }

    #[test]
    fn test_hex_formatting() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let editor = HexEditor::new(Some(model));
        
        assert_eq!(editor.format_hex(255, 2), "ff");
        assert_eq!(editor.format_hex(16, 2), "10");
        assert_eq!(editor.format_hex(0, 2), "00");
        assert_eq!(editor.format_hex(4095, 4), "0fff");
    }

    #[test]
    fn test_visible_range_calculation() {
        let model = Rc::new(RefCell::new(VecHexModel::new(256, 8)));
        let editor = HexEditor::new(Some(model));
        
        // Test visible range calculation
        if let Some((start, end)) = editor.get_visible_range(0, 100) {
            assert!(start <= end);
            assert!(start < 256);
        }
    }

    #[test]
    fn test_render_info() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let editor = HexEditor::new(Some(model));
        
        // Get render info for visible area
        let render_info = editor.get_render_info(0, 100);
        
        // Should have some entries
        assert!(!render_info.is_empty());
        
        // Each entry should have valid coordinates and hex string
        for (addr, x, y, hex_str, _is_cursor) in render_info {
            assert!(addr < 64);
            assert!(x >= 0);
            assert!(y >= 0);
            assert!(!hex_str.is_empty());
        }
    }
}