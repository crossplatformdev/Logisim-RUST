//! Example plugin implementation
//!
//! This module provides a concrete example of how to implement a plugin for
//! Logisim-RUST using the extensibility framework. It demonstrates best practices
//! for plugin development and serves as a template for plugin authors.
//!
//! **API Stability: UNSTABLE** - This example may change as the plugin system evolves.

use crate::comp::component::{Component, ComponentId, UpdateResult};
use crate::comp::factory::ComponentFactory;
use crate::comp::event::{ExtensibleObserver, ComponentEvent, SimulationEvent, CircuitEvent, PluginEvent};
use crate::comp::pin::Pin;
use crate::data::{AttributeSet, Location, Bounds, BitWidth};
use crate::signal::BusWidth;
use crate::extensibility::{
    ComponentCreationExtension, SimulationExtension, UIExtension, 
    MenuItem, ToolbarButton, PropertyEditor, ExtensionResult, ExtensionError,
    ComponentTypeInfo,
};
use crate::integrations::plugins::{PluginLibrary, PluginInfo, ComponentInfo, PluginResult};
use std::collections::HashMap;
use std::sync::Arc;

/// Example plugin that demonstrates extensibility features
/// 
/// This plugin provides:
/// - A custom LED component with blinking functionality
/// - Simulation hooks for timing control
/// - UI extensions for plugin management
/// - Observer pattern implementation for event handling
pub struct ExamplePlugin {
    info: PluginInfo,
    components: Vec<ComponentInfo>,
    initialized: bool,
}

impl ExamplePlugin {
    /// Create a new example plugin
    pub fn new() -> Self {
        let info = PluginInfo {
            name: "Example Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Demonstrates Logisim-RUST plugin capabilities".to_string(),
            author: "Logisim-RUST Team".to_string(),
            homepage: Some("https://github.com/crossplatformdev/Logisim-RUST".to_string()),
            dependencies: Vec::new(),
            entry_point: "example_plugin".to_string(),
        };
        
        let components = vec![
            ComponentInfo {
                name: "blinking_led".to_string(),
                category: "I/O".to_string(),
                description: "LED that blinks at configurable frequency".to_string(),
                icon_path: Some("icons/blinking_led.png".to_string()),
                input_count: Some(1),
                output_count: Some(0),
            },
            ComponentInfo {
                name: "counter".to_string(),
                category: "Memory".to_string(),
                description: "N-bit counter with enable and reset".to_string(),
                icon_path: Some("icons/counter.png".to_string()),
                input_count: Some(3), // clock, enable, reset
                output_count: Some(1), // count output
            },
        ];
        
        Self {
            info,
            components,
            initialized: false,
        }
    }
}

impl Default for ExamplePlugin {
    fn default() -> Self {
        Self::new()
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
        if !self.initialized {
            return Err(crate::integrations::plugins::PluginError::NotImplemented);
        }
        
        match component_type {
            "blinking_led" => Ok(Box::new(BlinkingLED::new(id))),
            "counter" => Ok(Box::new(Counter::new(id))),
            _ => Err(crate::integrations::plugins::PluginError::NotImplemented),
        }
    }
    
    fn initialize(&mut self) -> PluginResult<()> {
        log::info!("Initializing Example Plugin v{}", self.info.version);
        self.initialized = true;
        Ok(())
    }
    
    fn cleanup(&mut self) -> PluginResult<()> {
        log::info!("Cleaning up Example Plugin");
        self.initialized = false;
        Ok(())
    }
}

/// Example component: Blinking LED
/// 
/// This component demonstrates:
/// - Custom component implementation
/// - Configurable attributes
/// - Simulation state management
#[derive(Debug)]
pub struct BlinkingLED {
    id: ComponentId,
    location: Location,
    frequency: u32, // Blink frequency in Hz
    state: bool,    // Current LED state
    last_toggle: u64, // Last toggle time
}

impl BlinkingLED {
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            location: Location::new(0, 0),
            frequency: 1, // 1 Hz default
            state: false,
            last_toggle: 0,
        }
    }
    
    /// Set the blink frequency
    pub fn set_frequency(&mut self, frequency: u32) {
        self.frequency = frequency.max(1); // Minimum 1 Hz
    }
    
    /// Update the LED state based on simulation time
    pub fn update(&mut self, current_time: u64) {
        if self.frequency > 0 {
            let period = 1000 / self.frequency; // Period in milliseconds
            if current_time - self.last_toggle >= period as u64 {
                self.state = !self.state;
                self.last_toggle = current_time;
            }
        }
    }
    
    /// Get the current LED state
    pub fn is_on(&self) -> bool {
        self.state
    }
}

