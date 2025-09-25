/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Highlighter - Visual highlighting for hex editor
//!
//! Rust port of Highlighter.java

use super::hex_model::HexModel;
use super::measures::Measures;

#[cfg(feature = "gui")]
use egui::{Color32, Painter, Rect, Rounding, Stroke};

/// Represents a highlighted range in the hex editor
#[derive(Debug, Clone)]
pub struct HighlightEntry {
    pub start: u64,
    pub end: u64,
    #[cfg(feature = "gui")]
    pub color: Color32,
    #[cfg(not(feature = "gui"))]
    pub color: [u8; 4], // RGBA fallback
    pub id: usize,
}

/// Manages visual highlighting of address ranges in the hex editor
pub struct Highlighter {
    entries: Vec<HighlightEntry>,
    next_id: usize,
}

impl Highlighter {
    /// Create a new highlighter
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a highlight for the given address range
    #[cfg(feature = "gui")]
    pub fn add(
        &mut self,
        start: u64,
        end: u64,
        color: Color32,
        model: Option<&dyn HexModel>,
    ) -> Option<usize> {
        let model = model?;

        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        let start = start.max(model.get_first_offset());
        let end = end.min(model.get_last_offset());

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
        Some(id)
    }

    /// Add a highlight for the given address range (non-GUI version)
    #[cfg(not(feature = "gui"))]
    pub fn add(
        &mut self,
        start: u64,
        end: u64,
        color: [u8; 4],
        model: Option<&dyn HexModel>,
    ) -> Option<usize> {
        let model = model?;

        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        let start = start.max(model.get_first_offset());
        let end = end.min(model.get_last_offset());

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
        Some(id)
    }

    /// Remove a highlight by ID
    pub fn remove(&mut self, id: usize) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    /// Clear all highlights
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get all highlight entries
    pub fn get_entries(&self) -> &[HighlightEntry] {
        &self.entries
    }

    /// Paint highlights for the visible address range
    #[cfg(feature = "gui")]
    pub fn paint(
        &self,
        painter: &Painter,
        measures: &Measures,
        start_addr: u64,
        end_addr: u64,
        model: Option<&dyn HexModel>,
    ) {
        if self.entries.is_empty() {
            return;
        }

        let line_start = measures.get_values_x();
        let line_width = measures.get_values_width();
        let cell_width = measures.get_cell_width();
        let cell_height = measures.get_cell_height();

        for entry in &self.entries {
            if entry.start <= end_addr && entry.end >= start_addr {
                let y0 = measures.to_y(entry.start, model);
                let y1 = measures.to_y(entry.end, model);
                let x0 = measures.to_x(entry.start, model);
                let x1 = measures.to_x(entry.end, model);

                if (y0 - y1).abs() < 0.1 {
                    // Single line highlight
                    let rect = Rect::from_min_size(
                        [x0, y0].into(),
                        [x1 - x0 + cell_width, cell_height].into(),
                    );
                    painter.rect_filled(rect, Rounding::ZERO, entry.color);
                } else {
                    // Multi-line highlight

                    // First line
                    let first_rect = Rect::from_min_size(
                        [x0, y0].into(),
                        [line_start + line_width - x0, cell_height].into(),
                    );
                    painter.rect_filled(first_rect, Rounding::ZERO, entry.color);

                    // Middle lines (if any)
                    let mid_height = y1 - (y0 + cell_height);
                    if mid_height > 0.1 {
                        let mid_rect = Rect::from_min_size(
                            [line_start, y0 + cell_height].into(),
                            [line_width, mid_height].into(),
                        );
                        painter.rect_filled(mid_rect, Rounding::ZERO, entry.color);
                    }

                    // Last line
                    let last_rect = Rect::from_min_size(
                        [line_start, y1].into(),
                        [x1 + cell_width - line_start, cell_height].into(),
                    );
                    painter.rect_filled(last_rect, Rounding::ZERO, entry.color);
                }
            }
        }
    }

    /// Check if an address is highlighted
    #[cfg(feature = "gui")]
    pub fn is_highlighted(&self, address: u64) -> Option<Color32> {
        for entry in &self.entries {
            if address >= entry.start && address <= entry.end {
                return Some(entry.color);
            }
        }
        None
    }

