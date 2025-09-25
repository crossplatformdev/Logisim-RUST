/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Memory component library
//!
//! This module implements the memory component library equivalent to MemoryLibrary.java.
//! It contains all memory-related components like RAM, ROM, registers, flip-flops, etc.

use crate::ComponentId;
use crate::instance::InstanceFactory;
use std::collections::HashMap;

/// Unique identifier for the memory library
/// Do NOT change as it will prevent project files from loading.
pub const MEMORY_LIBRARY_ID: &str = "Memory";

/// Memory component factory trait
pub trait MemoryComponentFactory: Send + Sync {
    /// Create a memory component with the given ID
    fn create_component(&self, component_id: ComponentId) -> Option<Box<dyn InstanceFactory>>;
    
    /// Get the component type identifier
    fn get_id(&self) -> &str;
    
    /// Get the display name
    fn get_display_name(&self) -> String;
    
    /// Get the icon path (optional)
    fn get_icon_path(&self) -> Option<&str> { None }
}

/// Information about a memory component type
#[derive(Debug, Clone)]
pub struct MemoryComponentInfo {
    pub id: String,
    pub display_name: String,
    pub icon_path: Option<String>,
    pub description: String,
}

impl MemoryComponentInfo {
    pub fn new(id: String, display_name: String, icon_path: Option<String>, description: String) -> Self {
        Self {
            id,
            display_name,
            icon_path,
            description,
        }
    }
}

/// The memory library containing all memory components
pub struct MemoryLibrary {
    factories: HashMap<String, Box<dyn MemoryComponentFactory>>,
    component_infos: Vec<MemoryComponentInfo>,
}

impl MemoryLibrary {
    /// Create a new memory library with all standard memory components
    pub fn new() -> Self {
        let mut library = Self {
            factories: HashMap::new(),
            component_infos: Vec::new(),
        };
        
        // Register all memory components
        library.register_components();
        library
    }

    /// Get the unique identifier for this library
    pub fn id(&self) -> &'static str {
        MEMORY_LIBRARY_ID
    }

    /// Get the display name for this library
    pub fn display_name(&self) -> String {
        "Memory".to_string() // TODO: Implement proper localization
    }

    /// Get all component information
    pub fn get_component_infos(&self) -> &[MemoryComponentInfo] {
        &self.component_infos
    }

    /// Register a component factory
    pub fn register_factory(&mut self, factory: Box<dyn MemoryComponentFactory>) {
        let id = factory.get_id().to_string();
        let display_name = factory.get_display_name();
        let icon_path = factory.get_icon_path().map(|s| s.to_string());
        
        let info = MemoryComponentInfo::new(
            id.clone(),
            display_name.clone(),
            icon_path,
            display_name, // Use display name as description for now
        );
        
        self.component_infos.push(info);
        self.factories.insert(id, factory);
    }

    /// Create a component by ID
    pub fn create_component(&self, component_id: &str, id: ComponentId) -> Option<Box<dyn InstanceFactory>> {
        self.factories.get(component_id)?.create_component(id)
    }

    /// Register all standard memory components
    fn register_components(&mut self) {
        // TODO: Register memory components as they are implemented
        // For now, register placeholder factories
        
        // Register D Flip-Flop
        // self.register_factory(Box::new(DFlipFlopFactory));
        
        // Register T Flip-Flop  
        // self.register_factory(Box::new(TFlipFlopFactory));
        
        // Register JK Flip-Flop
        // self.register_factory(Box::new(JKFlipFlopFactory));
        
        // Register SR Flip-Flop
        // self.register_factory(Box::new(SRFlipFlopFactory));
        
        // Register Register
        // self.register_factory(Box::new(RegisterFactory));
        
        // Register Counter
        // self.register_factory(Box::new(CounterFactory));
        
        // Register Shift Register
        // self.register_factory(Box::new(ShiftRegisterFactory));
        
        // Register Random
        // self.register_factory(Box::new(RandomFactory));
        
        // Register RAM
        // self.register_factory(Box::new(RamFactory));
        
        // Register ROM
        // self.register_factory(Box::new(RomFactory));
    }
}

impl Default for MemoryLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMemoryFactory {
        id: String,
        display_name: String,
    }

    impl TestMemoryFactory {
        fn new(id: &str, display_name: &str) -> Self {
            Self {
                id: id.to_string(),
                display_name: display_name.to_string(),
            }
        }
    }

    impl MemoryComponentFactory for TestMemoryFactory {
        fn create_component(&self, _component_id: ComponentId) -> Option<Box<dyn InstanceFactory>> {
            None // Test factory doesn't create real components
        }

        fn get_id(&self) -> &str {
            &self.id
        }

        fn get_display_name(&self) -> String {
            self.display_name.clone()
        }

        fn get_icon_path(&self) -> Option<&str> {
            Some("test.gif")
        }
    }

    #[test]
    fn test_memory_library_creation() {
        let library = MemoryLibrary::new();
        assert_eq!(library.id(), MEMORY_LIBRARY_ID);
        assert_eq!(library.display_name(), "Memory");
    }

    #[test]
    fn test_memory_library_constants() {
        assert_eq!(MEMORY_LIBRARY_ID, "Memory");
    }

    #[test]
    fn test_memory_component_info() {
        let info = MemoryComponentInfo::new(
            "TestComponent".to_string(),
            "Test Component".to_string(),
            Some("test.gif".to_string()),
            "A test component".to_string(),
        );

        assert_eq!(info.id, "TestComponent");
        assert_eq!(info.display_name, "Test Component");
        assert_eq!(info.icon_path, Some("test.gif".to_string()));
        assert_eq!(info.description, "A test component");
    }

    #[test]
    fn test_memory_library_factory_registration() {
        let mut library = MemoryLibrary::new();
        let factory = Box::new(TestMemoryFactory::new("TestMem", "Test Memory"));

        library.register_factory(factory);

        let infos = library.get_component_infos();
        assert!(!infos.is_empty());
        
        let test_info = infos.iter().find(|info| info.id == "TestMem");
        assert!(test_info.is_some());
        assert_eq!(test_info.unwrap().display_name, "Test Memory");
    }

    #[test]
    fn test_memory_library_as_library_trait() {
        let library = MemoryLibrary::new();
        
        // Test basic library functionality
        assert_eq!(library.id(), MEMORY_LIBRARY_ID);
        assert_eq!(library.display_name(), "Memory");
    }
}