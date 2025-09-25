/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Caret - Cursor management for hex editor
//!
//! Rust port of Caret.java

use super::hex_model::HexModel;
use super::highlighter::Highlighter;
use super::measures::Measures;

#[cfg(feature = "gui")]
use egui::{Color32, Key, Modifiers, Painter, Rect, Rounding, Stroke};

/// Selection colors
#[cfg(feature = "gui")]
const SELECT_COLOR: Color32 = Color32::from_rgb(192, 192, 255);
#[cfg(feature = "gui")]
const CURSOR_COLOR: Color32 = Color32::BLACK;

/// Represents the cursor and selection state in the hex editor
pub struct Caret {
    /// Current cursor position
    cursor: i64,
    /// Selection mark position (-1 if no selection)
    mark: i64,
    /// Highlight ID for selection display
    selection_highlight_id: Option<usize>,
    /// Whether the caret is visible
    visible: bool,
    /// Blink state for cursor
    blink_state: bool,
    /// Last blink time
    last_blink_time: std::time::Instant,
}

impl Caret {
    /// Create a new caret
    pub fn new() -> Self {
        Self {
            cursor: -1,
            mark: -1,
            selection_highlight_id: None,
            visible: true,
            blink_state: true,
            last_blink_time: std::time::Instant::now(),
        }
    }

    /// Get the current cursor position
    pub fn get_dot(&self) -> i64 {
        self.cursor
    }

    /// Get the selection mark position
    pub fn get_mark(&self) -> i64 {
        self.mark
    }

    /// Set cursor position
    pub fn set_dot(
        &mut self,
        value: i64,
        keep_mark: bool,
        highlighter: &mut Highlighter,
        model: Option<&dyn HexModel>,
    ) {
        let old_cursor = self.cursor;
        self.cursor = value;

        if !keep_mark {
            self.mark = value;
        }

        self.update_selection_highlight(highlighter, model);

        // Reset blink state when cursor moves
        if old_cursor != value {
            self.blink_state = true;
            self.last_blink_time = std::time::Instant::now();
        }
    }

    /// Set selection range
    pub fn set_selection(
        &mut self,
        start: i64,
        end: i64,
        highlighter: &mut Highlighter,
        model: Option<&dyn HexModel>,
    ) {
        self.cursor = end;
        self.mark = start;
        self.update_selection_highlight(highlighter, model);
    }

    /// Clear selection
    pub fn clear_selection(&mut self, highlighter: &mut Highlighter) {
        self.mark = self.cursor;
        self.clear_selection_highlight(highlighter);
    }

    /// Check if there is a selection
    pub fn has_selection(&self) -> bool {
        self.cursor >= 0 && self.mark >= 0 && self.cursor != self.mark
    }

    /// Get selection range (start, end) where start <= end
    pub fn get_selection_range(&self) -> Option<(u64, u64)> {
        if self.has_selection() {
            let start = self.cursor.min(self.mark) as u64;
            let end = self.cursor.max(self.mark) as u64;
            Some((start, end))
        } else {
            None
        }
    }

    /// Handle key input
    #[cfg(feature = "gui")]
    pub fn handle_key_input(
        &mut self,
        key: Key,
        modifiers: Modifiers,
        measures: &Measures,
        highlighter: &mut Highlighter,
        model: Option<&dyn HexModel>,
    ) -> bool {
        let model = match model {
            Some(m) => m,
            None => return false,
        };

        let shift_held = modifiers.shift;
        let ctrl_held = modifiers.ctrl;

        match key {
            Key::ArrowLeft => {
                self.move_cursor(-1, shift_held, highlighter, Some(model));
                true
            }
            Key::ArrowRight => {
                self.move_cursor(1, shift_held, highlighter, Some(model));
                true
            }
            Key::ArrowUp => {
                let cols = measures.get_column_count() as i64;
                self.move_cursor(-cols, shift_held, highlighter, Some(model));
                true
            }
            Key::ArrowDown => {
                let cols = measures.get_column_count() as i64;
                self.move_cursor(cols, shift_held, highlighter, Some(model));
                true
            }
            Key::Home => {
                if ctrl_held {
                    // Go to beginning of document
                    let new_pos = model.get_first_offset() as i64;
                    self.set_dot(new_pos, shift_held, highlighter, Some(model));
                } else {
                    // Go to beginning of line
                    let cols = measures.get_column_count() as i64;
                    let current_row = self.cursor / cols;
                    let new_pos = current_row * cols + model.get_first_offset() as i64;
                    self.set_dot(new_pos, shift_held, highlighter, Some(model));
                }
                true
            }
            Key::End => {
                if ctrl_held {
                    // Go to end of document
                    let new_pos = model.get_last_offset() as i64;
                    self.set_dot(new_pos, shift_held, highlighter, Some(model));
                } else {
                    // Go to end of line
                    let cols = measures.get_column_count() as i64;
                    let current_row = self.cursor / cols;
                    let line_end = (current_row + 1) * cols + model.get_first_offset() as i64 - 1;
                    let new_pos = line_end.min(model.get_last_offset() as i64);
                    self.set_dot(new_pos, shift_held, highlighter, Some(model));
                }
                true
            }
            Key::PageUp => {
                // Move up by visible page (approximate)
                let page_rows = 16; // Could be made configurable
                let cols = measures.get_column_count() as i64;
                self.move_cursor(-page_rows * cols, shift_held, highlighter, Some(model));
                true
            }
            Key::PageDown => {
                // Move down by visible page (approximate)
                let page_rows = 16; // Could be made configurable
                let cols = measures.get_column_count() as i64;
                self.move_cursor(page_rows * cols, shift_held, highlighter, Some(model));
                true
            }
            _ => false,
        }
    }