    /// Get all highlights that contain the given address
    pub fn get_highlights_at(&self, address: u64) -> Vec<&HighlightEntry> {
        self.entries
            .iter()
            .filter(|entry| address >= entry.start && address <= entry.end)
            .collect()
    }

    /// Get highlight entries that overlap with the given range
    pub fn get_overlapping_highlights(&self, start: u64, end: u64) -> Vec<&HighlightEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.start <= end && entry.end >= start)
            .collect()
    }

    /// Get the bounding rectangle for a highlight entry
    #[cfg(feature = "gui")]
    pub fn get_highlight_bounds(
        &self,
        entry: &HighlightEntry,
        measures: &Measures,
        model: Option<&dyn HexModel>,
    ) -> Option<Rect> {
        let y0 = measures.to_y(entry.start, model);
        let y1 = measures.to_y(entry.end, model);
        let x0 = measures.to_x(entry.start, model);
        let x1 = measures.to_x(entry.end, model);

        let cell_width = measures.get_cell_width();
        let cell_height = measures.get_cell_height();

        if (y0 - y1).abs() < 0.1 {
            // Single line
            Some(Rect::from_min_size(
                [x0, y0].into(),
                [x1 - x0 + cell_width, cell_height].into(),
            ))
        } else {
            // Multi-line - return bounding rectangle
            let line_start = measures.get_values_x();
            let line_width = measures.get_values_width();

            Some(Rect::from_min_size(
                [line_start, y0].into(),
                [line_width, y1 - y0 + cell_height].into(),
            ))
        }
    }

    /// Update highlight color
    #[cfg(feature = "gui")]
    pub fn update_color(&mut self, id: usize, color: Color32) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.color = color;
            true
        } else {
            false
        }
    }

    /// Update highlight range
    pub fn update_range(
        &mut self,
        id: usize,
        start: u64,
        end: u64,
        model: Option<&dyn HexModel>,
    ) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            if let Some(model) = model {
                let (start, end) = if start > end {
                    (end, start)
                } else {
                    (start, end)
                };

                let start = start.max(model.get_first_offset());
                let end = end.min(model.get_last_offset());

                if start < end {
                    entry.start = start;
                    entry.end = end;
                    return true;
                }
            }
        }
        false
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
    use crate::hex::hex_model::MemoryHexModel;

    #[test]
    fn test_highlighter_creation() {
        let highlighter = Highlighter::new();
        assert_eq!(highlighter.entries.len(), 0);
    }

    #[cfg(feature = "gui")]
    #[test]
    fn test_add_highlight() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        let id = highlighter.add(10, 20, Color32::RED, Some(&model));
        assert!(id.is_some());
        assert_eq!(highlighter.entries.len(), 1);

        let entry = &highlighter.entries[0];
        assert_eq!(entry.start, 10);
        assert_eq!(entry.end, 20);
        assert_eq!(entry.color, Color32::RED);
    }

    #[cfg(not(feature = "gui"))]
    #[test]
    fn test_add_highlight() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        let id = highlighter.add(10, 20, [255, 0, 0, 255], Some(&model));
        assert!(id.is_some());
        assert_eq!(highlighter.entries.len(), 1);

        let entry = &highlighter.entries[0];
        assert_eq!(entry.start, 10);
        assert_eq!(entry.end, 20);
        assert_eq!(entry.color, [255, 0, 0, 255]);
    }

    #[test]
    fn test_remove_highlight() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        #[cfg(feature = "gui")]
        let id = highlighter.add(10, 20, Color32::RED, Some(&model)).unwrap();
        #[cfg(not(feature = "gui"))]
        let id = highlighter
            .add(10, 20, [255, 0, 0, 255], Some(&model))
            .unwrap();

        assert_eq!(highlighter.entries.len(), 1);

        let removed = highlighter.remove(id);
        assert!(removed);
        assert_eq!(highlighter.entries.len(), 0);

        // Try to remove non-existent highlight
        let removed_again = highlighter.remove(id);
        assert!(!removed_again);
    }

    #[test]
    fn test_clear_highlights() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        #[cfg(feature = "gui")]
        {
            highlighter.add(10, 20, Color32::RED, Some(&model));
            highlighter.add(30, 40, Color32::BLUE, Some(&model));
        }
        #[cfg(not(feature = "gui"))]
        {
            highlighter.add(10, 20, [255, 0, 0, 255], Some(&model));
            highlighter.add(30, 40, [0, 0, 255, 255], Some(&model));
        }

        assert_eq!(highlighter.entries.len(), 2);

        highlighter.clear();
        assert_eq!(highlighter.entries.len(), 0);
    }

    #[cfg(feature = "gui")]
    #[test]
    fn test_is_highlighted() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        highlighter.add(10, 20, Color32::RED, Some(&model));

        assert_eq!(highlighter.is_highlighted(5), None);
        assert_eq!(highlighter.is_highlighted(15), Some(Color32::RED));
        assert_eq!(highlighter.is_highlighted(25), None);
    }

    #[test]
    fn test_get_highlights_at() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        #[cfg(feature = "gui")]
        {
            highlighter.add(10, 20, Color32::RED, Some(&model));
            highlighter.add(15, 25, Color32::BLUE, Some(&model));
        }
        #[cfg(not(feature = "gui"))]
        {
            highlighter.add(10, 20, [255, 0, 0, 255], Some(&model));
            highlighter.add(15, 25, [0, 0, 255, 255], Some(&model));
        }

        let highlights_at_5 = highlighter.get_highlights_at(5);
        assert_eq!(highlights_at_5.len(), 0);

        let highlights_at_12 = highlighter.get_highlights_at(12);
        assert_eq!(highlights_at_12.len(), 1);
        #[cfg(feature = "gui")]
        assert_eq!(highlights_at_12[0].color, Color32::RED);
        #[cfg(not(feature = "gui"))]
        assert_eq!(highlights_at_12[0].color, [255, 0, 0, 255]);

        let highlights_at_17 = highlighter.get_highlights_at(17);
        assert_eq!(highlights_at_17.len(), 2); // Both highlights overlap here
    }

    #[test]
    fn test_overlapping_highlights() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        #[cfg(feature = "gui")]
        {
            highlighter.add(10, 20, Color32::RED, Some(&model));
            highlighter.add(25, 35, Color32::BLUE, Some(&model));
            highlighter.add(15, 30, Color32::GREEN, Some(&model));
        }
        #[cfg(not(feature = "gui"))]
        {
            highlighter.add(10, 20, [255, 0, 0, 255], Some(&model));
            highlighter.add(25, 35, [0, 0, 255, 255], Some(&model));
            highlighter.add(15, 30, [0, 255, 0, 255], Some(&model));
        }

        let overlapping = highlighter.get_overlapping_highlights(12, 24);
        assert_eq!(overlapping.len(), 2); // RED and GREEN overlap with range 12-24

        let overlapping_all = highlighter.get_overlapping_highlights(0, 100);
        assert_eq!(overlapping_all.len(), 3); // All highlights overlap with large range
    }

    #[cfg(feature = "gui")]
    #[test]
    fn test_update_highlight() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(256, 8);

        let id = highlighter.add(10, 20, Color32::RED, Some(&model)).unwrap();

        // Update color
        let updated = highlighter.update_color(id, Color32::GREEN);
        assert!(updated);
        assert_eq!(highlighter.entries[0].color, Color32::GREEN);

        // Update range
        let updated = highlighter.update_range(id, 5, 15, Some(&model));
        assert!(updated);
        assert_eq!(highlighter.entries[0].start, 5);
        assert_eq!(highlighter.entries[0].end, 15);
    }

    #[test]
    fn test_highlight_bounds_clamping() {
        let mut highlighter = Highlighter::new();
        let model = MemoryHexModel::new(100, 8); // Model with addresses 0-99

        // Try to add highlight beyond model bounds
        #[cfg(feature = "gui")]
        let id = highlighter.add(50, 150, Color32::RED, Some(&model));
        #[cfg(not(feature = "gui"))]
        let id = highlighter.add(50, 150, [255, 0, 0, 255], Some(&model));

        assert!(id.is_some());

        let entry = &highlighter.entries[0];
        assert_eq!(entry.start, 50);
        assert_eq!(entry.end, 99); // Clamped to model's last offset
    }
}
