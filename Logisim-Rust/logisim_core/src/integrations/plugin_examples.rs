//! Example plugin implementations demonstrating extensibility features
//!
//! This module provides concrete examples of how to implement plugins for
//! Logisim-RUST, showcasing the extensibility hooks and advanced modeling features.
//!
//! # Stability
//! 
//! **⚠️ UNSTABLE API**: Example plugin interfaces are for demonstration purposes
//! and may change as the plugin system evolves.

use super::plugins::*;
use crate::{Component, ComponentId, Location, Signal, Timestamp, BusWidth};
use crate::comp::{Pin, UpdateResult};
use crate::event_system::{Observer, CircuitEvent, SimulationEvent, EventResult, Event};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Example custom gate component demonstrating plugin component creation
/// 
/// **⚠️ UNSTABLE API**: Component interface may change
#[derive(Debug)]
pub struct ExampleCustomGate {
    id: ComponentId,
    location: Option<Location>,
    pins: HashMap<String, Pin>,
    properties: HashMap<String, String>,
}

impl ExampleCustomGate {
    pub fn new(id: ComponentId, location: Location) -> Self {
        let mut properties = HashMap::new();
        properties.insert("gate_type".to_string(), "CUSTOM_XOR".to_string());
        properties.insert("delay".to_string(), "10".to_string());
        
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth::new(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth::new(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth::new(1)));
        
        Self {
            id,
            location: Some(location),
            pins,
            properties,
        }
    }
}

impl Component for ExampleCustomGate {
    fn id(&self) -> ComponentId {
        self.id
    }
    
    fn name(&self) -> &str {
        "CustomXORGate"
    }
    
    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }
    
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }
    
    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Example XOR logic implementation
        // In a real implementation, would process input pins and update output pins
        UpdateResult::new() // Placeholder
    }
    
    fn reset(&mut self) {
        // Reset component state - could reset internal state variables
        // Pins themselves don't need reset as they'll be updated by simulation
        log::debug!("Resetting custom XOR gate {:?}", self.id);
    }
    
    fn location(&self) -> Option<Location> {
        self.location
    }
    
    fn bounds(&self) -> Option<crate::data::Bounds> {
        self.location.map(|loc| crate::data::Bounds::create(
            loc.get_x() - 15,
            loc.get_y() - 10,
            30,
            20
        ))
    }
    
    fn propagation_delay(&self) -> u64 {
        self.properties.get("delay")
            .and_then(|s| s.parse().ok())
            .unwrap_or(10)
    }
}

/// Example component factory for creating custom gates
pub struct ExampleGateFactory;

impl ComponentFactory for ExampleGateFactory {
    fn create(&self, id: ComponentId, location: Location) -> PluginResult<Box<dyn Component>> {
        Ok(Box::new(ExampleCustomGate::new(id, location)))
    }
    
    fn component_info(&self) -> ComponentInfo {
        ComponentInfo {
            name: "Custom XOR Gate".to_string(),
            category: "Custom Gates".to_string(),
            description: "A custom XOR gate with configurable delay".to_string(),
            icon_path: Some("icons/custom_xor.png".to_string()),
            input_count: Some(2),
            output_count: Some(1),
        }
    }
    
    fn validate_placement(&self, _location: Location) -> bool {
        // Custom validation logic could go here
        true
    }
}

/// Example modeling extension for advanced timing simulation
pub struct ExampleTimingExtension {
    name: String,
    timing_data: HashMap<ComponentId, TimingInfo>,
}

impl ExampleTimingExtension {
    pub fn new() -> Self {
        Self {
            name: "Advanced Timing Analyzer".to_string(),
            timing_data: HashMap::new(),
        }
    }
}

impl ModelingExtension for ExampleTimingExtension {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn initialize(&mut self) -> PluginResult<()> {
        log::info!("Initializing advanced timing extension");
        Ok(())
    }
    
    fn process_step(&mut self, step_data: &SimulationStepData) -> PluginResult<()> {
        // Example timing analysis
        for (component_id, signal) in &step_data.changed_signals {
            let timing_info = self.timing_data.entry(*component_id)
                .or_insert_with(|| TimingInfo::new(*component_id));
            
            timing_info.record_transition(step_data.current_time, signal.clone());
        }
        
        Ok(())
    }
    
    fn cleanup(&mut self) -> PluginResult<()> {
        log::info!("Cleaning up timing extension");
        self.timing_data.clear();
        Ok(())
    }
}

