/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Standard library of components for Logisim
//!
//! This module contains the standard set of digital logic components
//! that are available in Logisim, organized by category.

pub mod wiring;

// Re-export standard components
pub use wiring::*;