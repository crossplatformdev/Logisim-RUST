//! Main simulation engine and orchestration.
//!
//! This module implements the core simulation loop, event processing,
//! and component management for the digital logic simulator.

use crate::comp::{Component, ComponentId, ClockEdge, UpdateResult};
use crate::event::{EventQueue, EventType};
use crate::netlist::{Netlist, NodeId};
use crate::signal::{Signal, Timestamp, Value};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during simulation
#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("Component not found: {0}")]
    ComponentNotFound(ComponentId),
    #[error("Node not found: {0}")]
    NodeNotFound(NodeId),
    #[error("Simulation oscillation detected at time {0}")]
    OscillationDetected(Timestamp),
    #[error("Maximum simulation time exceeded: {0}")]
    TimeoutExceeded(Timestamp),
    #[error("Netlist error: {0}")]
    NetlistError(String),
}

/// Configuration for simulation behavior
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Maximum simulation time before timeout
    pub max_time: Option<Timestamp>,
    /// Maximum number of events to process
    pub max_events: Option<usize>,
    /// Oscillation detection threshold (events at same time)
    pub oscillation_threshold: usize,
    /// Enable debug output
    pub debug: bool,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            max_time: Some(Timestamp(10_000)), // Reduced timeout for testing
            max_events: Some(1_000),           // Reduced max events
            oscillation_threshold: 100,        // Reduced oscillation threshold
            debug: false,
        }
    }
}

/// Callback for signal change events (used by chronogram)
pub type SignalChangeCallback = Box<dyn FnMut(NodeId, Timestamp, &Signal) + Send + Sync>;

/// Statistics collected during simulation
#[derive(Debug, Clone, Default)]
pub struct SimulationStats {
    /// Total events processed
    pub events_processed: usize,
    /// Current simulation time
    pub current_time: Timestamp,
    /// Number of propagation steps
    pub propagation_steps: usize,
    /// Number of components updated
    pub components_updated: usize,
    /// Number of clock ticks
    pub clock_ticks: usize,
}

/// Main simulation engine
pub struct Simulation {
    /// Event queue for scheduling simulation events
    event_queue: EventQueue,
    /// The netlist representing the circuit
    netlist: Netlist,
    /// All components in the simulation
    components: HashMap<ComponentId, Box<dyn Component>>,
    /// Simulation configuration
    config: SimulationConfig,
    /// Simulation statistics
    stats: SimulationStats,
    /// Current clock state for sequential components
    clock_state: Value,
    /// Previous clock state for edge detection
    prev_clock_state: Value,
    /// Signal change callbacks for external observers (e.g., chronogram)
    signal_callbacks: Vec<SignalChangeCallback>,
}

impl Simulation {
    /// Create a new simulation
    pub fn new() -> Self {
        Simulation {
            event_queue: EventQueue::new(),
            netlist: Netlist::new(),
            components: HashMap::new(),
            config: SimulationConfig::default(),
            stats: SimulationStats::default(),
            clock_state: Value::Low,
            prev_clock_state: Value::Low,
            signal_callbacks: Vec::new(),
        }
    }

    /// Create a new simulation with custom configuration
    pub fn with_config(config: SimulationConfig) -> Self {
        Simulation {
            event_queue: EventQueue::new(),
            netlist: Netlist::new(),
            components: HashMap::new(),
            config,
            stats: SimulationStats::default(),
            clock_state: Value::Low,
            prev_clock_state: Value::Low,
            signal_callbacks: Vec::new(),
        }
    }

    /// Add a component to the simulation
    pub fn add_component(&mut self, component: Box<dyn Component>) -> ComponentId {
        let id = component.id();
        self.components.insert(id, component);
        id
    }

    /// Remove a component from the simulation
    pub fn remove_component(&mut self, id: ComponentId) -> Option<Box<dyn Component>> {
        self.components.remove(&id)
    }

    /// Get a component by ID
    pub fn get_component(&self, id: ComponentId) -> Option<&dyn Component> {
        self.components.get(&id).map(|c| c.as_ref())
    }

    /// Get a component by ID (mutable)
    pub fn get_component_mut(&mut self, id: ComponentId) -> Option<&mut Box<dyn Component>> {
        self.components.get_mut(&id)
    }

    /// Get the netlist
    pub fn netlist(&self) -> &Netlist {
        &self.netlist
    }

    /// Get the netlist (mutable)
    pub fn netlist_mut(&mut self) -> &mut Netlist {
        &mut self.netlist
    }

    /// Get current simulation time
    pub fn current_time(&self) -> Timestamp {
        self.event_queue.current_time()
    }

    /// Get simulation statistics
    pub fn stats(&self) -> &SimulationStats {
        &self.stats
    }

