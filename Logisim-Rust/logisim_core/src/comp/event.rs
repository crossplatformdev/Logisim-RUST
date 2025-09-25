/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component event handling
//!
//! This module provides event handling for component interactions,
//! equivalent to Java's `ComponentEvent`, `ComponentListener`,
//! and `ComponentUserEvent` classes.

use super::component::ComponentId;
use crate::data::Location;
use serde::{Deserialize, Serialize};

/// Types of component events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentEventType {
    /// Component was added to the circuit
    ComponentAdded,
    /// Component was removed from the circuit
    ComponentRemoved,
    /// Component was moved to a new location
    ComponentMoved,
    /// Component attributes were changed
    AttributeChanged,
    /// Component pins were modified
    PinsChanged,
}

/// Event fired when a component changes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentEvent {
    /// The component that triggered this event
    pub component_id: ComponentId,
    /// Type of event that occurred
    pub event_type: ComponentEventType,
    /// Optional old location (for move events)
    pub old_location: Option<Location>,
    /// Optional new location (for move events)
    pub new_location: Option<Location>,
    /// Optional attribute name (for attribute change events)
    pub attribute_name: Option<String>,
}

impl ComponentEvent {
    /// Create a new component added event
    pub fn component_added(component_id: ComponentId) -> Self {
        ComponentEvent {
            component_id,
            event_type: ComponentEventType::ComponentAdded,
            old_location: None,
            new_location: None,
            attribute_name: None,
        }
    }

    /// Create a new component removed event
    pub fn component_removed(component_id: ComponentId) -> Self {
        ComponentEvent {
            component_id,
            event_type: ComponentEventType::ComponentRemoved,
            old_location: None,
            new_location: None,
            attribute_name: None,
        }
    }

    /// Create a new component moved event
    pub fn component_moved(
        component_id: ComponentId,
        old_location: Location,
        new_location: Location,
    ) -> Self {
        ComponentEvent {
            component_id,
            event_type: ComponentEventType::ComponentMoved,
            old_location: Some(old_location),
            new_location: Some(new_location),
            attribute_name: None,
        }
    }

    /// Create a new attribute changed event
    pub fn attribute_changed(component_id: ComponentId, attribute_name: String) -> Self {
        ComponentEvent {
            component_id,
            event_type: ComponentEventType::AttributeChanged,
            old_location: None,
            new_location: None,
            attribute_name: Some(attribute_name),
        }
    }

    /// Create a new pins changed event
    pub fn pins_changed(component_id: ComponentId) -> Self {
        ComponentEvent {
            component_id,
            event_type: ComponentEventType::PinsChanged,
            old_location: None,
            new_location: None,
            attribute_name: None,
        }
    }
}

/// Types of user interaction events with components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserEventType {
    /// User clicked on the component
    Click,
    /// User double-clicked on the component
    DoubleClick,
    /// User pressed a key while component was focused
    KeyPress,
    /// User started dragging the component
    DragStart,
    /// User is dragging the component
    Drag,
    /// User finished dragging the component
    DragEnd,
    /// User right-clicked on the component (context menu)
    ContextMenu,
}

/// Event representing user interaction with a component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentUserEvent {
    /// The component being interacted with
    pub component_id: ComponentId,
    /// Type of user interaction
    pub event_type: UserEventType,
    /// Location where the interaction occurred
    pub location: Location,
    /// Mouse button for click events (0=left, 1=right, 2=middle)
    pub button: Option<u8>,
    /// Keyboard key for key press events
    pub key: Option<String>,
    /// Modifier keys (shift, ctrl, alt)
    pub modifiers: Vec<String>,
}

impl ComponentUserEvent {
    /// Create a new click event
    pub fn click(component_id: ComponentId, location: Location, button: u8) -> Self {
        ComponentUserEvent {
            component_id,
            event_type: UserEventType::Click,
            location,
            button: Some(button),
            key: None,
            modifiers: Vec::new(),
        }
    }

    /// Create a new double-click event
    pub fn double_click(component_id: ComponentId, location: Location, button: u8) -> Self {
        ComponentUserEvent {
            component_id,
            event_type: UserEventType::DoubleClick,
            location,
            button: Some(button),
            key: None,
            modifiers: Vec::new(),
        }
    }

