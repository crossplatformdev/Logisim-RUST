//! Core drawing model types and traits
//!
//! This module defines the fundamental abstractions for the drawing framework,
//! corresponding to the Java com.cburch.draw.model package.

use crate::draw::{DrawError, DrawResult};
use logisim_core::data::{Attribute, AttributeSet, Bounds, Location};
use std::collections::HashMap;
use std::sync::{Arc, Weak};

pub mod canvas_object;
pub mod drawing;
pub mod handle;
pub mod canvas_model;
pub mod overlaps;
pub mod reorder;

// Re-export key types
pub use canvas_object::{CanvasObject, AbstractCanvasObject};
pub use drawing::Drawing;
pub use handle::{Handle, HandleGesture};
pub use canvas_model::{CanvasModel, CanvasModelEvent, CanvasModelListener};

/// Identifies a canvas object for fast lookup and comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanvasObjectId(pub u64);

/// Key for attribute-based object identification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeMapKey {
    attributes: HashMap<String, String>,
}

impl AttributeMapKey {
    pub fn new(attr_set: &AttributeSet) -> Self {
        let mut attributes = HashMap::new();
        
        // Extract key attributes for identification
        // This is a simplified version - in practice would include all relevant attributes
        Self { attributes }
    }
    
    pub fn matches(&self, other: &Self) -> bool {
        self.attributes == other.attributes
    }
}

/// Event types for canvas model changes
#[derive(Debug, Clone)]
pub enum ModelEventType {
    ObjectsAdded {
        objects: Vec<Arc<dyn CanvasObject>>,
        index: usize,
    },
    ObjectsRemoved {
        objects: Vec<Arc<dyn CanvasObject>>,
        index: usize,
    },
    ObjectsMoved {
        objects: Vec<Arc<dyn CanvasObject>>,
        dx: i32,
        dy: i32,
    },
    ObjectsReordered {
        objects: Vec<Arc<dyn CanvasObject>>,
        from_index: usize,
        to_index: usize,
    },
    AttributeChanged {
        object: Arc<dyn CanvasObject>,
        attribute: String,
        old_value: String,
        new_value: String,
    },
}

/// Manages canvas model state and provides event notification
pub struct ModelState {
    listeners: Vec<Weak<dyn CanvasModelListener>>,
    id_counter: u64,
}

impl ModelState {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
            id_counter: 0,
        }
    }
    
    pub fn next_id(&mut self) -> CanvasObjectId {
        self.id_counter += 1;
        CanvasObjectId(self.id_counter)
    }
    
    pub fn add_listener(&mut self, listener: Weak<dyn CanvasModelListener>) {
        self.listeners.push(listener);
    }
    
    pub fn remove_listener(&mut self, listener: &dyn CanvasModelListener) {
        self.listeners.retain(|weak_ref| {
            if let Some(strong_ref) = weak_ref.upgrade() {
                !std::ptr::eq(strong_ref.as_ref(), listener)
            } else {
                false // Remove dead weak references
            }
        });
    }
    
    pub fn fire_event(&mut self, event: ModelEventType) {
        let event = CanvasModelEvent::new(event);
        
        // Clean up dead weak references and notify live ones
        self.listeners.retain(|weak_ref| {
            if let Some(listener) = weak_ref.upgrade() {
                listener.model_changed(&event);
                true
            } else {
                false
            }
        });
    }
}

impl Default for ModelState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_state_id_generation() {
        let mut state = ModelState::new();
        let id1 = state.next_id();
        let id2 = state.next_id();
        
        assert_ne!(id1, id2);
        assert_eq!(id1.0, 1);
        assert_eq!(id2.0, 2);
    }
    
    #[test]
    fn test_attribute_map_key() {
        let attr_set = AttributeSet::new();
        let key1 = AttributeMapKey::new(&attr_set);
        let key2 = AttributeMapKey::new(&attr_set);
        
        assert!(key1.matches(&key2));
    }
}