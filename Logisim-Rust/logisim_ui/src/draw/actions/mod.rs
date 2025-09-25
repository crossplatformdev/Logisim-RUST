//! Drawing actions for undo/redo support
//!
//! This module corresponds to the Java com.cburch.draw.actions package.

/// Model action trait (placeholder)
pub trait ModelAction {
    fn execute(&self);
    fn undo(&self);
}