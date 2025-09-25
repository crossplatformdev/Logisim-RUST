//! Integration stubs for external systems and extensibility framework
//!
//! This module provides compatibility stubs for external integrations including
//! VHDL generation, TCL scripting, and FPGA toolchain integration, along with
//! a comprehensive extensibility framework for plugins and advanced modeling.

pub mod fpga;
pub mod plugins;
pub mod plugin_examples;
pub mod tcl;
pub mod vhdl;

pub use fpga::*;
pub use plugins::*;
pub use plugin_examples::*;
pub use tcl::*;
pub use vhdl::*;
