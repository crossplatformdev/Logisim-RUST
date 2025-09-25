/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Singleton Instance Data Helper
//!
//! This module provides the `InstanceDataSingleton` struct for components
//! that store a single value as their instance data.

use crate::instance::InstanceData;
use std::any::Any;
use std::fmt;

/// A simple wrapper for storing a single value as instance data.
///
/// This is equivalent to Java's `InstanceDataSingleton` class.
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceDataSingleton<T> {
    value: T,
}

impl<T> InstanceDataSingleton<T> {
    /// Creates a new singleton data wrapper.
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Gets a mutable reference to the wrapped value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Sets the wrapped value.
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Consumes the wrapper and returns the value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> InstanceData for InstanceDataSingleton<T>
where
    T: Clone + fmt::Debug + Send + Sync + 'static,
{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_creation_and_access() {
        let data = InstanceDataSingleton::new(42i32);
        assert_eq!(*data.get(), 42);
    }

    #[test]
    fn test_singleton_mutation() {
        let mut data = InstanceDataSingleton::new(10u32);
        assert_eq!(*data.get(), 10);

        data.set(20);
        assert_eq!(*data.get(), 20);

        *data.get_mut() = 30;
        assert_eq!(*data.get(), 30);
    }

    #[test]
    fn test_instance_data_trait() {
        let data: Box<dyn InstanceData> = Box::new(InstanceDataSingleton::new("hello"));
        let cloned = data.clone_data();

        let original_ref = data
            .as_any()
            .downcast_ref::<InstanceDataSingleton<&str>>()
            .unwrap();
        let cloned_ref = cloned
            .as_any()
            .downcast_ref::<InstanceDataSingleton<&str>>()
            .unwrap();

        assert_eq!(original_ref.get(), cloned_ref.get());
    }
}
