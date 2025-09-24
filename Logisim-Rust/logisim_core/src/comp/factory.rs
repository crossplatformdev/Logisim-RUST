/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component factory abstractions
//!
//! This module provides the factory pattern for creating components,
//! equivalent to Java's `ComponentFactory` and `AbstractComponentFactory` classes.
//! Factories are responsible for creating component instances and managing
//! their properties and behavior.

use super::component::{Component, ComponentId};
use crate::data::{AttributeSet, Bounds, Location};
use std::collections::HashMap;

/// Trait for component factories
/// 
/// This is equivalent to Java's `ComponentFactory` interface and defines
/// how component types are created and managed.
pub trait ComponentFactory: Send + Sync {
    /// Get the name of this component type
    fn name(&self) -> &str;

    /// Get the display name shown to users
    fn display_name(&self) -> &str;

    /// Create a new component instance at the specified location
    fn create_component(&self, id: ComponentId, location: Location, attrs: &AttributeSet) -> Box<dyn Component>;

    /// Create the default attribute set for this component type
    fn create_attribute_set(&self) -> AttributeSet;

    /// Get the default bounds for this component type
    fn get_bounds(&self, attrs: &AttributeSet) -> Bounds;

    /// Get the number of input pins for this component type
    fn input_count(&self, _attrs: &AttributeSet) -> usize {
        0 // Default: no inputs
    }

    /// Get the number of output pins for this component type
    fn output_count(&self, _attrs: &AttributeSet) -> usize {
        0 // Default: no outputs
    }

    /// Check if this component type supports the given attribute
    fn supports_attribute(&self, _attr_name: &str) -> bool {
        false // Default: no attributes supported
    }

    /// Get the default value for an attribute
    fn get_default_attribute_value(&self, _attr_name: &str) -> Option<String> {
        None // Default: no default values
    }

    /// Check if this component requires a label
    fn requires_label(&self) -> bool {
        false // Default: no label required
    }

    /// Check if this component requires a global clock
    fn requires_global_clock(&self) -> bool {
        false // Default: no global clock required
    }

    /// Get the category this component belongs to
    fn category(&self) -> &str {
        "Basic" // Default category
    }

    /// Get a description of this component
    fn description(&self) -> &str {
        "A digital logic component" // Default description
    }

    /// Check if this component can be placed at the given location
    fn can_place_at(&self, _location: Location, _attrs: &AttributeSet) -> bool {
        true // Default: can place anywhere
    }
}

/// Abstract base implementation for component factories
/// 
/// This provides common functionality for component factories,
/// equivalent to Java's `AbstractComponentFactory` class.
pub struct AbstractComponentFactory {
    name: String,
    display_name: String,
    category: String,
    description: String,
    requires_label: bool,
    requires_global_clock: bool,
    supported_attributes: Vec<String>,
    default_attribute_values: HashMap<String, String>,
}

impl AbstractComponentFactory {
    /// Create a new abstract component factory
    pub fn new(name: String, display_name: String) -> Self {
        AbstractComponentFactory {
            name,
            display_name: display_name.clone(),
            category: "Basic".to_string(),
            description: format!("A {} component", display_name),
            requires_label: false,
            requires_global_clock: false,
            supported_attributes: Vec::new(),
            default_attribute_values: HashMap::new(),
        }
    }

    /// Set the category for this component factory
    pub fn with_category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    /// Set the description for this component factory
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Set whether this component requires a label
    pub fn with_label_requirement(mut self, requires_label: bool) -> Self {
        self.requires_label = requires_label;
        self
    }

    /// Set whether this component requires a global clock
    pub fn with_clock_requirement(mut self, requires_global_clock: bool) -> Self {
        self.requires_global_clock = requires_global_clock;
        self
    }

    /// Add a supported attribute
    pub fn with_attribute(mut self, attr_name: String, default_value: Option<String>) -> Self {
        self.supported_attributes.push(attr_name.clone());
        if let Some(value) = default_value {
            self.default_attribute_values.insert(attr_name, value);
        }
        self
    }

    /// Get the supported attributes
    pub fn supported_attributes(&self) -> &[String] {
        &self.supported_attributes
    }

    /// Get default attribute values
    pub fn default_attribute_values(&self) -> &HashMap<String, String> {
        &self.default_attribute_values
    }
}

impl ComponentFactory for AbstractComponentFactory {
    fn name(&self) -> &str {
        &self.name
    }

    fn display_name(&self) -> &str {
        &self.display_name
    }

    fn create_component(&self, _id: ComponentId, _location: Location, _attrs: &AttributeSet) -> Box<dyn Component> {
        panic!("create_component must be implemented by concrete factory")
    }

    fn create_attribute_set(&self) -> AttributeSet {
        let attrs = AttributeSet::new();
        for attr_name in &self.supported_attributes {
            if let Some(_default_value) = self.default_attribute_values.get(attr_name) {
                // For now, just store as string - in a full implementation,
                // we'd need proper attribute type handling
                // attrs.set_attribute(attr_name.clone(), default_value.clone());
            }
        }
        attrs
    }

    fn get_bounds(&self, _attrs: &AttributeSet) -> Bounds {
        // Default bounds - 40x30 pixels
        Bounds::create(0, 0, 40, 30)
    }

    fn supports_attribute(&self, attr_name: &str) -> bool {
        self.supported_attributes.iter().any(|a| a == attr_name)
    }

