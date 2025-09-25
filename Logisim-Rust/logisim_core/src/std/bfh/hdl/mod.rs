/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL Generation for BFH Components
//!
//! This module provides HDL (VHDL/Verilog) code generation for BFH components
//! to support FPGA synthesis and deployment.

pub mod bin_to_bcd_hdl;
pub mod bcd_to_seven_segment_hdl;

// Re-export HDL generators
pub use bin_to_bcd_hdl::BinToBcdHdlGenerator;
pub use bcd_to_seven_segment_hdl::BcdToSevenSegmentHdlGenerator;