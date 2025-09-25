//! Line shape implementation
//!
//! This module corresponds to the Java Line class.

use crate::draw::model::{CanvasObject, AbstractCanvasObject, DrawingContext, Handle, HandleGesture, CanvasObjectId, AttributeAccess, Color32, Stroke};
use crate::draw::{DrawError, DrawResult};
use logisim_core::data::{AttributeSet, Bounds, Location};
use super::DrawAttr;

/// A line shape that can be drawn on a canvas
#[derive(Debug, Clone)]
pub struct Line {
    base: AbstractCanvasObject,
    start: Location,
    end: Location,
}

impl Line {
    /// Create a new line from start to end
    pub fn new(id: CanvasObjectId, start: Location, end: Location) -> Self {
        let attributes = AttributeSet::new();
        
        Self {
            base: AbstractCanvasObject::with_attributes(id, "Line".to_string(), attributes),
            start,
            end,
        }
    }
    
    /// Get the start point of the line
    pub fn start(&self) -> Location {
        self.start
    }
    
    /// Get the end point of the line
    pub fn end(&self) -> Location {
        self.end
    }
    
    /// Set the start point of the line
    pub fn set_start(&mut self, start: Location) {
        self.start = start;
    }
    
    /// Set the end point of the line
    pub fn set_end(&mut self, end: Location) {
        self.end = end;
    }
    
