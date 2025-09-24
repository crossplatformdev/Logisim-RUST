/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Component Implementation
//!
//! This module provides the `InstanceComponent` struct which implements the Component
//! trait with instance-specific behavior. This is equivalent to Java's `InstanceComponent` class.

// Component trait will be integrated later
// use crate::component::Component;
use crate::data::{AttributeSet, Bounds, Location};
use crate::instance::{Instance, InstanceFactory, Port};
use std::fmt;
use std::sync::{Arc, Weak};

/// Component implementation that supports the instance system.
///
/// This struct is equivalent to Java's `InstanceComponent` class and provides
/// a concrete implementation of the Component trait that integrates with the
/// instance factory system.
///
/// # Design
///
/// InstanceComponent serves as the bridge between:
/// - The generic Component trait (simulation interface)
/// - The Instance wrapper (metadata and lifecycle)
/// - The InstanceFactory (creation and configuration)
///
/// It maintains references to its factory and provides access to instance-specific
/// operations like port management, attribute handling, and rendering integration.
#[derive(Debug)]
pub struct InstanceComponent {
    /// Location of this component in the circuit
    location: Location,
    /// Component attributes (configuration parameters)
    attributes: AttributeSet,
    /// Cached bounding box for this component
    bounds: Bounds,
    /// Weak reference to the instance wrapper (to avoid cycles)
    instance: Weak<Instance>,
    /// Reference to the factory that created this component
    factory: Option<Arc<dyn InstanceFactory>>,
}

impl InstanceComponent {
    /// Creates a new instance component.
    ///
    /// # Arguments
    ///
    /// * `location` - Position of the component
    /// * `attributes` - Initial attribute values
    ///
    /// # Returns
    ///
    /// A new InstanceComponent.
    pub fn new(location: Location, attributes: AttributeSet) -> Self {
        Self {
            location,
            attributes,
            bounds: Bounds::new(0, 0, 0, 0), // Will be computed when factory is set
            instance: Weak::new(),
            factory: None,
        }
    }

    /// Sets the factory for this component and computes initial bounds.
    ///
    /// # Arguments
    ///
    /// * `factory` - The factory that created this component
    pub fn set_factory(&mut self, factory: Arc<dyn InstanceFactory>) {
        self.bounds = factory
            .get_offset_bounds(&self.attributes)
            .translate(self.location.x(), self.location.y());
        self.factory = Some(factory);
    }

    /// Sets the instance wrapper for this component.
    ///
    /// # Arguments
    ///
    /// * `instance` - Weak reference to the instance wrapper
    pub fn set_instance(&mut self, instance: Weak<Instance>) {
        self.instance = instance;
    }

    /// Returns the location of this component.
    pub fn location(&self) -> Location {
        self.location
    }

    /// Returns the bounding box of this component.
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    /// Returns the attribute set for this component.
    pub fn attribute_set(&self) -> &AttributeSet {
        &self.attributes
    }

    /// Returns a mutable reference to the attribute set.
    pub fn attribute_set_mut(&mut self) -> &mut AttributeSet {
        &mut self.attributes
    }

    /// Returns the factory that created this component.
    pub fn factory(&self) -> Option<&dyn InstanceFactory> {
        self.factory.as_deref()
    }

    /// Returns the instance wrapper if it still exists.
    pub fn instance(&self) -> Option<Arc<Instance>> {
        self.instance.upgrade()
    }

    /// Returns the ports for this component.
    pub fn ports(&self) -> &[Port] {
        self.factory
            .as_ref()
            .map(|f| f.get_ports())
            .unwrap_or(&[])
    }

    /// Gets a port by its index.
    pub fn get_port(&self, index: usize) -> Option<&Port> {
        self.ports().get(index)
    }

    /// Returns the number of ports on this component.
    pub fn port_count(&self) -> usize {
        self.ports().len()
    }

    /// Recomputes the bounding box based on current attributes.
    pub fn recompute_bounds(&mut self) {
        if let Some(factory) = &self.factory {
            self.bounds = factory
                .get_offset_bounds(&self.attributes)
                .translate(self.location.x(), self.location.y());
        }
    }

    /// Signals that this component has been invalidated.
    ///
    /// This notifies the simulation engine that the component needs to be
    /// re-evaluated in the next simulation cycle.
    pub fn fire_invalidated(&self) {
        // In a full implementation, this would notify listeners
        // and mark the component for re-propagation
    }

    /// Updates the component's location and recomputes bounds.
    ///
    /// # Arguments
    ///
    /// * `new_location` - The new position for this component
    pub fn set_location(&mut self, new_location: Location) {
        let old_location = self.location;
        self.location = new_location;
        
        // Translate bounds by the difference
        let dx = new_location.x() - old_location.x();
        let dy = new_location.y() - old_location.y();
        self.bounds = self.bounds.translate(dx, dy);
    }

