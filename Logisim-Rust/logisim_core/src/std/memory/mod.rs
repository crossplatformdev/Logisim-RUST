/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Memory components module
//!
//! This module contains memory-related components equivalent to the Java
//! `com.cburch.logisim.std.memory` package. These components provide various
//! types of storage and memory functionality.

pub mod mem_contents;
pub mod mem_state;
pub mod mem;
pub mod memory_library;
pub mod rom;
pub mod ram;
pub mod register;
pub mod d_flip_flop;
pub mod jk_flip_flop;
pub mod sr_flip_flop;
pub mod t_flip_flop;
pub mod counter;
pub mod shift_register;
pub mod random;

// Re-export main types
pub use mem_contents::{MemContents, MemContentsSub};
pub use mem_state::MemState;
pub use mem::{MemFactory, MemoryComponent, EnableMode};
pub use memory_library::MemoryLibrary;
pub use rom::RomFactory;
pub use ram::Ram;
pub use register::Register;
pub use d_flip_flop::DFlipFlop;
pub use jk_flip_flop::JKFlipFlop;
pub use sr_flip_flop::SRFlipFlop;
pub use t_flip_flop::TFlipFlop;
pub use counter::Counter;
pub use shift_register::ShiftRegister;
pub use random::Random;

/// Memory library ID - must match Java implementation
pub const MEMORY_LIBRARY_ID: &str = "Memory";