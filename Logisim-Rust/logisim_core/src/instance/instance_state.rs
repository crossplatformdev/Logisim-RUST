/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance State Management
//!
//! This module provides the `InstanceState` trait for managing component runtime state
//! during simulation. This is equivalent to Java's `InstanceState` interface.

use crate::data::{Attribute, AttributeSet};
use crate::{Value};
use crate::instance::{Instance, InstanceData, InstanceFactory, Port};
use crate::netlist::NetId;
use crate::signal::Timestamp;
use std::fmt::Debug;

/// Runtime state management interface for component instances during simulation.
///
/// This trait provides access to component state, port values, and simulation context
/// during propagation and other runtime operations. It is equivalent to Java's
/// `InstanceState` interface.
///
/// # Example
///
/// ```rust
/// use logisim_core::instance::{InstanceState, InstanceData};
/// use logisim_core::Value;
///
/// #[derive(Debug, Clone)]
/// struct FlipFlopData {
///     output: Value,
///     last_clock: Value,
/// }
///
/// impl InstanceData for FlipFlopData {
///     fn clone_data(&self) -> Box<dyn InstanceData> {
///         Box::new(self.clone())
///     }
///     fn as_any(&self) -> &dyn std::any::Any { self }
///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
/// }
///
/// fn propagate_flip_flop(state: &mut dyn InstanceState) {
///     let clock = state.get_port_value(0);
///     let data_in = state.get_port_value(1);
///     
///     // Get or initialize component data
///     let data = if let Some(existing) = state.get_data() {
///         existing.as_any().downcast_ref::<FlipFlopData>().unwrap()
///     } else {
///         let new_data = FlipFlopData {
///             output: Value::Unknown,
///             last_clock: Value::Low,
///         };
///         state.set_data(Box::new(new_data));
///         state.get_data().unwrap().as_any().downcast_ref().unwrap()
///     };
///     
///     // Detect positive edge
///     if data.last_clock == Value::Low && clock == Value::High {
///         state.set_port_value(2, data_in, 1); // Output after 1 time unit
///     }
/// }
/// ```
pub trait InstanceState: Debug {
    /// Signals that this component instance needs to be re-evaluated.
    ///
    /// This triggers the simulation engine to schedule this component for
    /// re-propagation in the next simulation cycle.
    fn fire_invalidated(&mut self);

    /// Returns the attribute set for this component instance.
    ///
    /// The attribute set contains component configuration parameters that
    /// can be used to customize behavior.
    fn get_attribute_set(&self) -> &AttributeSet;

