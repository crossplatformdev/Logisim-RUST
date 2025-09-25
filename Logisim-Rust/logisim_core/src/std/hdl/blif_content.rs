/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! BLIF Content Component
//!
//! Equivalent to Java BlifContentComponent.java
//! Manages BLIF content for circuit components.

use super::HdlContent;
use crate::hdl::model::PortDescription;

/// BLIF Content Component
/// 
/// Manages BLIF circuit content and interface parsing.
/// Equivalent to Java BlifContentComponent.
#[derive(Debug, Clone)]
pub struct BlifContentComponent {
    content: String,
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    name: String,
    parsed: bool,
}

impl BlifContentComponent {
    /// Create a new BLIF content component
    pub fn new() -> Self {
        Self {
            content: Self::load_template(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            name: "circuit_name".to_string(),
            parsed: false,
        }
    }

    /// Create a new BLIF content component (alias for new)
    pub fn create() -> Self {
        Self::new()
    }

    /// Load the BLIF template
    fn load_template() -> String {
        r#".model circuit_name
.inputs 
.outputs 
.names 
.end"#.to_string()
    }

    /// Get the BLIF content as string
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Set the BLIF content
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

    /// Get circuit name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set circuit name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Check if content has been parsed
    pub fn is_parsed(&self) -> bool {
        self.parsed
    }

    /// Parse the BLIF content and extract interface information
    pub fn parse(&mut self) -> Result<(), String> {
        // This is a simplified parser - in a full implementation,
        // this would use the BlifParser to extract port information
        self.parsed = true;
        Ok(())
    }
}

impl Default for BlifContentComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl HdlContent for BlifContentComponent {
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
    fn test_blif_content_creation() {
        let content = BlifContentComponent::new();
        assert!(!content.get_content().is_empty());
        assert_eq!(content.get_name(), "circuit_name");
        assert!(!content.is_parsed());
    }

    #[test]
    fn test_blif_content_modification() {
        let mut content = BlifContentComponent::new();
        content.set_name("test_circuit".to_string());
        assert_eq!(content.get_name(), "test_circuit");

        let new_content = ".model test\n.inputs a b\n.outputs y\n.end".to_string();
        content.set_content(new_content.clone());
        assert_eq!(content.get_content(), new_content);
        assert!(!content.is_parsed());
    }

    #[test]
    fn test_blif_content_parsing() {
        let mut content = BlifContentComponent::new();
        assert!(content.parse().is_ok());
        assert!(content.is_parsed());
    }
}