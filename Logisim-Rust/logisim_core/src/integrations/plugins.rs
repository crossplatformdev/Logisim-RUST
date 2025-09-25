//! Plugin system and dynamic component registration
//!
//! This module provides a comprehensive plugin system for extending Logisim-RUST
//! with custom components, tools, and functionality. It supports multiple plugin
//! formats and provides extensibility hooks for advanced modeling features.
//!
//! # Stability
//! 
//! **⚠️ UNSTABLE API**: Plugin system APIs are experimental and subject to change
//! in future versions. The plugin interface may be extended or modified.

use crate::{Component, ComponentId, Location};
use crate::event_system::{Observer, CircuitEvent, SimulationEvent};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Plugin system errors
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin system not implemented in current version")]
    NotImplemented,
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
    #[error("Plugin loading failed: {0}")]
    LoadingFailed(String),
    #[error("Invalid plugin format: {0}")]
    InvalidFormat(String),
    #[error("Plugin dependency missing: {0}")]
    DependencyMissing(String),
    #[error("Plugin version incompatible: {0}")]
    VersionIncompatible(String),
    #[error("Component type already registered: {0}")]
    ComponentTypeExists(String),
    #[error("Extension point not found: {0}")]
    ExtensionPointNotFound(String),
    #[error("Hook registration failed: {0}")]
    HookRegistrationFailed(String),
}

/// Plugin operation result
pub type PluginResult<T> = Result<T, PluginError>;

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub homepage: Option<String>,
    pub dependencies: Vec<PluginDependency>,
    pub entry_point: String,
}

/// Plugin dependency specification
#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub name: String,
    pub version_requirement: String, // e.g., ">=1.0.0"
    pub optional: bool,
}

/// Plugin library definition with extensibility hooks
/// 
/// **⚠️ UNSTABLE API**: This trait may be extended with additional methods
pub trait PluginLibrary: Send + Sync {
    /// Get library information
    fn info(&self) -> &PluginInfo;

    /// Get available components in this library
    fn components(&self) -> Vec<ComponentInfo>;

    /// Create a component instance
    fn create_component(
        &self,
        component_type: &str,
        id: ComponentId,
    ) -> PluginResult<Box<dyn Component>>;

    /// Initialize the plugin
    fn initialize(&mut self) -> PluginResult<()>;

    /// Cleanup the plugin
    fn cleanup(&mut self) -> PluginResult<()>;
    
    /// Register extension hooks (extensibility hook)
    fn register_hooks(&mut self, registry: &mut ExtensionRegistry) -> PluginResult<()> {
        let _ = registry; // Default implementation does nothing
        Ok(())
    }
    
    /// Get plugin-specific configuration schema
    fn config_schema(&self) -> Option<ConfigSchema> {
        None
    }
    
    /// Handle plugin-specific events
    fn on_plugin_event(&mut self, event: &PluginEvent) -> PluginResult<()> {
        let _ = event; // Default implementation ignores events
        Ok(())
    }
}

/// Extension registry for managing plugin hooks and extension points
/// 
/// **⚠️ UNSTABLE API**: Extension system is experimental
pub struct ExtensionRegistry {
    component_factories: HashMap<String, Box<dyn ComponentFactory>>,
    modeling_extensions: HashMap<String, Box<dyn ModelingExtension>>,
    ui_extensions: HashMap<String, Box<dyn UiExtension>>,
    simulation_hooks: Vec<Box<dyn SimulationHook>>,
    circuit_observers: Vec<Arc<Mutex<dyn Observer<CircuitEvent>>>>,
    simulation_observers: Vec<Arc<Mutex<dyn Observer<SimulationEvent>>>>,
}

impl ExtensionRegistry {
    /// Create a new extension registry
    pub fn new() -> Self {
        Self {
            component_factories: HashMap::new(),
            modeling_extensions: HashMap::new(),
            ui_extensions: HashMap::new(),
            simulation_hooks: Vec::new(),
            circuit_observers: Vec::new(),
            simulation_observers: Vec::new(),
        }
    }
    
