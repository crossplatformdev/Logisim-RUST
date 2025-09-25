/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logsim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! I/O Components Library
//!
//! Provides the main library class that aggregates all I/O components
//! and makes them available to the Logisim system.

use crate::{
    instance::InstanceFactory,
    std::io::{Button, Led},
    tools::Library,
};

/// The I/O Components Library
/// 
/// This library provides input/output components for user interaction
/// and external interfacing, equivalent to the Java IoLibrary class.
#[derive(Debug)]
pub struct IoLibrary {
    components: Vec<Box<dyn InstanceFactory>>,
}

impl IoLibrary {
    /// Unique identifier of the library, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "I/O";
    
    pub fn new() -> Self {
        let mut components: Vec<Box<dyn InstanceFactory>> = Vec::new();
        
        // Add basic I/O components
        components.push(Box::new(Button::new()));
        components.push(Box::new(Led::new()));
        
        // TODO: Add more components as they are implemented
        // components.push(Box::new(DipSwitch::new()));
        // components.push(Box::new(SevenSegment::new()));
        // etc.
        
        Self { components }
    }
    
    pub fn get_components(&self) -> &[Box<dyn InstanceFactory>] {
        &self.components
    }
}

impl Default for IoLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl Library for IoLibrary {
    fn get_name(&self) -> &str {
        Self::ID
    }
    
    fn get_display_name(&self) -> String {
        "I/O".to_string() // TODO: Use localization S.get("ioLibrary")
    }
    
    fn contains(&self, factory: &dyn InstanceFactory) -> bool {
        // Check if this library contains the given factory
        let factory_name = factory.get_name();
        
        for component in &self.components {
            if component.get_name() == factory_name {
                return true;
            }
        }
        
        false
    }
}