//! Stub Plugin Example for Logisim-RUST
//!
//! This is an example plugin that demonstrates the extensibility features
//! of Logisim-RUST, including:
//! - Custom component creation
//! - Observer pattern integration  
//! - Dynamic component registration
//! - Plugin lifecycle management
//!
//! # API Stability Warning
//! 
//! **ALL APIS USED IN THIS PLUGIN ARE UNSTABLE** and may change without notice.
//! This is an example/demonstration plugin and should not be used in production.
//! Plugin developers should expect breaking changes and plan for migration.

use logisim_core::{
    PluginLibrary, PluginInfo, ComponentInfo, PluginResult, PluginError,
    PluginCapabilities, PluginConfig, DynamicComponentFactory, PluginManager,
    SystemObserver,
};
use ::std::collections::HashMap;
use log::{info, warn};

pub mod components;
pub mod observers;

pub use components::{CustomXor, CustomCounter};
pub use observers::{PluginEventLogger, ComponentStateTracker};

/// Plugin entry point - this is the main struct that implements PluginLibrary
/// 
/// # API Stability
/// This implementation uses **UNSTABLE** APIs that may change.
pub struct StubPlugin {
    info: PluginInfo,
    observers: Vec<Box<dyn SystemObserver>>,
}

impl StubPlugin {
    /// Create a new instance of the stub plugin
    pub fn new() -> Self {
        let info = PluginInfo {
            name: "Stub Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Example plugin demonstrating extensibility features".to_string(),
            author: "Logisim-RUST Contributors".to_string(),
            homepage: Some("https://github.com/crossplatformdev/Logisim-RUST".to_string()),
            dependencies: vec![],
            entry_point: "stub_plugin::create_plugin".to_string(),
        };

        Self {
            info,
            observers: vec![
                Box::new(PluginEventLogger::new()),
            ],
        }
    }
}

impl PluginLibrary for StubPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    fn components(&self) -> Vec<ComponentInfo> {
        vec![
            ComponentInfo {
                name: "CustomXOR".to_string(),
                category: "Custom Logic".to_string(),
                description: "A custom XOR gate implementation with enhanced features".to_string(),
                icon_path: None,
                input_count: Some(2),
                output_count: Some(1),
            },
            ComponentInfo {
                name: "CustomCounter".to_string(),
                category: "Custom Memory".to_string(),
                description: "A configurable counter component".to_string(),
                icon_path: None,
                input_count: Some(3), // CLK, EN, RST
                output_count: Some(2), // Q, CARRY
            },
        ]
    }

    fn create_component(
        &self,
        component_type: &str,
        id: logisim_core::ComponentId,
    ) -> PluginResult<Box<dyn logisim_core::Component>> {
        match component_type {
            "CustomXOR" => {
                info!("Creating CustomXOR component with ID: {}", id);
                Ok(Box::new(CustomXor::new(id)))
            }
            "CustomCounter" => {
                info!("Creating CustomCounter component with ID: {}", id);
                Ok(Box::new(CustomCounter::new(id, 8))) // 8-bit counter by default
            }
            _ => {
                warn!("Unknown component type requested: {}", component_type);
                Err(PluginError::PluginNotFound(format!("Component type: {}", component_type)))
            }
        }
    }

    fn initialize(&mut self) -> PluginResult<()> {
        info!("Initializing Stub Plugin v{}", self.info.version);
        
        // In a real plugin, you might:
        // - Set up internal resources
        // - Register observers
        // - Initialize external dependencies
        // - Validate configuration
        
        info!("Stub Plugin initialized successfully");
        Ok(())
    }

    fn cleanup(&mut self) -> PluginResult<()> {
        info!("Cleaning up Stub Plugin");
        
        // In a real plugin, you might:
        // - Release resources
        // - Unregister observers
        // - Save state
        // - Close external connections
        
        info!("Stub Plugin cleaned up successfully");
        Ok(())
    }

    fn api_version(&self) -> u32 {
        1 // Hard-coded for now since API_VERSION isn't exported
    }

    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities {
            native_plugins: true,
            jar_plugins: false,
            wasm_plugins: false,
            dynamic_loading: true,
            hot_reload: false,
            observer_support: true,
            custom_events: true,
            custom_rendering: false,
            ui_extensions: false,
            custom_formats: false,
        }
    }

    fn supports_hot_reload(&self) -> bool {
        false
    }

    fn prepare_for_reload(&mut self) -> PluginResult<()> {
        Err(PluginError::NotImplemented)
    }

    fn validate_config(&self, config: &PluginConfig) -> PluginResult<()> {
        info!("Validating plugin configuration");
        
        // Example validation - check for required parameters
        if let Some(required_param) = config.parameters.get("required_setting") {
            if required_param.is_empty() {
                return Err(PluginError::InvalidFormat("required_setting cannot be empty".to_string()));
            }
        }
        
        // Check resource limits are reasonable
        if config.resource_limits.max_memory > 0 && config.resource_limits.max_memory < 1024 * 1024 {
            warn!("Memory limit is very low ({}), plugin may not function correctly", 
                  config.resource_limits.max_memory);
        }
        
        info!("Plugin configuration validation passed");
        Ok(())
    }
}

