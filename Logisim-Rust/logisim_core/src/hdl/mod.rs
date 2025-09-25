/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL (Hardware Description Language) support infrastructure
//!
//! This module provides fundamental HDL support functionality including:
//! - File I/O operations for HDL files
//! - HDL model representation and management
//! - Event-driven architecture for HDL model changes
//! - Internationalization support

pub mod hdl_file;
pub mod hdl_model;
pub mod hdl_model_listener;
pub mod strings;

pub use hdl_file::HdlFile;
pub use hdl_model::{HdlModel, PortDescription};
pub use hdl_model_listener::HdlModelListener;
pub use strings::Strings;
