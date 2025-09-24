/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Poker System
//!
//! This module provides the `InstancePoker` trait for components that support
//! interactive manipulation during simulation.

use crate::data::Location;
use crate::instance::InstanceState;

/// Trait for components that support interactive manipulation ("poking").
///
/// This trait allows components to respond to user interactions during simulation,
/// such as clicking buttons, toggling switches, or adjusting controls.
pub trait InstancePoker {
    /// Handles a mouse press event.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `location` - Location of the mouse press relative to component
    ///
    /// # Returns
    ///
    /// True if the event was handled, false otherwise.
    fn mouse_pressed(&mut self, state: &mut dyn InstanceState, location: Location) -> bool {
        let _ = (state, location);
        false
    }

    /// Handles a mouse release event.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `location` - Location of the mouse release relative to component
    ///
    /// # Returns
    ///
    /// True if the event was handled, false otherwise.
    fn mouse_released(&mut self, state: &mut dyn InstanceState, location: Location) -> bool {
        let _ = (state, location);
        false
    }

    /// Handles a key press event.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `key_code` - The key that was pressed
    ///
    /// # Returns
    ///
    /// True if the event was handled, false otherwise.
    fn key_pressed(&mut self, state: &mut dyn InstanceState, key_code: u32) -> bool {
        let _ = (state, key_code);
        false
    }

    /// Checks if the poker wants to handle events at a specific location.
    ///
    /// # Arguments
    ///
    /// * `state` - Current instance state
    /// * `location` - Location to test relative to component
    ///
    /// # Returns
    ///
    /// True if events at this location should be sent to this poker.
    fn contains(&self, state: &dyn InstanceState, location: Location) -> bool {
        let _ = (state, location);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Location;
    
    struct MockPoker;
    
    impl InstancePoker for MockPoker {
        fn mouse_pressed(&mut self, _state: &mut dyn InstanceState, _location: Location) -> bool {
            true // Simulate handling the event
        }

        fn contains(&self, _state: &dyn InstanceState, location: Location) -> bool {
            // Simple bounds check for testing
            location.x >= 0 && location.x <= 100 && location.y >= 0 && location.y <= 50
        }
    }

    // Note: Cannot easily test without a concrete InstanceState implementation
    // Tests would be added when integrated with the full system
}