/// Plugin factory functions for dynamic component creation
pub struct CustomXorFactory;

impl DynamicComponentFactory for CustomXorFactory {
    fn component_type(&self) -> &str {
        "CustomXOR"
    }
    
    fn component_info(&self) -> ComponentInfo {
        ComponentInfo {
            name: "CustomXOR".to_string(),
            category: "Custom Logic".to_string(),
            description: "A custom XOR gate with enhanced debugging features".to_string(),
            icon_path: None,
            input_count: Some(2),
            output_count: Some(1),
        }
    }
    
    fn create_component(&self, id: logisim_core::ComponentId) -> PluginResult<Box<dyn logisim_core::Component>> {
        Ok(Box::new(CustomXor::new(id)))
    }
    
    fn create_component_with_params(
        &self, 
        id: logisim_core::ComponentId, 
        params: &HashMap<String, String>
    ) -> PluginResult<Box<dyn logisim_core::Component>> {
        let mut component = CustomXor::new(id);
        
        // Configure component based on parameters
        if let Some(debug_mode) = params.get("debug_mode") {
            if debug_mode == "true" {
                component.enable_debug_mode();
            }
        }
        
        Ok(Box::new(component))
    }
    
    fn validate_parameters(&self, params: &HashMap<String, String>) -> PluginResult<()> {
        // Validate debug_mode parameter
        if let Some(debug_mode) = params.get("debug_mode") {
            if debug_mode != "true" && debug_mode != "false" {
                return Err(PluginError::InvalidFormat(
                    "debug_mode must be 'true' or 'false'".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    fn default_parameters(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("debug_mode".to_string(), "false".to_string());
        params
    }
}

pub struct CustomCounterFactory;

impl DynamicComponentFactory for CustomCounterFactory {
    fn component_type(&self) -> &str {
        "CustomCounter"
    }
    
    fn component_info(&self) -> ComponentInfo {
        ComponentInfo {
            name: "CustomCounter".to_string(),
            category: "Custom Memory".to_string(),
            description: "A configurable counter with variable bit width".to_string(),
            icon_path: None,
            input_count: Some(3), // CLK, EN, RST
            output_count: Some(2), // Q, CARRY
        }
    }
    
    fn create_component(&self, id: logisim_core::ComponentId) -> PluginResult<Box<dyn logisim_core::Component>> {
        Ok(Box::new(CustomCounter::new(id, 8))) // Default 8-bit
    }
    
    fn create_component_with_params(
        &self, 
        id: logisim_core::ComponentId, 
        params: &HashMap<String, String>
    ) -> PluginResult<Box<dyn logisim_core::Component>> {
        let bit_width = if let Some(width_str) = params.get("bit_width") {
            width_str.parse::<u32>().map_err(|_| {
                PluginError::InvalidFormat("bit_width must be a valid integer".to_string())
            })?
        } else {
            8 // Default
        };
        
        if bit_width == 0 || bit_width > 32 {
            return Err(PluginError::InvalidFormat(
                "bit_width must be between 1 and 32".to_string()
            ));
        }
        
        Ok(Box::new(CustomCounter::new(id, bit_width)))
    }
    
    fn validate_parameters(&self, params: &HashMap<String, String>) -> PluginResult<()> {
        if let Some(width_str) = params.get("bit_width") {
            let width: u32 = width_str.parse().map_err(|_| {
                PluginError::InvalidFormat("bit_width must be a valid integer".to_string())
            })?;
            
            if width == 0 || width > 32 {
                return Err(PluginError::InvalidFormat(
                    "bit_width must be between 1 and 32".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    fn default_parameters(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("bit_width".to_string(), "8".to_string());
        params
    }
}

/// Plugin entry point function that would be called by the plugin loader
/// 
/// # API Stability
/// This function signature is **UNSTABLE** and may change.
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn PluginLibrary {
    Box::into_raw(Box::new(StubPlugin::new()))
}

/// Alternative entry point for Rust-based plugin loading
/// 
/// # API Stability
/// This function is **UNSTABLE** and may change.
pub fn create_stub_plugin() -> Box<dyn PluginLibrary> {
    Box::new(StubPlugin::new())
}

/// Register the plugin's component factories with a plugin manager
/// 
/// # API Stability
/// This function is **UNSTABLE** and may change.
pub fn register_components(plugin_manager: &mut PluginManager) -> PluginResult<()> {
    info!("Registering stub plugin components");
    
    plugin_manager.register_component_factory(
        Box::new(CustomXorFactory), 
        "stub_plugin"
    )?;
    
    plugin_manager.register_component_factory(
        Box::new(CustomCounterFactory), 
        "stub_plugin"
    )?;
    
    info!("Stub plugin components registered successfully");
    Ok(())
}

/// Register the plugin's observers with a plugin manager
/// 
/// # API Stability
/// This function is **UNSTABLE** and may change.
pub fn register_observers(plugin_manager: &mut PluginManager) -> PluginResult<()> {
    info!("Registering stub plugin observers");
    
    let logger = Box::new(PluginEventLogger::new());
    plugin_manager.observer_managers_mut().system.register_observer(logger)?;
    
    let tracker = Box::new(ComponentStateTracker::new());
    plugin_manager.observer_managers_mut().component.register_observer(tracker)?;
    
    info!("Stub plugin observers registered successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use logisim_core::ComponentId;

    #[test]
    fn test_plugin_creation() {
        let plugin = StubPlugin::new();
        assert_eq!(plugin.info().name, "Stub Plugin");
        assert_eq!(plugin.info().version, "0.1.0");
    }

    #[test]
    fn test_plugin_components() {
        let plugin = StubPlugin::new();
        let components = plugin.components();
        assert_eq!(components.len(), 2);
        
        let xor_comp = &components[0];
        assert_eq!(xor_comp.name, "CustomXOR");
        assert_eq!(xor_comp.category, "Custom Logic");
    }

    #[test]
    fn test_component_creation() {
        let plugin = StubPlugin::new();
        
        // Test CustomXOR creation
        let xor_result = plugin.create_component("CustomXOR", ComponentId::new(1));
        assert!(xor_result.is_ok());
        
        // Test CustomCounter creation
        let counter_result = plugin.create_component("CustomCounter", ComponentId::new(2));
        assert!(counter_result.is_ok());
        
        // Test unknown component
        let unknown_result = plugin.create_component("UnknownComponent", ComponentId::new(3));
        assert!(unknown_result.is_err());
    }

    #[test]
    fn test_component_factories() {
        let xor_factory = CustomXorFactory;
        assert_eq!(xor_factory.component_type(), "CustomXOR");
        
        let counter_factory = CustomCounterFactory;
        assert_eq!(counter_factory.component_type(), "CustomCounter");
    }

    #[test]
    fn test_parameter_validation() {
        let counter_factory = CustomCounterFactory;
        
        // Valid parameters
        let mut valid_params = HashMap::new();
        valid_params.insert("bit_width".to_string(), "16".to_string());
        assert!(counter_factory.validate_parameters(&valid_params).is_ok());
        
        // Invalid parameters
        let mut invalid_params = HashMap::new();
        invalid_params.insert("bit_width".to_string(), "0".to_string());
        assert!(counter_factory.validate_parameters(&invalid_params).is_err());
        
        invalid_params.insert("bit_width".to_string(), "64".to_string());
        assert!(counter_factory.validate_parameters(&invalid_params).is_err());
    }
}