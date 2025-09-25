//! Object reordering utilities
//!
//! This module corresponds to the Java ReorderRequest class.

use super::CanvasObject;
use std::sync::Arc;

/// Request to reorder objects in a drawing
/// 
/// This represents a request to move objects from one z-order position to another.
#[derive(Debug, Clone)]
pub struct ReorderRequest {
    objects: Vec<Arc<dyn CanvasObject>>,
    destination: ReorderDestination,
}

impl ReorderRequest {
    /// Create a new reorder request
    pub fn new(objects: Vec<Arc<dyn CanvasObject>>, destination: ReorderDestination) -> Self {
        Self {
            objects,
            destination,
        }
    }
    
    /// Get the objects to be reordered
    pub fn objects(&self) -> &[Arc<dyn CanvasObject>] {
        &self.objects
    }
    
    /// Get the destination for the reorder
    pub fn destination(&self) -> ReorderDestination {
        self.destination
    }
    
    /// Check if this request is valid (has objects to move)
    pub fn is_valid(&self) -> bool {
        !self.objects.is_empty()
    }
}

/// Destination for reordering objects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReorderDestination {
    /// Move to the front (highest z-order)
    ToFront,
    /// Move to the back (lowest z-order)
    ToBack,
    /// Move forward one position
    Forward,
    /// Move backward one position
    Backward,
    /// Move to a specific index
    ToIndex(usize),
}

impl ReorderDestination {
    /// Calculate the target index for the reorder operation
    pub fn calculate_target_index(&self, current_indices: &[usize], total_count: usize) -> Option<usize> {
        if current_indices.is_empty() {
            return None;
        }
        
        match self {
            ReorderDestination::ToFront => Some(total_count - current_indices.len()),
            ReorderDestination::ToBack => Some(0),
            ReorderDestination::Forward => {
                let max_current = current_indices.iter().max().unwrap();
                if *max_current + 1 < total_count {
                    Some(max_current + 1)
                } else {
                    None // Already at front
                }
            },
            ReorderDestination::Backward => {
                let min_current = current_indices.iter().min().unwrap();
                if *min_current > 0 {
                    Some(min_current - 1)
                } else {
                    None // Already at back
                }
            },
            ReorderDestination::ToIndex(index) => Some(*index),
        }
    }
}

/// Utility functions for reordering operations
pub struct ReorderUtils;

impl ReorderUtils {
    /// Check if objects can be moved forward in z-order
    pub fn can_move_forward(indices: &[usize], total_count: usize) -> bool {
        if indices.is_empty() {
            return false;
        }
        
        let max_index = indices.iter().max().unwrap();
        *max_index + 1 < total_count
    }
    
    /// Check if objects can be moved backward in z-order
    pub fn can_move_backward(indices: &[usize]) -> bool {
        if indices.is_empty() {
            return false;
        }
        
        let min_index = indices.iter().min().unwrap();
        *min_index > 0
    }
    
    /// Check if objects can be moved to front
    pub fn can_move_to_front(indices: &[usize], total_count: usize) -> bool {
        if indices.is_empty() {
            return false;
        }
        
        // Can move to front if any object is not already at the front
        let expected_front_indices: Vec<usize> = ((total_count - indices.len())..total_count).collect();
        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort_unstable();
        
        sorted_indices != expected_front_indices
    }
    
    /// Check if objects can be moved to back
    pub fn can_move_to_back(indices: &[usize]) -> bool {
        if indices.is_empty() {
            return false;
        }
        
        // Can move to back if any object is not already at the back
        let expected_back_indices: Vec<usize> = (0..indices.len()).collect();
        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort_unstable();
        
        sorted_indices != expected_back_indices
    }
    
    /// Calculate the new indices after a reorder operation
    pub fn calculate_new_indices(
        objects_to_move: &[usize], 
        destination: ReorderDestination, 
        total_count: usize
    ) -> Vec<usize> {
        let mut result = Vec::new();
        
        if let Some(target_index) = destination.calculate_target_index(objects_to_move, total_count) {
            for i in 0..objects_to_move.len() {
                result.push(target_index + i);
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw::model::{AbstractCanvasObject, CanvasObjectId};
    
    #[test]
    fn test_reorder_request_creation() {
        let objects = vec![
            Arc::new(AbstractCanvasObject::new(
                CanvasObjectId(1), 
                "Object 1".to_string()
            )) as Arc<dyn CanvasObject>
        ];
        
        let request = ReorderRequest::new(objects.clone(), ReorderDestination::ToFront);
        
        assert!(request.is_valid());
        assert_eq!(request.objects().len(), 1);
        assert_eq!(request.destination(), ReorderDestination::ToFront);
    }
    
    #[test]
    fn test_reorder_destination_calculations() {
        // Test moving to front
        let indices = vec![1, 2];
        let target = ReorderDestination::ToFront.calculate_target_index(&indices, 5);
        assert_eq!(target, Some(3)); // Move to positions 3,4 (front of 5 total)
        
        // Test moving to back
        let target = ReorderDestination::ToBack.calculate_target_index(&indices, 5);
        assert_eq!(target, Some(0)); // Move to positions 0,1 (back)
        
        // Test moving forward
        let target = ReorderDestination::Forward.calculate_target_index(&indices, 5);
        assert_eq!(target, Some(3)); // Move one position forward from max index 2
        
        // Test moving backward
        let target = ReorderDestination::Backward.calculate_target_index(&indices, 5);
        assert_eq!(target, Some(0)); // Move one position backward from min index 1
    }
    
    #[test]
    fn test_reorder_utils() {
        // Test can move forward
        assert!(ReorderUtils::can_move_forward(&[0, 1], 5));
        assert!(!ReorderUtils::can_move_forward(&[3, 4], 5));
        
        // Test can move backward
        assert!(ReorderUtils::can_move_backward(&[1, 2]));
        assert!(!ReorderUtils::can_move_backward(&[0, 1]));
        
        // Test can move to front
        assert!(ReorderUtils::can_move_to_front(&[0, 1], 5));
        assert!(!ReorderUtils::can_move_to_front(&[3, 4], 5));
        
        // Test can move to back
        assert!(ReorderUtils::can_move_to_back(&[2, 3]));
        assert!(!ReorderUtils::can_move_to_back(&[0, 1]));
    }
}