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
//! - `ttl`: TTL integrated circuits (TtlLibrary)
//! - `wiring`: Wiring and connection components (WiringLibrary)
//!
//! ## Migration Status
//!
//! This represents the standard component library migration from Java to Rust,
//! focusing on providing 1:1 functional equivalence with the original implementation.

pub mod base;
pub mod bfh;
pub mod gates;
<<<<<<< HEAD
pub mod io;
=======
pub mod ttl;    // TTL integrated circuits
>>>>>>> origin/copilot/fix-8670ab67-e80b-4622-811f-2cfa65e1bade
pub mod wiring;
pub mod arith;


// Re-export commonly used types
pub use base::*;
pub use bfh::*;
pub use gates::*;
<<<<<<< HEAD
pub use io::*;
=======
pub use ttl::*;  // Export TTL components
>>>>>>> origin/copilot/fix-8670ab67-e80b-4622-811f-2cfa65e1bade
pub use wiring::*;
pub use arith::*;
