//! HDL Support Module
//!
//! This module provides Hardware Description Language (HDL) support for Logisim-RUST,
//! including VHDL and BLIF format parsing, content management, and code generation.
//! It serves as a port of the Java com.cburch.hdl package and related HDL functionality.
//!
//! ## Architecture
//!
//! The HDL module is organized into several key components:
//! - **Model**: Core HDL model interfaces and data structures
//! - **Content**: Base classes for HDL content management
//! - **Parsers**: VHDL and BLIF format parsers
//! - **Components**: HDL entity components and attributes
//! - **Generation**: HDL code generation and template systems
//! - **File I/O**: HDL file loading and saving operations
//!  
//! ## Migration from Java
//!
//! This module ports functionality from:
//! - `com.cburch.hdl.*` (4 files)
//! - `com.cburch.logisim.std.hdl.*` (17 files)
//!
//! The Rust implementation maintains API compatibility while leveraging
//! Rust's type safety and memory management features.

pub mod model;
pub mod content;
pub mod parsers;
pub mod components;
pub mod file_io;
pub mod strings;

// Re-export public types for convenience
pub use model::*;
pub use content::{HdlContent, HdlContentEditor, HdlContentAttribute, BasicHdlContentEditor};
pub use parsers::*;
pub use components::{VhdlEntityComponent, BlifCircuitComponent, HdlLibrary, VhdlEntityAttributes, BlifCircuitAttributes, GenericInterfaceAttributes, HdlAttributeFactory, HdlAttributeConstants};
pub use file_io::*;
pub use strings::*;