    /// Create a new key press event
    pub fn key_press(component_id: ComponentId, location: Location, key: String) -> Self {
        ComponentUserEvent {
            component_id,
            event_type: UserEventType::KeyPress,
            location,
            button: None,
            key: Some(key),
            modifiers: Vec::new(),
        }
    }

    /// Create a new drag start event
    pub fn drag_start(component_id: ComponentId, location: Location) -> Self {
        ComponentUserEvent {
            component_id,
            event_type: UserEventType::DragStart,
            location,
            button: None,
            key: None,
            modifiers: Vec::new(),
        }
    }

    /// Create a new drag event
    pub fn drag(component_id: ComponentId, location: Location) -> Self {
        ComponentUserEvent {
            component_id,
            event_type: UserEventType::Drag,
            location,
            button: None,
            key: None,
            modifiers: Vec::new(),
        }
    }

    /// Create a new drag end event
    pub fn drag_end(component_id: ComponentId, location: Location) -> Self {
        ComponentUserEvent {
            component_id,
            event_type: UserEventType::DragEnd,
            location,
            button: None,
            key: None,
            modifiers: Vec::new(),
        }
    }

    /// Create a new context menu event
    pub fn context_menu(component_id: ComponentId, location: Location) -> Self {
        ComponentUserEvent {
            component_id,
            event_type: UserEventType::ContextMenu,
            location,
            button: Some(1), // Right click
            key: None,
            modifiers: Vec::new(),
        }
    }

    /// Add a modifier key to this event
    pub fn with_modifier(mut self, modifier: String) -> Self {
        self.modifiers.push(modifier);
        self
    }
}

/// Trait for listening to component events
pub trait ComponentListener: Send + Sync {
    /// Called when a component event occurs
    fn component_changed(&mut self, event: &ComponentEvent);

    /// Called when a user interacts with a component
    fn user_event(&mut self, _event: &ComponentUserEvent) {
        // Default implementation does nothing
    }
}

/// Advanced observer pattern for extensible event handling
/// 
/// **API Stability: UNSTABLE** - This trait may change in future versions
pub trait ExtensibleObserver: Send + Sync {
    /// Handle component lifecycle events
    fn on_component_event(&mut self, event: &ComponentEvent) {
        // Default: delegate to simple listener interface
        self.on_legacy_event(event);
    }
    
    /// Handle simulation state changes
    fn on_simulation_event(&mut self, _event: &SimulationEvent) {
        // Default implementation does nothing
    }
    
    /// Handle circuit structure changes
    fn on_circuit_event(&mut self, _event: &CircuitEvent) {
        // Default implementation does nothing
    }
    
    /// Handle plugin lifecycle events
    fn on_plugin_event(&mut self, _event: &PluginEvent) {
        // Default implementation does nothing
    }
    
    /// Priority for event handling (higher numbers = higher priority)
    fn priority(&self) -> i32 {
        0
    }
    
    /// Backward compatibility with ComponentListener
    fn on_legacy_event(&mut self, _event: &ComponentEvent) {
        // Default implementation for legacy compatibility
    }
}

/// Simulation state change events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulationEvent {
    /// Simulation started
    Started,
    /// Simulation paused
    Paused,
    /// Simulation stopped
    Stopped,
    /// Simulation step completed
    StepCompleted { step: u64 },
    /// Signal value changed
    SignalChanged { node_id: String, value: String },
    /// Clock tick occurred
    ClockTick { clock_name: String },
}

/// Circuit structure change events  
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CircuitEvent {
    /// New circuit created
    CircuitCreated { name: String },
    /// Circuit deleted
    CircuitDeleted { name: String },
    /// Wire added
    WireAdded { from: Location, to: Location },
    /// Wire removed
    WireRemoved { from: Location, to: Location },
    /// Circuit hierarchy changed
    HierarchyChanged,
}

/// Plugin lifecycle events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PluginEvent {
    /// Plugin loaded
    PluginLoaded { name: String, version: String },
    /// Plugin unloaded
    PluginUnloaded { name: String },
    /// Plugin error occurred
    PluginError { name: String, error: String },
    /// Component registered by plugin
    ComponentRegistered { plugin_name: String, component_name: String },
}