    /// Register a component factory
    pub fn register_component_factory(&mut self, name: String, factory: Box<dyn ComponentFactory>) -> PluginResult<()> {
        if self.component_factories.contains_key(&name) {
            return Err(PluginError::ComponentTypeExists(name));
        }
        self.component_factories.insert(name, factory);
        Ok(())
    }
    
    /// Register a modeling extension
    pub fn register_modeling_extension(&mut self, name: String, extension: Box<dyn ModelingExtension>) -> PluginResult<()> {
        self.modeling_extensions.insert(name, extension);
        Ok(())
    }
    
    /// Register a UI extension
    pub fn register_ui_extension(&mut self, name: String, extension: Box<dyn UiExtension>) -> PluginResult<()> {
        self.ui_extensions.insert(name, extension);
        Ok(())
    }
    
    /// Add a simulation hook
    pub fn add_simulation_hook(&mut self, hook: Box<dyn SimulationHook>) {
        self.simulation_hooks.push(hook);
    }
    
    /// Add a circuit event observer
    pub fn add_circuit_observer(&mut self, observer: Arc<Mutex<dyn Observer<CircuitEvent>>>) {
        self.circuit_observers.push(observer);
    }
    
    /// Add a simulation event observer
    pub fn add_simulation_observer(&mut self, observer: Arc<Mutex<dyn Observer<SimulationEvent>>>) {
        self.simulation_observers.push(observer);
    }
    
    /// Get all registered component factories
    pub fn component_factories(&self) -> &HashMap<String, Box<dyn ComponentFactory>> {
        &self.component_factories
    }
    
    /// Get all registered modeling extensions
    pub fn modeling_extensions(&self) -> &HashMap<String, Box<dyn ModelingExtension>> {
        &self.modeling_extensions
    }
    
    /// Get all simulation hooks
    pub fn simulation_hooks(&self) -> &[Box<dyn SimulationHook>] {
        &self.simulation_hooks
    }
    
    /// Get all circuit event observers
    pub fn circuit_observers(&self) -> &[Arc<Mutex<dyn Observer<CircuitEvent>>>] {
        &self.circuit_observers
    }
    
