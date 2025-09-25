/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Extra IO Library
//!
//! Rust port of `com.cburch.logisim.std.io.extra.ExtraIoLibrary`
//!
//! Library containing specialized I/O components for user interaction.

use crate::comp::ComponentId;
use super::Switch;

/// Extra IO Library - collection of specialized I/O components
/// 
/// This library provides access to interactive components for user interface,
/// including switches, sliders, audio output, and signal visualization.
pub struct ExtraIoLibrary {
    id: String,
}

impl ExtraIoLibrary {
    /// Unique identifier for the Extra IO library
    /// 
    /// IMPORTANT: This ID must match the Java implementation exactly
    /// to maintain compatibility with existing circuit files.
    pub const ID: &'static str = "Input/Output-Extra";
    
    /// Create a new Extra IO library
    pub fn new() -> Self {
        ExtraIoLibrary {
            id: Self::ID.to_string(),
        }
    }
    
    /// Get display name for the library
    pub fn display_name() -> &'static str {
        "Input/Output-Extra"
    }
    
    /// Get the library identifier
    pub fn get_id(&self) -> &str {
        &self.id
    }
    
    /// Create a Switch component
    pub fn create_switch(id: ComponentId) -> Switch {
        Switch::new(id)
    }
    
    // TODO: Implement other components when their compilation issues are resolved
    /*
    /// Create a Buzzer component
    pub fn create_buzzer(id: ComponentId) -> Buzzer {
        Buzzer::new(id)
    }
    
    /// Create a Slider component
    pub fn create_slider(id: ComponentId) -> Slider {
        Slider::new(id)
    }
    
    /// Create a Digital Oscilloscope component
    pub fn create_digital_oscilloscope(id: ComponentId) -> DigitalOscilloscope {
        DigitalOscilloscope::new(id)
    }
    
    /// Create a PLA ROM component
    pub fn create_pla_rom(id: ComponentId) -> PlaRom {
        PlaRom::new(id)
    }
    */
    
    /// Get list of all component names in this library
    pub fn get_component_names() -> Vec<&'static str> {
        vec![
            "Switch",
            // TODO: Add back when components are fixed
            // "Buzzer", 
            // "Slider",
            // "Digital Oscilloscope",
            // "PlaRom",
        ]
    }
}

impl Default for ExtraIoLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extra_io_library_creation() {
        let library = ExtraIoLibrary::new();
        assert_eq!(library.get_id(), ExtraIoLibrary::ID);
    }

    #[test]
    fn test_library_id_compatibility() {
        // Verify the ID matches the Java implementation exactly
        assert_eq!(ExtraIoLibrary::ID, "Input/Output-Extra");
    }

    #[test]
    fn test_component_creation() {
        let id = ComponentId::new(1);
        
        // Test Switch creation
        let switch = ExtraIoLibrary::create_switch(id);
        assert_eq!(switch.name(), "Switch");
        assert_eq!(switch.id(), id);
    }

    #[test]
    fn test_component_names() {
        let names = ExtraIoLibrary::get_component_names();
        assert_eq!(names.len(), 1); // Only Switch for now
        assert!(names.contains(&"Switch"));
    }
}