/// Observer registry for managing extensible observers
/// 
/// **API Stability: UNSTABLE** - This struct may change in future versions
pub struct ObserverRegistry {
    observers: Vec<Box<dyn ExtensibleObserver>>,
    enabled: bool,
}

impl ObserverRegistry {
    /// Create a new observer registry
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            enabled: true,
        }
    }
    
    /// Register a new observer
    pub fn register(&mut self, observer: Box<dyn ExtensibleObserver>) {
        self.observers.push(observer);
        // Sort by priority (highest first)
        self.observers.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }
    
    /// Remove all observers
    pub fn clear(&mut self) {
        self.observers.clear();
    }
    
    /// Enable/disable observer notifications
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Notify all observers of a component event
    pub fn notify_component_event(&mut self, event: &ComponentEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_component_event(event);
        }
    }
    
    /// Notify all observers of a simulation event
    pub fn notify_simulation_event(&mut self, event: &SimulationEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_simulation_event(event);
        }
    }
    
    /// Notify all observers of a circuit event
    pub fn notify_circuit_event(&mut self, event: &CircuitEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_circuit_event(event);
        }
    }
    
    /// Notify all observers of a plugin event
    pub fn notify_plugin_event(&mut self, event: &PluginEvent) {
        if !self.enabled {
            return;
        }
        for observer in &mut self.observers {
            observer.on_plugin_event(event);
        }
    }
    
    /// Get number of registered observers
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }
}

impl Default for ObserverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple component listener that stores events for testing
#[derive(Debug, Default)]
pub struct TestComponentListener {
    pub events: Vec<ComponentEvent>,
    pub user_events: Vec<ComponentUserEvent>,
}

impl ComponentListener for TestComponentListener {
    fn component_changed(&mut self, event: &ComponentEvent) {
        self.events.push(event.clone());
    }

    fn user_event(&mut self, event: &ComponentUserEvent) {
        self.user_events.push(event.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Location;

    #[test]
    fn test_component_event_creation() {
        let comp_id = ComponentId::new(42);

        let added = ComponentEvent::component_added(comp_id);
        assert_eq!(added.component_id, comp_id);
        assert_eq!(added.event_type, ComponentEventType::ComponentAdded);

        let removed = ComponentEvent::component_removed(comp_id);
        assert_eq!(removed.event_type, ComponentEventType::ComponentRemoved);

        let old_loc = Location::new(10, 20);
        let new_loc = Location::new(30, 40);
        let moved = ComponentEvent::component_moved(comp_id, old_loc, new_loc);
        assert_eq!(moved.event_type, ComponentEventType::ComponentMoved);
        assert_eq!(moved.old_location, Some(old_loc));
        assert_eq!(moved.new_location, Some(new_loc));

        let attr_changed = ComponentEvent::attribute_changed(comp_id, "color".to_string());
        assert_eq!(
            attr_changed.event_type,
            ComponentEventType::AttributeChanged
        );
        assert_eq!(attr_changed.attribute_name, Some("color".to_string()));
    }

    #[test]
    fn test_user_event_creation() {
        let comp_id = ComponentId::new(1);
        let location = Location::new(50, 60);

        let click = ComponentUserEvent::click(comp_id, location, 0);
        assert_eq!(click.event_type, UserEventType::Click);
        assert_eq!(click.button, Some(0));

        let key_press = ComponentUserEvent::key_press(comp_id, location, "Enter".to_string());
        assert_eq!(key_press.event_type, UserEventType::KeyPress);
        assert_eq!(key_press.key, Some("Enter".to_string()));

        let drag =
            ComponentUserEvent::drag_start(comp_id, location).with_modifier("shift".to_string());
        assert_eq!(drag.event_type, UserEventType::DragStart);
        assert!(drag.modifiers.contains(&"shift".to_string()));
    }

    #[test]
    fn test_component_listener() {
        let mut listener = TestComponentListener::default();
        let comp_id = ComponentId::new(5);

        let event = ComponentEvent::component_added(comp_id);
        listener.component_changed(&event);

        assert_eq!(listener.events.len(), 1);
        assert_eq!(listener.events[0], event);

        let user_event = ComponentUserEvent::click(comp_id, Location::new(0, 0), 0);
        listener.user_event(&user_event);

        assert_eq!(listener.user_events.len(), 1);
        assert_eq!(listener.user_events[0], user_event);
    }
}
