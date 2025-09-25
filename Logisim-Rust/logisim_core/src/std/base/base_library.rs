/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Base Library
//!
//! Rust port of `com.cburch.logisim.std.base.BaseLibrary`

use super::*;
use crate::component::ComponentId;

/// Base Library - collection of basic tools and utilities
///
/// This library provides fundamental tools for circuit creation and editing,
/// including text annotations and basic editing tools.
pub struct BaseLibrary {
    id: String,
}

impl BaseLibrary {
    /// Unique identifier for the base library
    pub const ID: &'static str = "Base";

    /// Create a new base library
    pub fn new() -> Self {
        BaseLibrary {
            id: Self::ID.to_string(),
        }
    }

    /// Get display name for the library
    pub fn display_name() -> &'static str {
        "Base"
    }

    /// Create a text component
    pub fn create_text(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Text::new(id))
    }

    /// Get list of all available component types
    pub fn get_component_types() -> Vec<&'static str> {
        vec!["Text"]
    }

    /// Create a component by type name
    pub fn create_component_by_name(
        name: &str,
        id: ComponentId,
    ) -> Option<Box<dyn crate::component::Component>> {
        match name {
            "Text" => Some(Self::create_text(id)),
            _ => None,
        }
    }
}

impl Default for BaseLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_library_creation() {
        let library = BaseLibrary::new();
        assert_eq!(library.id, "Base");
    }

    #[test]
    fn test_text_creation() {
        let text = BaseLibrary::create_text(ComponentId(1));
        assert_eq!(text.name(), "Text");
    }

    #[test]
    fn test_component_creation_by_name() {
        let text = BaseLibrary::create_component_by_name("Text", ComponentId(1));
        assert!(text.is_some());
        assert_eq!(text.unwrap().name(), "Text");

        let invalid = BaseLibrary::create_component_by_name("Invalid", ComponentId(1));
        assert!(invalid.is_none());
    }

    #[test]
    fn test_component_types_list() {
        let types = BaseLibrary::get_component_types();
        assert!(types.contains(&"Text"));
        assert_eq!(types.len(), 1);
    }
}
