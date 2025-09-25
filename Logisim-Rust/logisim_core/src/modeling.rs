//! Advanced modeling features and extensibility hooks
//!
//! This module provides advanced modeling capabilities including observer patterns,
//! dynamic component registration, and extensibility hooks for plugin development.
//! 
//! # Warning: Unstable API
//! 
//! ⚠️ **APIs in this module are experimental and subject to change** ⚠️
//! 
//! These interfaces are provided for early adoption and feedback, but may be
//! modified, renamed, or removed in future versions. Use with caution in
//! production code.

use crate::{Component, ComponentId};
use crate::netlist::NodeId;
use crate::signal::{Signal, Timestamp};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use thiserror::Error;

/// Errors that can occur in the modeling system
#[derive(Error, Debug)]
pub enum ModelingError {
    #[error("Observer registration failed: {0}")]
    ObserverRegistrationFailed(String),
    #[error("Extension point not found: {0}")]
    ExtensionPointNotFound(String),
    #[error("Component registration failed: {0}")]
    ComponentRegistrationFailed(String),
    #[error("Modeling feature not implemented: {0}")]
    NotImplemented(String),
}

/// Result type for modeling operations
pub type ModelingResult<T> = Result<T, ModelingError>;

/// Events that can be observed in the simulation
#[derive(Debug, Clone)]  
pub enum SimulationEvent {
    /// Signal changed on a node
    SignalChanged {
        node_id: NodeId,
        old_signal: Signal,
        new_signal: Signal,
        timestamp: Timestamp,
        source: ComponentId,
    },
    /// Component state changed
    ComponentStateChanged {
        component_id: ComponentId,
        event_type: String,
        data: Arc<dyn Any + Send + Sync>,
    },
    /// Simulation step completed
    StepCompleted {
        timestamp: Timestamp,
        events_processed: usize,
    },
    /// Simulation reset
    SimulationReset {
        timestamp: Timestamp,
    },
    /// Clock edge detected
    ClockEdge {
        node_id: NodeId,
        edge_type: ClockEdgeType,
        timestamp: Timestamp,
    },
}

/// Types of clock edges that can be detected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockEdgeType {
    Rising,
    Falling,
}

/// Observer trait for simulation events
/// 
/// # Stability: Unstable API
/// This trait may change in future versions as the observer system evolves.
pub trait SimulationObserver: Send + Sync {
    /// Called when a simulation event occurs
    fn on_event(&mut self, event: &SimulationEvent);
    
    /// Get the name of this observer (for debugging)
    fn name(&self) -> &str {
        "Unknown Observer"
    }
    
    /// Check if this observer is interested in a specific event type
    fn is_interested_in(&self, event: &SimulationEvent) -> bool {
        // By default, interested in all events
        let _ = event;
        true
    }
}

/// Manages observers for simulation events
pub struct ObserverManager {
    observers: Vec<Box<dyn SimulationObserver>>,
    weak_observers: Vec<Weak<Mutex<dyn SimulationObserver>>>,
}

impl ObserverManager {
    /// Create a new observer manager
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            weak_observers: Vec::new(),
        }
    }
    
    /// Add an observer (takes ownership)
    pub fn add_observer(&mut self, observer: Box<dyn SimulationObserver>) {
        self.observers.push(observer);
    }
    
    /// Add a weak observer (doesn't take ownership)
    pub fn add_weak_observer(&mut self, observer: Weak<Mutex<dyn SimulationObserver>>) {
        self.weak_observers.push(observer);
    }
    
    /// Notify all observers of an event
    pub fn notify(&mut self, event: &SimulationEvent) {
        // Notify owned observers
        for observer in &mut self.observers {
            if observer.is_interested_in(event) {
                observer.on_event(event);
            }
        }
        
        // Notify weak observers (and clean up dead references)
        self.weak_observers.retain(|weak_observer| {
            if let Some(observer) = weak_observer.upgrade() {
                if let Ok(mut guard) = observer.try_lock() {
                    if guard.is_interested_in(event) {
                        guard.on_event(event);
                    }
                }
                true // Keep the reference
            } else {
                false // Remove dead reference
            }
        });
    }
    
    /// Remove all observers
    pub fn clear(&mut self) {
        self.observers.clear();
        self.weak_observers.clear();
    }
    
    /// Get count of active observers
    pub fn observer_count(&self) -> usize {
        let weak_count = self.weak_observers.iter()
            .filter(|weak| weak.strong_count() > 0)
            .count();
        self.observers.len() + weak_count
    }
}

impl Default for ObserverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension point for registering custom behavior
/// 
/// # Stability: Unstable API
/// Extension points are experimental and may change significantly.
pub trait ExtensionPoint: Send + Sync + 'static {
    /// Get the name of this extension point
    fn name(&self) -> &str;
    
    /// Get the type ID for type-safe extension registration
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    
    /// Initialize the extension point
    fn initialize(&mut self) -> ModelingResult<()> {
        Ok(())
    }
    
    /// Cleanup the extension point
    fn cleanup(&mut self) -> ModelingResult<()> {
        Ok(())
    }
}

