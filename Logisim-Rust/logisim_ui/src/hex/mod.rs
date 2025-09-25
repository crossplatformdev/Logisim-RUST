/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Hex Editor Module
//!
//! This module provides a complete hex editor implementation equivalent to
//! the Java `com.cburch.hex` package. It includes:
//!
//! - `HexModel` trait for data model abstraction
//! - `HexEditor` component for viewing and editing hex data
//! - `Caret` for cursor management and text selection
//! - `Highlighter` for visual highlighting of ranges
//! - `Measures` for layout calculations and metrics
//!
//! The implementation uses egui for rendering while maintaining compatibility
//! with the original Java Logisim-Evolution hex editor functionality.

pub mod caret;
pub mod hex_editor;
pub mod hex_model;
pub mod highlighter;
pub mod measures;

// Re-export commonly used types
pub use hex_model::{HexModel, HexModelEvent, HexModelListener, MemoryHexModel};
pub use measures::Measures;

#[cfg(feature = "gui")]
pub use caret::Caret;
#[cfg(feature = "gui")]
pub use hex_editor::HexEditor;
#[cfg(feature = "gui")]
pub use highlighter::Highlighter;
