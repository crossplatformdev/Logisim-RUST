//! HDL Component Attributes
//!
//! HDL-specific attributes for components.
//! This module ports functionality from Java HDL attribute classes.

use crate::data::{Attribute, AttributeSet, AttributeValue};
use crate::hdl::content::HdlContent;
use crate::hdl::model::HdlModel;

/// HDL content attribute value
///
/// Wrapper for HDL content that implements AttributeValue.
#[derive(Debug, Clone)]
pub struct HdlContentValue {
    content_name: String,
    content_text: String,
}

impl HdlContentValue {
    /// Create new HDL content value
    pub fn new(content: Box<dyn HdlModel>) -> Self {
        Self {
            content_name: content.get_name().to_string(),
            content_text: content.get_content().to_string(),
        }
    }

    /// Get the content name
    pub fn get_content_name(&self) -> &str {
        &self.content_name
    }

    /// Get the content text
    pub fn get_content_text(&self) -> &str {
        &self.content_text
    }

    /// Set the content from HDL model
    pub fn set_content(&mut self, content: Box<dyn HdlModel>) {
        self.content_name = content.get_name().to_string();
        self.content_text = content.get_content().to_string();
    }
}

impl AttributeValue for HdlContentValue {
    fn to_display_string(&self) -> String {
        format!("{} ({})", self.content_name, self.content_text)
    }

    fn to_standard_string(&self) -> String {
        self.content_text.clone()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        Ok(Self {
            content_name: "parsed".to_string(),
            content_text: s.to_string(),
        })
    }
}

/// HDL content attribute factory
///
/// Factory for creating HDL-specific attributes.
pub struct HdlContentAttribute;

impl HdlContentAttribute {
    /// Create VHDL content attribute
    pub fn create_vhdl_content_attribute() -> Attribute<HdlContentValue> {
        Attribute::new_with_display("vhdl_content".to_string(), "VHDL Content".to_string())
    }

    /// Create BLIF content attribute
    pub fn create_blif_content_attribute() -> Attribute<HdlContentValue> {
        Attribute::new_with_display("blif_content".to_string(), "BLIF Content".to_string())
    }
}

/// VHDL entity attributes wrapper
///
/// Wrapper around AttributeSet with VHDL-specific convenience methods.
/// Equivalent to Java VhdlEntityAttributes.
#[derive(Debug)]
pub struct VhdlEntityAttributes {
    attribute_set: AttributeSet,
    content_attr: Attribute<HdlContentValue>,
}

impl VhdlEntityAttributes {
    /// Create new VHDL entity attributes
    pub fn new() -> Self {
        let mut attributes = Self {
            attribute_set: AttributeSet::new(),
            content_attr: HdlContentAttribute::create_vhdl_content_attribute(),
        };

        // Set default content
        let default_content = HdlContentValue::new(Box::new(HdlContent::new("entity".to_string())));
        let _ = attributes
            .attribute_set
            .set_value(&attributes.content_attr, default_content);

        attributes
    }

    /// Get the HDL content
    pub fn get_content(&self) -> Option<&HdlContentValue> {
        self.attribute_set.get_value(&self.content_attr)
    }

    /// Set the HDL content
    pub fn set_content(&mut self, content: HdlContentValue) -> Result<(), String> {
        self.attribute_set.set_value(&self.content_attr, content)
    }

    /// Get the underlying attribute set
    pub fn get_attribute_set(&self) -> &AttributeSet {
        &self.attribute_set
    }

    /// Get mutable reference to the underlying attribute set
    pub fn get_attribute_set_mut(&mut self) -> &mut AttributeSet {
        &mut self.attribute_set
    }
}

impl Default for VhdlEntityAttributes {
    fn default() -> Self {
        Self::new()
    }
}

/// BLIF circuit attributes wrapper
///
/// Wrapper around AttributeSet with BLIF-specific convenience methods.
/// Equivalent to Java BlifCircuitAttributes.
#[derive(Debug)]
pub struct BlifCircuitAttributes {
    attribute_set: AttributeSet,
    content_attr: Attribute<HdlContentValue>,
}

