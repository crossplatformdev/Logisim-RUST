/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Standard component libraries
//!
//! This module contains the standard component implementations that are equivalent
//! to the Java `com.cburch.logisim.std` package. These are the core components
//! that users interact with in Logisim.
//!
//! ## Organization
//!
//! The module is organized to mirror the Java package structure:
//! - `base`: Basic utilities and text components (BaseLibrary)
//! - `gates`: Logic gates and related components (GatesLibrary)
//! - `io`: Input/output components
//! - `memory`: Memory components like RAM, ROM, flip-flops (MemoryLibrary)
//! - `wiring`: Wiring components like pins, tunnels, splitters (WiringLibrary)
//! - `ttl`: TTL integrated circuits (TtlLibrary)
//! - `hdl`: HDL components and parsers (HdlLibrary)
//! - `plexers`: Multiplexer and demultiplexer components
//! - `arith`: Arithmetic components
//!
//! ## Migration Status
//!
//! This represents the standard component library migration from Java to Rust,
//! focusing on providing 1:1 functional equivalence with the original implementation.

pub mod base;
pub mod bfh;
pub mod gates;
pub mod io;
pub mod memory;
pub mod wiring;
pub mod ttl;
pub mod hdl;
pub mod plexers;
pub mod arith;


// Re-export commonly used types
pub use base::*;
pub use bfh::*;
pub use gates::*;
pub use io::*;
pub use memory::*;
pub use wiring::*;
pub use ttl::*;
pub use hdl::*;
pub use plexers::*;
pub use arith::*;