impl Component for BlinkingLED {
    fn id(&self) -> ComponentId {
        self.id
    }
    
    fn name(&self) -> &str {
        "BlinkingLED"
    }
    
    fn pins(&self) -> &HashMap<String, Pin> {
        // For simplicity, return an empty HashMap
        // In a real implementation, this would contain the actual pins
        static PINS: std::sync::OnceLock<HashMap<String, Pin>> = std::sync::OnceLock::new();
        PINS.get_or_init(|| {
            let mut pins = HashMap::new();
            pins.insert("enable".to_string(), Pin::new_input("enable", BitWidth(1)));
            pins
        })
    }
    
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        // For this example, we'll use a thread-local static
        // In a real implementation, this would be a field on the component
        thread_local! {
            static PINS: std::cell::RefCell<HashMap<String, Pin>> = std::cell::RefCell::new({
                let mut pins = HashMap::new();
                pins.insert("enable".to_string(), Pin::new_input("enable", BusWidth(1)));
                pins
            });
        }
        
        // This is a simplified implementation for the example
        // In practice, components should own their pins
        unsafe {
            static mut PINS: Option<HashMap<String, Pin>> = None;
            if PINS.is_none() {
                let mut pins = HashMap::new();
                pins.insert("enable".to_string(), Pin::new_input("enable", BusWidth(1)));
                PINS = Some(pins);
            }
            PINS.as_mut().unwrap()
        }
    }
    
    fn update(&mut self, current_time: crate::signal::Timestamp) -> UpdateResult {
        self.update(current_time.0);
        UpdateResult::new()
    }
    
    fn reset(&mut self) {
        self.state = false;
        self.last_toggle = 0;
    }
    
    fn location(&self) -> Option<Location> {
        Some(self.location)
    }
    
    fn bounds(&self) -> Option<Bounds> {
        Some(Bounds::create(self.location.x, self.location.y, 20, 20))
    }
}

/// Example component: Counter
/// 
/// This component demonstrates:
/// - Multi-input components
/// - State storage
/// - Bus width handling
#[derive(Debug)]
pub struct Counter {
    id: ComponentId,
    location: Location,
    width: u8,    // Counter width in bits
    count: u32,   // Current count value
    max_count: u32, // Maximum count value
}

impl Counter {
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            location: Location::new(0, 0),
            width: 4, // 4-bit counter default
            count: 0,
            max_count: 15, // 2^4 - 1
        }
    }
    
    /// Set the counter width
    pub fn set_width(&mut self, width: u8) {
        self.width = width.clamp(1, 32);
        self.max_count = (1u32 << self.width) - 1;
        if self.count > self.max_count {
            self.count = 0;
        }
    }
    
    /// Increment the counter
    pub fn increment(&mut self) {
        if self.count >= self.max_count {
            self.count = 0;
        } else {
            self.count += 1;
        }
    }
    
    /// Reset the counter
    pub fn reset(&mut self) {
        self.count = 0;
    }
    
    /// Get the current count
    pub fn get_count(&self) -> u32 {
        self.count
    }
}

impl Component for Counter {
    fn id(&self) -> ComponentId {
        self.id
    }
    
    fn name(&self) -> &str {
        "Counter"
    }
    
