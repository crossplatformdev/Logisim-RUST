/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! VHDL Entity Attributes
//!
//! Equivalent to Java VhdlEntityAttributes.java
//! Provides attributes specific to VHDL entity components.

use crate::data::AttributeValue;

/// VHDL Entity Content Attribute
#[derive(Debug, Clone)]
pub struct VhdlEntityContentAttribute {
    name: String,
    display_name: String,
}

impl VhdlEntityContentAttribute {
    /// Create a new VHDL entity content attribute
    pub fn new() -> Self {
        Self {
            name: "vhdl_content".to_string(),
            display_name: "VHDL Content".to_string(),
        }
    }

    /// Get the attribute name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the display name
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Parse a value from string
    pub fn parse(&self, value: &str) -> Result<String, String> {
        Ok(value.to_string())
    }

    /// Convert value to display string
    pub fn to_display_string(&self, value: &String) -> String {
        // For display, show only the first few lines
        let lines: Vec<&str> = value.lines().take(3).collect();
        if value.lines().count() > 3 {
            format!("{}...", lines.join("\n"))
        } else {
            value.clone()
        }
    }

    /// Convert value to save string
    pub fn to_save_string(&self, value: &String) -> String {
        value.clone()
    }
}

/// VHDL Entity Name Attribute
#[derive(Debug, Clone)]
pub struct VhdlEntityNameAttribute {
    name: String,
    display_name: String,
}

impl VhdlEntityNameAttribute {
    /// Create a new VHDL entity name attribute
    pub fn new() -> Self {
        Self {
            name: "entity_name".to_string(),
            display_name: "Entity Name".to_string(),
        }
    }

    /// Get the attribute name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the display name
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Parse a value from string
    pub fn parse(&self, value: &str) -> Result<String, String> {
        if value.trim().is_empty() {
            return Err("Entity name cannot be empty".to_string());
        }
        
        // Basic VHDL identifier validation
        let trimmed = value.trim();
        if !trimmed.chars().next().unwrap_or('0').is_alphabetic() {
            return Err("Entity name must start with a letter".to_string());
        }
        
        if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Entity name can only contain letters, numbers, and underscores".to_string());
        }
        
        Ok(trimmed.to_string())
    }

    /// Convert value to display string
    pub fn to_display_string(&self, value: &String) -> String {
        value.clone()
    }

    /// Convert value to save string
    pub fn to_save_string(&self, value: &String) -> String {
        value.clone()
    }
}

/// Collection of standard VHDL entity attributes
pub struct VhdlEntityAttributes;

impl VhdlEntityAttributes {
    /// Get the VHDL content attribute
    pub fn content() -> VhdlEntityContentAttribute {
        VhdlEntityContentAttribute::new()
    }

    /// Get the entity name attribute
    pub fn entity_name() -> VhdlEntityNameAttribute {
        VhdlEntityNameAttribute::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vhdl_content_attribute() {
        let attr = VhdlEntityContentAttribute::new();
        assert_eq!(attr.name(), "vhdl_content");
        assert_eq!(attr.display_name(), "VHDL Content");
        
        let content = "library IEEE;\nuse IEEE.STD_LOGIC_1164.ALL;";
        let value = attr.parse(content).unwrap();
        assert_eq!(value, content);
    }

    #[test]
    fn test_vhdl_content_display() {
        let attr = VhdlEntityContentAttribute::new();
        let long_content = "line1\nline2\nline3\nline4\nline5";
        let display = attr.to_display_string(&long_content.to_string());
        assert!(display.contains("..."));
        assert!(display.contains("line1"));
        assert!(!display.contains("line5"));
    }

    #[test]
    fn test_entity_name_attribute() {
        let attr = VhdlEntityNameAttribute::new();
        assert_eq!(attr.name(), "entity_name");
        assert_eq!(attr.display_name(), "Entity Name");
        
        // Valid names
        assert!(attr.parse("test_entity").is_ok());
        assert!(attr.parse("Entity123").is_ok());
        assert!(attr.parse("my_component").is_ok());
        
        // Invalid names
        assert!(attr.parse("").is_err());
        assert!(attr.parse("  ").is_err());
        assert!(attr.parse("123entity").is_err());
        assert!(attr.parse("entity-name").is_err());
        assert!(attr.parse("entity name").is_err());
    }

    #[test]
    fn test_attribute_factory() {
        let content_attr = VhdlEntityAttributes::content();
        assert_eq!(content_attr.name(), "vhdl_content");
        
        let name_attr = VhdlEntityAttributes::entity_name();
        assert_eq!(name_attr.name(), "entity_name");
    }
}