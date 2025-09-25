/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Arithmetic Library
//!
//! Rust port of `com.cburch.logisim.std.arith.ArithmeticLibrary`

use crate::component::ComponentId;
use super::*;

/// Arithmetic Library - collection of all arithmetic components
/// 
/// This library provides access to all arithmetic and mathematical components
/// including basic integer arithmetic, floating-point operations, bit manipulation,
/// and comparison operations.
pub struct ArithmeticLibrary {
    id: String,
}

impl ArithmeticLibrary {
    /// Unique identifier for the arithmetic library
    /// Must match Java implementation for .circ file compatibility
    pub const ID: &'static str = "Arithmetic";
    
    /// Create a new arithmetic library
    pub fn new() -> Self {
        ArithmeticLibrary {
            id: Self::ID.to_string(),
        }
    }
    
    /// Get display name for the library
    pub fn display_name() -> &'static str {
        "Arithmetic"
    }
    
    /// Create an adder component
    pub fn create_adder(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Adder::new(id))
    }
    
    /// Create a subtractor component
    pub fn create_subtractor(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Subtractor::new(id))
    }
    
    /// Create a multiplier component
    pub fn create_multiplier(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Multiplier::new(id))
    }
    
    /// Create a divider component
    pub fn create_divider(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Divider::new(id))
    }
    
    /// Create a negator component
    pub fn create_negator(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Negator::new(id))
    }
    
    /// Create a comparator component
    pub fn create_comparator(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Comparator::new(id))
    }
    
    /// Create a shifter component
    pub fn create_shifter(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(Shifter::new(id))
    }
    
    /// Create a bit adder component
    pub fn create_bit_adder(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(BitAdder::new(id))
    }
    
    /// Create a bit finder component
    pub fn create_bit_finder(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(BitFinder::new(id))
    }
    
    /// Create a floating-point adder component
    pub fn create_fp_adder(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(FpAdder::new(id))
    }
    
    /// Create a floating-point subtractor component
    pub fn create_fp_subtractor(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(FpSubtractor::new(id))
    }
    
    /// Create a floating-point multiplier component
    pub fn create_fp_multiplier(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(FpMultiplier::new(id))
    }
    
    /// Create a floating-point divider component
    pub fn create_fp_divider(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(FpDivider::new(id))
    }
    
    /// Create a floating-point negator component
    pub fn create_fp_negator(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(FpNegator::new(id))
    }
    
    /// Create a floating-point comparator component
    pub fn create_fp_comparator(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(FpComparator::new(id))
    }
    
    /// Create a floating-point to integer converter component
    pub fn create_fp_to_int(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(FpToInt::new(id))
    }
    
    /// Create an integer to floating-point converter component
    pub fn create_int_to_fp(id: ComponentId) -> Box<dyn crate::component::Component> {
        Box::new(IntToFp::new(id))
    }
    
    /// Get all available component types in this library
    pub fn component_types() -> Vec<&'static str> {
        vec![
            "Adder",
            "Subtractor", 
            "Multiplier",
            "Divider",
            "Negator",
            "Comparator",
            "Shifter",
            "BitAdder",
            "BitFinder",
            "FpAdder",
            "FpSubtractor",
            "FpMultiplier",
            "FpDivider",
            "FpNegator",
            "FpComparator",
            "FpToInt",
            "IntToFp",
        ]
    }
    
    /// Create component by type name
    pub fn create_component(
        component_type: &str, 
        id: ComponentId
    ) -> Option<Box<dyn crate::component::Component>> {
        match component_type {
            "Adder" => Some(Self::create_adder(id)),
            "Subtractor" => Some(Self::create_subtractor(id)),
            "Multiplier" => Some(Self::create_multiplier(id)),
            "Divider" => Some(Self::create_divider(id)),
            "Negator" => Some(Self::create_negator(id)),
            "Comparator" => Some(Self::create_comparator(id)),
            "Shifter" => Some(Self::create_shifter(id)),
            "BitAdder" => Some(Self::create_bit_adder(id)),
            "BitFinder" => Some(Self::create_bit_finder(id)),
            "FpAdder" => Some(Self::create_fp_adder(id)),
            "FpSubtractor" => Some(Self::create_fp_subtractor(id)),
            "FpMultiplier" => Some(Self::create_fp_multiplier(id)),
            "FpDivider" => Some(Self::create_fp_divider(id)),
            "FpNegator" => Some(Self::create_fp_negator(id)),
            "FpComparator" => Some(Self::create_fp_comparator(id)),
            "FpToInt" => Some(Self::create_fp_to_int(id)),
            "IntToFp" => Some(Self::create_int_to_fp(id)),
            _ => None,
        }
    }
}

impl Default for ArithmeticLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation() {
        let lib = ArithmeticLibrary::new();
        assert_eq!(lib.id, "Arithmetic");
    }

    #[test]
    fn test_display_name() {
        assert_eq!(ArithmeticLibrary::display_name(), "Arithmetic");
    }

    #[test]
    fn test_component_types() {
        let types = ArithmeticLibrary::component_types();
        assert_eq!(types.len(), 17);
        assert!(types.contains(&"Adder"));
        assert!(types.contains(&"FpAdder"));
        assert!(types.contains(&"BitFinder"));
    }

    #[test]
    fn test_component_creation_by_name() {
        let component = ArithmeticLibrary::create_component("Adder", ComponentId(1));
        assert!(component.is_some());
        
        let component = ArithmeticLibrary::create_component("NonExistent", ComponentId(1));
        assert!(component.is_none());
    }
}