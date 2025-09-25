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
    comp::ComponentFactory,
    tools::{Library, Tool},
};
use std::any::Any;

/// The I/O Components Library
/// 
/// This library provides input/output components for user interaction
/// and external interfacing, equivalent to the Java IoLibrary class.
pub struct IoLibrary {
    // TODO: Add components when ComponentFactory integration is complete
}

impl IoLibrary {
    /// Unique identifier of the library, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "I/O";
    
    pub fn new() -> Self {
        Self {
            // TODO: Add basic I/O components
            // components.push(Box::new(Button::new()));
            // components.push(Box::new(Led::new()));
        }
    }
}

impl Default for IoLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl Library for IoLibrary {
    fn get_name(&self) -> String {
        Self::ID.to_string()
    }
    
    fn get_display_name(&self) -> String {
        "I/O".to_string() // TODO: Use localization S.get("ioLibrary")
    }
    
    fn contains(&self, _factory: &dyn ComponentFactory) -> bool {
        // TODO: Check if this library contains the given factory
        false
    }
    
    fn get_tools(&self) -> Vec<Box<dyn Tool>> {
        // TODO: Convert components to tools
        Vec::new()
    }
    
    fn set_hidden(&mut self) {
        // TODO: Implement library hiding functionality
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}