/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

use super::{HexModel, Highlighter, HighlightHandle, Measures};
use std::cell::RefCell;
use std::rc::Rc;

/// Selection and cursor management for hex editor
/// 
/// The caret handles cursor positioning, text selection, and user input
/// in the hex editor. It maintains both a cursor position (dot) and
/// selection start (mark) to support text selection operations.
pub struct Caret {
    model: Option<Rc<RefCell<dyn HexModel>>>,
    measures: Option<Rc<Measures>>,
    highlighter: Option<Rc<RefCell<Highlighter>>>,
    mark: Option<u64>,
    cursor: Option<u64>,
    selection_highlight: Option<HighlightHandle>,
    change_listeners: Vec<Box<dyn Fn()>>,
}

impl Caret {
    /// Create a new caret
    pub fn new(model: Option<Rc<RefCell<dyn HexModel>>>) -> Self {
        Self {
            model,
            measures: None,
            highlighter: None,
            mark: None,
            cursor: None,
            selection_highlight: None,
            change_listeners: Vec::new(),
        }
    }

    /// Set the measures reference for coordinate calculations
    pub fn set_measures(&mut self, measures: Rc<Measures>) {
        self.measures = Some(measures);
    }

    /// Set the highlighter reference for selection display
    pub fn set_highlighter(&mut self, highlighter: Rc<RefCell<Highlighter>>) {
        self.highlighter = Some(highlighter);
    }

    /// Add a change listener that will be called when the caret position changes
    pub fn add_change_listener<F>(&mut self, listener: F)
    where
        F: Fn() + 'static,
    {
        self.change_listeners.push(Box::new(listener));
    }

    /// Get the current cursor position (dot)
    pub fn get_dot(&self) -> Option<u64> {
        self.cursor
    }

    /// Get the selection start position (mark)
    pub fn get_mark(&self) -> Option<u64> {
        self.mark
    }

    /// Set the cursor position
    /// 
    /// # Arguments
    /// * `value` - New cursor position, or None to clear
    /// * `keep_mark` - If true, keeps the current mark position for selection
    pub fn set_dot(&mut self, value: Option<u64>, keep_mark: bool) {
        let validated_value = if let Some(addr) = value {
            self.validate_address(addr)
        } else {
            None
        };

        if self.cursor != validated_value {
            let old_cursor = self.cursor;
            
            // Clear existing selection highlight
            if let Some(highlight) = self.selection_highlight {
                if let Some(highlighter) = &self.highlighter {
                    highlighter.borrow_mut().remove(highlight);
                }
                self.selection_highlight = None;
            }

            // Update mark if not keeping it
            if !keep_mark {
                self.mark = validated_value;
            }

            // Update cursor
            self.cursor = validated_value;

            // Create new selection highlight if mark differs from cursor
            if let (Some(mark), Some(cursor)) = (self.mark, self.cursor) {
                if mark != cursor {
                    if let Some(highlighter) = &self.highlighter {
                        let (start, end) = if mark < cursor {
                            (mark, cursor)
                        } else {
                            (cursor, mark)
                        };
                        
                        self.selection_highlight = highlighter
                            .borrow_mut()
                            .add(start, end, (192, 192, 255)); // Light blue selection
                    }
                }
            }

            // Notify change listeners
            for listener in &self.change_listeners {
                listener();
            }
        }
    }

    /// Move cursor by a relative offset
    /// 
    /// # Arguments
    /// * `offset` - Number of positions to move (positive = forward, negative = backward)
    /// * `keep_mark` - If true, extends selection rather than moving cursor
    pub fn move_cursor(&mut self, offset: i64, keep_mark: bool) {
        if let Some(current) = self.cursor {
            let new_pos = if offset >= 0 {
                current.saturating_add(offset as u64)
            } else {
                current.saturating_sub((-offset) as u64)
            };
            
            self.set_dot(Some(new_pos), keep_mark);
        }
    }

    /// Move cursor up by one row
    pub fn move_up(&mut self, keep_mark: bool) {
        if let Some(measures) = &self.measures {
            let cols = measures.get_column_count() as i64;
            self.move_cursor(-cols, keep_mark);
        }
    }

    /// Move cursor down by one row
    pub fn move_down(&mut self, keep_mark: bool) {
        if let Some(measures) = &self.measures {
            let cols = measures.get_column_count() as i64;
            self.move_cursor(cols, keep_mark);
        }
    }

    /// Move cursor left by one position
    pub fn move_left(&mut self, keep_mark: bool) {
        self.move_cursor(-1, keep_mark);
    }

    /// Move cursor right by one position
    pub fn move_right(&mut self, keep_mark: bool) {
        self.move_cursor(1, keep_mark);
    }