    /// Checks if a point is contained within this component.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to test
    ///
    /// # Returns
    ///
    /// True if the point is inside the component bounds.
    pub fn contains(&self, point: Location) -> bool {
        self.bounds.contains(point)
    }

    /// Mock constructor for testing (when factory dependencies aren't available).
    #[cfg(test)]
    pub fn mock_new(location: Location, attributes: AttributeSet) -> Self {
        Self {
            location,
            attributes,
            bounds: Bounds::new(location.x() - 10, location.y() - 10, 20, 20),
            instance: Weak::new(),
            factory: None,
        }
    }
}

// Component trait will be integrated later - stub implementation for now

impl fmt::Display for InstanceComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InstanceComponent(name={}, location={}, bounds={})",
            self.get_name(),
            self.location,
            self.bounds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::BitWidth;
    use crate::instance::{PortType, PortWidth};

    #[derive(Debug)]
    struct MockFactory {
        name: String,
        ports: Vec<Port>,
    }

    impl MockFactory {
        fn new() -> Self {
            Self {
                name: "Mock".to_string(),
                ports: vec![
                    Port::new(-10, 0, PortType::Input, PortWidth::fixed_bits(1)),
                    Port::new(10, 0, PortType::Output, PortWidth::fixed_bits(1)),
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
            Bounds::new(-15, -10, 30, 20)
        }

        fn create_component(&self, location: Location, attrs: AttributeSet) -> Box<dyn std::any::Any> {
            let mut component = InstanceComponent::new(location, attrs);
            component.set_factory(std::sync::Arc::new(MockFactory::new()));
            Box::new(component)
        }

        fn paint_instance(&self, _painter: &mut crate::instance::InstancePainter) {
            // Mock implementation
        }

        fn propagate(&self, _state: &mut dyn crate::instance::InstanceState) {
            // Mock implementation
        }
    }

    #[test]
    fn test_instance_component_creation() {
        let location = Location::new(100, 200);
        let attrs = AttributeSet::new();
        let component = InstanceComponent::new(location, attrs);

        assert_eq!(component.location(), location);
        assert_eq!(component.port_count(), 0); // No factory set yet
    }

    #[test]
    fn test_instance_component_with_factory() {
        let location = Location::new(50, 75);
        let attrs = AttributeSet::new();
        let mut component = InstanceComponent::new(location, attrs);
        
        let factory = Arc::new(MockFactory::new());
        component.set_factory(factory);

        assert_eq!(component.location(), location);
        assert_eq!(component.port_count(), 2);
        assert_eq!(component.get_name(), "Mock");

        // Check that bounds were computed correctly
        let expected_bounds = Bounds::new(35, 65, 30, 20); // location + offset bounds
        assert_eq!(component.bounds(), expected_bounds);
    }

    #[test]
    fn test_instance_component_bounds_recomputation() {
        let location = Location::new(0, 0);
        let attrs = AttributeSet::new();
        let mut component = InstanceComponent::new(location, attrs);
        
        let factory = Arc::new(MockFactory::new());
        component.set_factory(factory);

        let initial_bounds = component.bounds();
        
        // Move the component
        component.set_location(Location::new(20, 30));
        
        // Bounds should have moved by the same amount
        let expected_bounds = initial_bounds.translate(20, 30);
        assert_eq!(component.bounds(), expected_bounds);
    }

    #[test]
    fn test_instance_component_contains() {
        let location = Location::new(0, 0);
        let attrs = AttributeSet::new();
        let mut component = InstanceComponent::new(location, attrs);
        
        let factory = Arc::new(MockFactory::new());
        component.set_factory(factory);

        // Test points inside and outside the bounds
        assert!(component.contains(Location::new(0, 0))); // Center should be inside
        assert!(!component.contains(Location::new(100, 100))); // Far outside
    }

    #[test]
    fn test_component_trait_implementation() {
        let location = Location::new(10, 20);
        let attrs = AttributeSet::new();
        let mut component = InstanceComponent::new(location, attrs);
        
        let factory = Arc::new(MockFactory::new());
        component.set_factory(factory);

        // Test Component trait methods
        assert_eq!(component.get_name(), "Mock");
        assert_eq!(component.get_location(), location);
        assert_eq!(component.get_bounds(), component.bounds());
        assert_eq!(component.get_attribute_set(), component.attribute_set());
    }

    #[test]
    fn test_port_access() {
        let location = Location::new(0, 0);
        let attrs = AttributeSet::new();
        let mut component = InstanceComponent::new(location, attrs);
        
        let factory = Arc::new(MockFactory::new());
        component.set_factory(factory);

        assert_eq!(component.port_count(), 2);
        
        let port0 = component.get_port(0).unwrap();
        let port1 = component.get_port(1).unwrap();
        
        assert_eq!(port0.port_type(), PortType::Input);
        assert_eq!(port1.port_type(), PortType::Output);
        
        assert!(component.get_port(2).is_none()); // Out of bounds
    }
}