    fn pins(&self) -> &HashMap<String, Pin> {
        static PINS: std::sync::OnceLock<HashMap<String, Pin>> = std::sync::OnceLock::new();
        PINS.get_or_init(|| {
            let mut pins = HashMap::new();
            pins.insert("clock".to_string(), Pin::new_input("clock", BusWidth(1)));
            pins.insert("enable".to_string(), Pin::new_input("enable", BusWidth(1)));
            pins.insert("reset".to_string(), Pin::new_input("reset", BusWidth(1)));
            pins.insert("count".to_string(), Pin::new_output("count", BusWidth(4))); // 4-bit output
            pins
        })
    }
    
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        // Simplified implementation for example
        unsafe {
            static mut PINS: Option<HashMap<String, Pin>> = None;
            if PINS.is_none() {
                let mut pins = HashMap::new();
                pins.insert("clock".to_string(), Pin::new_input("clock", BusWidth(1)));
                pins.insert("enable".to_string(), Pin::new_input("enable", BusWidth(1)));
                pins.insert("reset".to_string(), Pin::new_input("reset", BusWidth(1)));
                pins.insert("count".to_string(), Pin::new_output("count", BusWidth(4)));
                PINS = Some(pins);
            }
            PINS.as_mut().unwrap()
        }
    }
    
    fn update(&mut self, _current_time: crate::signal::Timestamp) -> UpdateResult {
        // Counter logic would go here
        UpdateResult::new()
    }
    
    fn reset(&mut self) {
        self.reset();
    }
    
    fn location(&self) -> Option<Location> {
        Some(self.location)
    }
    
    fn bounds(&self) -> Option<Bounds> {
        let width = 40 + (self.width as i32 * 2);
        Some(Bounds::create(self.location.x, self.location.y, width, 30))
    }
    
    fn is_sequential(&self) -> bool {
        true // Counters are sequential components
    }
}

/// Factory for creating example plugin components
pub struct ExampleComponentFactory {
    component_type: String,
    display_name: String,
}

impl ExampleComponentFactory {
    pub fn new(component_type: String, display_name: String) -> Self {
        Self {
            component_type,
            display_name,
        }
    }
}

impl ComponentFactory for ExampleComponentFactory {
    fn name(&self) -> &str {
        &self.component_type
    }
    
    fn display_name(&self) -> &str {
        &self.display_name
    }
    
    fn create_component(
        &self,
        id: ComponentId,
        location: Location,
        _attrs: &AttributeSet,
    ) -> Box<dyn Component> {
        match self.component_type.as_str() {
            "blinking_led" => {
                let mut led = BlinkingLED::new(id);
                led.location = location;
                Box::new(led)
            },
            "counter" => {
                let mut counter = Counter::new(id);
                counter.location = location;
                Box::new(counter)
            },
            _ => {
                // Fallback to a basic LED
                let mut led = BlinkingLED::new(id);
                led.location = location;
                Box::new(led)
            }
        }
    }
    
    fn create_attribute_set(&self) -> AttributeSet {
        let mut attrs = AttributeSet::new();
        
        match self.component_type.as_str() {
            "blinking_led" => {
                // Add frequency attribute
                // attrs.set_attribute("frequency", 1u32);
            },
            "counter" => {
                // Add width attribute
                // attrs.set_attribute("width", 4u8);
            },
            _ => {}
        }
        
        attrs
    }
    
    fn get_bounds(&self, _attrs: &AttributeSet) -> Bounds {
        match self.component_type.as_str() {
            "blinking_led" => Bounds::create(0, 0, 20, 20),
            "counter" => Bounds::create(0, 0, 50, 30),
            _ => Bounds::create(0, 0, 20, 20),
        }
    }
}

/// Example component creation extension
pub struct ExampleComponentExtension {
    plugin: Arc<ExamplePlugin>,
}

impl ExampleComponentExtension {
    pub fn new(plugin: Arc<ExamplePlugin>) -> Self {
        Self { plugin }
    }
}

impl ComponentCreationExtension for ExampleComponentExtension {
    fn name(&self) -> &str {
        "Example Plugin Components"
    }
    
    fn can_create(&self, component_type: &str) -> bool {
        matches!(component_type, "blinking_led" | "counter")
    }
    
    fn create_component(
        &self,
        component_type: &str,
        id: ComponentId,
        location: Location,
        _attrs: &AttributeSet,
    ) -> ExtensionResult<Box<dyn Component>> {
        self.plugin
            .create_component(component_type, id)
            .map(|mut comp| {
                // Set location directly since we can't call set_location method
                comp
            })
            .map_err(|_| ExtensionError::NotAvailable(format!("Cannot create {}", component_type)))
    }
    
    fn get_factory(&self, component_type: &str) -> Option<Arc<dyn ComponentFactory>> {
        if self.can_create(component_type) {
            let display_name = match component_type {
                "blinking_led" => "Blinking LED",
                "counter" => "Counter",
                _ => return None,
            };
            
            Some(Arc::new(ExampleComponentFactory::new(
                component_type.to_string(),
                display_name.to_string(),
            )))
        } else {
            None
        }
    }
    