    fn get_default_attribute_value(&self, attr_name: &str) -> Option<String> {
        self.default_attribute_values.get(attr_name).cloned()
    }

    fn requires_label(&self) -> bool {
        self.requires_label
    }

    fn requires_global_clock(&self) -> bool {
        self.requires_global_clock
    }

    fn category(&self) -> &str {
        &self.category
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Factory registry for managing component factories
#[derive(Default)]
pub struct ComponentFactoryRegistry {
    factories: HashMap<String, Box<dyn ComponentFactory>>,
    categories: HashMap<String, Vec<String>>,
}

impl ComponentFactoryRegistry {
    /// Create a new factory registry
    pub fn new() -> Self {
        ComponentFactoryRegistry {
            factories: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    /// Register a component factory
    pub fn register(&mut self, factory: Box<dyn ComponentFactory>) {
        let name = factory.name().to_string();
        let category = factory.category().to_string();
        
        // Add to category index
        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(name.clone());
        
        // Register the factory
        self.factories.insert(name, factory);
    }

    /// Get a factory by name
    pub fn get_factory(&self, name: &str) -> Option<&dyn ComponentFactory> {
        self.factories.get(name).map(|f| f.as_ref())
    }

    /// Get all factory names
    pub fn get_factory_names(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }

    /// Get all categories
    pub fn get_categories(&self) -> Vec<&str> {
        self.categories.keys().map(|s| s.as_str()).collect()
    }

    /// Get factories in a category
    pub fn get_factories_in_category(&self, category: &str) -> Vec<&dyn ComponentFactory> {
        if let Some(names) = self.categories.get(category) {
            names
                .iter()
                .filter_map(|name| self.get_factory(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Create a component using a registered factory
    pub fn create_component(
        &self,
        factory_name: &str,
        id: ComponentId,
        location: Location,
        attrs: &AttributeSet,
    ) -> Option<Box<dyn Component>> {
        self.get_factory(factory_name)
            .map(|factory| factory.create_component(id, location, attrs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{AttributeSet, Location};

    // Mock component for testing
    #[derive(Debug)]
    struct MockComponent {
        id: ComponentId,
        pins: HashMap<String, crate::comp::pin::Pin>,
    }

    impl Component for MockComponent {
        fn id(&self) -> ComponentId {
            self.id
        }

        fn name(&self) -> &str {
            "Mock"
        }

        fn pins(&self) -> &HashMap<String, crate::comp::pin::Pin> {
            &self.pins
        }

        fn pins_mut(&mut self) -> &mut HashMap<String, crate::comp::pin::Pin> {
            &mut self.pins
        }

        fn update(&mut self, _current_time: crate::signal::Timestamp) -> crate::comp::component::UpdateResult {
            crate::comp::component::UpdateResult::new()
        }

        fn reset(&mut self) {}
    }

    // Mock factory for testing
    struct MockFactory {
        base: AbstractComponentFactory,
    }

    impl MockFactory {
        fn new() -> Self {
            MockFactory {
                base: AbstractComponentFactory::new("mock".to_string(), "Mock Component".to_string())
                    .with_category("Test".to_string()),
            }
        }
    }

    impl ComponentFactory for MockFactory {
        fn name(&self) -> &str {
            self.base.name()
        }

        fn display_name(&self) -> &str {
            self.base.display_name()
        }

        fn create_component(&self, id: ComponentId, _location: Location, _attrs: &AttributeSet) -> Box<dyn Component> {
            Box::new(MockComponent { 
                id,
                pins: HashMap::new(), 
            })
        }

        fn create_attribute_set(&self) -> AttributeSet {
            self.base.create_attribute_set()
        }

        fn get_bounds(&self, attrs: &AttributeSet) -> Bounds {
            self.base.get_bounds(attrs)
        }

        fn category(&self) -> &str {
            self.base.category()
        }

        fn description(&self) -> &str {
            self.base.description()
        }
    }

    #[test]
    fn test_abstract_factory() {
        let factory = AbstractComponentFactory::new("test".to_string(), "Test Component".to_string())
            .with_category("Testing".to_string())
            .with_description("A test component".to_string())
            .with_label_requirement(true)
            .with_attribute("size".to_string(), Some("10".to_string()));

        assert_eq!(factory.name(), "test");
        assert_eq!(factory.display_name(), "Test Component");
        assert_eq!(factory.category(), "Testing");
        assert_eq!(factory.description(), "A test component");
        assert!(factory.requires_label());
        assert!(!factory.requires_global_clock());
        assert!(factory.supports_attribute("size"));
        assert_eq!(factory.get_default_attribute_value("size"), Some("10".to_string()));
    }

    #[test]
    fn test_factory_registry() {
        let mut registry = ComponentFactoryRegistry::new();
        let factory = Box::new(MockFactory::new());

        registry.register(factory);

        assert!(registry.get_factory("mock").is_some());
        assert_eq!(registry.get_factory_names(), vec!["mock"]);
        assert_eq!(registry.get_categories(), vec!["Test"]);

        let factories = registry.get_factories_in_category("Test");
        assert_eq!(factories.len(), 1);
        assert_eq!(factories[0].name(), "mock");

        // Test component creation
        let attrs = AttributeSet::new();
        let component = registry.create_component(
            "mock",
            ComponentId::new(42),
            Location::new(10, 20),
            &attrs,
        );
        assert!(component.is_some());
        assert_eq!(component.unwrap().name(), "Mock");
    }
}