    /// Get all simulation event observers  
    pub fn simulation_observers(&self) -> &[Arc<Mutex<dyn Observer<SimulationEvent>>>] {
        &self.simulation_observers
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory trait for creating components dynamically
/// 
/// **⚠️ UNSTABLE API**: Component factory interface may be extended
pub trait ComponentFactory: Send + Sync {
    /// Create a new component instance
    fn create(&self, id: ComponentId, location: Location) -> PluginResult<Box<dyn Component>>;
    
    /// Get component type information
    fn component_info(&self) -> ComponentInfo;
    
    /// Validate component placement at location
    fn validate_placement(&self, location: Location) -> bool {
        let _ = location; // Default allows placement anywhere
        true
    }
}

/// Modeling extension trait for advanced simulation features
/// 
/// **⚠️ UNSTABLE API**: Modeling extension interface is experimental
pub trait ModelingExtension: Send + Sync {
    /// Get extension name
    fn name(&self) -> &str;
    
    /// Initialize modeling extension
    fn initialize(&mut self) -> PluginResult<()>;
    
    /// Process simulation step with custom modeling
    fn process_step(&mut self, step_data: &SimulationStepData) -> PluginResult<()>;
    
    /// Cleanup modeling extension
    fn cleanup(&mut self) -> PluginResult<()>;
}

/// UI extension trait for custom user interface elements
/// 
/// **⚠️ UNSTABLE API**: UI extension interface may change significantly
pub trait UiExtension: Send + Sync {
    /// Get extension name
    fn name(&self) -> &str;
    
    /// Initialize UI extension
    fn initialize(&mut self) -> PluginResult<()>;
    
    /// Render UI extension elements
    fn render(&mut self, ui_context: &mut UiContext) -> PluginResult<()>;
    
    /// Handle UI events
    fn handle_event(&mut self, event: &UiEvent) -> PluginResult<()>;
    
    /// Cleanup UI extension
    fn cleanup(&mut self) -> PluginResult<()>;
}

/// Simulation hook trait for intercepting simulation events
/// 
/// **⚠️ UNSTABLE API**: Simulation hook interface is experimental
pub trait SimulationHook: Send + Sync {
    /// Called before simulation starts
    fn before_simulation_start(&mut self) -> PluginResult<()> {
        Ok(())
    }
    
    /// Called after simulation stops
    fn after_simulation_stop(&mut self) -> PluginResult<()> {
        Ok(())
    }
    
    /// Called before each simulation step
    fn before_step(&mut self, step_count: u64) -> PluginResult<()> {
        let _ = step_count;
        Ok(())
    }
    
    /// Called after each simulation step
    fn after_step(&mut self, step_count: u64) -> PluginResult<()> {
        let _ = step_count;
        Ok(())
    }
}

/// Configuration schema for plugin settings
/// 
/// **⚠️ UNSTABLE API**: Configuration schema may be redesigned
#[derive(Debug, Clone)]
pub struct ConfigSchema {
    pub fields: Vec<ConfigField>,
    pub version: String,
}

/// Configuration field definition
#[derive(Debug, Clone)]
pub struct ConfigField {
    pub name: String,
    pub field_type: ConfigFieldType,
    pub default_value: Option<String>,
    pub description: String,
    pub required: bool,
}

/// Configuration field types
#[derive(Debug, Clone)]
pub enum ConfigFieldType {
    String,
    Integer,
    Float,
    Boolean,
    Choice(Vec<String>),
    Path,
}

/// Plugin-specific events
/// 
/// **⚠️ UNSTABLE API**: Plugin event types may be extended
#[derive(Debug, Clone)]
pub enum PluginEvent {
    ConfigChanged {
        config: HashMap<String, String>,
    },
    PluginLoaded {
        plugin_name: String,
    },
    PluginUnloaded {
        plugin_name: String,
    },
    ExtensionRegistered {
        extension_name: String,
        extension_type: String,
    },
}

/// Simulation step data for modeling extensions
/// 
/// **⚠️ UNSTABLE API**: Step data structure may be extended
#[derive(Debug)]
pub struct SimulationStepData {
    pub step_count: u64,
    pub current_time: u64,
    pub changed_signals: Vec<(ComponentId, crate::Signal)>,
    pub active_components: Vec<ComponentId>,
}

/// UI context for UI extensions
/// 
/// **⚠️ UNSTABLE API**: UI context will be redesigned when GUI system is finalized
#[derive(Debug)]
pub struct UiContext {
    pub current_circuit: Option<String>,
    pub selection: Vec<ComponentId>,
    pub canvas_bounds: Option<crate::data::Bounds>,
}

/// UI events for extensions
/// 
/// **⚠️ UNSTABLE API**: UI event types may be expanded
#[derive(Debug, Clone)]
pub enum UiEvent {
    ComponentSelected {
        component_id: ComponentId,
    },
    CanvasClick {
        location: Location,
        button: MouseButton,
    },
    ToolChanged {
        tool_name: String,
    },
    MenuAction {
        action: String,
    },
}

/// Mouse button enumeration
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Dynamic component registration system
/// 
/// **⚠️ UNSTABLE API**: Registration system may be redesigned
pub struct ComponentRegistry {
    factories: HashMap<String, Box<dyn ComponentFactory>>,
    categories: HashMap<String, ComponentCategory>,
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            categories: HashMap::new(),
        }
    }
    
    /// Register a component type with dynamic factory
    pub fn register_component_type(
        &mut self,
        component_type: String,
        factory: Box<dyn ComponentFactory>,
        category: ComponentCategory,
    ) -> PluginResult<()> {
        if self.factories.contains_key(&component_type) {
            return Err(PluginError::ComponentTypeExists(component_type));
        }
        
        log::info!("Registered component type: {}", component_type);
        self.factories.insert(component_type.clone(), factory);
        self.categories.insert(component_type, category);
        Ok(())
    }
    
    /// Unregister a component type
    pub fn unregister_component_type(&mut self, component_type: &str) -> PluginResult<()> {
        self.factories.remove(component_type);
        self.categories.remove(component_type);
        log::info!("Unregistered component type: {}", component_type);
        Ok(())
    }
    
