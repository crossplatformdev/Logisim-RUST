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

pub mod build_info;
pub mod circ_format;
pub mod circ_parser;
pub mod circ_serializer;
pub mod comp;
pub mod component;
pub mod contracts;
pub mod components;
pub mod data;
pub mod event;
pub mod file;
pub mod hdl;
pub mod instance;
pub mod integrations;
pub mod netlist;
pub mod prefs;
pub mod signal;
pub mod simulation;
pub mod tools;
pub mod std;
pub mod util;

// Re-export core types for convenience
pub use build_info::BuildInfo;
pub use circ_parser::{CircParseError, CircParser, CircuitProject};
pub use circ_serializer::{CircSerializeError, CircSerializer};
pub use comp::{
    AbstractComponent, AbstractComponentFactory, Color, Component, ComponentDrawContext,
    ComponentEvent, ComponentFactory, ComponentId, ComponentListener, ComponentUserEvent,
    DrawCommand, EndData, GraphicsContext, Pin, PinDirection,
};
<<<<<<< HEAD
pub use component::{
    Component as SimComponent, ComponentId as SimComponentId, Pin as SimPin, ComponentFactory as SimComponentFactory,
};
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> origin/copilot/fix-18c0ad2a-b9ca-481a-a889-a9d22d7537a2
=======
>>>>>>> origin/copilot/fix-353cf926-c1da-412a-a398-aa596ad1efc8
=======
>>>>>>> origin/copilot/fix-8670ab67-e80b-4622-811f-2cfa65e1bade
=======
>>>>>>> origin/copilot/fix-f356266b-bb16-4b5b-92f6-f52c4c0f6a69
=======
>>>>>>> origin/copilot/fix-1c9fc52b-264e-4c3a-9b7c-05621b80788e
=======
>>>>>>> origin/copilot/fix-3257658f-2b32-41b0-9150-144ce65274f6
=======
>>>>>>> origin/copilot/fix-19481340-5a30-4cd5-b8ee-114850e6083a
pub use contracts::{
    BaseComponentListenerContract, BaseDocumentListenerContract, BaseKeyListenerContract,
    BaseLayoutManagerContract, BaseListDataListenerContract, BaseMouseInputListenerContract,
    BaseMouseListenerContract, BaseMouseMotionListenerContract, BaseWindowFocusListenerContract,
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
    BaseWindowListenerContract, ComponentEvent as ContractComponentEvent, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
=======
    BaseWindowListenerContract, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
>>>>>>> origin/copilot/fix-18c0ad2a-b9ca-481a-a889-a9d22d7537a2
=======
    BaseWindowListenerContract, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
>>>>>>> origin/copilot/fix-353cf926-c1da-412a-a398-aa596ad1efc8
=======
    BaseWindowListenerContract, ComponentEvent as ContractComponentEvent, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
>>>>>>> origin/copilot/fix-8670ab67-e80b-4622-811f-2cfa65e1bade
=======
    BaseWindowListenerContract, ComponentEvent as ContractComponentEvent, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
>>>>>>> origin/copilot/fix-f356266b-bb16-4b5b-92f6-f52c4c0f6a69
=======
    BaseWindowListenerContract, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
>>>>>>> origin/copilot/fix-1c9fc52b-264e-4c3a-9b7c-05621b80788e
=======
    BaseWindowListenerContract, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
>>>>>>> origin/copilot/fix-3257658f-2b32-41b0-9150-144ce65274f6
=======
    BaseWindowListenerContract, DocumentEvent, KeyEvent, MouseEvent, WindowEvent,
>>>>>>> origin/copilot/fix-19481340-5a30-4cd5-b8ee-114850e6083a
};
pub use components::gray::*;
pub use std::{base::*, gates::*, io::*};
pub use data::{
    Attribute, AttributeSet, AttributeValue, BitWidth, Bounds, Direction, Location, StdAttr,
};
pub use event::{EventQueue, SimulatorEvent};
pub use file::{LoadFailedException, Loader, LogisimFile};
pub use hdl::{
    HdlModel, HdlModelListener, PortDescription, HdlContent, HdlContentEditor,
    VhdlParser, VhdlContentComponent, BlifParser, BlifContentComponent,
    HdlFile, HdlFileType, HdlStrings, VhdlEntityComponent, BlifCircuitComponent, HdlLibrary,
};
pub use instance::{
    Instance, InstanceComponent, InstanceData, InstanceFactory, InstanceState, Port, PortType, PortWidth,
};
pub use integrations::{FpgaError, PluginError, TclError, VhdlError};
pub use netlist::{NetId, Netlist, NodeId};
pub use prefs::AppPreferences;
pub use signal::{Bus, BusWidth, Signal, Timestamp, Value};
pub use simulation::Simulation;
pub use tools::{Tool, Library, BasicLibrary, Canvas, Project, Circuit, Action, Selection, CursorType, ToolResult, ToolError};
pub use std::wiring::WiringLibrary;
pub use util::{
    Cache, CollectionUtil, FileUtil, LocaleManager, StringCache, StringGetter, StringUtil,
};
