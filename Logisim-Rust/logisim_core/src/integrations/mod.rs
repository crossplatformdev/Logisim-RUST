//! Integration stubs for external systems
//!
//! This module provides compatibility stubs for external integrations including
//! VHDL generation, TCL scripting, and FPGA toolchain integration. These stubs
//! maintain API compatibility while gracefully handling unsupported operations.

pub mod vhdl;
pub mod tcl;
pub mod fpga;
pub mod plugins;

pub use vhdl::*;
pub use tcl::*;
pub use fpga::*;
pub use plugins::*;