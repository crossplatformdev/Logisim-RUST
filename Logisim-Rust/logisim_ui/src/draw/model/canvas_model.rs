//! Canvas model trait and event system
//!
//! This module corresponds to the Java CanvasModel interface and related event classes.

use super::{CanvasObject, ModelEventType};
use std::sync::Arc;

/// Trait for canvas model implementations
/// 
/// This defines the interface for managing collections of canvas objects
/// and providing change notification to listeners.
pub trait CanvasModel: Send + Sync {
    /// Add a listener for model changes
    fn add_canvas_model_listener(&mut self, listener: Box<dyn CanvasModelListener>);
    
    /// Remove a listener for model changes
    fn remove_canvas_model_listener(&mut self, listener: &dyn CanvasModelListener);
    
    /// Add objects to the model at the specified index
    fn add_objects(&mut self, index: usize, objects: Vec<Arc<dyn CanvasObject>>);
    
    /// Remove objects from the model
    fn remove_objects(&mut self, objects: Vec<Arc<dyn CanvasObject>>);
    
    /// Get all objects in the model
    fn objects(&self) -> &[Arc<dyn CanvasObject>];
    
    /// Get the number of objects in the model
    fn object_count(&self) -> usize;
    
    /// Get an object by index
    fn get_object(&self, index: usize) -> Option<&Arc<dyn CanvasObject>>;
    
    /// Find the index of an object
    fn index_of(&self, object: &dyn CanvasObject) -> Option<usize>;
    
    /// Reorder objects in the model
    fn reorder_objects(&mut self, objects: Vec<Arc<dyn CanvasObject>>, from_index: usize, to_index: usize);
    
    /// Move objects by the specified offset
    fn translate_objects(&mut self, objects: Vec<Arc<dyn CanvasObject>>, dx: i32, dy: i32);
}

/// Listener for canvas model changes
pub trait CanvasModelListener: Send + Sync {
    /// Called when the model changes
    fn model_changed(&self, event: &CanvasModelEvent);
}

/// Event representing a change to the canvas model
#[derive(Debug, Clone)]
pub struct CanvasModelEvent {
    event_type: ModelEventType,
    timestamp: std::time::SystemTime,
}

impl CanvasModelEvent {
    /// Create a new canvas model event
    pub fn new(event_type: ModelEventType) -> Self {
        Self {
            event_type,
            timestamp: std::time::SystemTime::now(),
        }
    }
    
    /// Get the event type
    pub fn event_type(&self) -> &ModelEventType {
        &self.event_type
    }
    
    /// Get the timestamp when the event occurred
    pub fn timestamp(&self) -> std::time::SystemTime {
        self.timestamp
    }
    
    /// Check if this event represents objects being added
    pub fn is_objects_added(&self) -> bool {
        matches!(self.event_type, ModelEventType::ObjectsAdded { .. })
    }
    
    /// Check if this event represents objects being removed
    pub fn is_objects_removed(&self) -> bool {
        matches!(self.event_type, ModelEventType::ObjectsRemoved { .. })
    }
    
    /// Check if this event represents objects being moved
    pub fn is_objects_moved(&self) -> bool {
        matches!(self.event_type, ModelEventType::ObjectsMoved { .. })
    }
    
    /// Check if this event represents objects being reordered
    pub fn is_objects_reordered(&self) -> bool {
        matches!(self.event_type, ModelEventType::ObjectsReordered { .. })
    }
    
    /// Check if this event represents an attribute change
    pub fn is_attribute_changed(&self) -> bool {
        matches!(self.event_type, ModelEventType::AttributeChanged { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw::model::{AbstractCanvasObject, CanvasObjectId};
    
    #[test]
    fn test_canvas_model_event_creation() {
        let objects = vec![
            Arc::new(AbstractCanvasObject::new(
                CanvasObjectId(1), 
                "Test Object".to_string()
            )) as Arc<dyn CanvasObject>
        ];
        
        let event_type = ModelEventType::ObjectsAdded {
            objects: objects.clone(),
            index: 0,
        };
        
        let event = CanvasModelEvent::new(event_type);
        
        assert!(event.is_objects_added());
        assert!(!event.is_objects_removed());
        assert!(!event.is_objects_moved());
        assert!(!event.is_objects_reordered());
        assert!(!event.is_attribute_changed());
    }
    
    #[test]
    fn test_canvas_model_event_types() {
        let object = Arc::new(AbstractCanvasObject::new(
            CanvasObjectId(1), 
            "Test Object".to_string()
        )) as Arc<dyn CanvasObject>;
        
        let moved_event = CanvasModelEvent::new(ModelEventType::ObjectsMoved {
            objects: vec![object.clone()],
            dx: 10,
            dy: 20,
        });
        
        assert!(moved_event.is_objects_moved());
        assert!(!moved_event.is_objects_added());
        
        let attr_event = CanvasModelEvent::new(ModelEventType::AttributeChanged {
            object: object.clone(),
            attribute: "color".to_string(),
            old_value: "red".to_string(),
            new_value: "blue".to_string(),
        });
        
        assert!(attr_event.is_attribute_changed());
        assert!(!attr_event.is_objects_moved());
    }
}