/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HexEditor - Main hex editor component
//!
//! Rust port of HexEditor.java using egui for rendering

use super::hex_model::{HexModel, HexModelListener};
use super::measures::Measures;
use super::caret::Caret;
use super::highlighter::Highlighter;
use egui::{
    Color32, Context, FontId, Key, Modifiers, Painter, Pos2, Rect, Response, ScrollArea, Sense,
    Stroke, TextStyle, Ui, Vec2, Widget, Rounding,
};
use std::sync::{Arc, Mutex, Weak};

/// Main hex editor widget
pub struct HexEditor {
    /// Data model
    model: Option<Arc<Mutex<dyn HexModel>>>,
    /// Layout measurements
    measures: Measures,
    /// Cursor and selection
    caret: Caret,
    /// Visual highlighting
    highlighter: Highlighter,
    /// Preferred size
    preferred_size: Vec2,
    /// Font size
    font_size: f32,
    /// Show addresses
    show_addresses: bool,
    /// Bytes per row (when not auto-calculated)
    bytes_per_row: Option<usize>,
    /// Read-only mode
    read_only: bool,
    /// Last mouse position for drag detection
    last_mouse_pos: Option<Pos2>,
}

impl HexEditor {
    /// Create a new hex editor
    pub fn new() -> Self {
        Self {
            model: None,
            measures: Measures::new(),
            caret: Caret::new(),
            highlighter: Highlighter::new(),
            preferred_size: Vec2::new(400.0, 300.0),
            font_size: 14.0,
            show_addresses: true,
            bytes_per_row: None,
            read_only: false,
            last_mouse_pos: None,
        }
    }
    
    /// Create hex editor with model
    pub fn with_model(model: Arc<Mutex<dyn HexModel>>) -> Self {
        let mut editor = Self::new();
        editor.set_model(Some(model));
        editor
    }
    
    /// Set the data model
    pub fn set_model(&mut self, model: Option<Arc<Mutex<dyn HexModel>>>) {
        self.model = model;
        self.measures.recompute(self.get_model_ref().as_deref());
        self.caret.clear_selection(&mut self.highlighter);
        self.update_preferred_size();
    }
    
    /// Get the data model
    pub fn get_model(&self) -> Option<Arc<Mutex<dyn HexModel>>> {
        self.model.clone()
    }
    
