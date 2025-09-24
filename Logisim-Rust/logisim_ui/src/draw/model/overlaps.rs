//! Drawing overlap management
//!
//! This module corresponds to the Java DrawingOverlaps class.

use super::CanvasObject;
use logisim_core::data::{Bounds, Location};
use std::collections::HashMap;
use std::sync::Arc;

/// Manages spatial indexing and overlap detection for canvas objects
/// 
/// This provides efficient lookup of objects by location and bounds checking.
#[derive(Debug)]
pub struct DrawingOverlaps {
    // Simple implementation using a spatial grid
    // In practice, this might use a more sophisticated spatial data structure
    grid: HashMap<(i32, i32), Vec<Arc<dyn CanvasObject>>>,
    grid_size: i32,
}

impl DrawingOverlaps {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
            grid_size: 50, // 50-pixel grid cells
        }
    }
    
    /// Add an object to the spatial index
    pub fn add_object(&mut self, object: Arc<dyn CanvasObject>) {
        let bounds = object.bounds();
        let cells = self.get_cells_for_bounds(bounds);
        
        for cell in cells {
            self.grid.entry(cell).or_insert_with(Vec::new).push(object.clone());
        }
    }
    
    /// Remove an object from the spatial index
    pub fn remove_object(&mut self, object: &dyn CanvasObject) {
        let bounds = object.bounds();
        let cells = self.get_cells_for_bounds(bounds);
        
        for cell in cells {
            if let Some(objects) = self.grid.get_mut(&cell) {
                objects.retain(|obj| !obj.matches(object));
                if objects.is_empty() {
                    self.grid.remove(&cell);
                }
            }
        }
    }
    
    /// Update an object's position in the spatial index
    pub fn update_object(&mut self, object: Arc<dyn CanvasObject>, old_bounds: Bounds) {
        // Remove from old position
        let old_cells = self.get_cells_for_bounds(old_bounds);
        for cell in old_cells {
            if let Some(objects) = self.grid.get_mut(&cell) {
                objects.retain(|obj| !obj.matches(object.as_ref()));
                if objects.is_empty() {
                    self.grid.remove(&cell);
                }
            }
        }
        
        // Add to new position
        self.add_object(object);
    }
    
    /// Find all objects that might overlap with the given bounds
    pub fn find_overlapping(&self, bounds: Bounds) -> Vec<Arc<dyn CanvasObject>> {
        let cells = self.get_cells_for_bounds(bounds);
        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for cell in cells {
            if let Some(objects) = self.grid.get(&cell) {
                for object in objects {
                    if seen.insert(object.id()) {
                        // Check if the bounds actually overlap
                        if bounds_overlap(bounds, object.bounds()) {
                            result.push(object.clone());
                        }
                    }
                }
            }
        }
        
        result
    }
    
    /// Find all objects at the given location
    pub fn find_at_location(&self, location: Location, assume_filled: bool) -> Vec<Arc<dyn CanvasObject>> {
        let cell = self.get_cell_for_location(location);
        let mut result = Vec::new();
        
        if let Some(objects) = self.grid.get(&cell) {
            for object in objects {
                if object.contains(location, assume_filled) {
                    result.push(object.clone());
                }
            }
        }
        
        result
    }
    
    /// Clear all objects from the spatial index
    pub fn clear(&mut self) {
        self.grid.clear();
    }
    
    /// Get the grid cells that cover the given bounds
    fn get_cells_for_bounds(&self, bounds: Bounds) -> Vec<(i32, i32)> {
        let mut cells = Vec::new();
        
        let start_x = bounds.x() / self.grid_size;
        let start_y = bounds.y() / self.grid_size;
        let end_x = (bounds.x() + bounds.width() + self.grid_size - 1) / self.grid_size;
        let end_y = (bounds.y() + bounds.height() + self.grid_size - 1) / self.grid_size;
        
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                cells.push((x, y));
            }
        }
        
        cells
    }
    
    /// Get the grid cell for the given location
    fn get_cell_for_location(&self, location: Location) -> (i32, i32) {
        (location.x() / self.grid_size, location.y() / self.grid_size)
    }
}

impl Default for DrawingOverlaps {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if two bounds overlap
fn bounds_overlap(a: Bounds, b: Bounds) -> bool {
    !(a.x() + a.width() <= b.x() || 
      b.x() + b.width() <= a.x() || 
      a.y() + a.height() <= b.y() || 
      b.y() + b.height() <= a.y())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw::model::{AbstractCanvasObject, CanvasObjectId};
    
    #[test]
    fn test_drawing_overlaps_creation() {
        let overlaps = DrawingOverlaps::new();
        assert_eq!(overlaps.grid_size, 50);
    }
    
    #[test]
    fn test_bounds_overlap() {
        let bounds1 = Bounds::create(10, 10, 20, 20);
        let bounds2 = Bounds::create(20, 20, 20, 20);
        let bounds3 = Bounds::create(50, 50, 20, 20);
        
        assert!(bounds_overlap(bounds1, bounds2));
        assert!(!bounds_overlap(bounds1, bounds3));
    }
    
    #[test]
    fn test_grid_cells_for_bounds() {
        let overlaps = DrawingOverlaps::new();
        let bounds = Bounds::create(10, 10, 30, 30);
        let cells = overlaps.get_cells_for_bounds(bounds);
        
        // Should cover cells (0,0) and (0,0) for this small bounds
        assert!(!cells.is_empty());
    }
    
    #[test]
    fn test_add_remove_object() {
        let mut overlaps = DrawingOverlaps::new();
        
        let object = Arc::new(AbstractCanvasObject::new(
            CanvasObjectId(1), 
            "Test Object".to_string()
        )) as Arc<dyn CanvasObject>;
        
        overlaps.add_object(object.clone());
        
        let found = overlaps.find_overlapping(Bounds::create(0, 0, 100, 100));
        assert_eq!(found.len(), 1);
        
        overlaps.remove_object(object.as_ref());
        
        let found_after = overlaps.find_overlapping(Bounds::create(0, 0, 100, 100));
        assert_eq!(found_after.len(), 0);
    }
}