/// Example timing information storage
#[derive(Debug)]
struct TimingInfo {
    component_id: ComponentId,
    transitions: Vec<(u64, Signal)>,
    setup_violations: Vec<u64>,
    hold_violations: Vec<u64>,
}

impl TimingInfo {
    fn new(component_id: ComponentId) -> Self {
        Self {
            component_id,
            transitions: Vec::new(),
            setup_violations: Vec::new(),
            hold_violations: Vec::new(),
        }
    }
    
    fn record_transition(&mut self, time: u64, signal: Signal) {
        self.transitions.push((time, signal));
        
        // Example timing violation detection
        if self.transitions.len() >= 2 {
            let prev_time = self.transitions[self.transitions.len() - 2].0;
            if time - prev_time < 5 { // Minimum 5ns between transitions
                self.setup_violations.push(time);
            }
        }
    }
}

/// Example UI extension for custom toolbars
pub struct ExampleToolbarExtension {
    name: String,
    enabled: bool,
}

impl ExampleToolbarExtension {
    pub fn new() -> Self {
        Self {
            name: "Custom Analysis Tools".to_string(),
            enabled: false,
        }
    }
}

impl UiExtension for ExampleToolbarExtension {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn initialize(&mut self) -> PluginResult<()> {
        log::info!("Initializing custom toolbar extension");
        self.enabled = true;
        Ok(())
    }
    
    fn render(&mut self, _ui_context: &mut UiContext) -> PluginResult<()> {
        if !self.enabled {
            return Ok(());
        }
        
        // Example: Would render custom UI elements
        log::debug!("Rendering custom toolbar elements");
        Ok(())
    }
    
    fn handle_event(&mut self, event: &UiEvent) -> PluginResult<()> {
        match event {
            UiEvent::MenuAction { action } if action == "analyze_timing" => {
                log::info!("Custom timing analysis triggered");
                // Implement custom analysis
            }
            UiEvent::ComponentSelected { component_id } => {
                log::debug!("Component selected for analysis: {:?}", component_id);
            }
            _ => {}
        }
        Ok(())
    }
    
    fn cleanup(&mut self) -> PluginResult<()> {
        log::info!("Cleaning up toolbar extension");
        self.enabled = false;
        Ok(())
    }
}

/// Example simulation hook for logging
pub struct ExampleLoggingHook {
    log_file: Option<String>,
    step_count: u64,
}

impl ExampleLoggingHook {
    pub fn new(log_file: Option<String>) -> Self {
        Self {
            log_file,
            step_count: 0,
        }
    }
}

impl SimulationHook for ExampleLoggingHook {
    fn before_simulation_start(&mut self) -> PluginResult<()> {
        if let Some(ref log_file) = self.log_file {
            log::info!("Starting simulation logging to: {}", log_file);
        }
        self.step_count = 0;
        Ok(())
    }
    
    fn after_simulation_stop(&mut self) -> PluginResult<()> {
        log::info!("Simulation completed after {} steps", self.step_count);
        Ok(())
    }
    
    fn before_step(&mut self, step_count: u64) -> PluginResult<()> {
        self.step_count = step_count;
        if step_count % 1000 == 0 {
            log::debug!("Simulation step: {}", step_count);
        }
        Ok(())
    }
    
    fn after_step(&mut self, _step_count: u64) -> PluginResult<()> {
        // Could log step results here
        Ok(())
    }
}

/// Example circuit event observer for debugging
pub struct ExampleCircuitObserver {
    name: String,
    event_count: usize,
}

impl ExampleCircuitObserver {
    pub fn new() -> Self {
        Self {
            name: "Circuit Debug Observer".to_string(),
            event_count: 0,
        }
    }
}