/// Registry for extension points
pub struct ExtensionRegistry {
    extensions: HashMap<String, Box<dyn ExtensionPoint>>,
    type_map: HashMap<TypeId, String>,
}

impl ExtensionRegistry {
    /// Create a new extension registry
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            type_map: HashMap::new(),
        }
    }
    
    /// Register an extension point
    pub fn register_extension<T: ExtensionPoint + 'static>(
        &mut self, 
        extension: T
    ) -> ModelingResult<()> {
        let name = extension.name().to_string();
        let type_id = extension.type_id();
        
        if self.extensions.contains_key(&name) {
            return Err(ModelingError::ExtensionPointNotFound(
                format!("Extension point '{}' already registered", name)
            ));
        }
        
        self.extensions.insert(name.clone(), Box::new(extension));
        self.type_map.insert(type_id, name);
        
        Ok(())
    }
    
    /// Get an extension point by name
    pub fn get_extension(&self, name: &str) -> Option<&dyn ExtensionPoint> {
        self.extensions.get(name).map(|ext| ext.as_ref())
    }
    
    /// Get an extension point by type
    pub fn get_extension_by_type<T: ExtensionPoint + 'static>(&self) -> Option<&dyn ExtensionPoint> {
        let type_id = TypeId::of::<T>();
        let name = self.type_map.get(&type_id)?;
        self.get_extension(name)
    }
    
    /// List all registered extensions
    pub fn list_extensions(&self) -> Vec<&str> {
        self.extensions.keys().map(|s| s.as_str()).collect()
    }
    
    /// Initialize all extensions
    pub fn initialize_all(&mut self) -> ModelingResult<()> {
        for extension in self.extensions.values_mut() {
            extension.initialize()?;
        }
        Ok(())
    }
    
    /// Cleanup all extensions  
    pub fn cleanup_all(&mut self) -> ModelingResult<()> {
        for extension in self.extensions.values_mut() {
            extension.cleanup()?;
        }
        Ok(())
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic component registration for runtime component creation
/// 
/// # Stability: Unstable API
/// Dynamic component registration is experimental.
pub struct DynamicComponentRegistry {
    factories: HashMap<String, Box<dyn ComponentFactory>>,
    categories: HashMap<String, Vec<String>>, // category -> component names
}

/// Factory trait for creating components dynamically
/// 
/// # Stability: Unstable API
/// This trait extends the base ComponentFactory with dynamic capabilities.
pub trait ComponentFactory: Send + Sync {
    /// Create a component instance
    fn create_component(&self, id: ComponentId) -> Box<dyn Component>;
    
    /// Get the component type name
    fn component_type(&self) -> &str;
    
    /// Get the display name for UI
    fn display_name(&self) -> &str {
        self.component_type()
    }
    
    /// Get the category for organization
    fn category(&self) -> &str {
        "Custom"
    }
    
    /// Get a description of this component
    fn description(&self) -> &str {
        "Custom component"
    }
    
    /// Check if this factory can create components of the given type
    fn can_create(&self, component_type: &str) -> bool {
        self.component_type() == component_type
    }
}

impl DynamicComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            categories: HashMap::new(),
        }
    }
    
    /// Register a component factory
    pub fn register_factory(&mut self, factory: Box<dyn ComponentFactory>) -> ModelingResult<()> {
        let component_type = factory.component_type().to_string();
        let category = factory.category().to_string();
        
        if self.factories.contains_key(&component_type) {
            return Err(ModelingError::ComponentRegistrationFailed(
                format!("Component type '{}' already registered", component_type)
            ));
        }
        
        // Add to category
        self.categories.entry(category).or_insert_with(Vec::new).push(component_type.clone());
        
        self.factories.insert(component_type, factory);
        Ok(())
    }
    
    /// Create a component by type name
    pub fn create_component(&self, component_type: &str, id: ComponentId) -> ModelingResult<Box<dyn Component>> {
        let factory = self.factories.get(component_type).ok_or_else(|| {
            ModelingError::ComponentRegistrationFailed(
                format!("Unknown component type: {}", component_type)
            )
        })?;
        
        Ok(factory.create_component(id))
    }
    
    /// Get all registered component types
    pub fn list_component_types(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }
    
    /// Get component types by category
    pub fn list_by_category(&self, category: &str) -> Vec<&str> {
        self.categories.get(category)
            .map(|types| types.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    
    /// Get all categories
    pub fn list_categories(&self) -> Vec<&str> {
        self.categories.keys().map(|s| s.as_str()).collect()
    }
    
    /// Check if a component type is registered
    pub fn is_registered(&self, component_type: &str) -> bool {
        self.factories.contains_key(component_type)
    }
}

impl Default for DynamicComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced modeling context that ties everything together
/// 
/// # Stability: Unstable API
/// This is the main interface for advanced modeling features.
pub struct ModelingContext {
    observer_manager: ObserverManager,
    extension_registry: ExtensionRegistry,
    component_registry: DynamicComponentRegistry,
    enabled: bool,
}

impl ModelingContext {
    /// Create a new modeling context
    pub fn new() -> Self {
        Self {
            observer_manager: ObserverManager::new(),
            extension_registry: ExtensionRegistry::new(),
            component_registry: DynamicComponentRegistry::new(),
            enabled: true,
        }
    }
    
    /// Get the observer manager
    pub fn observer_manager(&mut self) -> &mut ObserverManager {
        &mut self.observer_manager
    }
    
    /// Get the extension registry
    pub fn extension_registry(&mut self) -> &mut ExtensionRegistry {
        &mut self.extension_registry
    }
    
    /// Get the component registry
    pub fn component_registry(&mut self) -> &mut DynamicComponentRegistry {
        &mut self.component_registry
    }
    
    /// Check if advanced modeling is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enable or disable advanced modeling features
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Notify observers of an event (if enabled)
    pub fn notify_event(&mut self, event: &SimulationEvent) {
        if self.enabled {
            self.observer_manager.notify(event);
        }
    }
    
    /// Initialize all components
    pub fn initialize(&mut self) -> ModelingResult<()> {
        self.extension_registry.initialize_all()?;
        Ok(())
    }
    
    /// Cleanup all components
    pub fn cleanup(&mut self) -> ModelingResult<()> {
        self.extension_registry.cleanup_all()?;
        self.observer_manager.clear();
        Ok(())
    }
}

impl Default for ModelingContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::{BusWidth, Value};
    use std::sync::{Arc, Mutex};
    
    struct TestObserver {
        name: String,
        events_received: usize,
    }
    
    impl TestObserver {
        fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                events_received: 0,
            }
        }
    }
    
    impl SimulationObserver for TestObserver {
        fn on_event(&mut self, _event: &SimulationEvent) {
            self.events_received += 1;
        }
        
        fn name(&self) -> &str {
            &self.name
        }
    }
    
    #[test]
    fn test_observer_manager() {
        let mut manager = ObserverManager::new();
        let observer = Box::new(TestObserver::new("test"));
        
        manager.add_observer(observer);
        assert_eq!(manager.observer_count(), 1);
        
        let event = SimulationEvent::SimulationReset {
            timestamp: Timestamp(0),
        };
        
        manager.notify(&event);
        // Event should be processed (we can't easily test the internal state change)
    }
    
    #[test]
    fn test_weak_observer() {
        let mut manager = ObserverManager::new();
        let observer = Arc::new(Mutex::new(TestObserver::new("weak_test")));
        let weak_observer = Arc::downgrade(&observer);
        
        manager.add_weak_observer(weak_observer);
        assert_eq!(manager.observer_count(), 1);
        
        let event = SimulationEvent::SimulationReset {
            timestamp: Timestamp(0),
        };
        
        manager.notify(&event);
        
        // Drop the strong reference
        drop(observer);
        
        // Next notify should clean up the dead reference
        manager.notify(&event);
        assert_eq!(manager.observer_count(), 0);
    }
    
    struct TestExtension {
        name: String,
    }
    
    impl TestExtension {
        fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
            }
        }
    }
    
    impl ExtensionPoint for TestExtension {
        fn name(&self) -> &str {
            &self.name
        }
    }
    
    #[test]
    fn test_extension_registry() {
        let mut registry = ExtensionRegistry::new();
        let extension = TestExtension::new("test_ext");
        
        registry.register_extension(extension).unwrap();
        assert_eq!(registry.list_extensions(), vec!["test_ext"]);
        
        let retrieved = registry.get_extension("test_ext");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test_ext");
    }
    
    #[test]
    fn test_dynamic_component_registry() {
        let mut registry = DynamicComponentRegistry::new();
        
        // We can't easily test this without implementing a full ComponentFactory
        // But we can test the basic structure
        assert_eq!(registry.list_component_types().len(), 0);
        assert_eq!(registry.list_categories().len(), 0);
        assert!(!registry.is_registered("test_component"));
    }
    
    #[test]
    fn test_modeling_context() {
        let mut context = ModelingContext::new();
        
        assert!(context.is_enabled());
        
        context.set_enabled(false);
        assert!(!context.is_enabled());
        
        // Test initialization and cleanup
        context.initialize().unwrap();
        context.cleanup().unwrap();
    }
    
    #[test]
    fn test_simulation_events() {
        let event = SimulationEvent::SignalChanged {
            node_id: NodeId(1),
            old_signal: Signal::new_single(Value::Low),
            new_signal: Signal::new_single(Value::High),
            timestamp: Timestamp(100),
            source: ComponentId(42),
        };
        
        match event {
            SimulationEvent::SignalChanged { node_id, .. } => {
                assert_eq!(node_id, NodeId(1));
            }
            _ => panic!("Wrong event type"),
        }
    }
}