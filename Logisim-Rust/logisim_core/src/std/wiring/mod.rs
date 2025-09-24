/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Wiring components for circuit connections and signal management
//!
//! This module contains all the wiring-related components such as pins,
//! tunnels, splitters, clocks, and power/ground components.

pub mod clock;
pub mod constant;
pub mod ground;
pub mod pin;
pub mod power;
pub mod wiring_library;

// Export wiring components
pub use clock::*;
pub use constant::*;
pub use ground::*;
pub use pin::*;
pub use power::*;
pub use wiring_library::*;