    fn supported_types(&self) -> Vec<String> {
        vec!["blinking_led".to_string(), "counter".to_string()]
    }
}

/// Example simulation extension
pub struct ExampleSimulationExtension {
    name: String,
    step_count: u64,
}

impl ExampleSimulationExtension {
    pub fn new() -> Self {
        Self {
            name: "Example Simulation Extension".to_string(),
            step_count: 0,
        }
    }
}

impl Default for ExampleSimulationExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationExtension for ExampleSimulationExtension {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn before_simulation_start(&mut self) -> ExtensionResult<()> {
        log::info!("Example plugin: Simulation starting");
        self.step_count = 0;
        Ok(())
    }
    
    fn after_simulation_stop(&mut self) -> ExtensionResult<()> {
        log::info!("Example plugin: Simulation stopped after {} steps", self.step_count);
        Ok(())
    }
    
    fn before_step(&mut self, step: u64) -> ExtensionResult<()> {
        self.step_count = step;
        if step % 1000 == 0 {
            log::debug!("Example plugin: Processing step {}", step);
        }
        Ok(())
    }
    
    fn handles_signal_change(&self, node_id: &str) -> bool {
        // Handle changes to LED nodes
        node_id.contains("led") || node_id.contains("counter")
    }
    
    fn process_signal_change(&mut self, node_id: &str, value: &str) -> ExtensionResult<()> {
        log::debug!("Example plugin: Signal {} changed to {}", node_id, value);
        Ok(())
    }
}

/// Example UI extension
pub struct ExampleUIExtension {
    name: String,
}

impl ExampleUIExtension {
    pub fn new() -> Self {
        Self {
            name: "Example Plugin UI".to_string(),
        }
    }
}

impl Default for ExampleUIExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl UIExtension for ExampleUIExtension {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn get_menu_items(&self) -> Vec<MenuItem> {
        vec![
            MenuItem::new(
                "example_plugin_menu".to_string(),
                "Example Plugin".to_string(),
            ).with_submenu(vec![
                MenuItem::new(
                    "example_about".to_string(),
                    "About Example Plugin".to_string(),
                ),
                MenuItem::new(
                    "example_settings".to_string(),
                    "Plugin Settings".to_string(),
                ),
            ]),
        ]
    }
    
    fn get_toolbar_buttons(&self) -> Vec<ToolbarButton> {
        vec![
            ToolbarButton::new(
                "example_led".to_string(),
                "LED".to_string(),
                "Add Blinking LED".to_string(),
            ).with_icon("icons/blinking_led.png".to_string()),
            ToolbarButton::new(
                "example_counter".to_string(),
                "Counter".to_string(),
                "Add Counter".to_string(),
            ).with_icon("icons/counter.png".to_string()),
        ]
    }
    
    fn get_property_editors(&self) -> Vec<PropertyEditor> {
        vec![
            PropertyEditor {
                component_type: "blinking_led".to_string(),
                editor_type: "frequency_editor".to_string(),
                properties: vec![
                    crate::extensibility::PropertyDefinition {
                        name: "frequency".to_string(),
                        display_name: "Blink Frequency (Hz)".to_string(),
                        property_type: "integer".to_string(),
                        default_value: "1".to_string(),
                        editable: true,
                    },
                ],
            },
            PropertyEditor {
                component_type: "counter".to_string(),
                editor_type: "counter_editor".to_string(),
                properties: vec![
                    crate::extensibility::PropertyDefinition {
                        name: "width".to_string(),
                        display_name: "Counter Width (bits)".to_string(),
                        property_type: "integer".to_string(),
                        default_value: "4".to_string(),
                        editable: true,
                    },
                ],
            },
        ]
    }
}

/// Example observer implementation
pub struct ExampleObserver {
    name: String,
    event_count: u64,
}

impl ExampleObserver {
    pub fn new() -> Self {
        Self {
            name: "Example Plugin Observer".to_string(),
            event_count: 0,
        }
    }
    
    pub fn get_event_count(&self) -> u64 {
        self.event_count
    }
}

