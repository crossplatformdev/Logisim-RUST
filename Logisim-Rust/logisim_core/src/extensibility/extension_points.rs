//! Extension points for plugin development
//!
//! This module defines the core extension points that plugins can use to extend
//! Logisim-RUST functionality. These provide stable interfaces for plugin authors
//! while allowing the core system to evolve independently.
//!
//! **API Stability: UNSTABLE** - These APIs are subject to change in future versions.

use crate::comp::event::{ExtensibleObserver, ComponentEvent, SimulationEvent, CircuitEvent, PluginEvent};
use crate::comp::factory::ComponentFactory;
use crate::comp::component::{Component, ComponentId};
use crate::data::{AttributeSet, Location};
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur in extension points
#[derive(Error, Debug)]
pub enum ExtensionError {
    #[error("Extension point not available: {0}")]
    NotAvailable(String),
    #[error("Extension registration failed: {0}")]
    RegistrationFailed(String),
    #[error("Extension not found: {0}")]
    NotFound(String),
    #[error("Extension point disabled: {0}")]
    Disabled(String),
}

/// Result type for extension operations
pub type ExtensionResult<T> = Result<T, ExtensionError>;

/// Extension point for component creation
/// 
/// This extension point allows plugins to register custom component factories
/// and override component creation behavior.
pub trait ComponentCreationExtension: Send + Sync {
    /// Get the extension name
    fn name(&self) -> &str;
    
    /// Check if this extension can handle the given component type
    fn can_create(&self, component_type: &str) -> bool;
    
    /// Create a component of the specified type
    fn create_component(
        &self,
        component_type: &str,
        id: ComponentId,
        location: Location,
        attrs: &AttributeSet,
    ) -> ExtensionResult<Box<dyn Component>>;
    
    /// Get the factory for a component type (if available)
    fn get_factory(&self, component_type: &str) -> Option<Arc<dyn ComponentFactory>>;
    
    /// List supported component types
    fn supported_types(&self) -> Vec<String>;
}

/// Extension point for simulation behavior
/// 
/// This extension point allows plugins to hook into the simulation engine
/// and modify simulation behavior.
pub trait SimulationExtension: Send + Sync {
    /// Get the extension name
    fn name(&self) -> &str;
    
    /// Called before simulation starts
    fn before_simulation_start(&mut self) -> ExtensionResult<()> {
        Ok(())
    }
    
    /// Called after simulation stops
    fn after_simulation_stop(&mut self) -> ExtensionResult<()> {
        Ok(())
    }
    
    /// Called before each simulation step
    fn before_step(&mut self, _step: u64) -> ExtensionResult<()> {
        Ok(())
    }
    
    /// Called after each simulation step
    fn after_step(&mut self, _step: u64) -> ExtensionResult<()> {
        Ok(())
    }
    
    /// Check if this extension wants to handle a signal change
    fn handles_signal_change(&self, _node_id: &str) -> bool {
        false
    }
    
    /// Process a signal change (if handles_signal_change returns true)
    fn process_signal_change(&mut self, _node_id: &str, _value: &str) -> ExtensionResult<()> {
        Ok(())
    }
}

/// Extension point for UI integration
/// 
/// This extension point allows plugins to add UI elements and modify
/// the user interface.
pub trait UIExtension: Send + Sync {
    /// Get the extension name
    fn name(&self) -> &str;
    
    /// Get menu items to add to the main menu
    fn get_menu_items(&self) -> Vec<MenuItem> {
        Vec::new()
    }
    
    /// Get toolbar buttons to add
    fn get_toolbar_buttons(&self) -> Vec<ToolbarButton> {
        Vec::new()
    }
    
    /// Get property editors for component types
    fn get_property_editors(&self) -> Vec<PropertyEditor> {
        Vec::new()
    }
    
    /// Called when the UI needs to update
    fn update_ui(&mut self) -> ExtensionResult<()> {
        Ok(())
    }
}

/// Menu item definition for UI extensions
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub submenu: Vec<MenuItem>,
    pub enabled: bool,
    pub visible: bool,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(id: String, label: String) -> Self {
        Self {
            id,
            label,
            submenu: Vec::new(),
            enabled: true,
            visible: true,
        }
    }
    
    /// Add a submenu item
    pub fn with_submenu(mut self, submenu: Vec<MenuItem>) -> Self {
        self.submenu = submenu;
        self
    }
    
    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Toolbar button definition for UI extensions
