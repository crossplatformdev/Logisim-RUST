//! Canvas object trait and base implementations
//!
//! This module corresponds to the Java CanvasObject interface and AbstractCanvasObject class.

use crate::draw::{DrawError, DrawResult};
use logisim_core::data::{AttributeSet, Bounds, Location};
use super::{Handle, HandleGesture, CanvasObjectId};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/// Trait for all objects that can be drawn on a canvas
/// 
/// This corresponds to the Java CanvasObject interface and defines the contract
/// for all drawable objects in the system.
pub trait CanvasObject: Debug + Send + Sync {
    /// Get the unique identifier for this object
    fn id(&self) -> CanvasObjectId;
    
    /// Check if a handle can be deleted at the desired location
    fn can_delete_handle(&self, desired: Location) -> Option<Handle>;
    
    /// Check if a handle can be inserted at the desired location
    fn can_insert_handle(&self, desired: Location) -> Option<Handle>;
    
    /// Check if the given handle can be moved
    fn can_move_handle(&self, handle: &Handle) -> bool;
    
    /// Check if this object can be removed from the canvas
    fn can_remove(&self) -> bool;
    
    /// Create a deep copy of this object
    fn clone_object(&self) -> Box<dyn CanvasObject>;
    
    /// Check if the given location is contained within this object
    /// 
    /// # Arguments
    /// * `loc` - The location to test
    /// * `assume_filled` - Whether to assume the object is filled when testing containment
    fn contains(&self, loc: Location, assume_filled: bool) -> bool;
    
    /// Delete the specified handle and return the new handle (if any)
    fn delete_handle(&mut self, handle: &Handle) -> Option<Handle>;
    
    /// Get the attribute set for this object
    fn attribute_set(&self) -> &AttributeSet;
    
    /// Get mutable access to the attribute set
    fn attribute_set_mut(&mut self) -> &mut AttributeSet;
    
    /// Get the bounding box of this object
    fn bounds(&self) -> Bounds;
    
    /// Get the display name for this object type
    fn display_name(&self) -> &str;
    
    /// Get the display name including any label
    fn display_name_and_label(&self) -> String {
        self.display_name().to_string()
    }
    
    /// Get the handles for this object based on the gesture context
    fn handles(&self, gesture: HandleGesture) -> Vec<Handle>;
    
    /// Insert a handle at the desired location, after the previous handle
    fn insert_handle(&mut self, desired: Handle, previous: Option<Handle>);
    
    /// Check if this object matches another object (for selection purposes)
    fn matches(&self, other: &dyn CanvasObject) -> bool;
    
    /// Get a hash code for matching purposes
    fn matches_hash_code(&self) -> u64;
    
    /// Move a handle to a new location
    fn move_handle(&mut self, handle: Handle, new_location: Location) -> DrawResult<Handle>;
    
    /// Move the entire object by the specified offset
    fn translate(&mut self, dx: i32, dy: i32);
    
    /// Paint this object to the graphics context
    fn paint(&self, g: &mut dyn DrawingContext, highlighted: bool);
    
    /// Get this object as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Get this object as mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Separate trait for attribute value operations to avoid generic methods in CanvasObject
pub trait AttributeAccess {
    /// Get the value of a specific attribute
    fn get_attribute_value(&self, attr_name: &str) -> Option<String>;
    
    /// Set the value of a specific attribute
    fn set_attribute_value(&mut self, attr_name: &str, value: String) -> DrawResult<()>;
}

/// Graphics context abstraction for drawing operations
pub trait DrawingContext {
    /// Set the current color
    fn set_color(&mut self, color: Color32);
    
    /// Set the current stroke
    fn set_stroke(&mut self, stroke: Stroke);
    
    /// Draw a line from (x1, y1) to (x2, y2)
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32);
    
    /// Draw a rectangle
    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32);
    
    /// Fill a rectangle
    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32);
    
    /// Draw an oval
    fn draw_oval(&mut self, x: f32, y: f32, width: f32, height: f32);
    
    /// Fill an oval
    fn fill_oval(&mut self, x: f32, y: f32, width: f32, height: f32);
    
    /// Draw text at the specified location
    fn draw_text(&mut self, text: &str, x: f32, y: f32);
}

/// Color type that works without egui dependency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color32 {
    pub r: u8,
    pub g: u8, 
    pub b: u8,
    pub a: u8,
}

impl Color32 {
    pub const BLACK: Color32 = Color32::from_rgba(0, 0, 0, 255);
    pub const WHITE: Color32 = Color32::from_rgba(255, 255, 255, 255);
    pub const RED: Color32 = Color32::from_rgba(255, 0, 0, 255);
    
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Stroke type that works without egui dependency
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub width: f32,
    pub color: Color32,
}

impl Stroke {
    pub fn new(width: f32, color: Color32) -> Self {
        Self { width, color }
    }
}

/// Base implementation for canvas objects
/// 
/// This corresponds to the Java AbstractCanvasObject class and provides
/// common functionality for all canvas objects.
pub struct AbstractCanvasObject {
    id: CanvasObjectId,
    attributes: AttributeSet,
    display_name: String,
}

