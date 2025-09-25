/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 *
 * Ported to Rust by the Logisim-RUST project
 * https://github.com/crossplatformdev/Logisim-RUST
 */

//! Tool Framework for Logisim-RUST
//!
//! This module provides the core tool framework for the digital logic design tool.
//! Tools are the primary means of interaction with the circuit canvas, allowing users
//! to add components, make connections, edit circuits, and perform various operations.
//!
//! ## Architecture
//!
//! The tool framework is built around several key concepts:
//!
//! - **Tool**: The base trait for all interactive tools
//! - **Library**: Collections of tools organized into logical groups
//! - **AddTool**: Tools for adding specific components to circuits
//! - **SelectTool**: Tool for selecting and manipulating existing components
//! - **EditTool**: Tool for in-place editing of component properties
//! - **WiringTool**: Tool for creating connections between components
//!
//! ## Tool Types
//!
//! ### Component Tools
//! - AddTool: Adds new components to the circuit
//! - EditTool: Edits existing component properties
//!
//! ### Interaction Tools  
//! - SelectTool: Selects and moves components
//! - PokeTool: Interacts with components during simulation
//! - TextTool: Adds and edits text labels
//!
//! ### Connection Tools
//! - WiringTool: Creates wire connections between components
//!
//! ### Menu Tools
//! - MenuTool: Provides context menu functionality
//!
//! ## Usage Example
//!
//! ```rust
//! use logisim_core::tools::{Tool, Library, BasicLibrary};
//!
//! // Create a library
//! let mut lib = BasicLibrary::new("My Library".to_string());
//!
//! // Add tools to the library would be done here
//! // lib.add_tool(some_tool);
//!
//! // Use tools from the library
//! let tools = lib.get_tools();
//! for tool in tools {
//!     println!("Tool: {}", tool.get_name());
//! }
//! ```

pub mod library;
pub mod tool;

// Tool implementations
pub mod add_tool;
pub mod select_tool;
pub mod wiring_tool;
// pub mod poke_tool;
// pub mod edit_tool;
// pub mod text_tool;
// pub mod menu_tool;

// Supporting infrastructure
// pub mod caret;
// pub mod custom_handles;
// pub mod factory_attributes;
// pub mod factory_description;
// pub mod library_tools;
// pub mod menu_extender;
// pub mod tool_tip_maker;

// Subsystems
// pub mod key;
// pub mod move;

// Re-export core types for convenience
pub use add_tool::{AddComponentAction, AddTool};
pub use library::{BasicLibrary, Library, LibraryClone};
pub use select_tool::SelectTool;
pub use tool::{
    Action, Canvas, Circuit, ComponentDrawContext, CursorType, KeyEvent, KeyModifiers,
    LogisimVersion, MouseButton, MouseEvent, Project, Selection, Tool,
};
pub use wiring_tool::{AddWireAction, WiringTool};

/// Tool framework version for compatibility tracking
pub const TOOLS_VERSION: &str = "1.0.0";

/// Common result type for tool operations
pub type ToolResult<T> = Result<T, ToolError>;

/// Error types for tool operations
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Tool not found: {name}")]
    ToolNotFound { name: String },

    #[error("Library not found: {name}")]
    LibraryNotFound { name: String },

    #[error("Invalid tool operation: {message}")]
    InvalidOperation { message: String },

    #[error("Tool attribute error: {message}")]
    AttributeError { message: String },

    #[error("Canvas operation error: {message}")]
    CanvasError { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl ToolError {
    /// Create a new tool not found error
    pub fn tool_not_found(name: &str) -> Self {
        Self::ToolNotFound {
            name: name.to_string(),
        }
    }

    /// Create a new library not found error
    pub fn library_not_found(name: &str) -> Self {
        Self::LibraryNotFound {
            name: name.to_string(),
        }
    }

    /// Create a new invalid operation error
    pub fn invalid_operation(message: &str) -> Self {
        Self::InvalidOperation {
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_version() {
        assert_eq!(TOOLS_VERSION, "1.0.0");
    }

    #[test]
    fn test_tool_error_creation() {
        let error = ToolError::tool_not_found("test_tool");
        assert!(matches!(error, ToolError::ToolNotFound { .. }));
        assert_eq!(error.to_string(), "Tool not found: test_tool");

        let error = ToolError::library_not_found("test_lib");
        assert!(matches!(error, ToolError::LibraryNotFound { .. }));
        assert_eq!(error.to_string(), "Library not found: test_lib");

        let error = ToolError::invalid_operation("test message");
        assert!(matches!(error, ToolError::InvalidOperation { .. }));
        assert_eq!(error.to_string(), "Invalid tool operation: test message");
    }
}
