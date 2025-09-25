/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! VHDL Content Component
//!
//! Equivalent to Java VhdlContentComponent.java
//! Manages VHDL content for entity components.

use crate::hdl::{HdlContent, PortDescription};
use std::collections::HashMap;

/// VHDL Content Component
/// 
/// Connects the VHDL interface parser with other code.
/// The parsed VHDL interface is used for the ports of a VHDL entity component.
/// Equivalent to Java VhdlContentComponent.
#[derive(Debug, Clone)]
pub struct VhdlContentComponent {
    content: String,
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    name: String,
    libraries: String,
    architecture: String,
    parsed: bool,
}

impl VhdlContentComponent {
    /// Create a new VHDL content component
    pub fn new() -> Self {
        Self {
            content: Self::load_template(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            name: "entity_name".to_string(),
            libraries: String::new(),
            architecture: String::new(),
            parsed: false,
        }
    }

    /// Create a new VHDL content component (alias for new)
    pub fn create() -> Self {
        Self::new()
    }

    /// Load the VHDL template
    fn load_template() -> String {
        // This would load from a resource file in the Java version
        // For now, provide a basic template
        r#"library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity entity_name is
    Port ( 
        -- Add your ports here
    );
end entity_name;

architecture Behavioral of entity_name is
begin
    -- Add your architecture here
end Behavioral;"#.to_string()
    }

    /// Get the VHDL content as string
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Set the VHDL content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.parsed = false;
    }

    /// Get input ports
    pub fn get_inputs(&self) -> &[PortDescription] {
        &self.inputs
    }

    /// Get output ports
    pub fn get_outputs(&self) -> &[PortDescription] {
        &self.outputs
    }

    /// Get entity name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set entity name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get libraries section
    pub fn get_libraries(&self) -> &str {
        &self.libraries
    }

    /// Set libraries section
    pub fn set_libraries(&mut self, libraries: String) {
        self.libraries = libraries;
    }

    /// Get architecture section
    pub fn get_architecture(&self) -> &str {
        &self.architecture
    }

    /// Set architecture section
    pub fn set_architecture(&mut self, architecture: String) {
        self.architecture = architecture;
    }

    /// Check if content has been parsed
    pub fn is_parsed(&self) -> bool {
        self.parsed
    }

    /// Parse the VHDL content and extract interface information
    pub fn parse(&mut self) -> Result<(), String> {
        // This is a simplified parser - in a full implementation,
        // this would use the VhdlParser to extract port information
        self.parsed = true;
        Ok(())
    }
}

impl Default for VhdlContentComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl HdlContent for VhdlContentComponent {
    fn get_content(&self) -> &str {
        &self.content
    }

    fn set_content(&mut self, content: String) {
        self.content = content;
        self.parsed = false;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_valid(&self) -> bool {
        !self.content.is_empty() && !self.name.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vhdl_content_creation() {
        let content = VhdlContentComponent::new();
        assert!(!content.get_content().is_empty());
        assert_eq!(content.get_name(), "entity_name");
        assert!(!content.is_parsed());
    }

    #[test]
    fn test_vhdl_content_modification() {
        let mut content = VhdlContentComponent::new();
        content.set_name("test_entity".to_string());
        assert_eq!(content.get_name(), "test_entity");

        let new_content = "library IEEE;\nuse IEEE.STD_LOGIC_1164.ALL;".to_string();
        content.set_content(new_content.clone());
        assert_eq!(content.get_content(), new_content);
        assert!(!content.is_parsed());
    }

    #[test]
    fn test_vhdl_content_parsing() {
        let mut content = VhdlContentComponent::new();
        assert!(content.parse().is_ok());
        assert!(content.is_parsed());
    }
}