//! Selection management system
//!
//! This module corresponds to the Java Selection, SelectionEvent, and SelectionListener classes.

use crate::draw::model::CanvasObject;
use std::collections::HashSet;
use std::sync::Arc;

/// Manages the selection of canvas objects
pub struct Selection {
    selected_ids: HashSet<u64>, // Store object IDs instead of object references
    selected_objects: Vec<Arc<dyn CanvasObject>>, // Keep references for access
    listeners: Vec<Box<dyn SelectionListener>>,
}

impl Selection {
    /// Create a new empty selection
    pub fn new() -> Self {
        Self {
            selected_ids: HashSet::new(),
            selected_objects: Vec::new(),
            listeners: Vec::new(),
        }
    }
    
    /// Check if the selection is empty
    pub fn is_empty(&self) -> bool {
        self.selected_ids.is_empty()
    }
    
    /// Get the number of selected objects
    pub fn size(&self) -> usize {
        self.selected_ids.len()
    }
    
    /// Add an object to the selection
    pub fn add(&mut self, object: Arc<dyn CanvasObject>) {
        let id = object.matches_hash_code();
        if self.selected_ids.insert(id) {
            self.selected_objects.push(object.clone());
            self.fire_selection_changed(SelectionEvent::Added(object));
        }
    }
    
    /// Remove an object from the selection
    pub fn remove(&mut self, object: &dyn CanvasObject) {
        let id = object.matches_hash_code();
        if self.selected_ids.remove(&id) {
            // Find and remove from the objects vec
            if let Some(pos) = self.selected_objects.iter().position(|obj| obj.matches(object)) {
                let removed = self.selected_objects.remove(pos);
                self.fire_selection_changed(SelectionEvent::Removed(removed));
            }
        }
    }
    
    /// Toggle an object in the selection
    pub fn toggle(&mut self, object: Arc<dyn CanvasObject>) {
        if self.is_selected(object.as_ref()) {
            self.remove(object.as_ref());
        } else {
            self.add(object);
        }
    }
    
    /// Clear the entire selection
    pub fn clear(&mut self) {
        if !self.selected_ids.is_empty() {
            let cleared = self.selected_objects.drain(..).collect();
            self.selected_ids.clear();
            self.fire_selection_changed(SelectionEvent::Cleared(cleared));
        }
    }
    
    /// Set the selection to contain only the specified objects
    pub fn set(&mut self, objects: Vec<Arc<dyn CanvasObject>>) {
        let old_selection = self.selected_objects.drain(..).collect();
        self.selected_ids.clear();
        
        for obj in &objects {
            let id = obj.matches_hash_code();
            self.selected_ids.insert(id);
        }
        self.selected_objects = objects.clone();
        
        self.fire_selection_changed(SelectionEvent::Changed {
            added: objects,
            removed: old_selection,
        });
    }
    
    /// Check if an object is selected
    pub fn is_selected(&self, object: &dyn CanvasObject) -> bool {
        let id = object.matches_hash_code();
        self.selected_ids.contains(&id)
    }
    
    /// Get all selected objects
    pub fn objects(&self) -> Vec<Arc<dyn CanvasObject>> {
        self.selected_objects.clone()
    }
    
    /// Get the first selected object (if any)
    pub fn first(&self) -> Option<Arc<dyn CanvasObject>> {
        self.selected_objects.first().cloned()
    }
    
    /// Add a selection listener
    pub fn add_listener(&mut self, listener: Box<dyn SelectionListener>) {
        self.listeners.push(listener);
    }
    
    /// Fire a selection changed event to all listeners
    fn fire_selection_changed(&mut self, event: SelectionEvent) {
        for listener in &mut self.listeners {
            listener.selection_changed(&event);
        }
    }
}

impl std::fmt::Debug for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Selection")
            .field("selected_ids", &self.selected_ids)
            .field("num_objects", &self.selected_objects.len())
            .field("num_listeners", &self.listeners.len())
            .finish()
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Selection {
    fn clone(&self) -> Self {
        Self {
            selected_ids: self.selected_ids.clone(),
            selected_objects: self.selected_objects.clone(),
            listeners: Vec::new(), // Don't clone listeners
        }
    }
}

/// Events representing changes to the selection
#[derive(Debug, Clone)]
pub enum SelectionEvent {
    /// Object added to selection
    Added(Arc<dyn CanvasObject>),
    /// Object removed from selection
    Removed(Arc<dyn CanvasObject>),
    /// Selection cleared
    Cleared(Vec<Arc<dyn CanvasObject>>),
    /// Selection completely changed
    Changed {
        added: Vec<Arc<dyn CanvasObject>>,
        removed: Vec<Arc<dyn CanvasObject>>,
    },
}

/// Listener for selection changes
pub trait SelectionListener: Send + Sync {
    /// Called when the selection changes
    fn selection_changed(&mut self, event: &SelectionEvent);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw::model::{AbstractCanvasObject, CanvasObjectId};
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct TestSelectionListener {
        event_count: AtomicUsize,
    }
    
    impl TestSelectionListener {
        fn new() -> Self {
            Self {
                event_count: AtomicUsize::new(0),
            }
        }
        
        fn event_count(&self) -> usize {
            self.event_count.load(Ordering::Relaxed)
        }
    }
    
    impl SelectionListener for TestSelectionListener {
        fn selection_changed(&mut self, _event: &SelectionEvent) {
            self.event_count.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    #[test]
    fn test_selection_creation() {
        let selection = Selection::new();
        assert!(selection.is_empty());
        assert_eq!(selection.size(), 0);
    }
    
    #[test]
    fn test_add_remove_objects() {
        let mut selection = Selection::new();
        
        let obj1 = Arc::new(AbstractCanvasObject::new(
            CanvasObjectId(1), 
            "Object 1".to_string()
        )) as Arc<dyn CanvasObject>;
        
        let obj2 = Arc::new(AbstractCanvasObject::new(
            CanvasObjectId(2), 
            "Object 2".to_string()
        )) as Arc<dyn CanvasObject>;
        
        selection.add(obj1.clone());
        assert_eq!(selection.size(), 1);
        assert!(selection.is_selected(obj1.as_ref()));
        
        selection.add(obj2.clone());
        assert_eq!(selection.size(), 2);
        assert!(selection.is_selected(obj2.as_ref()));
        
        selection.remove(obj1.as_ref());
        assert_eq!(selection.size(), 1);
        assert!(!selection.is_selected(obj1.as_ref()));
        assert!(selection.is_selected(obj2.as_ref()));
        
        selection.clear();
        assert!(selection.is_empty());
    }
    
    #[test]
    fn test_selection_toggle() {
        let mut selection = Selection::new();
        
        let obj = Arc::new(AbstractCanvasObject::new(
            CanvasObjectId(1), 
            "Object 1".to_string()
        )) as Arc<dyn CanvasObject>;
        
        selection.toggle(obj.clone());
        assert!(selection.is_selected(obj.as_ref()));
        
        selection.toggle(obj.clone());
        assert!(!selection.is_selected(obj.as_ref()));
    }
}