    /// Move to beginning of current line
    pub fn move_to_line_start(&mut self, keep_mark: bool) {
        if let (Some(cursor), Some(measures)) = (self.cursor, &self.measures) {
            let cols = measures.get_column_count() as u64;
            let line_start = cursor - (cursor % cols);
            self.set_dot(Some(line_start), keep_mark);
        }
    }

    /// Move to end of current line
    pub fn move_to_line_end(&mut self, keep_mark: bool) {
        if let (Some(cursor), Some(measures)) = (self.cursor, &self.measures) {
            let cols = measures.get_column_count() as u64;
            let line_end = ((cursor / cols) + 1) * cols - 1;
            
            // Clamp to model bounds
            let validated_end = self.validate_address(line_end).unwrap_or(line_end);
            self.set_dot(Some(validated_end), keep_mark);
        }
    }

    /// Move to beginning of data
    pub fn move_to_start(&mut self, keep_mark: bool) {
        if let Some(model) = &self.model {
            let first_offset = model.borrow().get_first_offset();
            self.set_dot(Some(first_offset), keep_mark);
        }
    }

    /// Move to end of data
    pub fn move_to_end(&mut self, keep_mark: bool) {
        if let Some(model) = &self.model {
            let last_offset = model.borrow().get_last_offset();
            self.set_dot(Some(last_offset), keep_mark);
        }
    }

    /// Move by one page up
    pub fn page_up(&mut self, keep_mark: bool, visible_rows: u32) {
        if let Some(measures) = &self.measures {
            let cols = measures.get_column_count() as i64;
            let page_size = std::cmp::max(1, visible_rows.saturating_sub(1)) as i64;
            self.move_cursor(-cols * page_size, keep_mark);
        }
    }

    /// Move by one page down
    pub fn page_down(&mut self, keep_mark: bool, visible_rows: u32) {
        if let Some(measures) = &self.measures {
            let cols = measures.get_column_count() as i64;
            let page_size = std::cmp::max(1, visible_rows.saturating_sub(1)) as i64;
            self.move_cursor(cols * page_size, keep_mark);
        }
    }

    /// Set cursor position from screen coordinates
    pub fn set_cursor_from_coordinates(&mut self, x: i32, y: i32, keep_mark: bool) {
        let addr = if let (Some(model), Some(measures)) = (&self.model, &self.measures) {
            let model_ref = model.borrow();
            let first_offset = model_ref.get_first_offset();
            let last_offset = model_ref.get_last_offset();
            drop(model_ref); // Release the borrow early
            
            measures.to_address(x, y, first_offset, last_offset)
        } else {
            None
        };
        
        if let Some(addr) = addr {
            self.set_dot(Some(addr), keep_mark);
        }
    }

    /// Check if there is an active selection
    pub fn has_selection(&self) -> bool {
        if let (Some(mark), Some(cursor)) = (self.mark, self.cursor) {
            mark != cursor
        } else {
            false
        }
    }

