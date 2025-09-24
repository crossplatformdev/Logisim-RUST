//! Drawing framework module
//! 
//! This module provides a complete 2D drawing framework for creating and manipulating
//! graphical objects on a canvas. It is ported from the Java com.cburch.draw package
//! and maintains strict 1:1 compatibility with the original implementation.
//!
//! ## Architecture
//!
//! The drawing framework is organized into several key components:
//!
//! - **Model**: Core data structures for canvas objects, drawings, and attributes
//! - **Canvas**: Interactive canvas for displaying and manipulating objects
//! - **Shapes**: Concrete implementations of drawable objects (lines, rectangles, etc.)
//! - **Actions**: Command pattern implementation for undo/redo operations
//! - **Tools**: User interaction tools for creating and modifying objects
//! - **GUI**: Integration with the main UI framework
//!
//! ## Key Types
//!
//! - [`CanvasObject`] - Trait for all drawable objects
//! - [`Drawing`] - Container for collections of canvas objects
//! - [`Canvas`] - Interactive drawing surface
//! - [`Selection`] - Manages selected objects
//! - [`Handle`] - Control points for object manipulation

pub mod model;
pub mod canvas;
pub mod shapes;
pub mod actions;
pub mod tools;
pub mod gui;
pub mod util;

// Re-export commonly used types
pub use model::{CanvasObject, Drawing, Handle, HandleGesture};
pub use canvas::{Canvas, Selection};
pub use shapes::DrawAttr;

/// Drawing framework error types
#[derive(Debug, thiserror::Error)]
pub enum DrawError {
    #[error("Invalid canvas object: {0}")]
    InvalidObject(String),
    
    #[error("Operation not supported: {0}")]
    UnsupportedOperation(String),
    
    #[error("Selection error: {0}")]
    SelectionError(String),
    
    #[error("Canvas error: {0}")]
    CanvasError(String),
    
    #[error("Attribute error: {0}")]
    AttributeError(String),
    
    #[error("SVG parsing error: {0}")]
    SvgError(String),
}

/// Result type for drawing operations
pub type DrawResult<T> = Result<T, DrawError>;