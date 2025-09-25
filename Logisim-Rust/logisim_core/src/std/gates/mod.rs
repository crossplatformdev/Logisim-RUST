/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Logic Gates Library
//!
//! Rust port of `com.cburch.logisim.std.gates` package containing
//! digital logic gates and related components.
//!
//! ## Components
//!
//! This module provides the following logic gates:
//! - Basic gates: AND, OR, NOT, NAND, NOR, XOR, XNOR
//! - Specialized gates: Buffer, Controlled Buffer
//! - Parity gates: Odd Parity, Even Parity
//! - Programmable Logic Array (PLA)
//!
//! ## Architecture
//!
//! Each gate is implemented as a separate module following these patterns:
//! - Gate struct implementing Component and Propagator traits
//! - Factory pattern for component creation
//! - HDL generation support
//! - Configurable attributes (inputs, data width, etc.)

mod and_gate;
mod buffer;
mod controlled_buffer;
mod even_parity;
mod gates_library;
mod nand_gate;
mod nor_gate;
mod not_gate;
mod odd_parity;
mod or_gate;
mod pla;
mod xnor_gate;
mod xor_gate;

// Re-export all gate implementations
pub use and_gate::*;
pub use buffer::*;
pub use controlled_buffer::*;
pub use even_parity::*;
pub use gates_library::*;
pub use nand_gate::*;
pub use nor_gate::*;
pub use not_gate::*;
pub use odd_parity::*;
pub use or_gate::*;
pub use pla::*;
pub use xnor_gate::*;
pub use xor_gate::*;