    /// Get the current selection range
    /// 
    /// Returns (start, end) where start <= end, or None if no selection
    pub fn get_selection(&self) -> Option<(u64, u64)> {
        if let (Some(mark), Some(cursor)) = (self.mark, self.cursor) {
            if mark != cursor {
                Some(if mark < cursor {
                    (mark, cursor)
                } else {
                    (cursor, mark)
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Clear the selection, keeping only the cursor
    pub fn clear_selection(&mut self) {
        if self.has_selection() {
            self.set_dot(self.cursor, false);
        }
    }

    /// Select all data
    pub fn select_all(&mut self) {
        let (first_offset, last_offset) = if let Some(model) = &self.model {
            let model_ref = model.borrow();
            let first = model_ref.get_first_offset();
            let last = model_ref.get_last_offset();
            drop(model_ref); // Release the borrow early
            (first, last)
        } else {
            return;
        };
        
        self.set_dot(Some(first_offset), false);
        self.set_dot(Some(last_offset), true);
    }

    /// Validate that an address is within model bounds
    fn validate_address(&self, address: u64) -> Option<u64> {
        if let Some(model) = &self.model {
            let model_ref = model.borrow();
            let first_offset = model_ref.get_first_offset();
            let last_offset = model_ref.get_last_offset();
            
            if address >= first_offset && address <= last_offset {
                Some(address)
            } else if address < first_offset {
                Some(first_offset)
            } else {
                Some(last_offset)
            }
        } else {
            None
        }
    }

    /// Paint the cursor (for rendering)
    /// 
    /// Returns the cursor rectangle if it should be drawn, or None if not visible
    pub fn get_cursor_rect(&self, visible_start: u64, visible_end: u64) -> Option<(i32, i32, i32, i32)> {
        if let (Some(cursor), Some(measures)) = (self.cursor, &self.measures) {
            if cursor >= visible_start && cursor < visible_end {
                if let Some(model) = &self.model {
                    let model_ref = model.borrow();
                    let first_offset = model_ref.get_first_offset();
                    let base_address = measures.get_base_address(first_offset);
                    
                    let x = measures.to_x(cursor);
                    let y = measures.to_y(cursor, base_address);
                    let w = measures.get_cell_width();
                    let h = measures.get_cell_height();
                    
                    return Some((x, y, w, h));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::{VecHexModel, HexModel};

    #[test]
    fn test_caret_basic_operations() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut caret = Caret::new(Some(model.clone()));
        
        // Test initial state
        assert_eq!(caret.get_dot(), None);
        assert_eq!(caret.get_mark(), None);
        assert!(!caret.has_selection());
        
        // Test set dot
        caret.set_dot(Some(10), false);
        assert_eq!(caret.get_dot(), Some(10));
        assert_eq!(caret.get_mark(), Some(10));
        assert!(!caret.has_selection());
        
        // Test set dot with keeping mark
        caret.set_dot(Some(15), true);
        assert_eq!(caret.get_dot(), Some(15));
        assert_eq!(caret.get_mark(), Some(10));
        assert!(caret.has_selection());
    }

    #[test]
    fn test_caret_movement() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut caret = Caret::new(Some(model.clone()));
        let measures = Rc::new(Measures::new(8, 16));
        caret.set_measures(measures);
        
        // Set initial position
        caret.set_dot(Some(20), false);
        
        // Test basic movement
        caret.move_left(false);
        assert_eq!(caret.get_dot(), Some(19));
        
        caret.move_right(false);
        assert_eq!(caret.get_dot(), Some(20));
        
        // Test row movement
        caret.move_up(false);
        assert_eq!(caret.get_dot(), Some(4)); // 20 - 16 = 4
        
        caret.move_down(false);
        assert_eq!(caret.get_dot(), Some(20)); // 4 + 16 = 20
    }

    #[test]
    fn test_selection_operations() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut caret = Caret::new(Some(model.clone()));
        
        // Test selection creation
        caret.set_dot(Some(10), false);
        caret.set_dot(Some(20), true);
        
        assert!(caret.has_selection());
        if let Some((start, end)) = caret.get_selection() {
            assert_eq!(start, 10);
            assert_eq!(end, 20);
        }
        
        // Test selection clearing
        caret.clear_selection();
        assert!(!caret.has_selection());
        
        // Test select all
        caret.select_all();
        assert!(caret.has_selection());
        if let Some((start, end)) = caret.get_selection() {
            assert_eq!(start, 0);
            assert_eq!(end, 63);
        }
    }

    #[test]
    fn test_bounds_validation() {
        let model = Rc::new(RefCell::new(VecHexModel::new(16, 8)));
        let mut caret = Caret::new(Some(model.clone()));
        
        // Test setting cursor beyond bounds
        caret.set_dot(Some(100), false);
        assert_eq!(caret.get_dot(), Some(15)); // Should clamp to last valid address
        
        // Test setting cursor before bounds
        caret.set_dot(Some(0), false);
        caret.move_cursor(-10, false);
        assert_eq!(caret.get_dot(), Some(0)); // Should clamp to first valid address
    }

    #[test]
    fn test_line_movement() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut caret = Caret::new(Some(model.clone()));
        let measures = Rc::new(Measures::new(8, 16));
        caret.set_measures(measures);
        
        // Set position in middle of a line
        caret.set_dot(Some(22), false); // 22 % 16 = 6 (column 6)
        
        // Test move to line start
        caret.move_to_line_start(false);
        assert_eq!(caret.get_dot(), Some(16)); // Start of second line
        
        // Test move to line end
        caret.move_to_line_end(false);
        assert_eq!(caret.get_dot(), Some(31)); // End of second line
    }

    #[test]
    fn test_coordinate_conversion() {
        let model = Rc::new(RefCell::new(VecHexModel::new(64, 8)));
        let mut caret = Caret::new(Some(model.clone()));
        let measures = Rc::new(Measures::new(8, 16));
        caret.set_measures(measures.clone());
        
        // Set cursor and test coordinate conversion
        caret.set_dot(Some(10), false);
        
        let rect = caret.get_cursor_rect(0, 64);
        assert!(rect.is_some());
        
        if let Some((x, y, w, h)) = rect {
            assert!(x >= 0);
            assert!(y >= 0);
            assert!(w > 0);
            assert!(h > 0);
        }
    }
}