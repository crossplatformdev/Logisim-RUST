/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Hex editor module for viewing and editing binary data
//!
//! This module provides a complete hex editor interface for viewing and
//! modifying binary data with cursor management, selection, and highlighting.

pub mod caret;
pub mod hex_editor;
pub mod hex_model;
pub mod hex_model_listener;
pub mod highlighter;
pub mod measures;

pub use caret::Caret;
pub use hex_editor::HexEditor;
pub use hex_model::{HexModel, VecHexModel};
pub use hex_model_listener::HexModelListener;
pub use highlighter::{Highlighter, HighlightHandle, Color};
pub use measures::Measures;