    /// Set font size
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
        self.measures.recompute(self.get_model_ref().as_deref());
        self.update_preferred_size();
    }
    
    /// Set read-only mode
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }
    
    /// Set whether to show addresses
    pub fn set_show_addresses(&mut self, show: bool) {
        self.show_addresses = show;
        self.measures.recompute(self.get_model_ref().as_deref());
        self.update_preferred_size();
    }
    
    /// Set bytes per row (None for auto-calculation)
    pub fn set_bytes_per_row(&mut self, bytes: Option<usize>) {
        self.bytes_per_row = bytes;
        self.measures.recompute(self.get_model_ref().as_deref());
        self.update_preferred_size();
    }
    
    /// Add a highlight
    pub fn add_highlight(&mut self, start: u64, end: u64, color: Color32) -> Option<usize> {
        self.highlighter.add(start, end, color, self.get_model_ref().as_deref())
    }
    
    /// Remove a highlight
    pub fn remove_highlight(&mut self, id: usize) -> bool {
        self.highlighter.remove(id)
    }
    
    /// Clear all highlights
    pub fn clear_highlights(&mut self) {
        self.highlighter.clear();
    }
    
    /// Select all content
    pub fn select_all(&mut self) {
        self.caret.select_all(&mut self.highlighter, self.get_model_ref().as_deref());
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.caret.clear_selection(&mut self.highlighter);
    }
    
    /// Get current selection
    pub fn get_selection(&self) -> Option<(u64, u64)> {
        self.caret.get_selection_range()
    }
    
    /// Set cursor position
    pub fn set_cursor(&mut self, address: u64) {
        self.caret.set_dot(address as i64, false, &mut self.highlighter, self.get_model_ref().as_deref());
    }
    
    /// Get cursor position
    pub fn get_cursor(&self) -> Option<u64> {
        if self.caret.get_dot() >= 0 {
            Some(self.caret.get_dot() as u64)
        } else {
            None
        }
    }
    
    /// Delete selected content (fill with zeros)
    pub fn delete_selection(&mut self) {
        if let Some((start, end)) = self.caret.get_selection_range() {
            if let Some(model) = &self.model {
                if let Ok(mut model) = model.lock() {
                    model.fill(start, end - start + 1, 0);
                }
            }
        }
    }
    
    /// Get model reference for internal use
    fn get_model_ref(&self) -> Option<Arc<Mutex<dyn HexModel>>> {
        self.model.clone()
    }
    
    /// Update preferred size based on model and measurements
    fn update_preferred_size(&mut self) {
        let (width, height) = self.measures.get_preferred_size();
        self.preferred_size = Vec2::new(width, height);
    }
    
    /// Paint the hex editor content
    fn paint_content(&mut self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter_at(rect);
        
        // Get model reference
        let model_guard = match &self.model {
            Some(model) => match model.lock() {
                Ok(guard) => Some(guard),
                Err(_) => None,
            },
            None => None,
        };
        
        let model_ref = model_guard.as_deref();
        
        // Update measurements for current size
        if self.measures.width_changed(rect.width(), model_ref) {
            // Size changed, may need to repaint
        }
        
        // Calculate visible address range
        let visible_start = self.measures.to_address(0.0, rect.min.y, model_ref).unwrap_or(0);
        let visible_end = self.measures.to_address(rect.width(), rect.max.y, model_ref).unwrap_or(0);
        
        // Paint background
        painter.rect_filled(rect, Rounding::ZERO, Color32::WHITE);
        
        // Paint highlights
        self.highlighter.paint(&painter, &self.measures, visible_start, visible_end, model_ref);
        
        // Paint content
        if let Some(model) = model_ref {
            self.paint_hex_content(&painter, rect, model, visible_start, visible_end);
        }
        
        // Paint cursor
        self.caret.paint_cursor(&painter, &self.measures, model_ref);
    }
    
    /// Paint hex content (addresses and values)
    fn paint_hex_content(
        &self,
        painter: &Painter,
        rect: Rect,
        model: &dyn HexModel,
        start_addr: u64,
        end_addr: u64,
    ) {
        let font_id = FontId::monospace(self.font_size);
        let text_color = Color32::BLACK;
        let address_color = Color32::DARK_GRAY;
        
        let base_addr = self.measures.get_base_address(Some(model));
        let cols = self.measures.get_column_count();
        let cell_width = self.measures.get_cell_width();
        let cell_height = self.measures.get_cell_height();
        
        // Calculate rows to display
        let start_row = (start_addr.saturating_sub(base_addr)) / cols as u64;
        let end_row = (end_addr.saturating_sub(base_addr)) / cols as u64 + 1;
        
        for row in start_row..=end_row {
            let row_addr = base_addr + row * cols as u64;
            let y = self.measures.to_y(row_addr, Some(model));
            
            if y > rect.max.y {
                break;
            }
            if y + cell_height < rect.min.y {
                continue;
            }
            
            // Paint address label
            if self.show_addresses {
                let address_text = format!("{:0width$X}", row_addr, width = self.measures.get_label_chars());
                let address_pos = Pos2::new(rect.min.x, y + cell_height * 0.8);
                painter.text(address_pos, egui::Align2::LEFT_TOP, address_text, font_id.clone(), address_color);
            }
            
            // Paint hex values
            let values_x = self.measures.get_values_x();
            for col in 0..cols {
                let addr = row_addr + col as u64;
                
                if addr < model.get_first_offset() || addr > model.get_last_offset() {
                    continue;
                }
                
                let value = model.get(addr);
                let hex_text = format!("{:0width$X}", value, width = self.measures.get_cell_chars());
                
                let x = values_x + col as f32 * cell_width + (col / 4) as f32 * cell_width * 0.1;
                let pos = Pos2::new(x, y + cell_height * 0.8);
                
                painter.text(pos, egui::Align2::LEFT_TOP, hex_text, font_id.clone(), text_color);
            }
        }
    }
    
    /// Handle input events
    fn handle_input(&mut self, response: &Response, ui: &mut Ui) {
        // Handle keyboard input
        if response.has_focus() {
            let events = ui.input(|i| i.events.clone());
            for event in events {
                if let egui::Event::Key { key, pressed: true, modifiers, .. } = event {
                    if self.caret.handle_key_input(key, modifiers, &self.measures, &mut self.highlighter, self.get_model_ref().as_deref().map(|m| m.lock().ok()).flatten().as_deref()) {
                        ui.ctx().request_repaint();
                    }
                }
            }
        }
        
        // Handle mouse input
        if let Some(pos) = response.interact_pointer_pos() {
            if response.clicked() {
                let extend_selection = ui.input(|i| i.modifiers.shift);
                self.caret.handle_mouse_click(
                    pos,
                    extend_selection,
                    &self.measures,
                    &mut self.highlighter,
                    self.get_model_ref().as_deref().map(|m| m.lock().ok()).flatten().as_deref(),
                );
                ui.ctx().request_repaint();
            }
            
            if response.dragged() {
                self.caret.handle_mouse_drag(
                    pos,
                    &self.measures,
                    &mut self.highlighter,
                    self.get_model_ref().as_deref().map(|m| m.lock().ok()).flatten().as_deref(),
                );
                ui.ctx().request_repaint();
            }
        }
    }
}

