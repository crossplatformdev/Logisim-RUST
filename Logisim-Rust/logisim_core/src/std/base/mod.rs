/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Base Library
//!
//! Rust port of `com.cburch.logisim.std.base` package containing
//! basic utilities and text components.
//!
//! ## Components
//!
//! This module provides fundamental components:
//! - Text: Text annotation component for circuit documentation
//! - Base tools and utilities for circuit editing

mod base_library;
mod text;
mod text_attributes;

// Re-export all implementations
pub use base_library::*;
pub use text::*;
pub use text_attributes::*;