    /// Handle mouse click
    #[cfg(feature = "gui")]
    pub fn handle_mouse_click(
        &mut self,
        pos: egui::Pos2,
        extend_selection: bool,
        measures: &Measures,
        highlighter: &mut Highlighter,
        model: Option<&dyn HexModel>,
    ) {
        if let Some(addr) = measures.to_address(pos.x, pos.y, model) {
            self.set_dot(addr as i64, extend_selection, highlighter, model);
        }
    }

    /// Handle mouse drag
    #[cfg(feature = "gui")]
    pub fn handle_mouse_drag(
        &mut self,
        pos: egui::Pos2,
        measures: &Measures,
        highlighter: &mut Highlighter,
        model: Option<&dyn HexModel>,
    ) {
        if let Some(addr) = measures.to_address(pos.x, pos.y, model) {
            self.set_dot(addr as i64, true, highlighter, model); // Always extend selection when dragging
        }
    }

    /// Paint the cursor
    #[cfg(feature = "gui")]
    pub fn paint_cursor(
        &mut self,
        painter: &Painter,
        measures: &Measures,
        model: Option<&dyn HexModel>,
    ) {
        if !self.visible || self.cursor < 0 {
            return;
        }

        // Update blink state
        let now = std::time::Instant::now();
        if now.duration_since(self.last_blink_time).as_millis() > 500 {
            self.blink_state = !self.blink_state;
            self.last_blink_time = now;
        }

        if !self.blink_state {
            return;
        }

        let x = measures.to_x(self.cursor as u64, model);
        let y = measures.to_y(self.cursor as u64, model);
        let cell_height = measures.get_cell_height();

        // Draw cursor line
        let cursor_rect = Rect::from_min_size([x - 1.0, y].into(), [2.0, cell_height].into());

        painter.rect_filled(cursor_rect, Rounding::ZERO, CURSOR_COLOR);
    }

    /// Select all
    pub fn select_all(&mut self, highlighter: &mut Highlighter, model: Option<&dyn HexModel>) {
        if let Some(model) = model {
            let first = model.get_first_offset() as i64;
            let last = model.get_last_offset() as i64;
            self.set_selection(first, last, highlighter, Some(model));
        }
    }

    /// Move cursor by delta
    fn move_cursor(
        &mut self,
        delta: i64,
        extend_selection: bool,
        highlighter: &mut Highlighter,
        model: Option<&dyn HexModel>,
    ) {
        let model = match model {
            Some(m) => m,
            None => return,
        };

        let new_pos = (self.cursor + delta)
            .max(model.get_first_offset() as i64)
            .min(model.get_last_offset() as i64);

        self.set_dot(new_pos, extend_selection, highlighter, Some(model));
    }

    /// Update selection highlight
    fn update_selection_highlight(
        &mut self,
        highlighter: &mut Highlighter,
        model: Option<&dyn HexModel>,
    ) {
        // Clear existing selection highlight
        self.clear_selection_highlight(highlighter);

        // Add new selection highlight if there is a selection
        if self.has_selection() {
            let (start, end) = self.get_selection_range().unwrap();
            #[cfg(feature = "gui")]
            {
                self.selection_highlight_id = highlighter.add(start, end, SELECT_COLOR, model);
            }
            #[cfg(not(feature = "gui"))]
            {
                self.selection_highlight_id =
                    highlighter.add(start, end, [192, 192, 255, 255], model);
            }
        }
    }

