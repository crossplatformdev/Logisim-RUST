/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Poker Adapter
//!
//! Adapter between the instance poker system and external event handling.

use crate::data::Location;
use crate::instance::{InstancePoker, InstanceState};

/// Adapter that bridges InstancePoker implementations with external event systems.
///
/// This is equivalent to Java's `InstancePokerAdapter` class.
pub struct InstancePokerAdapter {
    poker: Box<dyn InstancePoker>,
}

impl InstancePokerAdapter {
    /// Creates a new poker adapter.
    pub fn new(poker: Box<dyn InstancePoker>) -> Self {
        Self { poker }
    }

    /// Delegates mouse press events to the wrapped poker.
    pub fn mouse_pressed(&mut self, state: &mut dyn InstanceState, location: Location) -> bool {
        self.poker.mouse_pressed(state, location)
    }

    /// Delegates mouse release events to the wrapped poker.
    pub fn mouse_released(&mut self, state: &mut dyn InstanceState, location: Location) -> bool {
        self.poker.mouse_released(state, location)
    }

    /// Delegates key press events to the wrapped poker.
    pub fn key_pressed(&mut self, state: &mut dyn InstanceState, key_code: u32) -> bool {
        self.poker.key_pressed(state, key_code)
    }

    /// Delegates containment checks to the wrapped poker.
    pub fn contains(&self, state: &dyn InstanceState, location: Location) -> bool {
        self.poker.contains(state, location)
    }
}
