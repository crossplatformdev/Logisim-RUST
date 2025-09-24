/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Wiring library containing all wiring-related components
//!
//! This module provides the WiringLibrary which registers and manages
//! all wiring components like pins, tunnels, splitters, clocks, etc.

use crate::ComponentId;
use std::collections::HashMap;

/// Unique identifier for the wiring library
/// Do NOT change as it will prevent project files from loading.
pub const WIRING_LIBRARY_ID: &str = "Wiring";

/// Component factory trait for creating wiring components
pub trait WiringComponentFactory: Send + Sync {
    /// Get the unique identifier for this component type
    fn id(&self) -> &'static str;
    
    /// Get the display name for this component
    fn display_name(&self) -> &str;
    
    /// Get the description of this component
    fn description(&self) -> &str;
    
    /// Get the icon path for this component (relative to resources)
    fn icon_path(&self) -> Option<&str>;
    
    /// Create a new instance of this component with the given ID
    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component>;
}

/// Information about a wiring component type
#[derive(Debug, Clone)]
pub struct WiringComponentInfo {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub icon_path: Option<String>,
    pub category: String,
}

/// The wiring library that contains all wiring components
pub struct WiringLibrary {
    factories: HashMap<String, Box<dyn WiringComponentFactory>>,
    component_infos: Vec<WiringComponentInfo>,
}

impl WiringLibrary {
    /// Create a new wiring library with all standard wiring components
    pub fn new() -> Self {
        let mut library = Self {
            factories: HashMap::new(),
            component_infos: Vec::new(),
        };
        
        // Register all wiring components
        library.register_components();
        library
    }
    
    /// Get the unique identifier for this library
    pub fn id(&self) -> &'static str {
        WIRING_LIBRARY_ID
    }
    
    /// Get the display name for this library
    pub fn display_name(&self) -> &str {
        "Wiring"
    }
    
    /// Get all component information
    pub fn get_component_infos(&self) -> &[WiringComponentInfo] {
        &self.component_infos
    }
    
    /// Create a component by its ID
    pub fn create_component(&self, component_id: &str, instance_id: ComponentId) -> Option<Box<dyn crate::Component>> {
        self.factories.get(component_id)
            .map(|factory| factory.create_component(instance_id))
    }
    
    /// Register a component factory
    fn register_factory(&mut self, factory: Box<dyn WiringComponentFactory>) {
        let info = WiringComponentInfo {
            id: factory.id().to_string(),
            display_name: factory.display_name().to_string(),
            description: factory.description().to_string(),
            icon_path: factory.icon_path().map(|s| s.to_string()),
            category: "Wiring".to_string(),
        };
        
        self.component_infos.push(info);
        self.factories.insert(factory.id().to_string(), factory);
    }
    
    /// Register all standard wiring components
    fn register_components(&mut self) {
        // Register Pin component
        self.register_factory(Box::new(crate::std::wiring::pin::PinFactory));
        
        // Register Constant component
        self.register_factory(Box::new(crate::std::wiring::constant::ConstantFactory));
        
        // Register Ground component
        self.register_factory(Box::new(crate::std::wiring::ground::GroundFactory));
        
        // Register Power component
        self.register_factory(Box::new(crate::std::wiring::power::PowerFactory));
        
        // TODO: Register other wiring components as they are implemented
    }
}

impl Default for WiringLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiring_library_creation() {
        let library = WiringLibrary::new();
        assert_eq!(library.id(), WIRING_LIBRARY_ID);
        assert_eq!(library.display_name(), "Wiring");
    }

    #[test]
    fn test_wiring_library_constants() {
        assert_eq!(WIRING_LIBRARY_ID, "Wiring");
    }
    
    #[test]
    fn test_wiring_library_components() {
        let library = WiringLibrary::new();
        let infos = library.get_component_infos();
        
        // Should have at least 4 components registered
        assert!(infos.len() >= 4);
        
        // Check that all expected components are registered
        let component_ids: Vec<&str> = infos.iter().map(|info| info.id.as_str()).collect();
        assert!(component_ids.contains(&"Pin"));
        assert!(component_ids.contains(&"Constant"));
        assert!(component_ids.contains(&"Ground"));
        assert!(component_ids.contains(&"Power"));
    }
    
    #[test]
    fn test_component_creation() {
        let library = WiringLibrary::new();
        
        // Test creating each component type
        let pin = library.create_component("Pin", ComponentId(1));
        assert!(pin.is_some());
        assert_eq!(pin.unwrap().name(), "Pin");
        
        let constant = library.create_component("Constant", ComponentId(2));
        assert!(constant.is_some());
        assert_eq!(constant.unwrap().name(), "Constant");
        
        let ground = library.create_component("Ground", ComponentId(3));
        assert!(ground.is_some());
        assert_eq!(ground.unwrap().name(), "Ground");
        
        let power = library.create_component("Power", ComponentId(4));
        assert!(power.is_some());
        assert_eq!(power.unwrap().name(), "Power");
        
        // Test invalid component ID
        let invalid = library.create_component("NonExistent", ComponentId(5));
        assert!(invalid.is_none());
    }
}