    /// Get the length of the line
    pub fn length(&self) -> f64 {
        let dx = (self.end.x - self.start.x) as f64;
        let dy = (self.end.y - self.start.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

impl CanvasObject for Line {
    fn id(&self) -> CanvasObjectId {
        self.base.id()
    }
    
    fn can_delete_handle(&self, _desired: Location) -> Option<Handle> {
        None // Lines don't support handle deletion
    }
    
    fn can_insert_handle(&self, _desired: Location) -> Option<Handle> {
        None // Lines don't support handle insertion
    }
    
    fn can_move_handle(&self, handle: &Handle) -> bool {
        // Lines can move their endpoint handles
        handle.location() == self.start || handle.location() == self.end
    }
    
    fn can_remove(&self) -> bool {
        true
    }
    
    fn clone_object(&self) -> Box<dyn CanvasObject> {
        Box::new(self.clone())
    }
    
    fn contains(&self, loc: Location, _assume_filled: bool) -> bool {
        // Check if the location is close to the line (within tolerance)
        let tolerance = 3; // pixels
        
        // Use point-to-line distance formula
        let a = (self.end.y - self.start.y) as f64;
        let b = (self.start.x - self.end.x) as f64;
        let c = (self.end.x * self.start.y - self.start.x * self.end.y) as f64;
        
        let distance = (a * loc.x as f64 + b * loc.y as f64 + c).abs() / (a * a + b * b).sqrt();
        
        distance <= tolerance as f64
    }
    
    fn delete_handle(&mut self, _handle: &Handle) -> Option<Handle> {
        None
    }
    
    fn attribute_set(&self) -> &AttributeSet {
        self.base.attribute_set()
    }
    
    fn attribute_set_mut(&mut self) -> &mut AttributeSet {
        self.base.attribute_set_mut()
    }
    
    fn bounds(&self) -> Bounds {
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);
        let max_x = self.start.x.max(self.end.x);
        let max_y = self.start.y.max(self.end.y);
        
        Bounds::create(min_x, min_y, max_x - min_x, max_y - min_y)
    }
    
    fn display_name(&self) -> &str {
        self.base.display_name()
    }
    
    fn handles(&self, gesture: HandleGesture) -> Vec<Handle> {
        if gesture.shows_handles() {
            vec![
                Handle::new(self.start),
                Handle::new(self.end),
            ]
        } else {
            Vec::new()
        }
    }
    
    fn insert_handle(&mut self, _desired: Handle, _previous: Option<Handle>) {
        // Lines don't support handle insertion
    }
    
    fn matches(&self, other: &dyn CanvasObject) -> bool {
        if let Some(other_line) = other.as_any().downcast_ref::<Line>() {
            self.start == other_line.start && self.end == other_line.end
        } else {
            false
        }
    }
    
    fn matches_hash_code(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.start.hash(&mut hasher);
        self.end.hash(&mut hasher);
        hasher.finish()
    }
    
    fn move_handle(&mut self, handle: Handle, new_location: Location) -> DrawResult<Handle> {
        if handle.location() == self.start {
            self.start = new_location;
            Ok(Handle::new(new_location))
        } else if handle.location() == self.end {
            self.end = new_location;
            Ok(Handle::new(new_location))
        } else {
            Err(DrawError::InvalidObject("Handle not found on line".to_string()))
        }
    }
    
    fn translate(&mut self, dx: i32, dy: i32) {
        self.start = Location::new(self.start.x + dx, self.start.y + dy);
        self.end = Location::new(self.end.x + dx, self.end.y + dy);
    }
    
    fn paint(&self, g: &mut dyn DrawingContext, highlighted: bool) {
        let stroke_width = self.get_attribute_value(DrawAttr::STROKE_WIDTH)
            .and_then(|s| s.parse().ok())
            .unwrap_or(DrawAttr::DEFAULT_STROKE_WIDTH) as f32;
        
        let stroke_color = if highlighted {
            Color32::RED
        } else {
            DrawAttr::DEFAULT_STROKE_COLOR
        };
        
        g.set_stroke(Stroke::new(stroke_width, stroke_color));
        g.draw_line(
            self.start.x as f32,
            self.start.y as f32,
            self.end.x as f32,
            self.end.y as f32,
        );
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl AttributeAccess for Line {
    fn get_attribute_value(&self, attr_name: &str) -> Option<String> {
        match attr_name {
            DrawAttr::STROKE_WIDTH => Some(DrawAttr::DEFAULT_STROKE_WIDTH.to_string()),
            DrawAttr::STROKE_COLOR => Some("black".to_string()),
            _ => self.base.get_attribute_value(attr_name),
        }
    }
    
    fn set_attribute_value(&mut self, attr_name: &str, value: String) -> DrawResult<()> {
        self.base.set_attribute_value(attr_name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_line_creation() {
        let start = Location::new(10, 20);
        let end = Location::new(30, 40);
        let line = Line::new(CanvasObjectId(1), start, end);
        
        assert_eq!(line.start(), start);
        assert_eq!(line.end(), end);
        assert_eq!(line.display_name(), "Line");
    }
    
    #[test]
    fn test_line_bounds() {
        let line = Line::new(
            CanvasObjectId(1),
            Location::new(10, 20),
            Location::new(30, 40)
        );
        
        let bounds = line.bounds();
        assert_eq!(bounds.get_x(), 10);
        assert_eq!(bounds.get_y(), 20);
        assert_eq!(bounds.get_width(), 20);
        assert_eq!(bounds.get_height(), 20);
    }
    
    #[test]
    fn test_line_translation() {
        let mut line = Line::new(
            CanvasObjectId(1),
            Location::new(10, 20),
            Location::new(30, 40)
        );
        
        line.translate(5, 10);
        
        assert_eq!(line.start(), Location::new(15, 30));
        assert_eq!(line.end(), Location::new(35, 50));
    }
    
    #[test]
    fn test_line_length() {
        let line = Line::new(
            CanvasObjectId(1),
            Location::new(0, 0),
            Location::new(3, 4)
        );
        
        assert_eq!(line.length(), 5.0); // 3-4-5 triangle
    }
    
    #[test] 
    fn test_line_attribute_access() {
        let line = Line::new(CanvasObjectId(1), Location::new(0, 0), Location::new(10, 10));
        
        assert_eq!(line.get_attribute_value(DrawAttr::STROKE_WIDTH), Some("1".to_string()));
        assert_eq!(line.get_attribute_value(DrawAttr::STROKE_COLOR), Some("black".to_string()));
    }
}