impl BlifCircuitAttributes {
    /// Create new BLIF circuit attributes
    pub fn new() -> Self {
        let mut attributes = Self {
            attribute_set: AttributeSet::new(),
            content_attr: HdlContentAttribute::create_blif_content_attribute(),
        };

        // Set default content
        let default_content =
            HdlContentValue::new(Box::new(HdlContent::new("circuit".to_string())));
        let _ = attributes
            .attribute_set
            .set_value(&attributes.content_attr, default_content);

        attributes
    }

    /// Get the HDL content
    pub fn get_content(&self) -> Option<&HdlContentValue> {
        self.attribute_set.get_value(&self.content_attr)
    }

    /// Set the HDL content
    pub fn set_content(&mut self, content: HdlContentValue) -> Result<(), String> {
        self.attribute_set.set_value(&self.content_attr, content)
    }

    /// Get the underlying attribute set
    pub fn get_attribute_set(&self) -> &AttributeSet {
        &self.attribute_set
    }

    /// Get mutable reference to the underlying attribute set
    pub fn get_attribute_set_mut(&mut self) -> &mut AttributeSet {
        &mut self.attribute_set
    }
}

impl Default for BlifCircuitAttributes {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic interface component attributes wrapper
///
/// Base attributes for generic HDL interface components.
/// Equivalent to Java GenericInterfaceComponent attributes.
#[derive(Debug)]
pub struct GenericInterfaceAttributes {
    attribute_set: AttributeSet,
    interface_type_attr: Attribute<String>,
}

impl GenericInterfaceAttributes {
    /// Create new generic interface attributes
    pub fn new(interface_type: String) -> Self {
        let mut attributes = Self {
            attribute_set: AttributeSet::new(),
            interface_type_attr: Attribute::new_with_display(
                "interface_type".to_string(),
                "Interface Type".to_string(),
            ),
        };

        let _ = attributes
            .attribute_set
            .set_value(&attributes.interface_type_attr, interface_type);
        attributes
    }

    /// Get the interface type
    pub fn get_interface_type(&self) -> Option<&String> {
        self.attribute_set.get_value(&self.interface_type_attr)
    }

    /// Set the interface type
    pub fn set_interface_type(&mut self, interface_type: String) -> Result<(), String> {
        self.attribute_set
            .set_value(&self.interface_type_attr, interface_type)
    }

    /// Get the underlying attribute set
    pub fn get_attribute_set(&self) -> &AttributeSet {
        &self.attribute_set
    }

    /// Get mutable reference to the underlying attribute set
    pub fn get_attribute_set_mut(&mut self) -> &mut AttributeSet {
        &mut self.attribute_set
    }
}

/// HDL attribute factory
///
/// Factory for creating HDL-specific attributes.
pub struct HdlAttributeFactory;

impl HdlAttributeFactory {
    /// Create entity name attribute
    pub fn create_entity_name_attribute() -> Attribute<String> {
        Attribute::new_with_display("entity_name".to_string(), "Entity Name".to_string())
    }

    /// Create model name attribute
    pub fn create_model_name_attribute() -> Attribute<String> {
        Attribute::new_with_display("model_name".to_string(), "Model Name".to_string())
    }

    /// Create architecture name attribute
    pub fn create_architecture_attribute() -> Attribute<String> {
        Attribute::new_with_display("architecture".to_string(), "Architecture".to_string())
    }

    /// Create libraries attribute
    pub fn create_libraries_attribute() -> Attribute<String> {
        Attribute::new_with_display("libraries".to_string(), "Libraries".to_string())
    }
}

/// HDL attribute constants
///
/// Common HDL attribute names and values.
pub struct HdlAttributeConstants;

impl HdlAttributeConstants {
    // Attribute names
    pub const CONTENT_ATTR: &'static str = "content";
    pub const ENTITY_NAME_ATTR: &'static str = "entity_name";
    pub const MODEL_NAME_ATTR: &'static str = "model_name";
    pub const ARCHITECTURE_ATTR: &'static str = "architecture";
    pub const LIBRARIES_ATTR: &'static str = "libraries";
    pub const INPUTS_ATTR: &'static str = "inputs";
    pub const OUTPUTS_ATTR: &'static str = "outputs";