    /// Gets the value of a specific attribute (type-erased version).
    ///
    /// This method uses type erasure to be compatible with trait objects.
    /// Use the `InstanceStateExt::get_typed_attribute` method for type-safe access.
    fn get_attribute_value_erased(&self, attr: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>>;

    /// Returns the component-specific runtime data.
    ///
    /// This data persists across simulation steps and can be used to store
    /// component state like flip-flop values, counter states, etc.
    fn get_data(&self) -> Option<&dyn InstanceData>;

    /// Returns a mutable reference to the component-specific runtime data.
    fn get_data_mut(&mut self) -> Option<&mut (dyn InstanceData + '_)>;

    /// Returns the factory that created this component instance.
    fn get_factory(&self) -> &dyn InstanceFactory;

    /// Returns the instance wrapper for this component.
    fn get_instance(&self) -> &Instance;

    /// Gets the port index for a given port definition.
    ///
    /// # Arguments
    ///
    /// * `port` - The port to find the index for
    ///
    /// # Returns
    ///
    /// The zero-based index of the port, or None if not found.
    fn get_port_index(&self, port: &Port) -> Option<usize>;

    /// Gets the current value on a port.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the port
    ///
    /// # Returns
    ///
    /// The current signal value on the port, or Unknown if invalid index.
    fn get_port_value(&self, port_index: usize) -> Value;

    /// Gets the network ID connected to a specific port.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the port
    ///
    /// # Returns
    ///
    /// The network ID if the port is connected, None otherwise.
    fn get_port_net(&self, port_index: usize) -> Option<NetId>;

    /// Returns the current simulation tick count.
    ///
    /// This can be used for components that need to track simulation time
    /// or implement time-based behavior.
    fn get_tick_count(&self) -> u64;

    /// Returns the current simulation timestamp.
    fn get_timestamp(&self) -> Timestamp;

    /// Checks if this component is in the root circuit (not a subcircuit).
    fn is_circuit_root(&self) -> bool;

    /// Checks if a port is connected to a network.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the port
    ///
    /// # Returns
    ///
    /// True if the port is connected, false otherwise.
    fn is_port_connected(&self, port_index: usize) -> bool;

    /// Sets the component-specific runtime data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store with this component instance
    fn set_data(&mut self, data: Box<dyn InstanceData>);

    /// Sets the value on an output port with a specified delay.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the output port
    /// * `value` - The value to output
    /// * `delay` - Propagation delay in simulation time units
    fn set_port_value(&mut self, port_index: usize, value: Value, delay: u32);

    /// Sets the value on an output port immediately (zero delay).
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the output port  
    /// * `value` - The value to output
    fn set_port_value_immediate(&mut self, port_index: usize, value: Value) {
        self.set_port_value(port_index, value, 0);
    }

    /// Schedules a future re-evaluation of this component.
    ///
    /// # Arguments
    ///
    /// * `delay` - Time units in the future to schedule re-evaluation
    fn schedule_evaluation(&mut self, delay: u32);

    /// Gets a port by its index.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based port index
    ///
    /// # Returns
    ///
    /// Reference to the port, or None if invalid index.
    fn get_port(&self, index: usize) -> Option<&Port>;

    /// Returns the total number of ports on this component.
    fn get_port_count(&self) -> usize;

    /// Checks if a port is an input port.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the port
    ///
    /// # Returns
    ///
    /// True if the port is an input, false otherwise.
    fn is_input_port(&self, port_index: usize) -> bool;

    /// Checks if a port is an output port.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the port
    ///
    /// # Returns
    ///
    /// True if the port is an output, false otherwise.
    fn is_output_port(&self, port_index: usize) -> bool;
}

/// Helper trait for components to easily access typed instance data.
pub trait InstanceStateExt: InstanceState {
    /// Gets the value of a specific attribute with type safety.
    ///
    /// # Arguments
    ///
    /// * `attr` - The attribute to retrieve
    ///
    /// # Returns
    ///
    /// The attribute value, or None if the attribute is not set.
    fn get_attribute_value<T>(&self, attr: &Attribute<T>) -> Option<&T>
    where
        T: Clone + PartialEq + crate::data::AttributeValue + 'static,
    {
        // This implementation would use the type-erased method
        // For now, return None as a placeholder
        let _ = attr;
        None
    }

    /// Gets typed component data, initializing it if not present.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of data to retrieve
    /// * `F` - The initialization function type
    ///
    /// # Arguments
    ///
    /// * `init_fn` - Function to create initial data if none exists
    ///
    /// # Returns
    ///
    /// Reference to the typed data.
    fn get_or_init_data<T, F>(&mut self, init_fn: F) -> &T
    where
        T: InstanceData + 'static,
        F: FnOnce() -> T;

    /// Gets a mutable reference to typed component data.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of data to retrieve
    ///
    /// # Returns
    ///
    /// Mutable reference to the typed data, or None if not present or wrong type.
    fn get_typed_data_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static;
}

impl<S: InstanceState + ?Sized> InstanceStateExt for S {
    fn get_or_init_data<T, F>(&mut self, init_fn: F) -> &T
    where
        T: InstanceData + 'static,
        F: FnOnce() -> T,
    {
        if self.get_data().is_none() {
            let data = init_fn();
            self.set_data(Box::new(data));
        }

        self.get_data()
            .unwrap()
            .as_any()
            .downcast_ref::<T>()
            .expect("Data type mismatch")
    }

    fn get_typed_data_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static,
    {
        self.get_data_mut()?
            .as_any_mut()
            .downcast_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::BitWidth;
    use crate::instance::{PortType, PortWidth};
    use std::collections::HashMap;

    // Mock implementation for testing
    #[derive(Debug)]
    struct MockInstanceState {
        data: Option<Box<dyn InstanceData>>,
        port_values: HashMap<usize, Value>,
        attributes: AttributeSet,
        tick_count: u64,
    }

    impl MockInstanceState {
        fn new() -> Self {
            Self {
                data: None,
                port_values: HashMap::new(),
                attributes: AttributeSet::new(),
                tick_count: 0,
            }
        }
    }

    impl InstanceState for MockInstanceState {
        fn fire_invalidated(&mut self) {
            // Mock implementation
        }

        fn get_attribute_set(&self) -> &AttributeSet {
            &self.attributes
        }

        fn get_attribute_value_erased(&self, attr: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
            // Mock implementation
            let _ = attr;
            None
        }

        fn get_data(&self) -> Option<&dyn InstanceData> {
            self.data.as_deref()
        }

        fn get_data_mut(&mut self) -> Option<&mut (dyn InstanceData + '_)> {
            self.data.as_deref_mut()
        }

        fn get_factory(&self) -> &dyn InstanceFactory {
            panic!("Mock implementation")
        }

        fn get_instance(&self) -> &Instance {
            panic!("Mock implementation")
        }

        fn get_port_index(&self, _port: &Port) -> Option<usize> {
            Some(0) // Mock implementation
        }

        fn get_port_value(&self, port_index: usize) -> Value {
            self.port_values.get(&port_index).copied().unwrap_or(Value::Unknown)
        }

        fn get_port_net(&self, _port_index: usize) -> Option<NetId> {
            None // Mock implementation
        }

        fn get_tick_count(&self) -> u64 {
            self.tick_count
        }

        fn get_timestamp(&self) -> Timestamp {
            Timestamp::new(self.tick_count)
        }

        fn is_circuit_root(&self) -> bool {
            true // Mock implementation
        }

        fn is_port_connected(&self, _port_index: usize) -> bool {
            true // Mock implementation
        }

        fn set_data(&mut self, data: Box<dyn InstanceData>) {
            self.data = Some(data);
        }

        fn set_port_value(&mut self, port_index: usize, value: Value, _delay: u32) {
            self.port_values.insert(port_index, value);
        }

        fn schedule_evaluation(&mut self, _delay: u32) {
            // Mock implementation
        }

        fn get_port(&self, _index: usize) -> Option<&Port> {
            None // Mock implementation
        }

        fn get_port_count(&self) -> usize {
            2 // Mock implementation
        }

        fn is_input_port(&self, port_index: usize) -> bool {
            port_index == 0 // Mock: port 0 is input
        }

        fn is_output_port(&self, port_index: usize) -> bool {
            port_index == 1 // Mock: port 1 is output
        }
    }

    #[derive(Debug, Clone)]
    struct TestData {
        value: i32,
    }

    impl InstanceData for TestData {
        fn clone_data(&self) -> Box<dyn InstanceData> {
            Box::new(self.clone())
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_instance_state_data_operations() {
        let mut state = MockInstanceState::new();

        // Initially no data
        assert!(state.get_data().is_none());

        // Set data
        let test_data = TestData { value: 42 };
        state.set_data(Box::new(test_data));

        // Retrieve data
        assert!(state.get_data().is_some());
        let data = state.get_data().unwrap().as_any().downcast_ref::<TestData>().unwrap();
        assert_eq!(data.value, 42);
    }

    #[test]
    fn test_instance_state_port_operations() {
        let mut state = MockInstanceState::new();

        // Initially unknown values  
        assert_eq!(state.get_port_value(0), Value::Unknown);
        assert_eq!(state.get_port_value(1), Value::Unknown);

        // Set port values
        state.set_port_value(0, Value::High, 0);
        state.set_port_value(1, Value::Low, 5);

        // Check values
        assert_eq!(state.get_port_value(0), Value::High);
        assert_eq!(state.get_port_value(1), Value::Low);

        // Test immediate set
        state.set_port_value_immediate(0, Value::Unknown);
        assert_eq!(state.get_port_value(0), Value::Unknown);
    }

    #[test]
    fn test_instance_state_ext() {
        let mut state = MockInstanceState::new();

        // Get or initialize data
        let data = state.get_or_init_data(|| TestData { value: 100 });
        assert_eq!(data.value, 100);

        // Data should persist
        let data2 = state.get_or_init_data(|| TestData { value: 200 });
        assert_eq!(data2.value, 100); // Original value, not reinitialized

        // Get mutable reference
        {
            let data_mut = state.get_typed_data_mut::<TestData>().unwrap();
            data_mut.value = 150;
        }

        let data3 = state.get_or_init_data(|| TestData { value: 300 });
        assert_eq!(data3.value, 150); // Modified value
    }
}