    /// Create component instance from registry
    pub fn create_component(
        &self,
        component_type: &str,
        id: ComponentId,
        location: Location,
    ) -> PluginResult<Box<dyn Component>> {
        let factory = self.factories.get(component_type)
            .ok_or_else(|| PluginError::PluginNotFound(component_type.to_string()))?;
        
        factory.create(id, location)
    }
    
    /// Get all registered component types
    pub fn component_types(&self) -> Vec<&String> {
        self.factories.keys().collect()
    }
    
    /// Get component category
    pub fn get_category(&self, component_type: &str) -> Option<&ComponentCategory> {
        self.categories.get(component_type)
    }
    
    /// Get components by category
    pub fn components_in_category(&self, category: &ComponentCategory) -> Vec<&String> {
        self.categories.iter()
            .filter(|(_, cat)| *cat == category)
            .map(|(name, _)| name)
            .collect()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Component category for organization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentCategory {
    Gates,
    Memory,
    IO,
    Arithmetic,
    Plexers,
    Wiring,
    Custom(String),
}

impl ComponentCategory {
    /// Get display name for category
    pub fn display_name(&self) -> &str {
        match self {
            ComponentCategory::Gates => "Logic Gates",
            ComponentCategory::Memory => "Memory",
            ComponentCategory::IO => "Input/Output",
            ComponentCategory::Arithmetic => "Arithmetic",
            ComponentCategory::Plexers => "Plexers",
            ComponentCategory::Wiring => "Wiring",
            ComponentCategory::Custom(name) => name,
        }
    }
}

/// Component information from plugin
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub name: String,
    pub category: String,
    pub description: String,
    pub icon_path: Option<String>,
    pub input_count: Option<u32>,
    pub output_count: Option<u32>,
}

/// Plugin discovery and management system with extensibility support
/// 
/// **⚠️ UNSTABLE API**: Plugin manager interface may be extended
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn PluginLibrary>>,
    search_paths: Vec<PathBuf>,
    loaded_plugins: Vec<String>,
    extension_registry: ExtensionRegistry,
    component_registry: ComponentRegistry,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            search_paths: Vec::new(),
            loaded_plugins: Vec::new(),
            extension_registry: ExtensionRegistry::new(),
            component_registry: ComponentRegistry::new(),
        }
    }

    /// Add a search path for plugins
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    /// Discover plugins in search paths
    pub fn discover_plugins(&mut self) -> PluginResult<Vec<PluginInfo>> {
        // Stub implementation - maintains API compatibility
        log::warn!("Plugin discovery not implemented in current version");

        // In full implementation, would:
        // 1. Scan search paths for plugin files (.dll, .so, .dylib, .wasm)
        // 2. Load plugin metadata without fully loading
        // 3. Resolve dependencies
        // 4. Return list of available plugins

        Err(PluginError::NotImplemented)
    }

    /// Load a specific plugin with extensibility hooks
    pub fn load_plugin(&mut self, _plugin_name: &str) -> PluginResult<()> {
        // Stub implementation - maintains API compatibility
        log::warn!("Plugin loading not implemented in current version");

        // In full implementation, would:
        // 1. Load dynamic library
        // 2. Get plugin entry point
        // 3. Initialize plugin
        // 4. Register extension hooks
        // 5. Register components

        Err(PluginError::NotImplemented)
    }

    /// Unload a plugin and cleanup extensions
    pub fn unload_plugin(&mut self, _plugin_name: &str) -> PluginResult<()> {
        // Stub implementation - maintains API compatibility
        log::warn!("Plugin unloading not implemented in current version");
        
        // In full implementation, would:
        // 1. Cleanup plugin resources
        // 2. Unregister extension hooks
        // 3. Remove components from registry
        // 4. Unload dynamic library
        
        Err(PluginError::NotImplemented)
    }

    /// Get loaded plugin
    pub fn get_plugin(&self, name: &str) -> Option<&dyn PluginLibrary> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// List all loaded plugins
    pub fn list_plugins(&self) -> Vec<&String> {
        self.plugins.keys().collect()
    }

    /// Get all available components from loaded plugins and registry
    pub fn get_all_components(&self) -> Vec<(String, ComponentInfo)> {
        let mut components = Vec::new();
        
        // Components from loaded plugins
        for (plugin_name, plugin) in &self.plugins {
            for comp in plugin.components() {
                components.push((plugin_name.clone(), comp));
            }
        }
        
        // Components from dynamic registry
        for component_type in self.component_registry.component_types() {
            if let Some(factory) = self.component_registry.factories.get(component_type) {
                let info = factory.component_info();
                components.push((format!("dynamic:{}", component_type), info));
            }
        }
        
        components
    }

    /// Create component from plugin or registry
    pub fn create_component(
        &self,
        plugin_name: &str,
        component_type: &str,
        id: ComponentId,
        location: Location,
    ) -> PluginResult<Box<dyn Component>> {
        // Try plugin first
        if let Some(plugin) = self.plugins.get(plugin_name) {
            return plugin.create_component(component_type, id);
        }
        
        // Try dynamic registry
        if plugin_name.starts_with("dynamic:") {
            let actual_type = &plugin_name[8..]; // Remove "dynamic:" prefix
            return self.component_registry.create_component(actual_type, id, location);
        }
        
        Err(PluginError::PluginNotFound(plugin_name.to_string()))
    }
    
    /// Get extension registry for advanced features
    pub fn extension_registry(&mut self) -> &mut ExtensionRegistry {
        &mut self.extension_registry
    }
    
    /// Get component registry for dynamic registration
    pub fn component_registry(&mut self) -> &mut ComponentRegistry {
        &mut self.component_registry
    }
    
    /// Register a component type dynamically
    pub fn register_component_type(
        &mut self,
        component_type: String,
        factory: Box<dyn ComponentFactory>,
        category: ComponentCategory,
    ) -> PluginResult<()> {
        self.component_registry.register_component_type(component_type, factory, category)
    }
    
    /// Process plugin events across all loaded plugins
    pub fn broadcast_plugin_event(&mut self, event: &PluginEvent) -> PluginResult<()> {
        let mut errors = Vec::new();
        
        for (name, plugin) in &mut self.plugins {
            if let Err(e) = plugin.on_plugin_event(event) {
                errors.push(format!("Plugin {}: {}", name, e));
            }
        }
        
        if !errors.is_empty() {
            return Err(PluginError::LoadingFailed(errors.join("; ")));
        }
        
        Ok(())
    }
    
    /// Get plugin system statistics
    pub fn stats(&self) -> PluginManagerStats {
        PluginManagerStats {
            loaded_plugins: self.plugins.len(),
            search_paths: self.search_paths.len(),
            registered_components: self.component_registry.factories.len(),
            extension_hooks: self.extension_registry.simulation_hooks.len(),
            circuit_observers: self.extension_registry.circuit_observers.len(),
            simulation_observers: self.extension_registry.simulation_observers.len(),
        }
    }
}

