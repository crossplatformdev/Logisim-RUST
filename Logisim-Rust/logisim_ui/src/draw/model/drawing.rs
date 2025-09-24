//! Drawing implementation of the canvas model
//!
//! This module corresponds to the Java Drawing class.

use super::{CanvasModel, CanvasModelListener, CanvasModelEvent, CanvasObject, ModelEventType};
use crate::draw::DrawResult;
use logisim_core::data::Bounds;
use std::sync::Arc;

/// Main implementation of the canvas model
/// 
/// A Drawing manages a collection of canvas objects and provides
/// event notification when the collection changes.
#[derive(Debug)]
pub struct Drawing {
    objects: Vec<Arc<dyn CanvasObject>>,
    listeners: Vec<Box<dyn CanvasModelListener>>,
}

impl Drawing {
    /// Create a new empty drawing
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            listeners: Vec::new(),
        }
    }
    
    /// Create a drawing with initial objects
    pub fn with_objects(objects: Vec<Arc<dyn CanvasObject>>) -> Self {
        Self {
            objects,
            listeners: Vec::new(),
        }
    }
    
    /// Add a single object to the drawing
    pub fn add_object(&mut self, object: Arc<dyn CanvasObject>) {
        let index = self.objects.len();
        self.objects.push(object.clone());
        
        self.fire_event(ModelEventType::ObjectsAdded {
            objects: vec![object],
            index,
        });
    }
    
    /// Add a single object at a specific index
    pub fn add_object_at(&mut self, index: usize, object: Arc<dyn CanvasObject>) {
        if index > self.objects.len() {
            self.add_object(object);
        } else {
            self.objects.insert(index, object.clone());
            
            self.fire_event(ModelEventType::ObjectsAdded {
                objects: vec![object],
                index,
            });
        }
    }
    
    /// Remove a single object from the drawing
    pub fn remove_object(&mut self, object: &dyn CanvasObject) -> bool {
        if let Some(index) = self.index_of(object) {
            let removed = self.objects.remove(index);
            
            self.fire_event(ModelEventType::ObjectsRemoved {
                objects: vec![removed],
                index,
            });
            
            true
        } else {
            false
        }
    }
    
    /// Clear all objects from the drawing
    pub fn clear(&mut self) {
        if !self.objects.is_empty() {
            let objects = self.objects.drain(..).collect();
            
            self.fire_event(ModelEventType::ObjectsRemoved {
                objects,
                index: 0,
            });
        }
    }
    
    /// Get the bounds that contain all objects in the drawing
    pub fn bounds(&self) -> Bounds {
        if self.objects.is_empty() {
            return Bounds::create(0, 0, 0, 0);
        }
        
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        
        for object in &self.objects {
            let bounds = object.bounds();
            min_x = min_x.min(bounds.x());
            min_y = min_y.min(bounds.y());
            max_x = max_x.max(bounds.x() + bounds.width());
            max_y = max_y.max(bounds.y() + bounds.height());
        }
        
        Bounds::create(min_x, min_y, max_x - min_x, max_y - min_y)
    }
    
    /// Find objects that contain the specified location
    pub fn find_objects_at(&self, x: i32, y: i32, assume_filled: bool) -> Vec<Arc<dyn CanvasObject>> {
        let location = logisim_core::data::Location::create(x, y);
        
        self.objects
            .iter()
            .filter(|obj| obj.contains(location, assume_filled))
            .cloned()
            .collect()
    }
    
    /// Check if the drawing is empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
    
    /// Fire an event to all listeners
    fn fire_event(&mut self, event_type: ModelEventType) {
        let event = CanvasModelEvent::new(event_type);
        
        // Keep only listeners that are still valid
        self.listeners.retain(|listener| {
            listener.model_changed(&event);
            true // For now, we keep all listeners
        });
    }
}

impl Default for Drawing {
    fn default() -> Self {
        Self::new()
    }
}

impl CanvasModel for Drawing {
    fn add_canvas_model_listener(&mut self, listener: Box<dyn CanvasModelListener>) {
        self.listeners.push(listener);
    }
    
    fn remove_canvas_model_listener(&mut self, target: &dyn CanvasModelListener) {
        self.listeners.retain(|listener| {
            !std::ptr::eq(listener.as_ref(), target)
        });
    }
    
    fn add_objects(&mut self, index: usize, objects: Vec<Arc<dyn CanvasObject>>) {
        if objects.is_empty() {
            return;
        }
        
        let actual_index = index.min(self.objects.len());
        
        // Insert all objects at the specified index
        for (i, object) in objects.iter().enumerate() {
            self.objects.insert(actual_index + i, object.clone());
        }
        
        self.fire_event(ModelEventType::ObjectsAdded {
            objects,
            index: actual_index,
        });
    }
    
