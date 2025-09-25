/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Input/Output Components Library
//!
//! This module contains the I/O component implementations that are equivalent
//! to the Java `com.cburch.logisim.std.io` package. These components handle
//! user interaction and external communication.
//!
//! ## Organization
//!
//! The module is organized by component type:
//! - Basic I/O: Button, Led, DipSwitch, LedBar
//! - Display: SevenSegment, HexDigit, DotMatrix, RgbLed
//! - Input: Keyboard, Joystick
//! - Communication: TTY, Telnet, PortIo, ReptarLocalBus
//! - Video: Video components and matrix displays
//! - HDL Generation: HDL generator factories for FPGA synthesis
//!
//! ## Migration Status
//!
//! This represents the I/O component library migration from Java to Rust,
//! providing 1:1 functional equivalence with the original Java implementation
//! while leveraging Rust's type safety and performance benefits.
//!
//! ## Component Categories
//!
//! ### Basic Components
//! - **Button**: Push button for user input
//! - **Led**: Light-emitting diode for output display
//! - **DipSwitch**: Multi-position switch for configuration
//! - **LedBar**: Bar of LEDs for multi-bit display
//!
//! ### Display Components
//! - **SevenSegment**: 7-segment numeric display
//! - **HexDigit**: Hexadecimal digit display
//! - **DotMatrix**: Programmable LED matrix
//! - **RgbLed**: RGB color LED
//!
//! ### Input Devices
//! - **Keyboard**: Computer keyboard interface
//! - **Joystick**: Analog joystick input
//!
//! ### Communication
//! - **TTY**: Terminal interface
//! - **Telnet**: Network terminal
//! - **PortIo**: Bidirectional I/O port
//! - **ReptarLocalBus**: Local bus interface
//!
//! ### Specialized
//! - **Video**: Video output components

pub mod abstracts;
pub mod button;
pub mod dip_switch;
pub mod dot_matrix;
pub mod hex_digit;
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
pub mod hdl_generators;

// Re-export library
pub use crate::std::io::io_library::IoLibrary;

pub mod io_library;

// Re-export all components for convenience
pub use abstracts::*;
pub use button::*;
pub use dip_switch::*;
pub use dot_matrix::*;
pub use hex_digit::*;
pub use joystick::*;
pub use keyboard::*;
pub use led::*;
pub use led_bar::*;
pub use port_io::*;
pub use reptar_local_bus::*;
pub use rgb_led::*;
pub use seven_segment::*;
pub use telnet::*;
pub use tty::*;
pub use video::*;
pub use hdl_generators::*;

use crate::data::{Attribute, Attributes};
use crate::comp::Color;

/// Default background color (transparent white)
pub const DEFAULT_BACKGROUND: Color = Color::new(255, 255, 255, 0);