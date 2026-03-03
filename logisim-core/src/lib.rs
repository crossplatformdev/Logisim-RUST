//! Logisim-RUST Core Library
//!
//! This crate provides the foundational types and logic for the Logisim-RUST
//! digital circuit simulator, including:
//!
//! - Circuit data model (components, wires, ports)
//! - Complete standard component library (gates, flip-flops, memory, arithmetic, I/O, plexers)
//! - Simulation engine (signal propagation, clock management, short-circuit detection)
//! - Project and subcircuit management

pub mod circuit;
pub mod component;
pub mod error;
pub mod history;
pub mod project;
pub mod simulation;
pub mod value;

pub use circuit::{Circuit, CircuitId, Wire, WireEnd};
pub use component::{Component, ComponentId, ComponentKind, Port, PortDirection};
pub use error::{LogisimError, Result};
pub use history::{UndoAction, UndoHistory};
pub use project::Project;
pub use simulation::{SimulationState, Simulator};
pub use value::{BitWidth, Value};
