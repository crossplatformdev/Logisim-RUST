//! Stub plugin implementation as an example
//!
//! This module provides a complete example of how to implement a plugin for
//! the Logisim-RUST system, including advanced modeling features.
//!
//! # Warning: Example Code
//! 
//! This is example/template code to demonstrate plugin development patterns.
//! Use this as a starting point for developing real plugins.

use super::{ComponentInfo, PluginDependency, PluginInfo, PluginLibrary, PluginResult};
use crate::{Component, ComponentId};
use crate::comp::{Pin, UpdateResult};
use crate::modeling::{
    ExtensionPoint, ModelingContext, ModelingResult, SimulationEvent, SimulationObserver
};
use crate::netlist::NodeId;
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use std::collections::HashMap;

/// Example plugin that demonstrates advanced modeling features
pub struct ExamplePlugin {
    info: PluginInfo,
    initialized: bool,
}

impl ExamplePlugin {
    /// Create a new example plugin
    pub fn new() -> Self {
        let info = PluginInfo {
            name: "Example Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Demonstrates plugin development with advanced modeling".to_string(),
            author: "Logisim-RUST Contributors".to_string(),
            homepage: Some("https://github.com/crossplatformdev/Logisim-RUST".to_string()),
            dependencies: vec![
                PluginDependency {
                    name: "logisim_core".to_string(),
                    version_requirement: ">=1.0.0".to_string(),
                    optional: false,
                },
            ],
            entry_point: "example_plugin_main".to_string(),
        };
        
        Self {
            info,
            initialized: false,
        }
    }
}

impl PluginLibrary for ExamplePlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    fn components(&self) -> Vec<ComponentInfo> {
        vec![
            ComponentInfo {
                name: "Example Counter".to_string(),
                category: "Example".to_string(),
                description: "A simple counter component for demonstration".to_string(),
                icon_path: Some("icons/counter.png".to_string()),
                input_count: Some(2), // clock, reset
                output_count: Some(1), // count output
            },
            ComponentInfo {
                name: "Example Monitor".to_string(),
                category: "Example".to_string(),
                description: "Monitors signal changes for debugging".to_string(),
                icon_path: Some("icons/monitor.png".to_string()),
                input_count: Some(1), // monitored signal
                output_count: Some(0),
            },
        ]
    }

    fn create_component(
        &self,
        component_type: &str,
        id: ComponentId,
    ) -> PluginResult<Box<dyn Component>> {
        match component_type {
            "Example Counter" => Ok(Box::new(ExampleCounter::new(id))),
            "Example Monitor" => Ok(Box::new(ExampleMonitor::new(id))),
            _ => Err(super::PluginError::LoadingFailed(
                format!("Unknown component type: {}", component_type)
            )),
        }
    }

    fn initialize(&mut self) -> PluginResult<()> {
        log::info!("Initializing example plugin v{}", self.info.version);
        self.initialized = true;
        Ok(())
    }

    fn cleanup(&mut self) -> PluginResult<()> {
        log::info!("Cleaning up example plugin");
        self.initialized = false;
        Ok(())
    }
    
    fn extension_points(&self) -> Vec<Box<dyn ExtensionPoint>> {
        vec![
            Box::new(ExampleExtensionPoint::new()),
        ]
    }
    
    fn observers(&self) -> Vec<Box<dyn SimulationObserver>> {
        vec![
            Box::new(ExampleObserver::new()),
            Box::new(ClockTracker::new()),
        ]
    }
    
    fn setup_modeling(&mut self, context: &mut ModelingContext) -> PluginResult<()> {
        log::info!("Setting up advanced modeling for example plugin");
        
        // Register our extension points
        // Note: This would need a different API in the real implementation
        // For now, we'll just log that we would register them
        log::info!("Would register {} extension points", self.extension_points().len());
        
        // Register our observers
        for observer in self.observers() {
            context.observer_manager().add_observer(observer);
        }
        
        Ok(())
    }
}

impl Default for ExamplePlugin {
    fn default() -> Self {
        Self::new()
    }
}

/// Example counter component
#[derive(Debug)]
pub struct ExampleCounter {
    id: ComponentId,
    count: u32,
    pins: HashMap<String, Pin>,
}

impl ExampleCounter {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        
        // Create input pins
        pins.insert("CLK".to_string(), Pin::new_input("CLK", BusWidth(1)));
        pins.insert("RST".to_string(), Pin::new_input("RST", BusWidth(1)));
        
        // Create output pin
        pins.insert("OUT".to_string(), Pin::new_output("OUT", BusWidth(1)));
        
        Self {
            id,
            count: 0,
            pins,
        }
    }
}

impl Component for ExampleCounter {
    fn id(&self) -> ComponentId {
        self.id
    }
    