impl Clone for AbstractCanvasObject {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            attributes: AttributeSet::new(), // Create new empty attributes - in practice would clone properly
            display_name: self.display_name.clone(),
        }
    }
}

impl std::fmt::Debug for AbstractCanvasObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbstractCanvasObject")
            .field("id", &self.id)
            .field("display_name", &self.display_name)
            .finish()
    }
}

impl AbstractCanvasObject {
    pub fn new(id: CanvasObjectId, display_name: String) -> Self {
        Self {
            id,
            attributes: AttributeSet::new(),
            display_name,
        }
    }
    
    pub fn with_attributes(id: CanvasObjectId, display_name: String, attributes: AttributeSet) -> Self {
        Self {
            id,
            attributes,
            display_name,
        }
    }
}

impl CanvasObject for AbstractCanvasObject {
    fn id(&self) -> CanvasObjectId {
        self.id
    }
    
    fn can_delete_handle(&self, _desired: Location) -> Option<Handle> {
        None // Default implementation - no handles can be deleted
    }
    
    fn can_insert_handle(&self, _desired: Location) -> Option<Handle> {
        None // Default implementation - no handles can be inserted
    }
    
    fn can_move_handle(&self, _handle: &Handle) -> bool {
        false // Default implementation - no handles can be moved
    }
    
    fn can_remove(&self) -> bool {
        true // Default implementation - most objects can be removed
    }
    
    fn clone_object(&self) -> Box<dyn CanvasObject> {
        Box::new(self.clone())
    }
    
    fn contains(&self, _loc: Location, _assume_filled: bool) -> bool {
        false // Default implementation - must be overridden by subclasses
    }
    
    fn delete_handle(&mut self, _handle: &Handle) -> Option<Handle> {
        None // Default implementation - no handle deletion supported
    }
    
    fn attribute_set(&self) -> &AttributeSet {
        &self.attributes
    }
    
    fn attribute_set_mut(&mut self) -> &mut AttributeSet {
        &mut self.attributes
    }
    
    fn bounds(&self) -> Bounds {
        Bounds::create(0, 0, 0, 0) // Default implementation - must be overridden
    }
    
    fn display_name(&self) -> &str {
        &self.display_name
    }
    
    fn handles(&self, _gesture: HandleGesture) -> Vec<Handle> {
        Vec::new() // Default implementation - no handles
    }
    
    fn insert_handle(&mut self, _desired: Handle, _previous: Option<Handle>) {
        // Default implementation - no handle insertion supported
    }
    
    fn matches(&self, other: &dyn CanvasObject) -> bool {
        self.id == other.id()
    }
    
    fn matches_hash_code(&self) -> u64 {
        self.id.0
    }
    
    fn move_handle(&mut self, _handle: Handle, _new_location: Location) -> DrawResult<Handle> {
        Err(DrawError::UnsupportedOperation("Handle movement not supported".to_string()))
    }
    
    fn translate(&mut self, _dx: i32, _dy: i32) {
        // Default implementation - no translation supported, must be overridden
    }
    
    fn paint(&self, _g: &mut dyn DrawingContext, _highlighted: bool) {
        // Default implementation - no painting, must be overridden by subclasses
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl AttributeAccess for AbstractCanvasObject {
    fn get_attribute_value(&self, attr_name: &str) -> Option<String> {
        // Simple string-based attribute access for dyn compatibility
        match attr_name {
            "stroke_width" => Some("1".to_string()),
            "stroke_color" => Some("black".to_string()),
            "fill_color" => Some("none".to_string()),
            _ => None,
        }
    }
    
    fn set_attribute_value(&mut self, attr_name: &str, value: String) -> DrawResult<()> {
        // Simple string-based attribute setting
        // In a real implementation, this would update the actual attribute set
        log::debug!("Setting attribute {} to {}", attr_name, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::CanvasObjectId;
    
    #[test]
    fn test_abstract_canvas_object_creation() {
        let id = CanvasObjectId(1);
        let obj = AbstractCanvasObject::new(id, "Test Object".to_string());
        
        assert_eq!(obj.id(), id);
        assert_eq!(obj.display_name(), "Test Object");
        assert!(obj.can_remove());
        assert!(!obj.can_move_handle(&Handle::new(Location::new(0, 0))));
    }
    
    #[test]
    fn test_object_matching() {
        let id1 = CanvasObjectId(1);
        let id2 = CanvasObjectId(2);
        
        let obj1 = AbstractCanvasObject::new(id1, "Object 1".to_string());
        let obj2 = AbstractCanvasObject::new(id2, "Object 2".to_string());
        let obj1_clone = AbstractCanvasObject::new(id1, "Object 1 Clone".to_string());
        
        assert!(obj1.matches(&obj1_clone));
        assert!(!obj1.matches(&obj2));
        
        assert_eq!(obj1.matches_hash_code(), obj1_clone.matches_hash_code());
        assert_ne!(obj1.matches_hash_code(), obj2.matches_hash_code());
    }
    
    #[test]
    fn test_attribute_access() {
        let obj = AbstractCanvasObject::new(CanvasObjectId(1), "Test".to_string());
        
        assert_eq!(obj.get_attribute_value("stroke_width"), Some("1".to_string()));
        assert_eq!(obj.get_attribute_value("unknown"), None);
    }
}