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

use super::{Switch, Buzzer, Slider, DigitalOscilloscope, PlaRom};

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
    pub fn create_switch(id: u32) -> Switch {
        Switch::new(id)
    }
    
    /// Create a Buzzer component
    pub fn create_buzzer(id: u32) -> Buzzer {
        Buzzer::new(id)
    }
    
    /// Create a Slider component
    pub fn create_slider(id: u32) -> Slider {
        Slider::new(id)
    }
    
    /// Create a Digital Oscilloscope component
    pub fn create_digital_oscilloscope(id: u32) -> DigitalOscilloscope {
        DigitalOscilloscope::new(id)
    }
    
    /// Create a PLA ROM component
    pub fn create_pla_rom(id: u32) -> PlaRom {
        PlaRom::new(id)
    }
    
    /// Get list of all component names in this library
    pub fn get_component_names() -> Vec<&'static str> {
        vec![
            "Switch",
            "Buzzer", 
            "Slider",
            "Digital Oscilloscope",
            "PlaRom",
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
        let id = 1;
        
        // Test Switch creation
        let switch = ExtraIoLibrary::create_switch(id);
        assert_eq!(switch.get_type_name(), "Switch");
        assert_eq!(switch.get_id(), id);
    }

    #[test]
    fn test_component_names() {
        let names = ExtraIoLibrary::get_component_names();
        assert_eq!(names.len(), 5);
        assert!(names.contains(&"Switch"));
        assert!(names.contains(&"Buzzer"));
        assert!(names.contains(&"Slider"));
        assert!(names.contains(&"Digital Oscilloscope"));
        assert!(names.contains(&"PlaRom"));
    }
}