/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Extra Input/Output Components
//!
//! Rust port of `com.cburch.logisim.std.io.extra` package containing
//! specialized I/O components for user interaction.
//!
//! ## Components
//!
//! This module provides the following Extra I/O components:
//! - **Switch**: Toggle switch for manual input control
//! - **Buzzer**: Audio output component with configurable waveforms
//! - **Slider**: Variable value input with visual position control
//! - **DigitalOscilloscope**: Multi-channel signal visualization
//! - **PlaRom**: Programmable Logic Array ROM with data editor
//!
//! ## Architecture
//!
//! Each component follows the standard Logisim component pattern:
//! - Component struct implementing required traits
//! - Factory pattern for creation and configuration
//! - State management for interactive components
//! - Integration with the simulation kernel

mod buzzer;
mod digital_oscilloscope;
mod extra_io_library;
mod pla_rom;
mod slider;
mod switch;

// Re-export all Extra IO implementations
pub use buzzer::*;
pub use digital_oscilloscope::*;
pub use extra_io_library::*;
pub use pla_rom::*;
pub use slider::*;
pub use switch::*;