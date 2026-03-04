//! Error types for Logisim-RUST.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, LogisimError>;

#[derive(Debug, Error)]
pub enum LogisimError {
    #[error("Circuit not found: {0}")]
    CircuitNotFound(String),

    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Wire endpoint not found")]
    WireEndpointNotFound,

    #[error("Port not found: component={component}, port={port}")]
    PortNotFound { component: String, port: String },

    #[error("Short circuit detected on wire at ({x}, {y})")]
    ShortCircuit { x: i32, y: i32 },

    #[error("Simulation oscillation detected (step limit exceeded)")]
    OscillationDetected,

    #[error("Subcircuit recursion detected: {0}")]
    RecursiveSubcircuit(String),

    #[error("Invalid bit width: {0}")]
    InvalidBitWidth(u32),

    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Simulation error: {0}")]
    Simulation(String),

    #[error("Parse error: {0}")]
    Parse(String),
}
