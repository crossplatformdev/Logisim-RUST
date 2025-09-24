/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Contract interfaces for event handling
//!
//! This module provides default trait implementations for various UI event handlers,
//! allowing implementors to only override the methods they need rather than
//! implementing all methods of the parent trait.

pub mod component_listener;
pub mod document_listener;
pub mod key_listener;
pub mod layout_manager;
pub mod list_data_listener;
pub mod mouse_input_listener;
pub mod mouse_listener;
pub mod mouse_motion_listener;
pub mod window_focus_listener;
pub mod window_listener;

// Re-export all contract traits
pub use component_listener::*;
pub use document_listener::*;
pub use key_listener::*;
pub use layout_manager::*;
pub use list_data_listener::*;
pub use mouse_input_listener::*;
pub use mouse_listener::*;
pub use mouse_motion_listener::*;
pub use window_focus_listener::*;
pub use window_listener::*;
