/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance State Implementation
//!
//! This module provides a concrete implementation of the InstanceState trait
//! for use during simulation. This is equivalent to Java's `InstanceStateImpl` class.

use crate::data::{Attribute, AttributeSet};
use crate::{Value};
use crate::instance::{Instance, InstanceData, InstanceFactory, InstanceState, Port};
use crate::netlist::NetId;
use crate::signal::Timestamp;
use std::collections::HashMap;

/// Concrete implementation of InstanceState for simulation contexts.
///
/// This struct provides the runtime state management for component instances
/// during simulation, bridging between the simulation engine and component logic.
#[derive(Debug)]
pub struct InstanceStateImpl {
    /// The instance this state belongs to
    instance: Instance,
    /// Component-specific data storage
    data: Option<Box<dyn InstanceData>>,
    /// Current port values (cached from simulation)
    port_values: HashMap<usize, Value>,
    /// Current simulation timestamp
    timestamp: Timestamp,
    /// Simulation tick counter
    tick_count: u64,
    /// Whether this component is in the root circuit
    is_root: bool,
}

impl InstanceStateImpl {
    /// Creates a new instance state implementation.
    pub fn new(instance: Instance) -> Self {
        Self {
            instance,
            data: None,
            port_values: HashMap::new(),
            timestamp: Timestamp::new(0),
            tick_count: 0,
            is_root: true,
        }   
    }

    /// Updates the port value cache from simulation.
    pub fn update_port_value(&mut self, port_index: usize, value: Value) {
        self.port_values.insert(port_index, value);
    }

    /// Updates the simulation timestamp.
    pub fn update_timestamp(&mut self, timestamp: Timestamp, tick_count: u64) {
        self.timestamp = timestamp;
        self.tick_count = tick_count;
    }
}

impl InstanceState for InstanceStateImpl {
    fn fire_invalidated(&mut self) {
        self.instance.fire_invalidated();
    }

    fn get_attribute_set(&self) -> &AttributeSet {
        self.instance.attribute_set()
    }

    fn get_attribute_value_erased(&self, attr: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        // In a full implementation, this would handle the type erasure properly
        // For now, return None as a placeholder
        let _ = attr;
        None
    }

    fn get_data(&self) -> Option<&dyn InstanceData> {
        self.data.as_deref()
    }

    fn get_data_mut(&mut self) -> Option<&mut (dyn InstanceData + '_)> {
        self.data.as_deref_mut()
    }

    fn get_factory(&self) -> &dyn InstanceFactory {
        self.instance.factory()
    }

    fn get_instance(&self) -> &Instance {
        &self.instance
    }

    fn get_port_index(&self, port: &Port) -> Option<usize> {
        self.instance.ports().iter().position(|p| std::ptr::eq(p, port))
    }

    fn get_port_value(&self, port_index: usize) -> Value {
        self.port_values.get(&port_index).copied().unwrap_or(Value::Unknown)
    }

    fn get_port_net(&self, _port_index: usize) -> Option<NetId> {
        // Would need access to netlist/simulation context
        None
    }

    fn get_tick_count(&self) -> u64 {
        self.tick_count
    }

    fn get_timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn is_circuit_root(&self) -> bool {
        self.is_root
    }

    fn is_port_connected(&self, _port_index: usize) -> bool {
        // Would need access to netlist
        true // Default assumption
    }

    fn set_data(&mut self, data: Box<dyn InstanceData>) {
        self.data = Some(data);
    }

    fn set_port_value(&mut self, port_index: usize, value: Value, _delay: u32) {
        // In a full implementation, this would schedule the value change
        // through the simulation engine. For now, just update immediately.
        self.port_values.insert(port_index, value);
    }

    fn schedule_evaluation(&mut self, _delay: u32) {
        // Would schedule re-evaluation through simulation engine
    }

    fn get_port(&self, index: usize) -> Option<&Port> {
        self.instance.get_port(index)
    }

    fn get_port_count(&self) -> usize {
        self.instance.ports().len()
    }

    fn is_input_port(&self, port_index: usize) -> bool {
        self.get_port(port_index)
            .map(|p| matches!(p.port_type(), crate::instance::PortType::Input | crate::instance::PortType::InOut))
            .unwrap_or(false)
    }

    fn is_output_port(&self, port_index: usize) -> bool {
        self.get_port(port_index)
            .map(|p| matches!(p.port_type(), crate::instance::PortType::Output | crate::instance::PortType::InOut))
            .unwrap_or(false)
    }
}