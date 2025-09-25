/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component Instance Wrapper
//!
//! This module provides the `Instance` struct which wraps components with
//! metadata and lifecycle management. This is equivalent to Java's `Instance` class.

use crate::data::{Attribute, AttributeSet, Bounds, Location};
use crate::instance::{InstanceComponent, InstanceData, InstanceFactory, Port};
use std::fmt;
use std::sync::{Arc, Weak};

/// Unique identifier for component instances.
pub type InstanceId = u64;

/// A wrapper around components providing metadata and lifecycle management.
///
/// This struct is equivalent to Java's `Instance` class and provides a facade
/// for accessing component properties, state, and operations in a consistent way.
///
/// # Design
///
/// The Instance acts as a bridge between the generic Component trait and the
/// specific InstanceComponent implementation, providing:
/// - Type-safe access to component properties
/// - Consistent interface for attribute management
/// - Integration with the instance factory system
/// - Support for component lifecycle operations
#[derive(Debug, Clone)]
pub struct Instance {
    /// Unique identifier for this instance
    id: InstanceId,
    /// Reference to the underlying component
    component: Arc<InstanceComponent>,
    /// Factory that created this instance
    factory: Arc<dyn InstanceFactory>,
}

/// Weak reference to an instance, used to avoid circular references.
pub type InstanceRef = Weak<Instance>;

impl Instance {
    /// Creates a new instance wrapper around a component.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this instance
    /// * `component` - The underlying component implementation
    /// * `factory` - Factory that created this component
    ///
    /// # Returns
    ///
    /// A new Instance wrapper.
    pub fn new(
        id: InstanceId,
        component: Arc<InstanceComponent>,
        factory: Arc<dyn InstanceFactory>,
    ) -> Self {
        Self {
            id,
            component,
            factory,
        }
    }

    /// Returns the unique identifier for this instance.
    pub fn id(&self) -> InstanceId {
        self.id
    }

    /// Returns a reference to the underlying component.
    pub fn component(&self) -> &InstanceComponent {
        &self.component
    }

    /// Returns the factory that created this instance.
    pub fn factory(&self) -> &dyn InstanceFactory {
        &*self.factory
    }

    /// Returns the location of this component instance.
    pub fn location(&self) -> Location {
        self.component.location()
    }

    /// Returns the bounding box of this component instance.
    pub fn bounds(&self) -> Bounds {
        self.component.bounds()
    }

    /// Returns the attribute set for this component instance.
    pub fn attribute_set(&self) -> &AttributeSet {
        self.component.attribute_set()
    }

    /// Gets the value of a specific attribute.
    ///
    /// # Arguments
    ///
    /// * `attr` - The attribute to retrieve
    ///
    /// # Returns
    ///
    /// The attribute value, or None if not set.
    pub fn get_attribute_value<T>(&self, attr: &Attribute<T>) -> Option<&T>
    where
        T: Clone + PartialEq + crate::data::AttributeValue + 'static,
    {
        self.component.attribute_set().get_value(attr)
    }

    /// Returns the ports for this component instance.
    pub fn ports(&self) -> &[Port] {
        self.factory.get_ports()
    }

    /// Gets a port by its index.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based port index
    ///
    /// # Returns
    ///
    /// Reference to the port, or None if invalid index.
    pub fn get_port(&self, index: usize) -> Option<&Port> {
        self.ports().get(index)
    }

    /// Returns the location of a specific port.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Zero-based index of the port
    ///
    /// # Returns
    ///
    /// The absolute location of the port, or None if invalid index.
    pub fn get_port_location(&self, port_index: usize) -> Option<Location> {
        self.get_port(port_index)
            .map(|port| port.location(self.location()))
    }

    /// Triggers recomputation of the component's bounds.
    ///
    /// This should be called when component attributes change in a way
    /// that affects the component's size or positioning.
    pub fn recompute_bounds(&mut self) {
        // In a full implementation, this would update the component's bounds
        // based on current attributes and factory settings
    }

    /// Signals that this instance has been invalidated and needs re-evaluation.
    pub fn fire_invalidated(&self) {
        self.component.fire_invalidated();
    }

    /// Creates a weak reference to this instance.
    pub fn downgrade(instance: &Arc<Instance>) -> InstanceRef {
        Arc::downgrade(instance)
    }

    /// Attempts to upgrade a weak reference to a strong reference.
    pub fn upgrade(weak: &InstanceRef) -> Option<Arc<Instance>> {
        weak.upgrade()
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instance(id={}, factory={}, location={})",
            self.id,
            self.factory.get_name(),
            self.location()
        )
    }
}

/// Helper functions for instance management
impl Instance {
    /// Extracts an Instance from a generic component, if possible.
    ///
    /// # Arguments
    ///
    /// * `component` - A component that might be an InstanceComponent
    ///
    /// # Returns
    ///
    /// The wrapped Instance if the component is an InstanceComponent, None otherwise.
    pub fn from_component(_component: &dyn std::any::Any) -> Option<Arc<Instance>> {
        // In a full implementation, this would attempt to downcast the component
        // to an InstanceComponent and extract its Instance wrapper
        None
    }

