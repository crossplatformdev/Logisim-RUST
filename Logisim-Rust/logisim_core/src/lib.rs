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
//!
//! ## Example Usage
//!
//! ```rust
//! use logisim_core::*;
//! use logisim_core::component::AndGate;
//! use logisim_core::signal::{Value, BusWidth};
//! use logisim_core::simulation::Simulation;
//!
//! // Create a simulation
//! let mut sim = Simulation::new();
//!
//! // Add an AND gate
//! let gate = Box::new(AndGate::new(ComponentId(1)));
//! let gate_id = sim.add_component(gate);
//!
//! // Create nodes for connections
//! let input_a = sim.netlist_mut().create_named_node(BusWidth(1), "A".to_string());
//! let input_b = sim.netlist_mut().create_named_node(BusWidth(1), "B".to_string());
//! let output = sim.netlist_mut().create_named_node(BusWidth(1), "Y".to_string());
//!
//! // Connect the gate
//! sim.netlist_mut().connect(gate_id, "A".to_string(), input_a).unwrap();
//! sim.netlist_mut().connect(gate_id, "B".to_string(), input_b).unwrap();
//! sim.netlist_mut().connect(gate_id, "Y".to_string(), output).unwrap();
//!
//! // Set up initial conditions and run simulation
//! sim.reset();
//! sim.schedule_signal_change(Timestamp(10), input_a, Signal::new_single(Value::High), ComponentId(0));
//! sim.schedule_signal_change(Timestamp(10), input_b, Signal::new_single(Value::High), ComponentId(0));
//! sim.run().unwrap();
//! ```

pub mod circ_format;
pub mod circ_parser;
pub mod circ_serializer;
pub mod component;
pub mod event;
pub mod integrations;
pub mod netlist;
pub mod signal;
pub mod simulation;

// Re-export core types for convenience
pub use circ_parser::{CircParseError, CircParser, CircuitProject};
pub use circ_serializer::{CircSerializeError, CircSerializer};
pub use component::{Component, ComponentId, Pin};
pub use event::{EventQueue, SimulatorEvent};
pub use integrations::{FpgaError, PluginError, TclError, VhdlError};
pub use netlist::{NetId, Netlist, NodeId};
pub use signal::{Bus, BusWidth, Signal, Timestamp, Value};
pub use simulation::Simulation;