    fn remove_objects(&mut self, objects: Vec<Arc<dyn CanvasObject>>) {
        if objects.is_empty() {
            return;
        }
        
        // Find and remove objects (from highest index to lowest to avoid index shifting)
        let mut indices_and_objects: Vec<_> = objects
            .iter()
            .filter_map(|obj| {
                self.index_of(obj.as_ref()).map(|idx| (idx, obj.clone()))
            })
            .collect();
        
        // Sort by index in descending order
        indices_and_objects.sort_by(|a, b| b.0.cmp(&a.0));
        
        let mut removed_objects = Vec::new();
        let first_index = indices_and_objects.last().map(|(idx, _)| *idx).unwrap_or(0);
        
        for (index, _) in &indices_and_objects {
            removed_objects.push(self.objects.remove(*index));
        }
        
        if !removed_objects.is_empty() {
            self.fire_event(ModelEventType::ObjectsRemoved {
                objects: removed_objects,
                index: first_index,
            });
        }
    }
    
    fn objects(&self) -> &[Arc<dyn CanvasObject>] {
        &self.objects
    }
    
    fn object_count(&self) -> usize {
        self.objects.len()
    }
    
    fn get_object(&self, index: usize) -> Option<&Arc<dyn CanvasObject>> {
        self.objects.get(index)
    }
    
    fn index_of(&self, object: &dyn CanvasObject) -> Option<usize> {
        self.objects
            .iter()
            .position(|obj| obj.matches(object))
    }
    
    fn reorder_objects(&mut self, objects: Vec<Arc<dyn CanvasObject>>, from_index: usize, to_index: usize) {
        // This is a simplified implementation
        // In practice, this would need more sophisticated reordering logic
        if from_index == to_index || objects.is_empty() {
            return;
        }
        
        self.fire_event(ModelEventType::ObjectsReordered {
            objects,
            from_index,
            to_index,
        });
    }
    
    fn translate_objects(&mut self, objects: Vec<Arc<dyn CanvasObject>>, dx: i32, dy: i32) {
        if objects.is_empty() || (dx == 0 && dy == 0) {
            return;
        }
        
        // Note: In a real implementation, we would actually move the objects
        // For now, we just fire the event
        self.fire_event(ModelEventType::ObjectsMoved {
            objects,
            dx,
            dy,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw::model::{AbstractCanvasObject, CanvasObjectId};
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct TestListener {
        event_count: AtomicUsize,
    }
    
    impl TestListener {
        fn new() -> Self {
            Self {
                event_count: AtomicUsize::new(0),
            }
        }
        
        fn event_count(&self) -> usize {
            self.event_count.load(Ordering::Relaxed)
        }
    }
    
    impl CanvasModelListener for TestListener {
        fn model_changed(&self, _event: &CanvasModelEvent) {
            self.event_count.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    #[test]
    fn test_drawing_creation() {
        let drawing = Drawing::new();
        assert!(drawing.is_empty());
        assert_eq!(drawing.object_count(), 0);
    }
    
    #[test]
    fn test_add_remove_object() {
        let mut drawing = Drawing::new();
        let listener = Box::new(TestListener::new());
        let listener_ref = listener.as_ref();
        
        drawing.add_canvas_model_listener(listener);
        
        let object = Arc::new(AbstractCanvasObject::new(
            CanvasObjectId(1), 
            "Test Object".to_string()
        )) as Arc<dyn CanvasObject>;
        
        drawing.add_object(object.clone());
        assert_eq!(drawing.object_count(), 1);
        assert_eq!(listener_ref.event_count(), 1);
        
        let removed = drawing.remove_object(object.as_ref());
        assert!(removed);
        assert_eq!(drawing.object_count(), 0);
        assert_eq!(listener_ref.event_count(), 2);
    }
    
    #[test]
    fn test_drawing_bounds() {
        let drawing = Drawing::new();
        let empty_bounds = drawing.bounds();
        assert_eq!(empty_bounds.width(), 0);
        assert_eq!(empty_bounds.height(), 0);
        
        // In a real implementation, we would test with objects that have actual bounds
    }
    
    #[test]
    fn test_clear_drawing() {
        let mut drawing = Drawing::new();
        let listener = Box::new(TestListener::new());
        let listener_ref = listener.as_ref();
        
        drawing.add_canvas_model_listener(listener);
        
        // Add some objects
        for i in 1..=3 {
            let object = Arc::new(AbstractCanvasObject::new(
                CanvasObjectId(i), 
                format!("Object {}", i)
            )) as Arc<dyn CanvasObject>;
            drawing.add_object(object);
        }
        
        assert_eq!(drawing.object_count(), 3);
        assert_eq!(listener_ref.event_count(), 3); // One event per add
        
        drawing.clear();
        assert!(drawing.is_empty());
        assert_eq!(listener_ref.event_count(), 4); // One more event for clear
    }
}