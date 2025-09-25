//! HDL Library
//!
//! HDL library containing HDL components.
//! This module ports functionality from Java HdlLibrary.

use crate::hdl::components::{BlifCircuitComponent, VhdlEntityComponent};
use crate::hdl::strings::HdlStrings;
use crate::ComponentId;

/// HDL Library
///
/// Contains the HDL-IP library with VHDL and BLIF components.
/// Equivalent to Java HdlLibrary class.
pub struct HdlLibrary {
    library_id: String,
}

impl HdlLibrary {
    /// Unique identifier of the library
    pub const LIBRARY_ID: &'static str = "HDL-IP";

    /// Create a new HDL library
    pub fn new() -> Self {
        Self {
            library_id: Self::LIBRARY_ID.to_string(),
        }
    }

    /// Get the library ID
    pub fn get_id(&self) -> &str {
        &self.library_id
    }

    /// Get the display name
    pub fn get_display_name(&self) -> String {
        HdlStrings::get("hdlLibrary")
    }

    /// Create a VHDL entity component
    pub fn create_vhdl_entity(&self, id: ComponentId) -> VhdlEntityComponent {
        VhdlEntityComponent::new(id)
    }

    /// Create a BLIF circuit component
    pub fn create_blif_circuit(&self, id: ComponentId) -> BlifCircuitComponent {
        BlifCircuitComponent::new(id)
    }

    /// Get available component types
    pub fn get_component_types(&self) -> Vec<&'static str> {
        vec!["VHDL Entity", "BLIF Circuit"]
    }

    /// Check if component type is supported
    pub fn supports_component(&self, component_type: &str) -> bool {
        matches!(component_type, "VHDL Entity" | "BLIF Circuit")
    }
}

impl Default for HdlLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// HDL library factory
pub struct HdlLibraryFactory;

impl HdlLibraryFactory {
    /// Create the HDL library instance
    pub fn create_library() -> HdlLibrary {
        HdlLibrary::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Component;

    #[test]
    fn test_hdl_library_creation() {
        let library = HdlLibrary::new();
        assert_eq!(library.get_id(), "HDL-IP");
    }

    #[test]
    fn test_component_support() {
        let library = HdlLibrary::new();
        assert!(library.supports_component("VHDL Entity"));
        assert!(library.supports_component("BLIF Circuit"));
        assert!(!library.supports_component("Unknown"));
    }

    #[test]
    fn test_component_creation() {
        let library = HdlLibrary::new();
        let vhdl_comp = library.create_vhdl_entity(ComponentId(1));
        let blif_comp = library.create_blif_circuit(ComponentId(2));

        assert_eq!(vhdl_comp.id(), ComponentId(1));
        assert_eq!(blif_comp.id(), ComponentId(2));
    }

    #[test]
    fn test_component_types() {
        let library = HdlLibrary::new();
        let types = library.get_component_types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"VHDL Entity"));
        assert!(types.contains(&"BLIF Circuit"));
    }
}
