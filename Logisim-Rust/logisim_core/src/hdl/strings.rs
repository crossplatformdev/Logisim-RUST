//! HDL Strings
//!
//! String management and internationalization for HDL module.
//! This module ports functionality from Java com.cburch.hdl.Strings.

/// HDL Strings manager
///
/// Provides localized strings for HDL functionality.
/// Equivalent to Java com.cburch.hdl.Strings class.
pub struct HdlStrings;

impl HdlStrings {
    /// Get a localized string
    ///
    /// Equivalent to Java Strings.S.get()
    pub fn get(key: &str) -> String {
        Self::get_fallback(key)
    }

    /// Get a localized string getter
    ///
    /// Equivalent to Java Strings.S.getter()
    pub fn getter(key: &str) -> impl Fn() -> String {
        let key_owned = key.to_string();
        move || Self::get(&key_owned)
    }

    /// Get fallback string for key
    fn get_fallback(key: &str) -> String {
        match key {
            // File I/O error messages
            "hdlFileReaderError" => "Error reading HDL file".to_string(),
            "hdlFileWriterError" => "Error writing HDL file".to_string(),

            // Parser error messages
            "vhdlParseError" => "VHDL parsing error".to_string(),
            "blifParseError" => "BLIF parsing error".to_string(),
            "invalidHdlSyntax" => "Invalid HDL syntax".to_string(),
            "missingEntity" => "Missing entity declaration".to_string(),
            "missingModel" => "Missing model declaration".to_string(),

            // Component labels
            "vhdlComponent" => "VHDL Entity".to_string(),
            "blifComponent" => "BLIF Circuit".to_string(),
            "hdlLibrary" => "HDL-IP".to_string(),

            // Port types
            "inputPort" => "Input".to_string(),
            "outputPort" => "Output".to_string(),
            "inoutPort" => "Inout".to_string(),

            // HDL types
            "stdLogic" => "std_logic".to_string(),
            "stdLogicVector" => "std_logic_vector".to_string(),
            "logic" => "logic".to_string(),

            // Editor messages
            "contentModified" => "Content has been modified".to_string(),
            "saveChanges" => "Save changes?".to_string(),
            "discardChanges" => "Discard changes?".to_string(),

            // Template messages
            "defaultEntityName" => "entity_name".to_string(),
            "defaultModelName" => "circuit".to_string(),

            // Validation messages
            "invalidPortName" => "Invalid port name".to_string(),
            "duplicatePortName" => "Duplicate port name".to_string(),
            "invalidEntityName" => "Invalid entity name".to_string(),

            // Generation messages
            "generatingVhdl" => "Generating VHDL code".to_string(),
            "generatingBlif" => "Generating BLIF code".to_string(),
            "generationComplete" => "Code generation complete".to_string(),
            "generationFailed" => "Code generation failed".to_string(),

            // Architecture messages
            "behavioral" => "Behavioral".to_string(),
            "structural" => "Structural".to_string(),
            "dataflow" => "Dataflow".to_string(),

            // Library messages
            "ieee" => "IEEE".to_string(),
            "stdLogic1164" => "STD_LOGIC_1164".to_string(),
            "numericStd" => "NUMERIC_STD".to_string(),

            _ => format!("Missing translation: {}", key),
        }
    }
}

/// Convenience function to get localized string
///
/// Equivalent to direct usage of Java Strings.S.get()
pub fn get_string(key: &str) -> String {
    HdlStrings::get(key)
}

/// Convenience macro for getting localized strings
///
/// Usage: hdl_string!("key") -> String
#[macro_export]
macro_rules! hdl_string {
    ($key:expr) => {
        $crate::hdl::strings::get_string($key)
    };
}

/// HDL string constants
///
/// Provides commonly used HDL-related strings as constants.
pub struct HdlStringConstants;

impl HdlStringConstants {
    // File extensions
    pub const VHDL_EXTENSION: &'static str = "vhdl";
    pub const VHD_EXTENSION: &'static str = "vhd";
    pub const BLIF_EXTENSION: &'static str = "blif";
    pub const VERILOG_EXTENSION: &'static str = "v";
    pub const SYSTEM_VERILOG_EXTENSION: &'static str = "sv";

    // MIME types
    pub const VHDL_MIME_TYPE: &'static str = "text/x-vhdl";
    pub const BLIF_MIME_TYPE: &'static str = "text/x-blif";
    pub const VERILOG_MIME_TYPE: &'static str = "text/x-verilog";

