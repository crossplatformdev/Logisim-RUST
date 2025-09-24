//! Canvas system for interactive drawing
//!
//! This module corresponds to the Java com.cburch.draw.canvas package.

pub mod canvas;
pub mod selection;
pub mod listener;
pub mod tool;
pub mod action_dispatcher;

// Re-export key types
pub use canvas::Canvas;
pub use selection::{Selection, SelectionEvent, SelectionListener};
pub use listener::CanvasListener;
pub use tool::CanvasTool;
pub use action_dispatcher::ActionDispatcher;