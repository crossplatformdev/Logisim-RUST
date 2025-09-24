//! # Logisim UI
//!
//! User interface components for the Logisim-RUST digital logic designer and simulator.
//! This crate provides GUI components that integrate with the core simulation engine
//! to provide a complete schematic editor and simulation environment.
//!
//! ## Architecture
//!
//! The UI is structured around the main application window with several key components:
//!
//! - **MainFrame**: The primary application window containing all UI elements
//! - **Canvas**: The main schematic drawing and editing area
//! - **Toolbox**: Component library and tool selection
//! - **Explorer**: Circuit hierarchy and simulation state viewer
//! - **AttributeTable**: Properties panel for selected components
//! - **MenuBar**: Main application menu system
//!
//! ## Key Features
//!
//! - Schematic editing with drag-and-drop component placement
//! - Wire routing and connection management
//! - Component selection and manipulation
//! - Zoom controls and grid snapping
//! - File operations (open, save, export)
//! - Simulation control integration
//!
//! ## Design Philosophy
//!
//! This UI port aims for strict 1:1 compatibility with the original Java Logisim-Evolution
//! implementation. All user interface elements, behaviors, and workflows should match
//! the original as closely as possible to maintain user familiarity and ensure
//! compatibility with existing .circ files.

pub mod draw;
pub mod gui;
pub mod main {
    pub use crate::main_lib::*;
}
mod main_lib;

// Re-export main UI types for convenience
pub use gui::app::LogisimApp;
pub use gui::frame::MainFrame;

#[cfg(feature = "gui")]
pub use gui::canvas::Canvas;
#[cfg(feature = "gui")]
pub use gui::toolbox::Toolbox;

// Always export these (not GUI dependent)
pub use gui::edit_handler::EditHandler;
pub use gui::selection::Selection;

// Drawing framework exports
pub use draw::{DrawError, DrawResult};
pub use draw::model::{CanvasObject, Drawing, Handle, HandleGesture};
pub use draw::canvas::{Canvas as DrawCanvas, Selection as DrawSelection};
pub use draw::shapes::DrawAttr;

/// UI-specific error types
#[derive(Debug, thiserror::Error)]
pub enum UiError {
    #[error("Failed to initialize GUI framework: {0}")]
    GuiInitError(String),

    #[error("Canvas rendering error: {0}")]
    RenderError(String),

    #[error("Component placement error: {0}")]
    PlacementError(String),

    #[error("File operation error: {0}")]
    FileError(String),

    #[error("Feature not implemented: {0}")]
    NotImplemented(String),

    #[error("Core simulation error: {0}")]
    CoreError(#[from] logisim_core::simulation::SimulationError),
}

pub type UiResult<T> = Result<T, UiError>;
