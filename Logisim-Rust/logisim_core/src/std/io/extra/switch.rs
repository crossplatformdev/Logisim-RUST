/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Switch Component
//!
//! Rust port of `com.cburch.logisim.std.io.extra.Switch`
//!
//! A toggle switch component that allows manual input control.
//! Users can click the switch to toggle between on/off states.

use crate::data::{BitWidth, Bounds, Direction, Location};
use serde::{Deserialize, Serialize};

/// Unique identifier for the Switch component
pub const SWITCH_ID: &str = "Switch";

/// Switch component state data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchData {
    /// Current state of the switch (true = on/active, false = off/inactive)
    pub active: bool,
}

impl SwitchData {
    /// Create new switch data with default inactive state
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Toggle the switch state
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// Set the switch state
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Get the current switch state
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for SwitchData {
    fn default() -> Self {
        Self::new()
    }
}

/// Switch component implementation
///
/// A toggle switch that can be manually controlled to provide input signals.
/// When active, it passes the input signal through to the output.
/// When inactive, it outputs an unknown/floating value.
#[derive(Debug, Clone)]
pub struct Switch {
    /// Component identifier
    id: u32,
    /// Current switch state
    data: SwitchData,
}

impl Switch {
    /// Create a new switch component
    pub fn new(id: u32) -> Self {
        Self {
            id,
            data: SwitchData::new(),
        }
    }

    /// Get the current switch data
    pub fn get_data(&self) -> &SwitchData {
        &self.data
    }

    /// Get mutable reference to switch data
    pub fn get_data_mut(&mut self) -> &mut SwitchData {
        &mut self.data
    }

    /// Toggle the switch state
    pub fn toggle(&mut self) {
        self.data.toggle();
    }

    /// Handle mouse click to toggle switch
    pub fn handle_mouse_click(&mut self, _location: Location) {
        self.toggle();
    }

    /// Get the component's display name
    pub fn display_name() -> &'static str {
        "Switch"
    }

    /// Get the component's factory ID
    pub fn factory_id() -> &'static str {
        SWITCH_ID
    }

    /// Get component ID
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// Get component type name
    pub fn get_type_name(&self) -> &'static str {
        "Switch"
    }

    /// Get component bounds
    pub fn get_bounds(&self) -> Bounds {
        // Switch bounds: 20x30 pixels with depth effect
        Bounds::new(-20, -15, 20, 30)
    }

    /// Check if component is interactive
    pub fn is_interactive(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_creation() {
        let switch = Switch::new(1);
        assert_eq!(switch.get_id(), 1);
        assert_eq!(switch.get_type_name(), "Switch");
        assert!(!switch.get_data().is_active());
        assert!(switch.is_interactive());
    }

    #[test]
    fn test_switch_toggle() {
        let mut switch = Switch::new(1);
        
        // Initially inactive
        assert!(!switch.get_data().is_active());
        
        // Toggle to active
        switch.toggle();
        assert!(switch.get_data().is_active());
        
        // Toggle back to inactive
        switch.toggle();
        assert!(!switch.get_data().is_active());
    }

    #[test]
    fn test_switch_data_serialization() {
        let data = SwitchData::new();
        let json = serde_json::to_string(&data).unwrap();
        let deserialized: SwitchData = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_switch_bounds() {
        let switch = Switch::new(1);
        let bounds = switch.get_bounds();
        
        assert_eq!(bounds, Bounds::new(-20, -15, 20, 30));
    }
}