    fn name(&self) -> &str {
        "Example Counter"
    }
    
    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }
    
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }
    
    fn update(&mut self, _timestamp: Timestamp) -> UpdateResult {
        // Get input signals
        let clk_signal = self.pins.get("CLK").map(|p| p.get_signal()).cloned()
            .unwrap_or_else(|| Signal::unknown(BusWidth(1)));
        let rst_signal = self.pins.get("RST").map(|p| p.get_signal()).cloned()
            .unwrap_or_else(|| Signal::unknown(BusWidth(1)));
        
        // Simple counter logic: increment on clock rising edge, reset when reset is high
        if rst_signal.get_bit(0) == Some(Value::High) {
            self.count = 0;
        } else if clk_signal.get_bit(0) == Some(Value::High) {
            self.count = self.count.wrapping_add(1);
        }
        
        // Update output signal based on count
        let value = if self.count % 2 == 0 { Value::Low } else { Value::High };
        let output_signal = Signal::new_single(value);
        
        // Update output pin
        if let Some(out_pin) = self.pins.get_mut("OUT") {
            let _ = out_pin.set_signal(output_signal);
        }
        
        let mut result = UpdateResult::new();
        result.add_output("OUT".to_string(), Signal::new_single(value));
        result
    }
    
    fn reset(&mut self) {
        self.count = 0;
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Example monitor component that just observes signals
#[derive(Debug)]
pub struct ExampleMonitor {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    last_value: Option<Signal>,
}

impl ExampleMonitor {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("IN".to_string(), Pin::new_input("IN", BusWidth(1)));
        
        Self {
            id,
            pins,
            last_value: None,
        }
    }
}

impl Component for ExampleMonitor {
    fn id(&self) -> ComponentId {
        self.id
    }
    
