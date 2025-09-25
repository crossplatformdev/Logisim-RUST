/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Plexers Library
//!
//! This module contains the implementation of plexer components (multiplexers, demultiplexers,
//! decoders, encoders, and bit selectors) equivalent to the Java package 
//! `com.cburch.logisim.std.plexers`.
//!
//! ## Components
//! 
//! - **Multiplexer**: Data selector that routes one of several inputs to a single output
//! - **Demultiplexer**: Data router that routes a single input to one of several outputs  
//! - **Decoder**: Address decoder that activates one output based on binary input
//! - **PriorityEncoder**: Encoder that outputs the index of the highest priority active input
//! - **BitSelector**: Component for selecting specific bits from a bus
//!
//! ## Architecture
//!
//! Each component follows the standard Logisim-RUST component architecture:
//! - Implements the `Component` trait for basic functionality
//! - Implements the `Propagator` trait for signal propagation
//! - Uses standardized pin naming and positioning
//! - Supports configurable attributes like bit width and orientation

pub mod bit_selector;
pub mod decoder;
pub mod demultiplexer;
pub mod multiplexer;
pub mod plexers_library;
pub mod priority_encoder;

// Re-export all component types
pub use bit_selector::BitSelector;
pub use decoder::Decoder;
pub use demultiplexer::Demultiplexer;
pub use multiplexer::Multiplexer;
pub use plexers_library::PlexersLibrary;
pub use priority_encoder::PriorityEncoder;