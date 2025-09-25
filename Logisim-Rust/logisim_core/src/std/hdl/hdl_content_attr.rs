/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL Content Attribute
//!
//! Equivalent to Java HdlContentAttribute.java
//! Provides attribute support for HDL content components.

use crate::data::{Attribute, AttributeValue};
use std::fmt;

/// HDL Content Attribute
/// 
/// Attribute for managing HDL content in components.
/// Equivalent to Java HdlContentAttribute.
#[derive(Debug, Clone)]
pub struct HdlContentAttribute {
    name: String,
    display_name: String,
}

impl HdlContentAttribute {
    /// Create a new HDL content attribute
    pub fn new(name: &str, display_name: &str) -> Self {
        Self {
            name: name.to_string(),
            display_name: display_name.to_string(),
        }
    }

    /// Get the attribute name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the display name
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    /// Parse a value from string
    pub fn parse(&self, value: &str) -> Result<String, String> {
        Ok(value.to_string())
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

impl fmt::Display for HdlContentAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

/// Attribute value for HDL content
#[derive(Debug, Clone)]
pub struct HdlContentValue(pub String);

impl AttributeValue for HdlContentValue {
    fn to_display_string(&self) -> String {
        self.0.clone()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        Ok(HdlContentValue(s.to_string()))
    }
}

impl fmt::Display for HdlContentValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdl_content_attribute() {
        let attr = HdlContentAttribute::new("hdl_content", "HDL Content");
        assert_eq!(attr.get_name(), "hdl_content");
        assert_eq!(attr.get_display_name(), "HDL Content");
    }

    #[test]
    fn test_hdl_content_attribute_parsing() {
        let attr = HdlContentAttribute::new("hdl_content", "HDL Content");
        let content = "library IEEE;\nuse IEEE.STD_LOGIC_1164.ALL;";
        
        let parsed = attr.parse(content).unwrap();
        assert_eq!(parsed, content);
        
        let display = attr.to_display_string(&parsed);
        assert_eq!(display, content);
    }

    #[test]
    fn test_hdl_content_value() {
        let value = HdlContentValue("test content".to_string());
        assert_eq!(value.to_string(), "test content");
        
        let from_str = HdlContentValue::from_string("another test").unwrap();
        assert_eq!(from_str.0, "another test");
    }
}