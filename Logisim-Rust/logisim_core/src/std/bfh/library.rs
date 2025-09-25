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

use crate::component::ComponentId;
use super::{BinToBcd, BcdToSevenSegmentDisplay};

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
#[derive(Debug, Clone)]
pub struct BfhLibrary {
    id: String,
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
            id: Self::ID.to_string(),
        }
    }

    /// Get display name for the library
    pub fn display_name() -> &'static str {
        "BFH Mega Functions"
    }

    /// Create a Binary to BCD converter
    pub fn create_bin_to_bcd(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(BinToBcd::new(id))
    }

    /// Create a BCD to 7-Segment Display decoder
    pub fn create_bcd_to_seven_segment(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(BcdToSevenSegmentDisplay::new(id))
    }
}

impl Default for BfhLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation() {
        let library = BfhLibrary::new();
        assert_eq!(library.id, "BFH-Praktika");
    }

    #[test]
    fn test_library_id_constant() {
        // Ensure the ID never changes to maintain .circ file compatibility
        assert_eq!(BfhLibrary::ID, "BFH-Praktika");
    }

    #[test]
    fn test_display_name() {
        assert_eq!(BfhLibrary::display_name(), "BFH Mega Functions");
    }

    #[test]
    fn test_default_implementation() {
        let library = BfhLibrary::default();
        assert_eq!(library.id, "BFH-Praktika");
    }

    #[test]
    fn test_component_creation() {
        let bin_to_bcd = BfhLibrary::create_bin_to_bcd(ComponentId(1));
        assert!(bin_to_bcd.id() == ComponentId(1));

        let bcd_to_seven = BfhLibrary::create_bcd_to_seven_segment(ComponentId(2));
        assert!(bcd_to_seven.id() == ComponentId(2));
    }
}