    // HDL keywords
    pub const VHDL_ENTITY: &'static str = "entity";
    pub const VHDL_ARCHITECTURE: &'static str = "architecture";
    pub const VHDL_PORT: &'static str = "port";
    pub const VHDL_IS: &'static str = "is";
    pub const VHDL_END: &'static str = "end";
    pub const VHDL_IN: &'static str = "in";
    pub const VHDL_OUT: &'static str = "out";
    pub const VHDL_INOUT: &'static str = "inout";

    pub const BLIF_MODEL: &'static str = ".model";
    pub const BLIF_INPUTS: &'static str = ".inputs";
    pub const BLIF_OUTPUTS: &'static str = ".outputs";
    pub const BLIF_NAMES: &'static str = ".names";
    pub const BLIF_LATCH: &'static str = ".latch";
    pub const BLIF_END: &'static str = ".end";

    // Standard types
    pub const STD_LOGIC: &'static str = "std_logic";
    pub const STD_LOGIC_VECTOR: &'static str = "std_logic_vector";
    pub const LOGIC: &'static str = "logic";

    // Libraries
    pub const IEEE_LIBRARY: &'static str = "IEEE";
    pub const STD_LOGIC_1164: &'static str = "STD_LOGIC_1164";
    pub const NUMERIC_STD: &'static str = "NUMERIC_STD";

    // Template placeholders
    pub const ENTITY_NAME_PLACEHOLDER: &'static str = "entity_name";
    pub const MODEL_NAME_PLACEHOLDER: &'static str = "circuit";
    pub const ARCH_NAME_PLACEHOLDER: &'static str = "Behavioral";
}

/// HDL format detection utilities
pub struct HdlFormatUtil;

impl HdlFormatUtil {
    /// Check if content appears to be VHDL
    pub fn is_vhdl_content(content: &str) -> bool {
        let content_lower = content.to_lowercase();
        content_lower.contains("entity")
            && (content_lower.contains("architecture") || content_lower.contains("port"))
    }

    /// Check if content appears to be BLIF
    pub fn is_blif_content(content: &str) -> bool {
        let content_lower = content.to_lowercase();
        content_lower.contains(".model")
            && (content_lower.contains(".inputs") || content_lower.contains(".outputs"))
    }

    /// Check if content appears to be Verilog
    pub fn is_verilog_content(content: &str) -> bool {
        let content_lower = content.to_lowercase();
        content_lower.contains("module")
            && (content_lower.contains("input") || content_lower.contains("output"))
    }

    /// Detect HDL format from content
    pub fn detect_format(content: &str) -> Option<&'static str> {
        if Self::is_vhdl_content(content) {
            Some("VHDL")
        } else if Self::is_blif_content(content) {
            Some("BLIF")
        } else if Self::is_verilog_content(content) {
            Some("Verilog")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_constants() {
        assert_eq!(HdlStringConstants::VHDL_EXTENSION, "vhdl");
        assert_eq!(HdlStringConstants::BLIF_EXTENSION, "blif");
        assert_eq!(HdlStringConstants::STD_LOGIC, "std_logic");
    }

    #[test]
    fn test_fallback_strings() {
        let error_msg = HdlStrings::get("hdlFileReaderError");
        assert!(error_msg.contains("Error reading"));

        let component_name = HdlStrings::get("vhdlComponent");
        assert_eq!(component_name, "VHDL Entity");
    }

    #[test]
    fn test_string_getter() {
        let getter = HdlStrings::getter("hdlLibrary");
        assert_eq!(getter(), "HDL-IP");
    }

    #[test]
    fn test_missing_key() {
        let missing = HdlStrings::get("nonExistentKey");
        assert!(missing.contains("Missing translation"));
    }

    #[test]
    fn test_format_detection() {
        let vhdl_content = "entity test is port(clk: in std_logic); end test;";
        assert!(HdlFormatUtil::is_vhdl_content(vhdl_content));
        assert_eq!(HdlFormatUtil::detect_format(vhdl_content), Some("VHDL"));

        let blif_content = ".model test\n.inputs a b\n.outputs y\n.end";
        assert!(HdlFormatUtil::is_blif_content(blif_content));
        assert_eq!(HdlFormatUtil::detect_format(blif_content), Some("BLIF"));

        let verilog_content = "module test(input clk, output reg q);";
        assert!(HdlFormatUtil::is_verilog_content(verilog_content));
        assert_eq!(
            HdlFormatUtil::detect_format(verilog_content),
            Some("Verilog")
        );

        let unknown_content = "This is just plain text";
        assert_eq!(HdlFormatUtil::detect_format(unknown_content), None);
    }

    #[test]
    fn test_convenience_function() {
        let result = get_string("hdlLibrary");
        assert_eq!(result, "HDL-IP");
    }
}