/// Plugin manager statistics
#[derive(Debug, Clone)]
pub struct PluginManagerStats {
    pub loaded_plugins: usize,
    pub search_paths: usize,
    pub registered_components: usize,
    pub extension_hooks: usize,
    pub circuit_observers: usize,
    pub simulation_observers: usize,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Custom library loader for Java compatibility
pub struct CustomLibraryLoader {
    loaded_libraries: HashMap<String, PathBuf>,
    component_registry: HashMap<String, ComponentInfo>,
}

impl CustomLibraryLoader {
    /// Create a new library loader
    pub fn new() -> Self {
        Self {
            loaded_libraries: HashMap::new(),
            component_registry: HashMap::new(),
        }
    }

    /// Load a JAR-based library (compatibility stub)
    pub fn load_jar_library(&mut self, _jar_path: PathBuf) -> PluginResult<()> {
        // Stub implementation - maintains API compatibility
        log::warn!("JAR library loading not implemented in current version");

        // In full implementation, would:
        // 1. Extract JAR contents
        // 2. Parse component definitions
        // 3. Convert to Rust plugin format
        // 4. Load as native plugin

        Err(PluginError::NotImplemented)
    }

    /// Load a native Rust library
    pub fn load_native_library(&mut self, _lib_path: PathBuf) -> PluginResult<()> {
        // Stub implementation - maintains API compatibility
        log::warn!("Native library loading not implemented in current version");
        Err(PluginError::NotImplemented)
    }