    /// Check if there are events pending in the queue
    pub fn has_pending_events(&self) -> bool {
        !self.event_queue.is_empty()
    }

    /// Add a signal change callback (for chronogram or other observers)
    pub fn add_signal_callback(&mut self, callback: SignalChangeCallback) {
        self.signal_callbacks.push(callback);
    }

    /// Get all node IDs that have signals (for chronogram signal selection)
    pub fn get_all_node_ids(&self) -> Vec<NodeId> {
        self.netlist.get_all_node_ids()
    }

    /// Get the current signal value for a node
    pub fn get_node_signal(&self, node_id: NodeId) -> Option<Signal> {
        self.netlist.get_signal(node_id).cloned()
    }

    /// Reset the simulation to initial state
    pub fn reset(&mut self) {
        // Clear event queue
        self.event_queue.clear();

        // Reset all components
        for component in self.components.values_mut() {
            component.reset();
        }

        // Reset clock state
        self.clock_state = Value::Low;
        self.prev_clock_state = Value::Low;

        // Reset statistics
        self.stats = SimulationStats::default();

        // Schedule initial reset event
        self.event_queue.schedule_reset(Timestamp(0));

        if self.config.debug {
            println!("Simulation reset");
        }
    }

    /// Step the simulation by one event
    pub fn step(&mut self) -> Result<bool, SimulationError> {
        // Check if we have any events to process
        if self.event_queue.is_empty() {
            return Ok(false); // No more events
        }

        // Check timeout conditions
        if let Some(max_time) = self.config.max_time {
            if self.current_time() >= max_time {
                return Err(SimulationError::TimeoutExceeded(max_time));
            }
        }

        if let Some(max_events) = self.config.max_events {
            if self.stats.events_processed >= max_events {
                return Err(SimulationError::TimeoutExceeded(self.current_time()));
            }
        }

        // Get the next event
        let event = self.event_queue.pop().unwrap();
        self.stats.current_time = event.time;
        self.stats.events_processed += 1;

        if self.config.debug {
            println!(
                "Processing event at time {}: {:?}",
                event.time, event.event_type
            );
        }

        // Process the event
        match event.event_type {
            EventType::SignalChange {
                node_id,
                new_signal,
                source_component,
            } => {
                self.process_signal_change(event.time, node_id, new_signal, source_component)?;
            }
            EventType::ClockTick => {
                self.process_clock_tick(event.time)?;
            }
            EventType::ComponentUpdate { component_id } => {
                self.process_component_update(event.time, component_id)?;
            }
            EventType::Reset => {
                self.process_reset(event.time)?;
            }
        }

        Ok(true) // More events may be available
    }

    /// Run the simulation until completion or error
    pub fn run(&mut self) -> Result<(), SimulationError> {
        while self.step()? {
            // Continue until no more events or error
        }
        Ok(())
    }

    /// Run the simulation for a specific number of steps
    pub fn run_steps(&mut self, max_steps: usize) -> Result<usize, SimulationError> {
        let mut steps = 0;
        while steps < max_steps {
            if !self.step()? {
                break; // No more events
            }
            steps += 1;
        }
        Ok(steps)
    }

    /// Run the simulation until a specific time
    pub fn run_until(&mut self, target_time: Timestamp) -> Result<(), SimulationError> {
        while let Some(next_event_time) = self.event_queue.next_event_time() {
            if next_event_time >= target_time {
                break; // Next event would be at or after target time
            }
            if !self.step()? {
                break; // No more events
            }
        }
        Ok(())
    }

    /// Schedule a signal change
    pub fn schedule_signal_change(
        &mut self,
        time: Timestamp,
        node_id: NodeId,
        signal: Signal,
        source_component: ComponentId,
    ) {
        self.event_queue
            .schedule_signal_change(time, node_id, signal, source_component);
    }

    /// Schedule a clock tick
    pub fn schedule_clock_tick(&mut self, time: Timestamp) {
        self.event_queue.schedule_clock_tick(time);
    }

    /// Process a signal change event
    fn process_signal_change(
        &mut self,
        time: Timestamp,
        node_id: NodeId,
        new_signal: Signal,
        _source_component: ComponentId,
    ) -> Result<(), SimulationError> {
        // Check if the signal actually changed to avoid infinite loops
        let signal_changed = if let Some(current_signal) = self.netlist.get_node_signal(node_id) {
            current_signal != &new_signal
        } else {
            true
        };

        if !signal_changed {
            return Ok(()); // No change, no propagation needed
        }

        // Update the signal at the node
        self.netlist
            .set_node_signal(node_id, new_signal.clone())
            .map_err(|e| SimulationError::NetlistError(e.to_string()))?;

        // Trigger signal change callbacks (for chronogram, etc.)
        for callback in &mut self.signal_callbacks {
            callback(node_id, time, &new_signal);
        }

        // Get all components affected by this signal change
        let affected_components = self.netlist.get_affected_components(node_id);

        if self.config.debug {
            println!(
                "Signal change at node {} affects {} components",
                node_id,
                affected_components.len()
            );
        }

        // Schedule updates for all affected components
        for component_id in affected_components {
            // Schedule a component update event with a small delay
            self.event_queue
                .schedule_component_update(time.add_delay(1), component_id);
        }

        self.stats.propagation_steps += 1;
        Ok(())
    }

