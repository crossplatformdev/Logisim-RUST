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
<<<<<<< HEAD
//! - `ttl`: TTL integrated circuits (TtlLibrary)
//! - `wiring`: Wiring and connection components (WiringLibrary)
=======
//! - `hdl`: HDL components and parsers (HdlLibrary)
>>>>>>> origin/copilot/fix-1c9fc52b-264e-4c3a-9b7c-05621b80788e
//!
//! ## Migration Status
//!
//! This represents the standard component library migration from Java to Rust,
//! focusing on providing 1:1 functional equivalence with the original implementation.

pub mod base;
pub mod bfh;
pub mod gates;
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
pub mod io;
=======
pub mod ttl;    // TTL integrated circuits
>>>>>>> origin/copilot/fix-8670ab67-e80b-4622-811f-2cfa65e1bade
=======
pub mod io;
>>>>>>> origin/copilot/fix-f356266b-bb16-4b5b-92f6-f52c4c0f6a69
=======
pub mod hdl;
>>>>>>> origin/copilot/fix-1c9fc52b-264e-4c3a-9b7c-05621b80788e
=======
pub mod plexers;
>>>>>>> origin/copilot/fix-3257658f-2b32-41b0-9150-144ce65274f6
pub mod wiring;
pub mod arith;


// Re-export commonly used types
pub use base::*;
pub use bfh::*;
pub use gates::*;
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
pub use io::*;
=======
pub use ttl::*;  // Export TTL components
>>>>>>> origin/copilot/fix-8670ab67-e80b-4622-811f-2cfa65e1bade
=======
pub use io::*;
>>>>>>> origin/copilot/fix-f356266b-bb16-4b5b-92f6-f52c4c0f6a69
=======
pub use hdl::*;
>>>>>>> origin/copilot/fix-1c9fc52b-264e-4c3a-9b7c-05621b80788e
=======
pub use plexers::*;
>>>>>>> origin/copilot/fix-3257658f-2b32-41b0-9150-144ce65274f6
pub use wiring::*;
pub use arith::*;
