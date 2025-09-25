/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

use super::Measures;

/// Color representation as RGB tuple
pub type Color = (u8, u8, u8);

/// Handle for a highlight entry that can be used to remove it
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HighlightHandle(usize);

/// Entry representing a highlighted range
#[derive(Debug, Clone)]
struct HighlightEntry {
    start: u64,
    end: u64,
    color: Color,
    id: usize,
}

/// Manages highlighting of address ranges in the hex editor
///
/// The highlighter allows marking ranges of addresses with different colors
/// for visual emphasis and can handle overlapping highlights.
pub struct Highlighter {
    entries: Vec<HighlightEntry>,
    next_id: usize,
}

impl Highlighter {
    /// Create a new highlighter
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 0,
        }
    }

    /// Add a highlight for the given address range
    ///
    /// # Arguments
    /// * `start` - Starting address (inclusive)
    /// * `end` - Ending address (inclusive)
    /// * `color` - RGB color for the highlight
    ///
    /// Returns a handle that can be used to remove the highlight later,
    /// or None if the range is invalid.
    pub fn add(&mut self, start: u64, end: u64, color: Color) -> Option<HighlightHandle> {
        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        if start >= end {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let entry = HighlightEntry {
            start,
            end,
            color,
            id,
        };

        self.entries.push(entry);
        Some(HighlightHandle(id))
    }

    /// Remove all highlights
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Remove a specific highlight by handle
    ///
    /// Returns true if the highlight was found and removed.
    pub fn remove(&mut self, handle: HighlightHandle) -> bool {
        let handle_id = handle.0;
        if let Some(pos) = self.entries.iter().position(|e| e.id == handle_id) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all highlights that intersect with the given address range
    ///
    /// Returns a vector of (start, end, color) tuples for highlights
    /// that overlap with the given range.
    pub fn get_highlights_in_range(&self, start: u64, end: u64) -> Vec<(u64, u64, Color)> {
        self.entries
            .iter()
            .filter(|e| e.start <= end && e.end >= start)
            .map(|e| (e.start, e.end, e.color))
            .collect()
    }

    /// Paint highlights for the given address range
    ///
    /// This method would be called during rendering to draw the highlights.
    /// In a real GUI implementation, this would draw colored rectangles.
    ///
    /// # Arguments
    /// * `measures` - Layout measurements for coordinate conversion
    /// * `start` - Starting address of visible range
    /// * `end` - Ending address of visible range
    /// * `first_offset` - First offset in the data model
    ///
    /// Returns a list of rectangles to be drawn with their colors.
    pub fn paint_highlights(
        &self,
        measures: &Measures,
        start: u64,
        end: u64,
        first_offset: u64,
    ) -> Vec<(i32, i32, i32, i32, Color)> {
        let mut rectangles = Vec::new();
        let base_address = measures.get_base_address(first_offset);

        for entry in &self.entries {
            if entry.start <= end && entry.end >= start {
                let y0 = measures.to_y(entry.start, base_address);
                let y1 = measures.to_y(entry.end, base_address);
                let x0 = measures.to_x(entry.start);
                let x1 = measures.to_x(entry.end);
                let cell_width = measures.get_cell_width();
                let cell_height = measures.get_cell_height();

                if y0 == y1 {
                    // Single line highlight
                    rectangles.push((x0, y0, x1 - x0 + cell_width, cell_height, entry.color));
                } else {
                    // Multi-line highlight
                    let line_start_x = measures.get_values_x();
                    let line_width = measures.get_values_width();

                    // First line
                    rectangles.push((
                        x0,
                        y0,
                        line_start_x + line_width - x0,
                        cell_height,
                        entry.color,
                    ));

                    // Middle lines (if any)
                    let mid_height = y1 - (y0 + cell_height);
                    if mid_height > 0 {
                        rectangles.push((
                            line_start_x,
                            y0 + cell_height,
                            line_width,
                            mid_height,
                            entry.color,
                        ));
                    }

                    // Last line
                    rectangles.push((
                        line_start_x,
                        y1,
                        x1 + cell_width - line_start_x,
                        cell_height,
                        entry.color,
                    ));
                }
            }
        }

        rectangles
    }

    /// Get the number of active highlights
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if there are no highlights
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all highlight entries (for debugging/testing)
    pub fn entries(&self) -> &[HighlightEntry] {
        &self.entries
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlighter_basic_operations() {
        let mut highlighter = Highlighter::new();

        // Test initial state
        assert_eq!(highlighter.len(), 0);
        assert!(highlighter.is_empty());

        // Test adding highlights
        let handle1 = highlighter.add(0, 10, (255, 255, 0));
        assert!(handle1.is_some());
        assert_eq!(highlighter.len(), 1);

        let handle2 = highlighter.add(20, 30, (0, 255, 255));
        assert!(handle2.is_some());
        assert_eq!(highlighter.len(), 2);

        // Test removing highlights
        if let Some(h) = handle1 {
            assert!(highlighter.remove(h));
            assert_eq!(highlighter.len(), 1);
        }

        // Test clearing all highlights
        highlighter.clear();
        assert_eq!(highlighter.len(), 0);
        assert!(highlighter.is_empty());
    }

    #[test]
    fn test_highlight_range_queries() {
        let mut highlighter = Highlighter::new();

        // Add some highlights
        highlighter.add(0, 10, (255, 0, 0));
        highlighter.add(15, 25, (0, 255, 0));
        highlighter.add(30, 40, (0, 0, 255));

        // Test range queries
        let highlights = highlighter.get_highlights_in_range(5, 20);
        assert_eq!(highlights.len(), 2); // Should include first two highlights

        let highlights = highlighter.get_highlights_in_range(35, 45);
        assert_eq!(highlights.len(), 1); // Should include only the third highlight

        let highlights = highlighter.get_highlights_in_range(50, 60);
        assert_eq!(highlights.len(), 0); // Should include no highlights
    }

    #[test]
    fn test_invalid_ranges() {
        let mut highlighter = Highlighter::new();

        // Test invalid range (start == end)
        let handle = highlighter.add(10, 10, (255, 255, 255));
        assert!(handle.is_none());

        // Test reversed range (should be corrected)
        let handle = highlighter.add(20, 10, (255, 255, 255));
        assert!(handle.is_some());

        if let Some(h) = handle {
            let highlights = highlighter.get_highlights_in_range(5, 25);
            assert_eq!(highlights.len(), 1);
            assert_eq!(highlights[0].0, 10); // Should be corrected to start=10, end=20
            assert_eq!(highlights[0].1, 20);
        }
    }

    #[test]
    fn test_remove_nonexistent_highlight() {
        let mut highlighter = Highlighter::new();

        // Try to remove a highlight that doesn't exist
        let fake_handle = HighlightHandle(999);
        assert!(!highlighter.remove(fake_handle));
    }

    #[test]
    fn test_paint_highlights() {
        let mut highlighter = Highlighter::new();
        let measures = Measures::new(8, 16);

        // Add a highlight
        highlighter.add(0, 5, (255, 0, 0));

        // Test painting
        let rectangles = highlighter.paint_highlights(&measures, 0, 10, 0);
        assert!(!rectangles.is_empty());

        // Each rectangle should have proper dimensions
        for (x, y, w, h, color) in rectangles {
            assert!(x >= 0);
            assert!(y >= 0);
            assert!(w > 0);
            assert!(h > 0);
            assert_eq!(color, (255, 0, 0));
        }
    }

    #[test]
    fn test_multiline_highlight() {
        let mut highlighter = Highlighter::new();
        let measures = Measures::new(8, 4); // Small column count to force multiline

        // Add a highlight that spans multiple lines
        highlighter.add(2, 10, (0, 255, 0));

        let rectangles = highlighter.paint_highlights(&measures, 0, 15, 0);

        // Should have multiple rectangles for a multiline highlight
        assert!(!rectangles.is_empty());

        // All rectangles should have the same color
        for (_, _, _, _, color) in rectangles {
            assert_eq!(color, (0, 255, 0));
        }
    }
}