    /// Gets the InstanceComponent from an Instance.
    ///
    /// This is the inverse of `from_component()` and provides access to the
    /// underlying component implementation.
    ///
    /// # Arguments
    ///
    /// * `instance` - The instance to extract the component from
    ///
    /// # Returns
    ///
    /// Reference to the underlying InstanceComponent.
    pub fn get_component_for(instance: &Instance) -> &InstanceComponent {
        &instance.component
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::BitWidth;
    use crate::instance::{PortType, PortWidth};
    use std::sync::Arc;

    // Mock implementations for testing
    #[derive(Debug)]
    struct MockFactory {
        name: String,
        ports: Vec<Port>,
    }

    impl MockFactory {
        fn new() -> Self {
            Self {
                name: "Mock Component".to_string(),
                ports: vec![
                    Port::new(0, -10, PortType::Input, PortWidth::fixed_bits(1)),
                    Port::new(0, 10, PortType::Output, PortWidth::fixed_bits(1)),
                ],
            }
        }
    }

    impl InstanceFactory for MockFactory {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn create_attribute_set(&self) -> AttributeSet {
            AttributeSet::new()
        }
        fn get_ports(&self) -> &[Port] {
            &self.ports
        }

        fn get_offset_bounds(&self, _attrs: &AttributeSet) -> Bounds {
            Bounds::new(-10, -20, 20, 40)
        }

        fn create_component(
            &self,
            location: Location,
            attrs: AttributeSet,
        ) -> Box<dyn std::any::Any> {
            Box::new(InstanceComponent::mock_new(location, attrs))
        }

        fn paint_instance(&self, _painter: &mut crate::instance::InstancePainter) {
            // Mock implementation
        }

        fn propagate(&self, _state: &mut dyn crate::instance::InstanceState) {
            // Mock implementation
        }
    }

    #[test]
    fn test_instance_creation() {
        let factory = Arc::new(MockFactory::new());
        let component = Arc::new(InstanceComponent::mock_new(
            Location::new(100, 200),
            AttributeSet::new(),
        ));

        let instance = Instance::new(42, component, factory);

        assert_eq!(instance.id(), 42);
        assert_eq!(instance.factory().get_name(), "Mock Component");
        assert_eq!(instance.location(), Location::new(100, 200));
    }

    #[test]
    fn test_instance_ports() {
        let factory = Arc::new(MockFactory::new());
        let component = Arc::new(InstanceComponent::mock_new(
            Location::new(0, 0),
            AttributeSet::new(),
        ));

        let instance = Instance::new(1, component, factory);

        assert_eq!(instance.ports().len(), 2);
        assert_eq!(instance.ports()[0].port_type(), PortType::Input);
        assert_eq!(instance.ports()[1].port_type(), PortType::Output);

        // Test port access
        assert!(instance.get_port(0).is_some());
        assert!(instance.get_port(1).is_some());
        assert!(instance.get_port(2).is_none());
    }

    #[test]
    fn test_instance_port_locations() {
        let factory = Arc::new(MockFactory::new());
        let component = Arc::new(InstanceComponent::mock_new(
            Location::new(50, 100),
            AttributeSet::new(),
        ));

        let instance = Instance::new(1, component, factory);

        let port0_loc = instance.get_port_location(0).unwrap();
        let port1_loc = instance.get_port_location(1).unwrap();

        assert_eq!(port0_loc, Location::new(50, 90)); // 50 + 0, 100 + (-10)
        assert_eq!(port1_loc, Location::new(50, 110)); // 50 + 0, 100 + 10
    }

    #[test]
    fn test_instance_weak_references() {
        let factory = Arc::new(MockFactory::new());
        let component = Arc::new(InstanceComponent::mock_new(
            Location::new(0, 0),
            AttributeSet::new(),
        ));

        let instance = Arc::new(Instance::new(1, component, factory));
        let weak_ref = Instance::downgrade(&instance);

        // Should be able to upgrade while strong reference exists
        assert!(Instance::upgrade(&weak_ref).is_some());

        drop(instance);

        // Should not be able to upgrade after strong reference is dropped
        assert!(Instance::upgrade(&weak_ref).is_none());
    }

    #[test]
    fn test_instance_equality() {
        let factory = Arc::new(MockFactory::new());
        let component1 = Arc::new(InstanceComponent::mock_new(
            Location::new(0, 0),
            AttributeSet::new(),
        ));
        let component2 = Arc::new(InstanceComponent::mock_new(
            Location::new(10, 10),
            AttributeSet::new(),
        ));

        let instance1 = Instance::new(1, component1.clone(), factory.clone());
        let instance2 = Instance::new(1, component2, factory.clone());
        let instance3 = Instance::new(2, component1, factory);

        // Same ID should be equal
        assert_eq!(instance1, instance2);
        // Different ID should not be equal
        assert_ne!(instance1, instance3);
    }
}
