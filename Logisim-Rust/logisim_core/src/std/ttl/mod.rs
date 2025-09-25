/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! TTL (Transistor-Transistor Logic) integrated circuit components
//! 
//! This module contains the Rust port of the TTL components from the Java
//! package com.cburch.logisim.std.ttl, providing various TTL ICs used in
//! digital logic design and simulation.

pub mod abstract_ttl_gate;
pub mod display_decoder;
pub mod drawgates;
pub mod ttl_library;

// Basic TTL logic gates
pub mod ttl7400; // Quad 2-input NAND gate
pub mod ttl7402; // Quad 2-input NOR gate
pub mod ttl7404; // Hex inverter
pub mod ttl7408; // Quad 2-input AND gate
pub mod ttl7410; // Triple 3-input NAND gate

// Tests module
#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use abstract_ttl_gate::AbstractTtlGate;
pub use ttl_library::TtlLibrary;

// Re-export TTL components
pub use ttl7400::Ttl7400;
pub use ttl7402::Ttl7402;
pub use ttl7404::Ttl7404;
pub use ttl7408::Ttl7408;
pub use ttl7410::Ttl7410;

/// TTL pin configuration constants
pub const PIN_WIDTH: i32 = 10;
pub const PIN_HEIGHT: i32 = 7;

/// Standard TTL supply voltage pins (VCC and GND)
pub const VCC_GND_PINS: [u8; 2] = [7, 14]; // Typical 14-pin DIP positions

/// TTL library identifier for project file compatibility
pub const TTL_LIBRARY_ID: &str = "TTL";