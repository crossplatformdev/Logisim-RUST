//! Selection management system
//!
//! This module corresponds to the Java Selection, SelectionEvent, and SelectionListener classes.

use crate::draw::model::CanvasObject;
use std::collections::HashSet;
use std::sync::Arc;

/// Manages the selection of canvas objects
#[derive(Debug, Clone)]
pub struct Selection {
    selected: HashSet<Arc<dyn CanvasObject>>,
    listeners: Vec<Box<dyn SelectionListener>>,
}

impl Selection {
    /// Create a new empty selection
    pub fn new() -> Self {
        Self {
            selected: HashSet::new(),
            listeners: Vec::new(),
        }
    }
    
    /// Check if the selection is empty
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
    
    /// Get the number of selected objects
    pub fn size(&self) -> usize {
        self.selected.len()
    }
    
    /// Add an object to the selection
    pub fn add(&mut self, object: Arc<dyn CanvasObject>) {
        if self.selected.insert(object.clone()) {
            self.fire_selection_changed(SelectionEvent::Added(object));
        }
    }
    
    /// Remove an object from the selection
    pub fn remove(&mut self, object: &dyn CanvasObject) {
        // Find matching object by ID
        let to_remove = self.selected
            .iter()
            .find(|obj| obj.matches(object))
            .cloned();
            
        if let Some(obj) = to_remove {
            self.selected.remove(&obj);
            self.fire_selection_changed(SelectionEvent::Removed(obj));
        }
    }
    
    /// Toggle an object in the selection
    pub fn toggle(&mut self, object: Arc<dyn CanvasObject>) {
        if self.is_selected(&object) {
            self.remove(object.as_ref());
        } else {
            self.add(object);
        }
    }
    
    /// Clear the entire selection
    pub fn clear(&mut self) {
        if !self.selected.is_empty() {
            let cleared = self.selected.drain().collect();
            self.fire_selection_changed(SelectionEvent::Cleared(cleared));
        }
    }
    
    /// Set the selection to contain only the specified objects
    pub fn set(&mut self, objects: Vec<Arc<dyn CanvasObject>>) {
        let old_selection = self.selected.drain().collect();
        self.selected.extend(objects.clone());
        
        self.fire_selection_changed(SelectionEvent::Changed {
            added: objects,
            removed: old_selection,
        });
    }
    
    /// Check if an object is selected
    pub fn is_selected(&self, object: &dyn CanvasObject) -> bool {
        self.selected.iter().any(|obj| obj.matches(object))
    }
    
    /// Get all selected objects
    pub fn objects(&self) -> Vec<Arc<dyn CanvasObject>> {
        self.selected.iter().cloned().collect()
    }
    
    /// Get the first selected object (if any)
    pub fn first(&self) -> Option<Arc<dyn CanvasObject>> {
        self.selected.iter().next().cloned()
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

impl Default for Selection {
    fn default() -> Self {
        Self::new()
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