impl Widget for &mut HexEditor {
    type Response = Response;
    
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.preferred_size, Sense::click_and_drag());
        
        if ui.is_rect_visible(rect) {
            self.paint_content(ui, rect);
        }
        
        self.handle_input(&response, ui);
        
        response
    }
}

impl Default for HexEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper struct for creating a scrollable hex editor
pub struct ScrollableHexEditor<'a> {
    editor: &'a mut HexEditor,
    max_height: Option<f32>,
    max_width: Option<f32>,
}

impl<'a> ScrollableHexEditor<'a> {
    pub fn new(editor: &'a mut HexEditor) -> Self {
        Self {
            editor,
            max_height: None,
            max_width: None,
        }
    }
    
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }
    
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }
}

impl<'a> Widget for ScrollableHexEditor<'a> {
    type Response = Response;
    
    fn ui(self, ui: &mut Ui) -> Response {
        let mut scroll_area = ScrollArea::both().auto_shrink([false, false]);
        
        if let Some(height) = self.max_height {
            scroll_area = scroll_area.max_height(height);
        }
        
        if let Some(width) = self.max_width {
            scroll_area = scroll_area.max_width(width);
        }
        
        scroll_area.show(ui, |ui| {
            self.editor.ui(ui)
        }).inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::hex_model::MemoryHexModel;
    use std::sync::{Arc, Mutex};
    
    #[test]
    fn test_hex_editor_creation() {
        let editor = HexEditor::new();
        assert!(editor.get_model().is_none());
        assert!(editor.get_cursor().is_none());
    }
    
    #[test]
    fn test_hex_editor_with_model() {
        let model = Arc::new(Mutex::new(MemoryHexModel::new(256, 8)));
        let editor = HexEditor::with_model(model.clone());
        
        assert!(editor.get_model().is_some());
    }
    
    #[test]
    fn test_cursor_operations() {
        let model = Arc::new(Mutex::new(MemoryHexModel::new(256, 8)));
        let mut editor = HexEditor::with_model(model);
        
        editor.set_cursor(10);
        assert_eq!(editor.get_cursor(), Some(10));
        
        editor.select_all();
        assert!(editor.get_selection().is_some());
        
        editor.clear_selection();
        assert!(editor.get_selection().is_none());
    }
    
    #[test]
    fn test_highlight_operations() {
        let model = Arc::new(Mutex::new(MemoryHexModel::new(256, 8)));
        let mut editor = HexEditor::with_model(model);
        
        let id = editor.add_highlight(10, 20, Color32::RED);
        assert!(id.is_some());
        
        let removed = editor.remove_highlight(id.unwrap());
        assert!(removed);
    }
    
    #[test]
    fn test_configuration() {
        let mut editor = HexEditor::new();
        
        editor.set_font_size(16.0);
        editor.set_read_only(true);
        editor.set_show_addresses(false);
        editor.set_bytes_per_row(Some(8));
        
        // Configuration should be applied without panicking
    }
}