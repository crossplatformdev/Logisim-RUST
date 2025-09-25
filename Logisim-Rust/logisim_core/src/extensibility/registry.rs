//! Dynamic component registration system
//!
//! This module provides runtime component registration capabilities for plugins
//! and extensible component libraries. It maintains backward compatibility with
//! the existing factory system while adding dynamic discovery and registration.
//!
//! **API Stability: UNSTABLE** - These APIs are subject to change in future versions.

use crate::comp::factory::ComponentFactory;
use crate::comp::component::{Component, ComponentId};
use crate::data::{AttributeSet, Location};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

/// Errors that can occur during component registration
#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Component type '{0}' already registered")]
    AlreadyRegistered(String),
    #[error("Component type '{0}' not found")]
    NotFound(String),
    #[error("Invalid component factory: {0}")]
    InvalidFactory(String),
    #[error("Registration system not initialized")]
    NotInitialized,
}

/// Result type for registration operations
pub type RegistrationResult<T> = Result<T, RegistrationError>;

/// Metadata about a registered component type
#[derive(Debug, Clone)]
pub struct ComponentTypeInfo {
    /// Unique identifier for this component type
    pub type_id: String,
    /// Human-readable display name
    pub display_name: String,
    /// Category for UI organization
    pub category: String,
    /// Description for tooltips/help
    pub description: String,
    /// Version of the component implementation
    pub version: String,
    /// Plugin or library that provides this component
    pub provider: String,
    /// Whether this component is available (enabled)
    pub enabled: bool,
    /// Tags for searching and filtering
    pub tags: Vec<String>,
}

impl ComponentTypeInfo {
    /// Create new component type info
    pub fn new(
        type_id: String,
        display_name: String,
        category: String,
        description: String,
    ) -> Self {
        Self {
            type_id,
            display_name,
            category,
            description,
            version: "1.0.0".to_string(),
            provider: "core".to_string(),
            enabled: true,
            tags: Vec::new(),
        }
    }
    
    /// Set the version
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }
    
    /// Set the provider
    pub fn with_provider(mut self, provider: String) -> Self {
        self.provider = provider;
        self
    }
    
    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Dynamic component registry
