/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

/// Interface for listening to changes in a HexModel
/// 
/// Implementations of this trait can be registered with a HexModel
/// to receive notifications when the data or metadata changes.
pub trait HexModelListener {
    /// Called when bytes in the model have changed
    /// 
    /// # Arguments
    /// * `start` - The starting address of the changed bytes
    /// * `num_bytes` - The number of bytes that changed
    /// * `old_values` - The previous values of the changed bytes
    fn bytes_changed(&self, start: u64, num_bytes: u64, old_values: &[u64]);

    /// Called when the model's metadata has changed
    /// 
    /// This includes changes to the value width, display offset, or size.
    fn metainfo_changed(&self);
}

/// No-op implementation for when you don't need to handle events
pub struct NullHexModelListener;

impl HexModelListener for NullHexModelListener {
    fn bytes_changed(&self, _start: u64, _num_bytes: u64, _old_values: &[u64]) {
        // Do nothing
    }

    fn metainfo_changed(&self) {
        // Do nothing
    }
}

/// Function-based listener for simple use cases
pub struct FunctionHexModelListener<F1, F2>
where
    F1: Fn(u64, u64, &[u64]),
    F2: Fn(),
{
    bytes_changed_fn: F1,
    metainfo_changed_fn: F2,
}

impl<F1, F2> FunctionHexModelListener<F1, F2>
where
    F1: Fn(u64, u64, &[u64]),
    F2: Fn(),
{
    /// Create a new function-based listener
    pub fn new(bytes_changed_fn: F1, metainfo_changed_fn: F2) -> Self {
        Self {
            bytes_changed_fn,
            metainfo_changed_fn,
        }
    }
}

impl<F1, F2> HexModelListener for FunctionHexModelListener<F1, F2>
where
    F1: Fn(u64, u64, &[u64]),
    F2: Fn(),
{
    fn bytes_changed(&self, start: u64, num_bytes: u64, old_values: &[u64]) {
        (self.bytes_changed_fn)(start, num_bytes, old_values);
    }

    fn metainfo_changed(&self) {
        (self.metainfo_changed_fn)();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_null_listener() {
        let listener = NullHexModelListener;
        
        // Should not panic
        listener.bytes_changed(0, 10, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        listener.metainfo_changed();
    }

    #[test]
    fn test_function_listener() {
        let bytes_changed_calls = Rc::new(RefCell::new(Vec::new()));
        let metainfo_calls = Rc::new(RefCell::new(0u32));
        
        let bytes_calls = bytes_changed_calls.clone();
        let meta_calls = metainfo_calls.clone();
        
        let listener = FunctionHexModelListener::new(
            move |start, num_bytes, _old_values| {
                bytes_calls.borrow_mut().push((start, num_bytes));
            },
            move || {
                *meta_calls.borrow_mut() += 1;
            },
        );
        
        // Test bytes changed
        listener.bytes_changed(5, 3, &[0xAA, 0xBB, 0xCC]);
        assert_eq!(bytes_changed_calls.borrow().len(), 1);
        assert_eq!(bytes_changed_calls.borrow()[0], (5, 3));
        
        // Test metainfo changed
        listener.metainfo_changed();
        listener.metainfo_changed();
        assert_eq!(*metainfo_calls.borrow(), 2);
    }
}