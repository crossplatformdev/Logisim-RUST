/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Data Management
//!
//! This module provides the `InstanceData` trait and associated types for storing
//! component-specific runtime data. This is equivalent to Java's `InstanceData` interface.

use std::any::Any;
use std::fmt::Debug;

/// Component-specific runtime data that persists across simulation steps.
///
/// This trait is equivalent to Java's `InstanceData` interface and provides a way
/// for components to store persistent state information during simulation.
///
/// # Thread Safety
///
/// Implementations should be thread-safe if the simulation engine supports
/// multi-threaded execution.
///
/// # Example
///
/// ```rust
/// use logisim_core::instance::InstanceData;
///
/// #[derive(Debug, Clone)]
/// struct CounterData {
///     value: u32,
///     last_clock: bool,
/// }
///
/// impl InstanceData for CounterData {
///     fn clone_data(&self) -> Box<dyn InstanceData> {
///         Box::new(self.clone())
///     }
///
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
///
///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
///         self
///     }
/// }
///
/// impl CounterData {
///     pub fn new() -> Self {
///         Self {
///             value: 0,
///             last_clock: false,
///         }
///     }
///
///     pub fn increment(&mut self) {
///         self.value = self.value.wrapping_add(1);
///     }
///
///     pub fn get_value(&self) -> u32 {
///         self.value
///     }
/// }
/// ```
pub trait InstanceData: Debug + Send + Sync {
    /// Creates a deep copy of this instance data.
    ///
    /// This is equivalent to Java's `clone()` method and is used when
    /// copying circuit states or creating simulation snapshots.
    fn clone_data(&self) -> Box<dyn InstanceData>;

    /// Returns a reference to this data as `Any` for downcasting.
    ///
    /// This allows components to retrieve their specific data type
    /// from the generic `InstanceData` trait object.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to this data as `Any` for downcasting.
    ///
    /// This allows components to modify their specific data type
    /// through the generic `InstanceData` trait object.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Type alias for boxed instance data, commonly used in collections and storage.
pub type InstanceDataBox = Box<dyn InstanceData>;

/// Helper function to downcast instance data to a specific type.
///
/// # Arguments
///
/// * `data` - The instance data to downcast
///
/// # Returns
///
/// An optional reference to the data as the requested type, or `None` if
/// the downcast fails.
///
/// # Example
///
/// ```rust
/// use logisim_core::instance::{InstanceData, downcast_instance_data};
///
/// # #[derive(Debug, Clone)]
/// # struct CounterData { value: u32 }
/// # impl InstanceData for CounterData {
/// #     fn clone_data(&self) -> Box<dyn InstanceData> { Box::new(self.clone()) }
/// #     fn as_any(&self) -> &dyn std::any::Any { self }
/// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
/// # }
///
/// let data: Box<dyn InstanceData> = Box::new(CounterData { value: 42 });
/// if let Some(counter_data) = downcast_instance_data::<CounterData>(&*data) {
///     println!("Counter value: {}", counter_data.value);
/// }
/// ```
pub fn downcast_instance_data<T: 'static>(data: &dyn InstanceData) -> Option<&T> {
    data.as_any().downcast_ref::<T>()
}

/// Helper function to mutably downcast instance data to a specific type.
///
/// # Arguments
///
/// * `data` - The instance data to downcast
///
/// # Returns
///
/// An optional mutable reference to the data as the requested type, or `None` if
/// the downcast fails.
///
/// # Example
///
/// ```rust
/// use logisim_core::instance::{InstanceData, downcast_instance_data_mut};
///
/// # #[derive(Debug, Clone)]
/// # struct CounterData { value: u32 }
/// # impl InstanceData for CounterData {
/// #     fn clone_data(&self) -> Box<dyn InstanceData> { Box::new(self.clone()) }
/// #     fn as_any(&self) -> &dyn std::any::Any { self }
/// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
/// # }
///
/// let mut data: Box<dyn InstanceData> = Box::new(CounterData { value: 42 });
/// if let Some(counter_data) = downcast_instance_data_mut::<CounterData>(&mut *data) {
///     counter_data.value += 1;
/// }
/// ```
pub fn downcast_instance_data_mut<T: 'static>(data: &mut dyn InstanceData) -> Option<&mut T> {
    data.as_any_mut().downcast_mut::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestData {
        value: i32,
        name: String,
    }

    impl InstanceData for TestData {
        fn clone_data(&self) -> Box<dyn InstanceData> {
            Box::new(self.clone())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_instance_data_clone() {
        let original = TestData {
            value: 42,
            name: "test".to_string(),
        };

        let cloned = original.clone_data();
        let cloned_test = downcast_instance_data::<TestData>(&*cloned).unwrap();

        assert_eq!(&original, cloned_test);
    }

    #[test]
    fn test_downcast_success() {
        let data: Box<dyn InstanceData> = Box::new(TestData {
            value: 100,
            name: "example".to_string(),
        });

        let test_data = downcast_instance_data::<TestData>(&*data);
        assert!(test_data.is_some());
        assert_eq!(test_data.unwrap().value, 100);
        assert_eq!(test_data.unwrap().name, "example");
    }

    #[test]
    fn test_downcast_failure() {
        let data: Box<dyn InstanceData> = Box::new(TestData {
            value: 100,
            name: "example".to_string(),
        });

        // Try to downcast to a different type
        #[derive(Debug, Clone)]
        struct OtherData {
            other: f64,
        }

        impl InstanceData for OtherData {
            fn clone_data(&self) -> Box<dyn InstanceData> {
                Box::new(self.clone())
            }
            fn as_any(&self) -> &dyn Any { self }
            fn as_any_mut(&mut self) -> &mut dyn Any { self }
        }

        let other_data = downcast_instance_data::<OtherData>(&*data);
        assert!(other_data.is_none());
    }

    #[test]
    fn test_downcast_mut() {
        let mut data: Box<dyn InstanceData> = Box::new(TestData {
            value: 50,
            name: "mutable".to_string(),
        });

        {
            let test_data = downcast_instance_data_mut::<TestData>(&mut *data).unwrap();
            test_data.value = 75;
            test_data.name = "modified".to_string();
        }

        let test_data = downcast_instance_data::<TestData>(&*data).unwrap();
        assert_eq!(test_data.value, 75);
        assert_eq!(test_data.name, "modified");
    }
}