#[derive(Debug, Clone)]
pub struct ToolbarButton {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub icon_path: Option<String>,
    pub enabled: bool,
    pub visible: bool,
}

impl ToolbarButton {
    /// Create a new toolbar button
    pub fn new(id: String, label: String, tooltip: String) -> Self {
        Self {
            id,
            label,
            tooltip,
            icon_path: None,
            enabled: true,
            visible: true,
        }
    }
    
    /// Set the icon path
    pub fn with_icon(mut self, icon_path: String) -> Self {
        self.icon_path = Some(icon_path);
        self
    }
}

/// Property editor definition for component configuration
#[derive(Debug, Clone)]
pub struct PropertyEditor {
    pub component_type: String,
    pub editor_type: String,
    pub properties: Vec<PropertyDefinition>,
}

/// Property definition for component attributes
#[derive(Debug, Clone)]
pub struct PropertyDefinition {
    pub name: String,
    pub display_name: String,
    pub property_type: String,
    pub default_value: String,
    pub editable: bool,
}

/// Extension point registry for managing all extension points
/// 
/// **API Stability: UNSTABLE** - This registry may change significantly in future versions.
pub struct ExtensionPointRegistry {
    component_extensions: Vec<Box<dyn ComponentCreationExtension>>,
    simulation_extensions: Vec<Box<dyn SimulationExtension>>,
    ui_extensions: Vec<Box<dyn UIExtension>>,
    observers: Vec<Box<dyn ExtensibleObserver>>,
    enabled: bool,
}

impl ExtensionPointRegistry {
    /// Create a new extension point registry
    pub fn new() -> Self {
        Self {
            component_extensions: Vec::new(),
            simulation_extensions: Vec::new(),
            ui_extensions: Vec::new(),
            observers: Vec::new(),
            enabled: true,
        }
    }
    
    /// Register a component creation extension
    pub fn register_component_extension(&mut self, extension: Box<dyn ComponentCreationExtension>) {
        log::debug!("Registering component extension: {}", extension.name());
        self.component_extensions.push(extension);
    }
    
    /// Register a simulation extension
    pub fn register_simulation_extension(&mut self, extension: Box<dyn SimulationExtension>) {
        log::debug!("Registering simulation extension: {}", extension.name());
        self.simulation_extensions.push(extension);
    }
    
    /// Register a UI extension
    pub fn register_ui_extension(&mut self, extension: Box<dyn UIExtension>) {
        log::debug!("Registering UI extension: {}", extension.name());
        self.ui_extensions.push(extension);
    }
    
    /// Register an extensible observer
    pub fn register_observer(&mut self, observer: Box<dyn ExtensibleObserver>) {
        log::debug!("Registering observer with priority: {}", observer.priority());
        self.observers.push(observer);
        // Sort by priority (highest first)
        self.observers.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }
    
    /// Find a component extension that can create the specified type
    pub fn find_component_extension(&self, component_type: &str) -> Option<&dyn ComponentCreationExtension> {
        if !self.enabled {
            return None;
        }
        
        self.component_extensions
            .iter()
            .find(|ext| ext.can_create(component_type))
            .map(|ext| ext.as_ref())
    }
    
    /// Get all simulation extensions
    pub fn get_simulation_extensions(&mut self) -> &mut [Box<dyn SimulationExtension>] {
        if !self.enabled {
            return &mut [];
        }
        &mut self.simulation_extensions
    }
    
    /// Get all UI extensions
    pub fn get_ui_extensions(&self) -> &[Box<dyn UIExtension>] {
        if !self.enabled {
            return &[];
        }
        &self.ui_extensions
    }
    