    /// Process a clock tick event
    fn process_clock_tick(&mut self, time: Timestamp) -> Result<(), SimulationError> {
        self.prev_clock_state = self.clock_state;
        self.clock_state = match self.clock_state {
            Value::Low => Value::High,
            Value::High => Value::Low,
            _ => Value::Low, // Default to low for unknown states
        };

        let edge = if self.prev_clock_state == Value::Low && self.clock_state == Value::High {
            ClockEdge::Rising
        } else if self.prev_clock_state == Value::High && self.clock_state == Value::Low {
            ClockEdge::Falling
        } else {
            return Ok(()); // No edge detected
        };

        if self.config.debug {
            println!("Clock {:?} edge at time {}", edge, time);
        }

        // Notify all sequential components of the clock edge
        let component_ids: Vec<_> = self.components.keys().copied().collect();
        for component_id in component_ids {
            if let Some(component) = self.components.get_mut(&component_id) {
                if component.is_sequential() {
                    let result = component.clock_edge(edge, time);
                    self.handle_update_result(time, component_id, result)?;
                }
            }
        }

        self.stats.clock_ticks += 1;
        Ok(())
    }

    /// Process a component update event
    fn process_component_update(
        &mut self,
        time: Timestamp,
        component_id: ComponentId,
    ) -> Result<(), SimulationError> {
        if let Some(component) = self.components.get_mut(&component_id) {
            // Update component inputs from connected nodes
            let connections = self.netlist.get_component_connections(component_id);
            for connection in connections {
                if let Some(pin) = component.get_pin_mut(&connection.pin_name) {
                    if pin.is_input() {
                        if let Some(node_signal) = self.netlist.get_node_signal(connection.node_id)
                        {
                            let _ = pin.set_signal(node_signal.clone());
                        }
                    }
                }
            }

            // Update the component
            let result = component.update(time);
            self.handle_update_result(time, component_id, result)?;
            self.stats.components_updated += 1;
        } else {
            return Err(SimulationError::ComponentNotFound(component_id));
        }
        Ok(())
    }

    /// Process a reset event
    fn process_reset(&mut self, time: Timestamp) -> Result<(), SimulationError> {
        if self.config.debug {
            println!("Processing reset at time {}", time);
        }

        // Reset all components and schedule initial updates
        let component_ids: Vec<_> = self.components.keys().copied().collect();
        for component_id in component_ids {
            if let Some(component) = self.components.get_mut(&component_id) {
                component.reset();
                // Schedule initial update after reset
                self.event_queue
                    .schedule_component_update(time.add_delay(1), component_id);
            }
        }

        Ok(())
    }

