/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Counter poker for user interaction
//!
//! When the user clicks a counter using the Poke Tool, a CounterPoker object is created,
//! and that object will handle all user events.
//! Equivalent to Java's com.cburch.gray.CounterPoker class.

/// Mouse event coordinates
#[derive(Debug, Clone, Copy)]
pub struct MouseCoords {
    pub x: i32,
    pub y: i32,
}

/// Counter poker for handling user interaction with Gray counters.
///
/// This handles mouse clicks and keyboard events to allow the user to
/// directly modify the counter value using the Poke Tool.
/// This is equivalent to Java's CounterPoker class.
pub struct CounterPoker {
    initial_value: Option<u64>,
}

impl CounterPoker {
    pub fn new() -> Self {
        Self {
            initial_value: None,
        }
    }

    /// Handle mouse press event on the counter
    pub fn mouse_pressed(&mut self, _coords: MouseCoords) {
        // In full implementation, this would determine what part of the counter
        // was clicked and prepare for value editing
    }

    /// Handle mouse release event on the counter  
    pub fn mouse_released(&mut self, _coords: MouseCoords) {
        // In full implementation, this would complete the value change
    }

    /// Handle key press event when counter is being poked
    pub fn key_pressed(&mut self, _key_code: u32) {
        // In full implementation, this would handle numeric input
        // to directly set the counter value
    }

    /// Get the current value being edited
    pub fn get_current_value(&self) -> Option<u64> {
        self.initial_value
    }

    /// Set the initial value when starting to poke
    pub fn set_initial_value(&mut self, value: u64) {
        self.initial_value = Some(value);
    }

    /// Check if currently editing a value
    pub fn is_editing(&self) -> bool {
        self.initial_value.is_some()
    }

    /// Cancel the current editing operation
    pub fn cancel_edit(&mut self) {
        self.initial_value = None;
    }
}

impl Default for CounterPoker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_poker_creation() {
        let poker = CounterPoker::new();
        assert!(!poker.is_editing());
        assert_eq!(poker.get_current_value(), None);
    }

    #[test]
    fn test_set_initial_value() {
        let mut poker = CounterPoker::new();
        poker.set_initial_value(42);

        assert!(poker.is_editing());
        assert_eq!(poker.get_current_value(), Some(42));
    }

    #[test]
    fn test_cancel_edit() {
        let mut poker = CounterPoker::new();
        poker.set_initial_value(42);
        assert!(poker.is_editing());

        poker.cancel_edit();
        assert!(!poker.is_editing());
        assert_eq!(poker.get_current_value(), None);
    }

    #[test]
    fn test_mouse_events() {
        let mut poker = CounterPoker::new();
        let coords = MouseCoords { x: 10, y: 20 };

        // Should not panic
        poker.mouse_pressed(coords);
        poker.mouse_released(coords);
    }

    #[test]
    fn test_key_events() {
        let mut poker = CounterPoker::new();

        // Should not panic
        poker.key_pressed(65); // 'A' key
    }
}
