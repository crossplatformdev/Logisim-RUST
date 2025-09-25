/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Abstract base classes and traits for I/O components
//!
//! This module provides the foundational abstractions used by concrete
//! I/O component implementations. It includes base traits for simple
//! I/O components and common functionality.

/// Trait for simple I/O components that can be either input or output
pub trait SimpleIoComponent {
    /// Returns true if this is an input component (sends data to circuit)
    fn is_input_component(&self) -> bool;
    
    /// Returns true if this is an output component (receives data from circuit)
    fn is_output_component(&self) -> bool {
        !self.is_input_component()
    }
}

/// Common I/O library functionality
pub struct IoLibraryBase;

impl IoLibraryBase {
    pub const ID: &'static str = "I/O";
}