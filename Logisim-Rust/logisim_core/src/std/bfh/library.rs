/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! BFH Library Implementation
//!
//! This module provides the BFH (Bern University of Applied Sciences) component library,
//! equivalent to Java's `BfhLibrary` class. It contains educational and practical
//! digital components like BCD converters and display decoders.

use crate::tools::{Library, Tool};
use super::{BinToBcd, BcdToSevenSegmentDisplay};
use std::collections::HashMap;

/// BFH Components Library
///
/// Provides access to BFH educational and practical digital components.
/// This library includes specialized components for number system conversions
/// and display interfacing.
///
/// ## Components
///
/// - **Binary to BCD Converter**: Converts binary values to BCD format
/// - **BCD to 7-Segment Display**: Decodes BCD to 7-segment display outputs
///
/// ## Example
///
/// ```rust
/// use logisim_core::std::bfh::BfhLibrary;
///
/// let library = BfhLibrary::new();
/// println!("Library: {}", library.get_display_name());
/// let tools = library.get_tools();
/// ```
#[derive(Debug, Clone)]
pub struct BfhLibrary {
    tools: Vec<Box<dyn Tool>>,
    display_name: String,
}

impl BfhLibrary {
    /// Unique identifier of the library, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    ///
    /// Identifier value MUST be unique string among all libraries.
    pub const ID: &'static str = "BFH-Praktika";

    /// Creates a new BFH library instance
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
            display_name: "BFH Mega Functions".to_string(),
        }
    }

    /// Initialize the library with all available tools
    fn init_tools(&mut self) {
        if self.tools.is_empty() {
            // Create all BFH components
            self.tools.push(Box::new(BinToBcd::new()));
            self.tools.push(Box::new(BcdToSevenSegmentDisplay::new()));
        }
    }

    /// Get tool by name for dynamic component creation
    pub fn get_tool_by_name(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.iter()
            .find(|tool| tool.get_name() == name)
            .map(|tool| tool.as_ref())
    }

    /// Get all tool names available in this library
    pub fn get_tool_names(&self) -> Vec<String> {
        self.tools.iter()
            .map(|tool| tool.get_name().to_string())
            .collect()
    }
}

impl Default for BfhLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl Library for BfhLibrary {
    fn get_display_name(&self) -> &str {
        &self.display_name
    }

    fn get_tools(&mut self) -> &[Box<dyn Tool>] {
        self.init_tools();
        &self.tools
    }

    fn get_library_id(&self) -> &str {
        Self::ID
    }

    fn get_description(&self) -> Option<String> {
        Some("BFH (Bern University of Applied Sciences) educational and practical digital components including BCD converters and display decoders.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation() {
        let library = BfhLibrary::new();
        assert_eq!(library.get_display_name(), "BFH Mega Functions");
        assert_eq!(library.get_library_id(), "BFH-Praktika");
    }

    #[test]
    fn test_library_id_constant() {
        // Ensure the ID never changes to maintain .circ file compatibility
        assert_eq!(BfhLibrary::ID, "BFH-Praktika");
    }

    #[test]
    fn test_library_description() {
        let library = BfhLibrary::new();
        let desc = library.get_description();
        assert!(desc.is_some());
        assert!(desc.unwrap().contains("BFH"));
        assert!(desc.unwrap().contains("BCD"));
    }

    #[test]
    fn test_default_implementation() {
        let library = BfhLibrary::default();
        assert_eq!(library.get_display_name(), "BFH Mega Functions");
    }

    #[test]
    fn test_tools_initialization() {
        let mut library = BfhLibrary::new();
        let tools = library.get_tools();
        assert_eq!(tools.len(), 2); // BinToBcd and BcdToSevenSegmentDisplay
    }

    #[test]
    fn test_tool_names() {
        let mut library = BfhLibrary::new();
        library.init_tools();
        let names = library.get_tool_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Bin2BCD".to_string()));
        assert!(names.contains(&"BCD2SevenSegment".to_string()));
    }

    #[test]
    fn test_get_tool_by_name() {
        let mut library = BfhLibrary::new();
        library.init_tools();
        
        let bin_to_bcd = library.get_tool_by_name("Bin2BCD");
        assert!(bin_to_bcd.is_some());
        
        let bcd_to_seven = library.get_tool_by_name("BCD2SevenSegment");
        assert!(bcd_to_seven.is_some());
        
        let nonexistent = library.get_tool_by_name("NonExistent");
        assert!(nonexistent.is_none());
    }
}