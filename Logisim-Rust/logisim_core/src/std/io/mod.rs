/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Input/Output Components
//!
//! Rust port of `com.cburch.logisim.std.io` package containing
//! input/output components for interfacing with users.

// Core IO modules
pub mod abstracts;
pub mod button;
pub mod dip_switch;
pub mod dot_matrix;
pub mod hdl_generators;
pub mod hex_digit;
pub mod io_library;
pub mod joystick;
pub mod keyboard;
pub mod led;
pub mod led_bar;
pub mod port_io;
pub mod reptar_local_bus;
pub mod rgb_led;
pub mod seven_segment;
pub mod telnet;
pub mod tty;
pub mod video;

// Extra components
pub mod extra;

// Re-export main library
pub use io_library::IoLibrary;

// Re-export common components
pub use button::Button;
pub use dip_switch::DipSwitch;
pub use led::Led;

// Re-export extra IO components
pub use extra::*;

use crate::comp::Color;

/// Default background color (transparent white)
pub const DEFAULT_BACKGROUND: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 0,
};