    fn name(&self) -> &str {
        "Example Monitor"
    }
    
    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }
    
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }
    
    fn update(&mut self, timestamp: Timestamp) -> UpdateResult {
        // Get monitored signal
        let monitored_signal = self.pins.get("IN").map(|p| p.get_signal()).cloned()
            .unwrap_or_else(|| Signal::unknown(BusWidth(1)));
        
        // Check if the monitored signal changed
        let changed = if let Some(ref last) = self.last_value {
            last != &monitored_signal
        } else {
            true
        };
        
        if changed {
            log::debug!(
                "Monitor {}: Signal changed at {:?} - {:?}",
                self.id,
                timestamp,
                monitored_signal
            );
            self.last_value = Some(monitored_signal);
        }
        
        UpdateResult::new() // No outputs
    }
    
    fn reset(&mut self) {
        self.last_value = None;
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Example extension point for custom functionality
pub struct ExampleExtensionPoint {
    data: HashMap<String, String>,
}

impl ExampleExtensionPoint {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    pub fn set_property(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
}

impl ExtensionPoint for ExampleExtensionPoint {
    fn name(&self) -> &str {
        "example_extension"
    }
    
    fn initialize(&mut self) -> ModelingResult<()> {
        log::info!("Initializing example extension point");
        self.data.insert("initialized".to_string(), "true".to_string());
        Ok(())
    }
    
    fn cleanup(&mut self) -> ModelingResult<()> {
        log::info!("Cleaning up example extension point");
        self.data.clear();
        Ok(())
    }
}

impl Default for ExampleExtensionPoint {
    fn default() -> Self {
        Self::new()
    }
}

/// Example observer that logs simulation events
pub struct ExampleObserver {
    name: String,
    event_count: usize,
}

impl ExampleObserver {
    pub fn new() -> Self {
        Self {
            name: "Example Observer".to_string(),
            event_count: 0,
        }
    }
}

impl SimulationObserver for ExampleObserver {
    fn on_event(&mut self, event: &SimulationEvent) {
        self.event_count += 1;
        
        match event {
            SimulationEvent::SignalChanged { node_id, timestamp, .. } => {
                log::debug!("Observer: Signal changed on node {:?} at {:?}", node_id, timestamp);
            }
            SimulationEvent::SimulationReset { timestamp } => {
                log::info!("Observer: Simulation reset at {:?}", timestamp);
                self.event_count = 0;
            }
            SimulationEvent::StepCompleted { timestamp, events_processed } => {
                log::debug!("Observer: Step completed at {:?}, processed {} events", timestamp, events_processed);
            }
            _ => {
                log::trace!("Observer: Other event type received");
            }
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_interested_in(&self, event: &SimulationEvent) -> bool {
        // Only interested in certain events to reduce noise
        matches!(event, 
            SimulationEvent::SignalChanged { .. } | 
            SimulationEvent::SimulationReset { .. } |
            SimulationEvent::StepCompleted { .. }
        )
    }
}

impl Default for ExampleObserver {
    fn default() -> Self {
        Self::new()
    }
}

/// Clock edge tracking observer
pub struct ClockTracker {
    name: String,
    clock_nodes: HashMap<NodeId, Signal>,
}

impl ClockTracker {
    pub fn new() -> Self {
        Self {
            name: "Clock Tracker".to_string(),
            clock_nodes: HashMap::new(),
        }
    }
    
    pub fn register_clock_node(&mut self, node_id: NodeId) {
        self.clock_nodes.insert(node_id, Signal::unknown(BusWidth(1)));
    }
    
    pub fn get_tracked_clocks(&self) -> Vec<NodeId> {
        self.clock_nodes.keys().cloned().collect()
    }
}

impl SimulationObserver for ClockTracker {
    fn on_event(&mut self, event: &SimulationEvent) {
        match event {
            SimulationEvent::SignalChanged { node_id, old_signal, new_signal, timestamp, .. } => {
                if let Some(previous) = self.clock_nodes.get_mut(node_id) {
                    // Detect clock edges
                    if let (Some(old_bit), Some(new_bit)) = (old_signal.get_bit(0), new_signal.get_bit(0)) {
                        match (old_bit, new_bit) {
                            (Value::Low, Value::High) => {
                                log::debug!("Clock rising edge detected on node {:?} at {:?}", node_id, timestamp);
                            }
                            (Value::High, Value::Low) => {
                                log::debug!("Clock falling edge detected on node {:?} at {:?}", node_id, timestamp);
                            }
                            _ => {}
                        }
                    }
                    *previous = new_signal.clone();
                }
            }
            SimulationEvent::SimulationReset { .. } => {
                // Reset all tracked clock states
                for signal in self.clock_nodes.values_mut() {
                    *signal = Signal::unknown(BusWidth(1));
                }
            }
            _ => {}
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_interested_in(&self, event: &SimulationEvent) -> bool {
        match event {
            SimulationEvent::SignalChanged { node_id, .. } => {
                self.clock_nodes.contains_key(node_id)
            }
            SimulationEvent::SimulationReset { .. } => true,
            _ => false,
        }
    }
}

impl Default for ClockTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin entry point function (would be called by dynamic loading)
pub fn example_plugin_main() -> Box<dyn PluginLibrary> {
    Box::new(ExamplePlugin::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_plugin_creation() {
        let plugin = ExamplePlugin::new();
        assert_eq!(plugin.info().name, "Example Plugin");
        assert_eq!(plugin.info().version, "1.0.0");
        assert_eq!(plugin.components().len(), 2);
    }
    
    #[test]
    fn test_plugin_initialization() {
        let mut plugin = ExamplePlugin::new();
        assert!(!plugin.initialized);
        
        plugin.initialize().unwrap();
        assert!(plugin.initialized);
        
        plugin.cleanup().unwrap();
        assert!(!plugin.initialized);
    }
    
    #[test]
    fn test_example_counter() {
        let mut counter = ExampleCounter::new(ComponentId(1));
        
        // Test reset
        counter.reset();
        assert_eq!(counter.count, 0);
        
        // Test pin names
        let pins = counter.pins();
        assert!(pins.contains_key("CLK"));
        assert!(pins.contains_key("RST"));
        assert!(pins.contains_key("OUT"));
        
        // Test pin directions
        assert!(counter.get_pin("CLK").unwrap().is_input());
        assert!(counter.get_pin("RST").unwrap().is_input());
        assert!(counter.get_pin("OUT").unwrap().is_output());
    }
    
    #[test]
    fn test_example_observer() {
        let mut observer = ExampleObserver::new();
        assert_eq!(observer.name(), "Example Observer");
        
        let event = SimulationEvent::SimulationReset {
            timestamp: Timestamp(0),
        };
        
        assert!(observer.is_interested_in(&event));
        observer.on_event(&event);
        assert_eq!(observer.event_count, 0); // Reset resets count
    }
    
    #[test]
    fn test_clock_tracker() {
        let mut tracker = ClockTracker::new();
        let node_id = NodeId(1);
        
        tracker.register_clock_node(node_id);
        assert_eq!(tracker.get_tracked_clocks(), vec![node_id]);
        
        let event = SimulationEvent::SignalChanged {
            node_id,
            old_signal: Signal::new_single(Value::Low),
            new_signal: Signal::new_single(Value::High),
            timestamp: Timestamp(100),
            source: ComponentId(0),
        };
        
        assert!(tracker.is_interested_in(&event));
        tracker.on_event(&event);
    }
    
    #[test]
    fn test_extension_point() {
        let mut ext = ExampleExtensionPoint::new();
        assert_eq!(ext.name(), "example_extension");
        
        ext.initialize().unwrap();
        assert_eq!(ext.get_property("initialized"), Some("true"));
        
        ext.set_property("test".to_string(), "value".to_string());
        assert_eq!(ext.get_property("test"), Some("value"));
        
        ext.cleanup().unwrap();
        assert_eq!(ext.get_property("test"), None);
    }
}