//! # Logisim Core
//! 
//! Core simulation kernel for the Logisim-RUST digital logic simulator.
//! This crate provides the foundational types and algorithms for simulating
//! digital circuits, including event-driven simulation, signal propagation,
//! and component modeling.
//!
//! ## Architecture
//!
//! The simulation kernel is built around several key concepts:
//!
//! - **Signals**: Represent logical values (high, low, unknown, error) and buses
//! - **Components**: Digital logic elements that process signals 
//! - **Events**: Time-ordered simulation events that drive the simulation
//! - **Netlist**: The network of connected components and signals
//! - **Simulation**: The main simulation engine that processes events

pub mod signal;
pub mod component;
pub mod event;
pub mod netlist;
pub mod simulation;

// Re-export core types for convenience
pub use signal::{Signal, Value, Bus, BusWidth, Timestamp};
pub use component::{Component, Pin, ComponentId};
pub use event::{SimulatorEvent, EventQueue};
pub use netlist::{NodeId, NetId, Netlist};
pub use simulation::Simulation;