    /// Handle the result of a component update
    fn handle_update_result(
        &mut self,
        current_time: Timestamp,
        component_id: ComponentId,
        result: UpdateResult,
    ) -> Result<(), SimulationError> {
        if !result.state_changed {
            return Ok(());
        }

        // Process output signals
        for (pin_name, signal) in result.outputs {
            // Find the node connected to this output pin
            if let Some(node_id) = self.netlist.get_pin_node(component_id, &pin_name) {
                // Check if this would actually change the signal at the node
                let should_propagate =
                    if let Some(current_signal) = self.netlist.get_node_signal(node_id) {
                        current_signal != &signal
                    } else {
                        true
                    };

                if should_propagate {
                    // Schedule signal change with appropriate delay
                    let event_time = current_time.add_delay(result.delay);
                    self.event_queue.schedule_signal_change(
                        event_time,
                        node_id,
                        signal,
                        component_id,
                    );
                }
            }
        }

        Ok(())
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comp::{AndGate, ClockedLatch};
    use crate::signal::{BusWidth, Value};

    #[test]
    fn test_simulation_creation() {
        let sim = Simulation::new();
        assert_eq!(sim.current_time(), Timestamp(0));
        assert_eq!(sim.stats().events_processed, 0);
    }

    #[test]
    fn test_add_component() {
        let mut sim = Simulation::new();
        let gate = Box::new(AndGate::new(ComponentId(1)));
        let id = sim.add_component(gate);

        assert_eq!(id, ComponentId(1));
        assert!(sim.get_component(id).is_some());
    }

    #[test]
    fn test_simple_and_gate_simulation() {
        let mut sim = Simulation::new();

        // Create AND gate
        let gate = Box::new(AndGate::new(ComponentId(1)));
        let gate_id = sim.add_component(gate);

        // Create nodes for inputs and output
        let a_node = sim
            .netlist_mut()
            .create_named_node(BusWidth(1), "A".to_string());
        let b_node = sim
            .netlist_mut()
            .create_named_node(BusWidth(1), "B".to_string());
        let y_node = sim
            .netlist_mut()
            .create_named_node(BusWidth(1), "Y".to_string());

        // Connect gate pins to nodes
        sim.netlist_mut()
            .connect(gate_id, "A".to_string(), a_node)
            .unwrap();
        sim.netlist_mut()
            .connect(gate_id, "B".to_string(), b_node)
            .unwrap();
        sim.netlist_mut()
            .connect(gate_id, "Y".to_string(), y_node)
            .unwrap();

        // Reset simulation
        sim.reset();

        // Set initial inputs
        sim.schedule_signal_change(
            Timestamp(10),
            a_node,
            Signal::new_single(Value::High),
            ComponentId(0), // External source
        );
        sim.schedule_signal_change(
            Timestamp(15),
            b_node,
            Signal::new_single(Value::High),
            ComponentId(0), // External source
        );

        // Run simulation
        let result = sim.run();
        if let Err(e) = &result {
            println!("Simple AND gate simulation error: {}", e);
        }
        assert!(result.is_ok());

        // Check that events were processed
        assert!(sim.stats().events_processed > 0);
        assert!(sim.current_time() > Timestamp(0));
    }

    #[test]
    fn test_clocked_latch_simulation() {
        let mut sim = Simulation::new();

        // Create latch
        let latch = Box::new(ClockedLatch::new(ComponentId(1)));
        let latch_id = sim.add_component(latch);

        // Create nodes
        let d_node = sim
            .netlist_mut()
            .create_named_node(BusWidth(1), "D".to_string());
        let clk_node = sim
            .netlist_mut()
            .create_named_node(BusWidth(1), "CLK".to_string());
        let q_node = sim
            .netlist_mut()
            .create_named_node(BusWidth(1), "Q".to_string());

        // Connect latch pins to nodes
        sim.netlist_mut()
            .connect(latch_id, "D".to_string(), d_node)
            .unwrap();
        sim.netlist_mut()
            .connect(latch_id, "CLK".to_string(), clk_node)
            .unwrap();
        sim.netlist_mut()
            .connect(latch_id, "Q".to_string(), q_node)
            .unwrap();

        // Reset simulation
        sim.reset();

        // Set up test sequence
        sim.schedule_signal_change(
            Timestamp(10),
            d_node,
            Signal::new_single(Value::High),
            ComponentId(0),
        );
        sim.schedule_signal_change(
            Timestamp(20),
            clk_node,
            Signal::new_single(Value::High), // Rising edge
            ComponentId(0),
        );

        // Run simulation
        let result = sim.run();
        assert!(result.is_ok());

        // Verify simulation ran
        assert!(sim.stats().events_processed > 0);
    }

    #[test]
    fn test_simulation_reset() {
        let mut sim = Simulation::new();

        // Add some events
        sim.schedule_clock_tick(Timestamp(100));
        assert!(!sim.event_queue.is_empty());

        // Reset
        sim.reset();

        // Should have reset event scheduled
        assert!(!sim.event_queue.is_empty());
        assert_eq!(sim.current_time(), Timestamp(0));
        assert_eq!(sim.stats().events_processed, 0);
    }

    #[test]
    fn test_run_steps() {
        let mut sim = Simulation::new();

        // Schedule multiple events
        sim.schedule_clock_tick(Timestamp(10));
        sim.schedule_clock_tick(Timestamp(20));
        sim.schedule_clock_tick(Timestamp(30));

        // Run only 2 steps
        let steps = sim.run_steps(2).unwrap();
        assert_eq!(steps, 2);
        assert_eq!(sim.stats().events_processed, 2);

        // Should have one event left
        assert!(!sim.event_queue.is_empty());
    }

    #[test]
    fn test_run_until() {
        let mut sim = Simulation::new();

        sim.schedule_clock_tick(Timestamp(10));
        sim.schedule_clock_tick(Timestamp(20));
        sim.schedule_clock_tick(Timestamp(30));

        // Run until time 25
        let result = sim.run_until(Timestamp(25));
        assert!(result.is_ok());

        // Should have processed events up to time 25 (including at least the clock events at 10 and 20)
        // Note that events_processed may include other events like reset
        assert!(sim.stats().events_processed >= 2);
        assert_eq!(sim.current_time(), Timestamp(20));
    }
}