    /// Load a WebAssembly plugin
    pub fn load_wasm_plugin(&mut self, _wasm_path: PathBuf) -> PluginResult<()> {
        // Stub implementation - maintains API compatibility
        log::warn!("WebAssembly plugin loading not implemented in current version");

        // In full implementation, would:
        // 1. Load WASM module
        // 2. Instantiate WASM runtime
        // 3. Bind component interface
        // 4. Register WASM components

        Err(PluginError::NotImplemented)
    }

    /// List loaded libraries
    pub fn list_libraries(&self) -> Vec<&String> {
        self.loaded_libraries.keys().collect()
    }

    /// Get component registry
    pub fn get_components(&self) -> &HashMap<String, ComponentInfo> {
        &self.component_registry
    }
}

impl Default for CustomLibraryLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin development utilities
pub mod dev_utils {
    use super::*;

    /// Plugin template generator
    pub fn generate_plugin_template(_plugin_name: &str, _output_dir: PathBuf) -> PluginResult<()> {
        // Stub implementation - maintains API compatibility
        log::warn!("Plugin template generation not implemented in current version");
        Err(PluginError::NotImplemented)
    }

    /// Validate plugin structure
    pub fn validate_plugin(_plugin_path: PathBuf) -> PluginResult<PluginInfo> {
        // Stub implementation - maintains API compatibility
        log::warn!("Plugin validation not implemented in current version");
        Err(PluginError::NotImplemented)
    }

    /// Package plugin for distribution
    pub fn package_plugin(_plugin_dir: PathBuf, _output_path: PathBuf) -> PluginResult<()> {
        // Stub implementation - maintains API compatibility
        log::warn!("Plugin packaging not implemented in current version");
        Err(PluginError::NotImplemented)
    }
}

/// Check if plugin system is available
pub fn is_plugin_system_available() -> bool {
    // Always false in stub implementation
    log::debug!("Checking plugin system availability");
    false
}

/// Get plugin system capabilities
pub fn get_plugin_capabilities() -> PluginCapabilities {
    PluginCapabilities {
        native_plugins: false,
        jar_plugins: false,
        wasm_plugins: false,
        dynamic_loading: false,
        hot_reload: false,
    }
}

/// Plugin system capabilities
#[derive(Debug, Clone)]
pub struct PluginCapabilities {
    pub native_plugins: bool,
    pub jar_plugins: bool,
    pub wasm_plugins: bool,
    pub dynamic_loading: bool,
    pub hot_reload: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.plugins.is_empty());
        assert!(manager.search_paths.is_empty());
    }

    #[test]
    fn test_plugin_discovery_not_implemented() {
        let mut manager = PluginManager::new();
        assert!(matches!(
            manager.discover_plugins(),
            Err(PluginError::NotImplemented)
        ));
    }

    #[test]
    fn test_plugin_loading_not_implemented() {
        let mut manager = PluginManager::new();
        assert!(matches!(
            manager.load_plugin("test_plugin"),
            Err(PluginError::NotImplemented)
        ));
    }

    #[test]
    fn test_library_loader_creation() {
        let loader = CustomLibraryLoader::new();
        assert!(loader.loaded_libraries.is_empty());
        assert!(loader.component_registry.is_empty());
    }

    #[test]
    fn test_plugin_system_unavailable() {
        assert!(!is_plugin_system_available());

        let caps = get_plugin_capabilities();
        assert!(!caps.native_plugins);
        assert!(!caps.jar_plugins);
        assert!(!caps.wasm_plugins);
        assert!(!caps.dynamic_loading);
        assert!(!caps.hot_reload);
    }

    #[test]
    fn test_dev_utils_not_implemented() {
        use dev_utils::*;

        assert!(matches!(
            generate_plugin_template("test", PathBuf::from("/tmp")),
            Err(PluginError::NotImplemented)
        ));

        assert!(matches!(
            validate_plugin(PathBuf::from("/tmp/plugin")),
            Err(PluginError::NotImplemented)
        ));
    }
}