    // Default values
    pub const DEFAULT_ENTITY_NAME: &'static str = "entity_name";
    pub const DEFAULT_MODEL_NAME: &'static str = "circuit";
    pub const DEFAULT_ARCHITECTURE: &'static str = "Behavioral";

    // HDL types
    pub const VHDL_TYPE: &'static str = "VHDL";
    pub const BLIF_TYPE: &'static str = "BLIF";
    pub const VERILOG_TYPE: &'static str = "Verilog";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdl_content_value() {
        let content = HdlContent::new("test".to_string());
        let mut value = HdlContentValue::new(Box::new(content));

        assert_eq!(value.get_content_name(), "test");

        let new_content = HdlContent::new("new_test".to_string());
        value.set_content(Box::new(new_content));
        assert_eq!(value.get_content_name(), "new_test");
    }

    #[test]
    fn test_hdl_content_value_serialization() {
        let mut content = HdlContent::new("test".to_string());
        content.set_content("entity test is end;".to_string());
        let value = HdlContentValue::new(Box::new(content));

        let serialized = value.to_standard_string();
        assert_eq!(serialized, "entity test is end;");

        let parsed = HdlContentValue::parse_from_string(&serialized).unwrap();
        assert_eq!(parsed.get_content_text(), "entity test is end;");
    }

    #[test]
    fn test_vhdl_entity_attributes() {
        let mut attrs = VhdlEntityAttributes::new();

        // Test content access
        assert!(attrs.get_content().is_some());
        assert_eq!(attrs.get_content().unwrap().get_content_name(), "entity");

        // Test content setting
        let new_content = HdlContentValue::new(Box::new(HdlContent::new("new_entity".to_string())));
        assert!(attrs.set_content(new_content).is_ok());
        assert_eq!(
            attrs.get_content().unwrap().get_content_name(),
            "new_entity"
        );
    }

    #[test]
    fn test_blif_circuit_attributes() {
        let mut attrs = BlifCircuitAttributes::new();

        assert!(attrs.get_content().is_some());
        assert_eq!(attrs.get_content().unwrap().get_content_name(), "circuit");

        let new_content =
            HdlContentValue::new(Box::new(HdlContent::new("new_circuit".to_string())));
        assert!(attrs.set_content(new_content).is_ok());
        assert_eq!(
            attrs.get_content().unwrap().get_content_name(),
            "new_circuit"
        );
    }

    #[test]
    fn test_generic_interface_attributes() {
        let mut attrs = GenericInterfaceAttributes::new("VHDL".to_string());

        assert_eq!(attrs.get_interface_type(), Some(&"VHDL".to_string()));

        assert!(attrs.set_interface_type("BLIF".to_string()).is_ok());
        assert_eq!(attrs.get_interface_type(), Some(&"BLIF".to_string()));
    }

    #[test]
    fn test_attribute_factory() {
        let entity_attr = HdlAttributeFactory::create_entity_name_attribute();
        assert_eq!(entity_attr.get_name(), "entity_name");
        assert_eq!(entity_attr.get_display_name(), "Entity Name");

        let model_attr = HdlAttributeFactory::create_model_name_attribute();
        assert_eq!(model_attr.get_name(), "model_name");
        assert_eq!(model_attr.get_display_name(), "Model Name");
    }

    #[test]
    fn test_attribute_constants() {
        assert_eq!(HdlAttributeConstants::CONTENT_ATTR, "content");
        assert_eq!(HdlAttributeConstants::DEFAULT_ENTITY_NAME, "entity_name");
        assert_eq!(HdlAttributeConstants::VHDL_TYPE, "VHDL");
    }
}
