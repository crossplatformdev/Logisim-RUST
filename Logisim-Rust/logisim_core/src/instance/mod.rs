/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component Instance System
//!
//! This module provides the core infrastructure for component instantiation and management,
//! ported from Java's `com.cburch.logisim.instance` package.
//!
//! The instance system implements a factory pattern where `InstanceFactory` creates and manages
//! component instances. Each component instance is wrapped with metadata, state management,
//! and rendering capabilities.
//!
//! ## Core Architecture
//!
//! - **`InstanceFactory`**: Factory trait for creating and configuring component instances
//! - **`Instance`**: Wrapper around components providing metadata and lifecycle management
//! - **`InstanceComponent`**: Concrete component implementation with instance semantics
//! - **`InstanceState`**: Runtime state management for component instances
//! - **`Port`**: Pin/connection point definitions for components
//! - **`InstancePainter`**: Rendering integration for component visualization
//!
//! ## Example Usage
//!
//! ```rust
//! use logisim_core::instance::{InstanceFactory, Instance, Port};
//! use logisim_core::{AttributeSet, Location, BitWidth};
//!
//! // Define a simple component factory
//! struct AndGateFactory;
//!
//! impl InstanceFactory for AndGateFactory {
//!     fn create_instance(&self, location: Location, attrs: AttributeSet) -> Instance {
//!         // Create component instance with two input ports and one output
//!         let ports = vec![
//!             Port::new(-30, -10, "input", BitWidth::create(1)),
//!             Port::new(-30, 10, "input", BitWidth::create(1)),
//!             Port::new(0, 0, "output", BitWidth::create(1)),
//!         ];
//!         Instance::new(self, location, attrs, ports)
//!     }
//! }
//! ```

pub mod instance;
pub mod instance_component;
pub mod instance_data;
pub mod instance_data_singleton;
pub mod instance_factory;
pub mod instance_logger;
pub mod instance_logger_adapter;
pub mod instance_painter;
pub mod instance_poker;
pub mod instance_poker_adapter;
pub mod instance_state;
pub mod instance_state_impl;
pub mod instance_text_field;
pub mod port;
pub mod std_attr;

// Re-export core types for convenience
pub use instance::{Instance, InstanceRef};
pub use instance_component::InstanceComponent;
pub use instance_data::{InstanceData, InstanceDataBox};
pub use instance_data_singleton::InstanceDataSingleton;
pub use instance_factory::InstanceFactory;
pub use instance_logger::InstanceLogger;
pub use instance_logger_adapter::InstanceLoggerAdapter;
pub use instance_painter::InstancePainter;
pub use instance_poker::InstancePoker;
pub use instance_poker_adapter::InstancePokerAdapter;
pub use instance_state::InstanceState;
pub use instance_state_impl::InstanceStateImpl;
pub use instance_text_field::InstanceTextField;
pub use port::{Port, PortType, PortWidth};
pub use std_attr::StdAttr;