impl Default for ExampleObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtensibleObserver for ExampleObserver {
    fn on_component_event(&mut self, event: &ComponentEvent) {
        self.event_count += 1;
        log::debug!(
            "Example observer: Component {} event: {:?}",
            event.component_id.0,
            event.event_type
        );
    }
    
    fn on_simulation_event(&mut self, event: &SimulationEvent) {
        self.event_count += 1;
        log::debug!("Example observer: Simulation event: {:?}", event);
    }
    
    fn on_circuit_event(&mut self, event: &CircuitEvent) {
        self.event_count += 1;
        log::debug!("Example observer: Circuit event: {:?}", event);
    }
    
    fn on_plugin_event(&mut self, event: &PluginEvent) {
        self.event_count += 1;
        log::debug!("Example observer: Plugin event: {:?}", event);
    }
    
    fn priority(&self) -> i32 {
        10 // Higher priority than default
    }
    
    fn on_legacy_event(&mut self, event: &ComponentEvent) {
        // Backward compatibility implementation
        log::debug!("Example observer (legacy): Component event: {:?}", event.event_type);
    }
}

/// Utility function to register the example plugin with all extension points
pub fn register_example_plugin() -> Result<(), Box<dyn std::error::Error>> {
    use crate::extensibility::{with_extensions_mut, initialize_extension_points};
    use crate::extensibility::registry::{initialize_registry, register_global_component};
    
    // Initialize extension systems if not already done
    if !crate::extensibility::is_extensions_initialized() {
        initialize_extension_points()?;
    }
    
    if !crate::extensibility::registry::is_registry_initialized() {
        initialize_registry()?;
    }
    
    // Create plugin instance
    let plugin = Arc::new(ExamplePlugin::new());
    
    // Register with extension points
    with_extensions_mut(|registry| {
        // Register component extension
        let component_ext = Box::new(ExampleComponentExtension::new(plugin.clone()));
        registry.register_component_extension(component_ext);
        
        // Register simulation extension
        let sim_ext = Box::new(ExampleSimulationExtension::new());
        registry.register_simulation_extension(sim_ext);
        
        // Register UI extension
        let ui_ext = Box::new(ExampleUIExtension::new());
        registry.register_ui_extension(ui_ext);
        
        // Register observer
        let observer = Box::new(ExampleObserver::new());
        registry.register_observer(observer);
    })?;
    
    // Register component factories
    for component in plugin.components() {
        let factory = Arc::new(ExampleComponentFactory::new(
            component.name.clone(),
            component.description.clone(),
        ));
        
        let info = ComponentTypeInfo::new(
            component.name.clone(),
            component.description.clone(),
            component.category.clone(),
            format!("{} (from Example Plugin)", component.description),
        )
        .with_provider("example_plugin".to_string())
        .with_version("1.0.0".to_string());
        
        register_global_component(factory, info)?;
    }
    
    log::info!("Successfully registered Example Plugin with all extension points");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_plugin_creation() {
        let plugin = ExamplePlugin::new();
        assert_eq!(plugin.info().name, "Example Plugin");
        assert_eq!(plugin.components().len(), 2);
    }
    
    #[test]
    fn test_blinking_led() {
        let mut led = BlinkingLED::new(ComponentId(1));
        assert!(!led.is_on());
        
        led.set_frequency(10); // 10 Hz
        led.update(0);
        assert!(!led.is_on());
        
        led.update(100); // 100ms later, should toggle
        assert!(led.is_on());
    }
    
    #[test]
    fn test_counter() {
        let mut counter = Counter::new(ComponentId(2));
        assert_eq!(counter.get_count(), 0);
        
        counter.increment();
        assert_eq!(counter.get_count(), 1);
        
        counter.set_width(2); // 2-bit counter (max 3)
        for _ in 0..5 {
            counter.increment();
        }
        assert_eq!(counter.get_count(), 2); // Should wrap around
        
        counter.reset();
        assert_eq!(counter.get_count(), 0);
    }
    
    #[test]
    fn test_example_observer() {
        let mut observer = ExampleObserver::new();
        assert_eq!(observer.get_event_count(), 0);
        
        let event = ComponentEvent::component_added(ComponentId(1));
        observer.on_component_event(&event);
        assert_eq!(observer.get_event_count(), 1);
        
        assert_eq!(observer.priority(), 10);
    }
}