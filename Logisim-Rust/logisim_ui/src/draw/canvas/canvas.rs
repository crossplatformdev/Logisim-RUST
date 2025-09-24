//! Main canvas implementation
//!
//! This module corresponds to the Java Canvas class.

use crate::draw::model::{CanvasModel, Drawing};
use crate::draw::DrawResult;
use super::{Selection, CanvasListener, CanvasTool};

/// Interactive canvas for drawing and manipulating objects
/// 
/// The Canvas provides the main drawing surface and handles user interaction
/// with canvas objects through tools and selection.
#[derive(Debug)]
pub struct Canvas {
    model: Box<dyn CanvasModel>,
    selection: Selection,
    current_tool: Option<Box<dyn CanvasTool>>,
    listeners: Vec<Box<dyn CanvasListener>>,
    zoom: f64,
    offset_x: i32,
    offset_y: i32,
}

impl Canvas {
    /// Create a new canvas with an empty drawing
    pub fn new() -> Self {
        Self::with_model(Box::new(Drawing::new()))
    }
    
    /// Create a new canvas with the specified model
    pub fn with_model(model: Box<dyn CanvasModel>) -> Self {
        Self {
            model,
            selection: Selection::new(),
            current_tool: None,
            listeners: Vec::new(),
            zoom: 1.0,
            offset_x: 0,
            offset_y: 0,
        }
    }
    
    /// Get the canvas model
    pub fn model(&self) -> &dyn CanvasModel {
        self.model.as_ref()
    }
    
    /// Get mutable access to the canvas model
    pub fn model_mut(&mut self) -> &mut dyn CanvasModel {
        self.model.as_mut()
    }
    
    /// Get the current selection
    pub fn selection(&self) -> &Selection {
        &self.selection
    }
    
    /// Get mutable access to the current selection
    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }
    
    /// Set the current tool
    pub fn set_tool(&mut self, tool: Option<Box<dyn CanvasTool>>) {
        self.current_tool = tool;
    }
    
    /// Get the current tool
    pub fn tool(&self) -> Option<&dyn CanvasTool> {
        self.current_tool.as_ref().map(|t| t.as_ref())
    }
    
    /// Add a canvas listener
    pub fn add_listener(&mut self, listener: Box<dyn CanvasListener>) {
        self.listeners.push(listener);
    }
    
    /// Get the current zoom level
    pub fn zoom(&self) -> f64 {
        self.zoom
    }
    
    /// Set the zoom level
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom.max(0.1).min(10.0); // Clamp zoom to reasonable range
    }
    
    /// Get the canvas offset
    pub fn offset(&self) -> (i32, i32) {
        (self.offset_x, self.offset_y)
    }
    
    /// Set the canvas offset
    pub fn set_offset(&mut self, x: i32, y: i32) {
        self.offset_x = x;
        self.offset_y = y;
    }
    
    /// Convert screen coordinates to canvas coordinates
    pub fn screen_to_canvas(&self, screen_x: i32, screen_y: i32) -> (i32, i32) {
        let canvas_x = ((screen_x as f64 / self.zoom) as i32) + self.offset_x;
        let canvas_y = ((screen_y as f64 / self.zoom) as i32) + self.offset_y;
        (canvas_x, canvas_y)
    }
    
    /// Convert canvas coordinates to screen coordinates  
    pub fn canvas_to_screen(&self, canvas_x: i32, canvas_y: i32) -> (i32, i32) {
        let screen_x = ((canvas_x - self.offset_x) as f64 * self.zoom) as i32;
        let screen_y = ((canvas_y - self.offset_y) as f64 * self.zoom) as i32;
        (screen_x, screen_y)
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new();
        assert_eq!(canvas.zoom(), 1.0);
        assert_eq!(canvas.offset(), (0, 0));
        assert!(canvas.selection().is_empty());
    }
    
    #[test]
    fn test_zoom_and_offset() {
        let mut canvas = Canvas::new();
        
        canvas.set_zoom(2.0);
        assert_eq!(canvas.zoom(), 2.0);
        
        canvas.set_offset(10, 20);
        assert_eq!(canvas.offset(), (10, 20));
    }
    
    #[test]
    fn test_coordinate_conversion() {
        let mut canvas = Canvas::new();
        canvas.set_zoom(2.0);
        canvas.set_offset(10, 20);
        
        let (canvas_x, canvas_y) = canvas.screen_to_canvas(100, 200);
        assert_eq!((canvas_x, canvas_y), (60, 120)); // (100/2 + 10, 200/2 + 20)
        
        let (screen_x, screen_y) = canvas.canvas_to_screen(60, 120);
        assert_eq!((screen_x, screen_y), (100, 200)); // ((60-10)*2, (120-20)*2)
    }
}