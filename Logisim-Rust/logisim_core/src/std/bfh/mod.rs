/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! BFH Components Library
//!
//! This module contains components developed by the BFH (Bern University of Applied Sciences)
//! for educational and practical use. The library includes digital utility components
//! like BCD converters and display decoders.
//!
//! ## Components
//!
//! - [`BinToBcd`] - Binary to BCD converter with configurable bit width
//! - [`BcdToSevenSegmentDisplay`] - BCD to 7-segment display decoder
//!
//! ## Usage
//!
//! ```rust
//! use logisim_core::std::bfh::{BfhLibrary, BinToBcd, BcdToSevenSegmentDisplay};
//! use logisim_core::component::ComponentId;
//!
//! // Create components through the library
//! let library = BfhLibrary::new();
//! let bin_to_bcd = BfhLibrary::create_bin_to_bcd(ComponentId(1));
//! let bcd_to_seven = BfhLibrary::create_bcd_to_seven_segment(ComponentId(2));
//! ```

pub mod library;
pub mod bin_to_bcd;
pub mod bcd_to_seven_segment;

// Re-export main types
pub use library::BfhLibrary;
pub use bin_to_bcd::BinToBcd;
pub use bcd_to_seven_segment::BcdToSevenSegmentDisplay;