    /// Notify all observers of a component event
    pub fn notify_component_event(&mut self, event: &ComponentEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_component_event(event);
        }
    }
    
    /// Notify all observers of a simulation event
    pub fn notify_simulation_event(&mut self, event: &SimulationEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_simulation_event(event);
        }
    }
    
    /// Notify all observers of a circuit event
    pub fn notify_circuit_event(&mut self, event: &CircuitEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_circuit_event(event);
        }
    }
    
    /// Notify all observers of a plugin event
    pub fn notify_plugin_event(&mut self, event: &PluginEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_plugin_event(event);
        }
    }
    
    /// Enable or disable the extension system
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        log::info!("Extension point registry enabled: {}", enabled);
    }
    
    /// Check if the extension system is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get extension counts for diagnostics
    pub fn get_extension_counts(&self) -> ExtensionCounts {
        ExtensionCounts {
            component_extensions: self.component_extensions.len(),
            simulation_extensions: self.simulation_extensions.len(),
            ui_extensions: self.ui_extensions.len(),
            observers: self.observers.len(),
        }
    }
    
    /// Clear all extensions
    pub fn clear(&mut self) {
        self.component_extensions.clear();
        self.simulation_extensions.clear();
        self.ui_extensions.clear();
        self.observers.clear();
        log::info!("Cleared all extension points");
    }
}

impl Default for ExtensionPointRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension counts for diagnostics
#[derive(Debug, Clone)]
pub struct ExtensionCounts {
    pub component_extensions: usize,
    pub simulation_extensions: usize,
    pub ui_extensions: usize,
    pub observers: usize,
}

/// Global extension point registry
/// 
/// **API Stability: UNSTABLE** - This may be replaced with dependency injection in future versions.
static mut GLOBAL_EXTENSION_REGISTRY: Option<ExtensionPointRegistry> = None;

/// Initialize the global extension point registry
pub fn initialize_extension_points() -> ExtensionResult<()> {
    unsafe {
        GLOBAL_EXTENSION_REGISTRY = Some(ExtensionPointRegistry::new());
    }
    log::info!("Initialized global extension point registry");
    Ok(())
}

/// Get a reference to the global extension point registry
pub fn with_extensions<F, R>(f: F) -> ExtensionResult<R>
where
    F: FnOnce(&ExtensionPointRegistry) -> R,
{
    unsafe {
        match &GLOBAL_EXTENSION_REGISTRY {
            Some(registry) => Ok(f(registry)),
            None => Err(ExtensionError::NotAvailable("Extension registry not initialized".to_string())),
        }
    }
}

/// Get a mutable reference to the global extension point registry
pub fn with_extensions_mut<F, R>(f: F) -> ExtensionResult<R>
where
    F: FnOnce(&mut ExtensionPointRegistry) -> R,
{
    unsafe {
        match &mut GLOBAL_EXTENSION_REGISTRY {
            Some(registry) => Ok(f(registry)),
            None => Err(ExtensionError::NotAvailable("Extension registry not initialized".to_string())),
        }
    }
}

/// Check if the global extension registry is initialized
pub fn is_extensions_initialized() -> bool {
    unsafe { GLOBAL_EXTENSION_REGISTRY.is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestComponentExtension {
        name: String,
        types: Vec<String>,
    }
    
    impl ComponentCreationExtension for TestComponentExtension {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn can_create(&self, component_type: &str) -> bool {
            self.types.contains(&component_type.to_string())
        }
        
        fn create_component(
            &self,
            _component_type: &str,
            _id: ComponentId,
            _location: Location,
            _attrs: &AttributeSet,
        ) -> ExtensionResult<Box<dyn Component>> {
            Err(ExtensionError::NotAvailable("Test extension".to_string()))
        }
        
        fn get_factory(&self, _component_type: &str) -> Option<Arc<dyn ComponentFactory>> {
            None
        }
        
        fn supported_types(&self) -> Vec<String> {
            self.types.clone()
        }
    }
    
    #[test]
    fn test_extension_registry() {
        let mut registry = ExtensionPointRegistry::new();
        
        let extension = Box::new(TestComponentExtension {
            name: "test".to_string(),
            types: vec!["test_component".to_string()],
        });
        
        registry.register_component_extension(extension);
        
        // Test finding extension
        assert!(registry.find_component_extension("test_component").is_some());
        assert!(registry.find_component_extension("unknown").is_none());
        
        // Test counts
        let counts = registry.get_extension_counts();
        assert_eq!(counts.component_extensions, 1);
        assert_eq!(counts.simulation_extensions, 0);
    }
    
    #[test]
    fn test_menu_item() {
        let item = MenuItem::new("test".to_string(), "Test Item".to_string())
            .with_enabled(false);
        
        assert_eq!(item.id, "test");
        assert_eq!(item.label, "Test Item");
        assert!(!item.enabled);
        assert!(item.visible);
    }
}