impl Observer<CircuitEvent> for ExampleCircuitObserver {
    fn on_event(&mut self, event: &CircuitEvent) -> EventResult<()> {
        self.event_count += 1;
        log::debug!("Circuit event #{}: {:?}", self.event_count, event.event_type());
        
        match event {
            CircuitEvent::ComponentAdded { component_id, location, .. } => {
                log::info!("Component {:?} added at {:?}", component_id, location);
            }
            CircuitEvent::ComponentRemoved { component_id, .. } => {
                log::info!("Component {:?} removed", component_id);
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Example simulation event observer for performance monitoring
pub struct ExamplePerformanceObserver {
    name: String,
    start_time: Option<u64>,
    signal_changes: usize,
}

impl ExamplePerformanceObserver {
    pub fn new() -> Self {
        Self {
            name: "Performance Monitor".to_string(),
            start_time: None,
            signal_changes: 0,
        }
    }
}

impl Observer<SimulationEvent> for ExamplePerformanceObserver {
    fn on_event(&mut self, event: &SimulationEvent) -> EventResult<()> {
        match event {
            SimulationEvent::SimulationStarted { timestamp } => {
                self.start_time = Some(*timestamp);
                self.signal_changes = 0;
                log::info!("Performance monitoring started");
            }
            SimulationEvent::SimulationStopped { timestamp } => {
                if let Some(start) = self.start_time {
                    let duration = timestamp - start;
                    log::info!("Simulation completed in {}ms with {} signal changes", 
                             duration, self.signal_changes);
                }
            }
            SimulationEvent::SignalChanged { .. } => {
                self.signal_changes += 1;
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn should_handle(&self, event: &SimulationEvent) -> bool {
        // Only handle specific events for performance
        matches!(event, 
            SimulationEvent::SimulationStarted { .. } |
            SimulationEvent::SimulationStopped { .. } |
            SimulationEvent::SignalChanged { .. }
        )
    }
}

/// Example complete plugin implementation
pub struct ExamplePlugin {
    info: PluginInfo,
    components: Vec<ComponentInfo>,
    initialized: bool,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        let info = PluginInfo {
            name: "Example Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Demonstration plugin showing extensibility features".to_string(),
            author: "Logisim-RUST Team".to_string(),
            homepage: Some("https://github.com/crossplatformdev/Logisim-RUST".to_string()),
            dependencies: vec![],
            entry_point: "example_plugin_init".to_string(),
        };
        
        let components = vec![
            ComponentInfo {
                name: "Custom XOR Gate".to_string(),
                category: "Custom Gates".to_string(),
                description: "A custom XOR gate with configurable delay".to_string(),
                icon_path: Some("icons/custom_xor.png".to_string()),
                input_count: Some(2),
                output_count: Some(1),
            }
        ];
        
        Self {
            info,
            components,
            initialized: false,
        }
    }
}

impl PluginLibrary for ExamplePlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn components(&self) -> Vec<ComponentInfo> {
        self.components.clone()
    }
    
    fn create_component(
        &self,
        component_type: &str,
        id: ComponentId,
    ) -> PluginResult<Box<dyn Component>> {
        match component_type {
            "Custom XOR Gate" => {
                Ok(Box::new(ExampleCustomGate::new(id, Location::new(0, 0))))
            }
            _ => Err(PluginError::PluginNotFound(component_type.to_string()))
        }
    }
    
    fn initialize(&mut self) -> PluginResult<()> {
        log::info!("Initializing example plugin: {}", self.info.name);
        self.initialized = true;
        Ok(())
    }
    
    fn cleanup(&mut self) -> PluginResult<()> {
        log::info!("Cleaning up example plugin: {}", self.info.name);
        self.initialized = false;
        Ok(())
    }
    
    fn register_hooks(&mut self, registry: &mut ExtensionRegistry) -> PluginResult<()> {
        // Register component factory
        registry.register_component_factory(
            "ExampleCustomGate".to_string(),
            Box::new(ExampleGateFactory),
        )?;
        
        // Register modeling extension
        registry.register_modeling_extension(
            "TimingAnalysis".to_string(),
            Box::new(ExampleTimingExtension::new()),
        )?;
        
        // Register UI extension
        registry.register_ui_extension(
            "CustomToolbar".to_string(),
            Box::new(ExampleToolbarExtension::new()),
        )?;
        
        // Add simulation hook
        registry.add_simulation_hook(
            Box::new(ExampleLoggingHook::new(Some("simulation.log".to_string())))
        );
        
        // Add event observers
        registry.add_circuit_observer(
            Arc::new(Mutex::new(ExampleCircuitObserver::new()))
        );
        
        registry.add_simulation_observer(
            Arc::new(Mutex::new(ExamplePerformanceObserver::new()))
        );
        
        log::info!("Registered extension hooks for example plugin");
        Ok(())
    }
    
    fn config_schema(&self) -> Option<ConfigSchema> {
        Some(ConfigSchema {
            fields: vec![
                ConfigField {
                    name: "gate_delay".to_string(),
                    field_type: ConfigFieldType::Integer,
                    default_value: Some("10".to_string()),
                    description: "Default gate delay in nanoseconds".to_string(),
                    required: false,
                },
                ConfigField {
                    name: "enable_timing_analysis".to_string(),
                    field_type: ConfigFieldType::Boolean,
                    default_value: Some("true".to_string()),
                    description: "Enable advanced timing analysis".to_string(),
                    required: false,
                },
                ConfigField {
                    name: "log_level".to_string(),
                    field_type: ConfigFieldType::Choice(vec![
                        "DEBUG".to_string(),
                        "INFO".to_string(),
                        "WARN".to_string(),
                        "ERROR".to_string(),
                    ]),
                    default_value: Some("INFO".to_string()),
                    description: "Logging level for plugin".to_string(),
                    required: false,
                },
            ],
            version: "1.0".to_string(),
        })
    }
    
    fn on_plugin_event(&mut self, event: &PluginEvent) -> PluginResult<()> {
        match event {
            PluginEvent::ConfigChanged { config } => {
                log::info!("Plugin configuration updated: {:?}", config);
                // Handle configuration changes
            }
            PluginEvent::PluginLoaded { plugin_name } => {
                log::debug!("Plugin loaded: {}", plugin_name);
            }
            PluginEvent::PluginUnloaded { plugin_name } => {
                log::debug!("Plugin unloaded: {}", plugin_name);
            }
            PluginEvent::ExtensionRegistered { extension_name, extension_type } => {
                log::debug!("Extension registered: {} ({})", extension_name, extension_type);
            }
        }
        Ok(())
    }
}

/// Utility function to create and configure the example plugin
pub fn create_example_plugin() -> Box<dyn PluginLibrary> {
    Box::new(ExamplePlugin::new())
}

/// Example of how to register the plugin with a plugin manager
pub fn register_example_plugin(plugin_manager: &mut PluginManager) -> PluginResult<()> {
    // Register component types dynamically
    plugin_manager.register_component_type(
        "ExampleCustomXOR".to_string(),
        Box::new(ExampleGateFactory),
        ComponentCategory::Custom("Example Gates".to_string()),
    )?;
    
    // Register extensions
    let registry = plugin_manager.extension_registry();
    
    registry.register_modeling_extension(
        "ExampleTiming".to_string(),
        Box::new(ExampleTimingExtension::new()),
    )?;
    
    registry.register_ui_extension(
        "ExampleToolbar".to_string(),
        Box::new(ExampleToolbarExtension::new()),
    )?;
    
    log::info!("Example plugin registered successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_plugin_creation() {
        let mut plugin = ExamplePlugin::new();
        assert_eq!(plugin.info().name, "Example Plugin");
        assert_eq!(plugin.components().len(), 1);
        
        // Test initialization
        assert!(plugin.initialize().is_ok());
        assert!(plugin.initialized);
        
        // Test cleanup
        assert!(plugin.cleanup().is_ok());
        assert!(!plugin.initialized);
    }
    
    #[test]
    fn test_component_factory() {
        let factory = ExampleGateFactory;
        let component = factory.create(ComponentId::new(), Location::new(10, 20));
        assert!(component.is_ok());
        
        let info = factory.component_info();
        assert_eq!(info.name, "Custom XOR Gate");
        assert_eq!(info.input_count, Some(2));
        assert_eq!(info.output_count, Some(1));
    }
    
    #[test]
    fn test_modeling_extension() {
        let mut extension = ExampleTimingExtension::new();
        assert!(extension.initialize().is_ok());
        
        let step_data = SimulationStepData {
            step_count: 1,
            current_time: 1000,
            changed_signals: vec![(ComponentId::new(), Signal::High)],
            active_components: vec![],
        };
        
        assert!(extension.process_step(&step_data).is_ok());
        assert!(extension.cleanup().is_ok());
    }
    
    #[test]
    fn test_observer_functionality() {
        let mut observer = ExampleCircuitObserver::new();
        assert_eq!(observer.name(), "Circuit Debug Observer");
        
        let event = CircuitEvent::ComponentAdded {
            component_id: ComponentId::new(),
            location: Location::new(0, 0),
            timestamp: 1000,
        };
        
        assert!(observer.on_event(&event).is_ok());
        assert_eq!(observer.event_count, 1);
    }
}