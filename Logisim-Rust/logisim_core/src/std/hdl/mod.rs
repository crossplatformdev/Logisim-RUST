/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL Components Library
//!
//! This module provides the HDL components equivalent to the Java `logisim.std.hdl` package.
//! It contains HDL-based components including VHDL entities and BLIF circuits, along with
//! their parsers, attributes, and content management systems.
//!
//! ## Architecture
//!
//! The HDL module is organized into several key areas:
//! - **Components**: VHDL and BLIF component implementations
//! - **Parsers**: HDL content parsing and validation
//! - **Attributes**: Component-specific attributes and properties
//! - **Content**: HDL content management and editing
//! - **Library**: HDL component library registration
//!
//! ## Migration from Java
//!
//! This module ports functionality from:
//! - `com.cburch.logisim.std.hdl.*` (17 files)
//!
//! The Rust implementation maintains API compatibility while leveraging
//! Rust's type safety and performance characteristics.

// Core HDL types and infrastructure
pub mod hdl_content;
pub mod hdl_content_attr;
pub mod hdl_content_editor;
pub mod hdl_circuit;
pub mod hdl_library;

// VHDL-specific components
pub mod vhdl_entity;
pub mod vhdl_entity_attr;
pub mod vhdl_content;
pub mod vhdl_parser;
pub mod vhdl_generator;

// BLIF-specific components  
pub mod blif_circuit;
pub mod blif_content;
pub mod blif_parser;
pub mod attributes;

// Dense logic circuit support
pub mod dense_logic_circuit;
pub mod dense_logic_builder;

// Generic interface components
pub mod generic_interface;

// Re-export commonly used types
pub use hdl_content::{HdlContent, BasicHdlContent, HdlContentAttribute as HdlContentAttributeType, BasicHdlContentEditor};
pub use hdl_content_attr::HdlContentAttribute;
pub use hdl_content_editor::HdlContentEditor;
pub use hdl_circuit::*;
pub use hdl_library::*;

pub use vhdl_entity::VhdlEntityComponent;
pub use vhdl_entity_attr::{VhdlEntityContentAttribute, VhdlEntityNameAttribute, VhdlEntityAttributes};
pub use vhdl_content::VhdlContentComponent;
pub use vhdl_parser::{VhdlParser, VhdlResult};
pub use vhdl_generator::*;

pub use blif_circuit::BlifCircuitComponent;  
pub use blif_content::BlifContentComponent;
pub use blif_parser::{BlifParser, BlifResult};

pub use dense_logic_circuit::*;
pub use dense_logic_builder::*;
pub use generic_interface::*;