/// 
/// This registry allows runtime registration and discovery of component types,
/// enabling plugin systems and dynamic component loading.
#[derive(Default)]
pub struct ComponentRegistry {
    /// Map of component type ID to factory
    factories: HashMap<String, Arc<dyn ComponentFactory>>,
    /// Map of component type ID to metadata
    metadata: HashMap<String, ComponentTypeInfo>,
    /// Categories and their component types
    categories: HashMap<String, Vec<String>>,
    /// Registry state
    initialized: bool,
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            metadata: HashMap::new(),
            categories: HashMap::new(),
            initialized: true,
        }
    }
    
    /// Register a component factory with metadata
    pub fn register_component(
        &mut self,
        factory: Arc<dyn ComponentFactory>,
        info: ComponentTypeInfo,
    ) -> RegistrationResult<()> {
        if !self.initialized {
            return Err(RegistrationError::NotInitialized);
        }
        
        let type_id = info.type_id.clone();
        let category = info.category.clone();
        
        // Check if already registered
        if self.factories.contains_key(&type_id) {
            return Err(RegistrationError::AlreadyRegistered(type_id));
        }
        
        // Register factory and metadata
        self.factories.insert(type_id.clone(), factory);
        self.metadata.insert(type_id.clone(), info);
        
        // Add to category
        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(type_id);
        
        log::debug!("Registered component type: {}", type_id);
        Ok(())
    }
    
    /// Unregister a component factory
    pub fn unregister_component(&mut self, type_id: &str) -> RegistrationResult<()> {
        if !self.initialized {
            return Err(RegistrationError::NotInitialized);
        }
        
        let info = self.metadata.remove(type_id)
            .ok_or_else(|| RegistrationError::NotFound(type_id.to_string()))?;
        
        self.factories.remove(type_id);
        
        // Remove from category
        if let Some(category_list) = self.categories.get_mut(&info.category) {
            category_list.retain(|id| id != type_id);
        }
        
        log::debug!("Unregistered component type: {}", type_id);
        Ok(())
    }
    
    /// Get a component factory by type ID
    pub fn get_factory(&self, type_id: &str) -> Option<Arc<dyn ComponentFactory>> {
        self.factories.get(type_id).cloned()
    }
    
    /// Get component metadata by type ID
    pub fn get_metadata(&self, type_id: &str) -> Option<&ComponentTypeInfo> {
        self.metadata.get(type_id)
    }
    
    /// List all registered component types
    pub fn list_component_types(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
    
    /// List component types in a category
    pub fn list_category_components(&self, category: &str) -> Vec<String> {
        self.categories.get(category).cloned().unwrap_or_default()
    }
    
    /// List all categories
    pub fn list_categories(&self) -> Vec<String> {
        self.categories.keys().cloned().collect()
    }
    
    /// Search for components by tags or name
    pub fn search_components(&self, query: &str) -> Vec<String> {
        let query_lower = query.to_lowercase();
        self.metadata
            .iter()
            .filter(|(_, info)| {
                info.display_name.to_lowercase().contains(&query_lower)
                    || info.description.to_lowercase().contains(&query_lower)
                    || info.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .map(|(type_id, _)| type_id.clone())
            .collect()
    }
    
    /// Get component count
    pub fn component_count(&self) -> usize {
        self.factories.len()
    }
    
    /// Clear all registrations
    pub fn clear(&mut self) {
        self.factories.clear();
        self.metadata.clear();
        self.categories.clear();
        log::debug!("Cleared component registry");
    }
    
    /// Enable/disable a component type
    pub fn set_component_enabled(&mut self, type_id: &str, enabled: bool) -> RegistrationResult<()> {
        let info = self.metadata.get_mut(type_id)
            .ok_or_else(|| RegistrationError::NotFound(type_id.to_string()))?;
        
        info.enabled = enabled;
        log::debug!("Set component {} enabled: {}", type_id, enabled);
        Ok(())
    }
    
    /// Check if a component type is enabled
    pub fn is_component_enabled(&self, type_id: &str) -> bool {
        self.metadata.get(type_id)
            .map(|info| info.enabled)
            .unwrap_or(false)
    }
}

/// Global component registry instance
/// 
/// **API Stability: UNSTABLE** - This may be replaced with dependency injection in future versions.
static GLOBAL_REGISTRY: RwLock<Option<ComponentRegistry>> = RwLock::new(None);

/// Initialize the global component registry
pub fn initialize_registry() -> RegistrationResult<()> {
    let mut registry = GLOBAL_REGISTRY.write().map_err(|_| RegistrationError::NotInitialized)?;
    *registry = Some(ComponentRegistry::new());
    log::info!("Initialized global component registry");
    Ok(())
}

/// Get a reference to the global component registry
pub fn with_registry<F, R>(f: F) -> RegistrationResult<R>
where
    F: FnOnce(&ComponentRegistry) -> R,
{
    let registry = GLOBAL_REGISTRY.read().map_err(|_| RegistrationError::NotInitialized)?;
    let registry = registry.as_ref().ok_or(RegistrationError::NotInitialized)?;
    Ok(f(registry))
}

/// Get a mutable reference to the global component registry
pub fn with_registry_mut<F, R>(f: F) -> RegistrationResult<R>
where
    F: FnOnce(&mut ComponentRegistry) -> R,
{
    let mut registry = GLOBAL_REGISTRY.write().map_err(|_| RegistrationError::NotInitialized)?;
    let registry = registry.as_mut().ok_or(RegistrationError::NotInitialized)?;
    Ok(f(registry))
}

/// Register a component in the global registry
pub fn register_global_component(
    factory: Arc<dyn ComponentFactory>,
    info: ComponentTypeInfo,
) -> RegistrationResult<()> {
    with_registry_mut(|registry| registry.register_component(factory, info))?
}

/// Check if the global registry is initialized
pub fn is_registry_initialized() -> bool {
    GLOBAL_REGISTRY.read()
        .map(|r| r.is_some())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Bounds;
    
    // Mock component for testing
    struct MockComponent {
        id: ComponentId,
        pins: HashMap<String, Pin>,
    }
    
    impl Component for MockComponent {
        fn id(&self) -> ComponentId {
            self.id
        }
        
        fn name(&self) -> &str {
            "MockComponent"
        }
        
        fn pins(&self) -> &HashMap<String, Pin> {
            &self.pins
        }
        
        fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
            &mut self.pins
        }
        
        fn update(&mut self, _current_time: crate::signal::Timestamp) -> crate::comp::component::UpdateResult {
            crate::comp::component::UpdateResult::new()
        }
        
        fn reset(&mut self) {}
        
        fn location(&self) -> Option<crate::data::Location> {
            Some(crate::data::Location::new(0, 0))
        }
        
        fn bounds(&self) -> Option<Bounds> {
            Some(Bounds::new(0, 0, 10, 10))
        }
    }
    
    // Mock factory for testing
    struct MockFactory {
        name: String,
    }
    
    impl ComponentFactory for MockFactory {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn display_name(&self) -> &str {
            &self.name
        }
        
        fn create_component(
            &self,
            id: ComponentId,
            _location: crate::data::Location,
            _attrs: &AttributeSet,
        ) -> Box<dyn Component> {
            Box::new(MockComponent { 
                id,
                pins: HashMap::new(),
            })
        }
        
        fn create_attribute_set(&self) -> AttributeSet {
            AttributeSet::new()
        }
        
        fn get_bounds(&self, _attrs: &AttributeSet) -> Bounds {
            Bounds::new(0, 0, 10, 10)
        }
    }
    
    #[test]
    fn test_component_registry() {
        let mut registry = ComponentRegistry::new();
        
        let factory = Arc::new(MockFactory { name: "test".to_string() });
        let info = ComponentTypeInfo::new(
            "test".to_string(),
            "Test Component".to_string(),
            "test".to_string(),
            "A test component".to_string(),
        );
        
        // Test registration
        assert!(registry.register_component(factory.clone(), info).is_ok());
        assert_eq!(registry.component_count(), 1);
        
        // Test duplicate registration
        let info2 = ComponentTypeInfo::new(
            "test".to_string(),
            "Test Component 2".to_string(),
            "test".to_string(),
            "Another test component".to_string(),
        );
        assert!(matches!(
            registry.register_component(factory, info2),
            Err(RegistrationError::AlreadyRegistered(_))
        ));
        
        // Test retrieval
        assert!(registry.get_factory("test").is_some());
        assert!(registry.get_metadata("test").is_some());
        
        // Test unregistration
        assert!(registry.unregister_component("test").is_ok());
        assert_eq!(registry.component_count(), 0);
    }
    
    #[test]
    fn test_component_search() {
        let mut registry = ComponentRegistry::new();
        
        let factory = Arc::new(MockFactory { name: "and_gate".to_string() });
        let info = ComponentTypeInfo::new(
            "and_gate".to_string(),
            "AND Gate".to_string(),
            "logic".to_string(),
            "Logical AND operation".to_string(),
        ).with_tags(vec!["gate".to_string(), "basic".to_string()]);
        
        registry.register_component(factory, info).unwrap();
        
        // Search by name
        let results = registry.search_components("AND");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "and_gate");
        
        // Search by tag
        let results = registry.search_components("gate");
        assert_eq!(results.len(), 1);
        
        // Search by description
        let results = registry.search_components("logical");
        assert_eq!(results.len(), 1);
    }
}