//! Handle system for object manipulation
//!
//! This module corresponds to the Java Handle and HandleGesture classes.

use logisim_core::data::Location;

/// Represents a handle that can be used to manipulate canvas objects
/// 
/// Handles are small control points that appear around selected objects
/// and allow users to resize, reshape, or otherwise modify the objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Handle {
    location: Location,
    handle_type: HandleType,
}

impl Handle {
    /// Create a new handle at the specified location
    pub fn new(location: Location) -> Self {
        Self {
            location,
            handle_type: HandleType::Default,
        }
    }
    
    /// Create a new handle with a specific type
    pub fn with_type(location: Location, handle_type: HandleType) -> Self {
        Self {
            location,
            handle_type,
        }
    }
    
    /// Get the location of this handle
    pub fn location(&self) -> Location {
        self.location
    }
    
    /// Get the type of this handle
    pub fn handle_type(&self) -> HandleType {
        self.handle_type
    }
    
    /// Move this handle to a new location
    pub fn move_to(&mut self, new_location: Location) {
        self.location = new_location;
    }
    
    /// Check if this handle is at the specified location (within tolerance)
    pub fn is_at(&self, location: Location, tolerance: i32) -> bool {
        let dx = (self.location.x() - location.x()).abs();
        let dy = (self.location.y() - location.y()).abs();
        dx <= tolerance && dy <= tolerance
    }
}

/// Types of handles for different manipulation operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandleType {
    /// Default handle for general manipulation
    Default,
    /// Handle for resizing (corner or edge)
    Resize,
    /// Handle for moving the entire object
    Move,
    /// Handle for rotating the object
    Rotate,
    /// Handle for inserting new control points
    Insert,
    /// Handle for deleting control points
    Delete,
    /// Handle for curve control (for bezier curves)
    CurveControl,
}

/// Context information for handle gesture operations
/// 
/// This provides context about what type of operation is being performed
/// and helps determine which handles should be visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleGesture {
    /// No active gesture - show default handles
    None,
    /// User is selecting objects
    Select,
    /// User is moving objects
    Move,
    /// User is resizing objects
    Resize,
    /// User is creating new objects
    Create,
    /// User is editing object shape (adding/removing points)
    Edit,
}

impl Default for HandleGesture {
    fn default() -> Self {
        HandleGesture::None
    }
}

impl HandleGesture {
    /// Check if this gesture allows handle visibility
    pub fn shows_handles(&self) -> bool {
        match self {
            HandleGesture::None | HandleGesture::Select | HandleGesture::Edit => true,
            HandleGesture::Move | HandleGesture::Resize | HandleGesture::Create => false,
        }
    }
    
    /// Check if this gesture allows handle interaction
    pub fn allows_handle_interaction(&self) -> bool {
        match self {
            HandleGesture::None | HandleGesture::Select | HandleGesture::Edit => true,
            HandleGesture::Move | HandleGesture::Resize | HandleGesture::Create => false,
        }
    }
}

/// Collection of handles for an object
#[derive(Debug, Clone)]
pub struct HandleSet {
    handles: Vec<Handle>,
}

impl HandleSet {
    /// Create a new empty handle set
    pub fn new() -> Self {
        Self {
            handles: Vec::new(),
        }
    }
    
    /// Create a handle set with the given handles
    pub fn from_handles(handles: Vec<Handle>) -> Self {
        Self { handles }
    }
    
    /// Add a handle to the set
    pub fn add(&mut self, handle: Handle) {
        self.handles.push(handle);
    }
    
    /// Remove a handle from the set
    pub fn remove(&mut self, handle: &Handle) {
        self.handles.retain(|h| h != handle);
    }
    
    /// Get all handles in the set
    pub fn handles(&self) -> &[Handle] {
        &self.handles
    }
    
    /// Find the handle closest to the given location
    pub fn find_handle(&self, location: Location, tolerance: i32) -> Option<&Handle> {
        self.handles
            .iter()
            .find(|handle| handle.is_at(location, tolerance))
    }
    
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
    
    /// Get the number of handles
    pub fn len(&self) -> usize {
        self.handles.len()
    }
    
    /// Clear all handles
    pub fn clear(&mut self) {
        self.handles.clear();
    }
}

impl Default for HandleSet {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for HandleSet {
    type Item = Handle;
    type IntoIter = std::vec::IntoIter<Handle>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.handles.into_iter()
    }
}

impl<'a> IntoIterator for &'a HandleSet {
    type Item = &'a Handle;
    type IntoIter = std::slice::Iter<'a, Handle>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.handles.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_handle_creation() {
        let location = Location::create(10, 20);
        let handle = Handle::new(location);
        
        assert_eq!(handle.location(), location);
        assert_eq!(handle.handle_type(), HandleType::Default);
    }
    
    #[test]
    fn test_handle_with_type() {
        let location = Location::create(10, 20);
        let handle = Handle::with_type(location, HandleType::Resize);
        
        assert_eq!(handle.location(), location);
        assert_eq!(handle.handle_type(), HandleType::Resize);
    }
    
    #[test]
    fn test_handle_is_at() {
        let location = Location::create(10, 20);
        let handle = Handle::new(location);
        
        assert!(handle.is_at(Location::create(10, 20), 0));
        assert!(handle.is_at(Location::create(12, 22), 3));
        assert!(!handle.is_at(Location::create(15, 25), 3));
    }
    
    #[test]
    fn test_handle_gesture() {
        assert!(HandleGesture::None.shows_handles());
        assert!(HandleGesture::Select.shows_handles());
        assert!(!HandleGesture::Move.shows_handles());
        
        assert!(HandleGesture::None.allows_handle_interaction());
        assert!(!HandleGesture::Move.allows_handle_interaction());
    }
    
    #[test]
    fn test_handle_set() {
        let mut set = HandleSet::new();
        assert!(set.is_empty());
        
        let handle1 = Handle::new(Location::create(10, 20));
        let handle2 = Handle::new(Location::create(30, 40));
        
        set.add(handle1.clone());
        set.add(handle2.clone());
        
        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
        
        let found = set.find_handle(Location::create(11, 21), 2);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &handle1);
        
        set.remove(&handle1);
        assert_eq!(set.len(), 1);
        
        set.clear();
        assert!(set.is_empty());
    }
}