    /// Clear selection highlight
    fn clear_selection_highlight(&mut self, highlighter: &mut Highlighter) {
        if let Some(id) = self.selection_highlight_id.take() {
            highlighter.remove(id);
        }
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get current address bounds for scrolling
    #[cfg(feature = "gui")]
    pub fn get_cursor_bounds(
        &self,
        measures: &Measures,
        model: Option<&dyn HexModel>,
    ) -> Option<Rect> {
        if self.cursor < 0 {
            return None;
        }

        let x = measures.to_x(self.cursor as u64, model);
        let y = measures.to_y(self.cursor as u64, model);
        let cell_width = measures.get_cell_width();
        let cell_height = measures.get_cell_height();

        Some(Rect::from_min_size(
            [x, y].into(),
            [cell_width, cell_height].into(),
        ))
    }
}

impl Default for Caret {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::hex_model::MemoryHexModel;

    #[test]
    fn test_caret_creation() {
        let caret = Caret::new();
        assert_eq!(caret.get_dot(), -1);
        assert_eq!(caret.get_mark(), -1);
        assert!(!caret.has_selection());
    }

    #[test]
    fn test_set_dot() {
        let mut caret = Caret::new();
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        caret.set_dot(10, false, &mut highlighter, Some(&model));
        assert_eq!(caret.get_dot(), 10);
        assert_eq!(caret.get_mark(), 10);
        assert!(!caret.has_selection());
    }

    #[test]
    fn test_selection() {
        let mut caret = Caret::new();
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        caret.set_selection(10, 20, &mut highlighter, Some(&model));
        assert_eq!(caret.get_dot(), 20);
        assert_eq!(caret.get_mark(), 10);
        assert!(caret.has_selection());

        let (start, end) = caret.get_selection_range().unwrap();
        assert_eq!(start, 10);
        assert_eq!(end, 20);
    }

    #[test]
    fn test_clear_selection() {
        let mut caret = Caret::new();
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        caret.set_selection(10, 20, &mut highlighter, Some(&model));
        assert!(caret.has_selection());

        caret.clear_selection(&mut highlighter);
        assert!(!caret.has_selection());
        assert_eq!(caret.get_dot(), caret.get_mark());
    }

    #[test]
    fn test_select_all() {
        let mut caret = Caret::new();
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        caret.select_all(&mut highlighter, Some(&model));
        assert!(caret.has_selection());

        let (start, end) = caret.get_selection_range().unwrap();
        assert_eq!(start, model.get_first_offset());
        assert_eq!(end, model.get_last_offset());
    }

    #[cfg(feature = "gui")]
    #[test]
    fn test_key_navigation() {
        let mut caret = Caret::new();
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);
        let measures = Measures::new();

        // Set initial position
        caret.set_dot(10, false, &mut highlighter, Some(&model));

        // Test right arrow
        let handled = caret.handle_key_input(
            Key::ArrowRight,
            Modifiers::NONE,
            &measures,
            &mut highlighter,
            Some(&model),
        );
        assert!(handled);
        assert_eq!(caret.get_dot(), 11);

        // Test left arrow
        let handled = caret.handle_key_input(
            Key::ArrowLeft,
            Modifiers::NONE,
            &measures,
            &mut highlighter,
            Some(&model),
        );
        assert!(handled);
        assert_eq!(caret.get_dot(), 10);
    }

    #[cfg(feature = "gui")]
    #[test]
    fn test_key_selection() {
        let mut caret = Caret::new();
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);
        let measures = Measures::new();

        // Set initial position
        caret.set_dot(10, false, &mut highlighter, Some(&model));

        // Test shift+right arrow (extend selection)
        let handled = caret.handle_key_input(
            Key::ArrowRight,
            Modifiers::SHIFT,
            &measures,
            &mut highlighter,
            Some(&model),
        );
        assert!(handled);
        assert_eq!(caret.get_dot(), 11);
        assert_eq!(caret.get_mark(), 10);
        assert!(caret.has_selection());
    }

    #[cfg(feature = "gui")]
    #[test]
    fn test_home_end_keys() {
        let mut caret = Caret::new();
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);
        let measures = Measures::new();

        // Set position in middle
        caret.set_dot(100, false, &mut highlighter, Some(&model));

        // Test Ctrl+Home (go to start)
        let handled = caret.handle_key_input(
            Key::Home,
            Modifiers::CTRL,
            &measures,
            &mut highlighter,
            Some(&model),
        );
        assert!(handled);
        assert_eq!(caret.get_dot(), model.get_first_offset() as i64);

        // Test Ctrl+End (go to end)
        let handled = caret.handle_key_input(
            Key::End,
            Modifiers::CTRL,
            &measures,
            &mut highlighter,
            Some(&model),
        );
        assert!(handled);
        assert_eq!(caret.get_dot(), model.get_last_offset() as i64);
    }
}
