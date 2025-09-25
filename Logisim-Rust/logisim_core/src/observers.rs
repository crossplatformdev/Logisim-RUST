//! Observer pattern implementation for extensibility
//!
//! This module provides observer traits and management for extending Logisim-RUST
//! with custom behavior and plugins. The observer pattern allows external code
//! to react to simulation events, component state changes, and other system events.

use crate::{ComponentId, Signal, Timestamp};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur in the observer system
#[derive(Error, Debug)]
pub enum ObserverError {
    #[error("Observer registration failed: {0}")]
    RegistrationFailed(String),
    #[error("Observer notification failed: {0}")]
    NotificationFailed(String),
    #[error("Observer not found: {0}")]
    ObserverNotFound(String),
}

/// Result type for observer operations
pub type ObserverResult<T> = Result<T, ObserverError>;

/// Unique identifier for observers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObserverId(pub u64);

impl ObserverId {
    /// Create a new observer ID
    pub fn new(id: u64) -> Self {
        ObserverId(id)
    }
}

/// Events that can occur during simulation
#[derive(Debug, Clone)]
pub enum SimulationEvent {
    /// Simulation started
    Started { timestamp: Timestamp },
    /// Simulation stopped
    Stopped { timestamp: Timestamp },
    /// Simulation paused
    Paused { timestamp: Timestamp },
    /// Simulation resumed
    Resumed { timestamp: Timestamp },
    /// Simulation reset
    Reset,
    /// Time step completed
    StepCompleted { timestamp: Timestamp },
    /// Clock tick occurred
    ClockTick { timestamp: Timestamp, signal: Signal },
}

/// Events that can occur for components
#[derive(Debug, Clone)]
pub enum ComponentEvent {
    /// Component was created
    Created { 
        component_id: ComponentId, 
        component_type: String 
    },
    /// Component was removed
    Removed { component_id: ComponentId },
    /// Component state changed
    StateChanged { 
        component_id: ComponentId, 
        timestamp: Timestamp 
    },
    /// Component input changed
    InputChanged { 
        component_id: ComponentId, 
        pin_name: String, 
        old_signal: Signal, 
        new_signal: Signal, 
        timestamp: Timestamp 
    },
    /// Component output changed
    OutputChanged { 
        component_id: ComponentId, 
        pin_name: String, 
        old_signal: Signal, 
        new_signal: Signal, 
        timestamp: Timestamp 
    },
    /// Component was reset
    Reset { component_id: ComponentId },
}

/// Observer trait for simulation events
/// 
/// # API Stability
/// This trait is **UNSTABLE** and may change in future versions.
/// Plugin authors should be prepared for breaking changes.
pub trait SimulationObserver: Send + Sync {
    /// Get the unique identifier for this observer
    fn id(&self) -> ObserverId;
    
    /// Get a human-readable name for this observer
    fn name(&self) -> &str;
    
    /// Called when a simulation event occurs
    /// 
    /// # Arguments
    /// * `event` - The simulation event that occurred
    /// 
    /// # Returns
    /// * `Ok(())` if the event was handled successfully
    /// * `Err(ObserverError)` if an error occurred during handling
    fn on_simulation_event(&mut self, event: &SimulationEvent) -> ObserverResult<()>;
    
    /// Check if this observer is interested in a specific event type
    /// This can be used for performance optimization
    fn interested_in_event(&self, event: &SimulationEvent) -> bool {
        // By default, observers are interested in all events
        true
    }
}

/// Observer trait for component events
/// 
/// # API Stability  
/// This trait is **UNSTABLE** and may change in future versions.
/// Plugin authors should be prepared for breaking changes.
pub trait ComponentObserver: Send + Sync {
    /// Get the unique identifier for this observer
    fn id(&self) -> ObserverId;
    
    /// Get a human-readable name for this observer
    fn name(&self) -> &str;
    
    /// Called when a component event occurs
    /// 
    /// # Arguments
    /// * `event` - The component event that occurred
    /// 
    /// # Returns  
    /// * `Ok(())` if the event was handled successfully
    /// * `Err(ObserverError)` if an error occurred during handling
    fn on_component_event(&mut self, event: &ComponentEvent) -> ObserverResult<()>;
    
    /// Check if this observer is interested in events from a specific component
    fn interested_in_component(&self, component_id: ComponentId) -> bool {
        // By default, observers are interested in all components
        true
    }
    
    /// Check if this observer is interested in a specific event type
    fn interested_in_event(&self, event: &ComponentEvent) -> bool {
        // By default, observers are interested in all events
        true
    }
}

/// Generic observer trait for system-wide events
/// 
/// # API Stability
/// This trait is **UNSTABLE** and may change in future versions.
pub trait SystemObserver: Send + Sync {
    /// Get the unique identifier for this observer
    fn id(&self) -> ObserverId;
    
    /// Get a human-readable name for this observer  
    fn name(&self) -> &str;
    
    /// Called when the system is initializing
    fn on_system_init(&mut self) -> ObserverResult<()> {
        Ok(())
    }
    
    /// Called when the system is shutting down
    fn on_system_shutdown(&mut self) -> ObserverResult<()> {
        Ok(())
    }
    
    /// Called when a plugin is loaded
    fn on_plugin_loaded(&mut self, plugin_name: &str) -> ObserverResult<()> {
        Ok(())
    }
    
    /// Called when a plugin is unloaded
    fn on_plugin_unloaded(&mut self, plugin_name: &str) -> ObserverResult<()> {
        Ok(())
    }
}

/// Manager for simulation observers
pub struct SimulationObserverManager {
    observers: HashMap<ObserverId, Box<dyn SimulationObserver>>,
    next_id: u64,
}

impl SimulationObserverManager {
    /// Create a new simulation observer manager
    pub fn new() -> Self {
        Self {
            observers: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Register a new simulation observer
    pub fn register_observer(
        &mut self, 
        mut observer: Box<dyn SimulationObserver>
    ) -> ObserverResult<ObserverId> {
        let id = ObserverId::new(self.next_id);
        self.next_id += 1;
        
        // Update observer ID if needed
        let observer_id = observer.id();
        if observer_id.0 == 0 {
            // Observer doesn't have an ID, we could assign one but this is complex
            // For now, use the observer's provided ID
        }
        
        let final_id = if observer_id.0 == 0 { id } else { observer_id };
        
        self.observers.insert(final_id, observer);
        Ok(final_id)
    }
    
    /// Unregister a simulation observer
    pub fn unregister_observer(&mut self, id: ObserverId) -> ObserverResult<()> {
        if self.observers.remove(&id).is_some() {
            Ok(())
        } else {
            Err(ObserverError::ObserverNotFound(format!("{:?}", id)))
        }
    }
    
    /// Notify all observers of a simulation event
    pub fn notify_observers(&mut self, event: &SimulationEvent) -> Vec<ObserverError> {
        let mut errors = Vec::new();
        
        // Collect observers that are interested in this event
        let interested_ids: Vec<ObserverId> = self.observers
            .iter()
            .filter(|(_, observer)| observer.interested_in_event(event))
            .map(|(id, _)| *id)
            .collect();
        
        // Notify interested observers
        for id in interested_ids {
            if let Some(observer) = self.observers.get_mut(&id) {
                if let Err(e) = observer.on_simulation_event(event) {
                    errors.push(e);
                }
            }
        }
        
        errors
    }
    
    /// Get the number of registered observers
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }
    
    /// Check if an observer is registered
    pub fn is_observer_registered(&self, id: ObserverId) -> bool {
        self.observers.contains_key(&id)
    }
}

impl Default for SimulationObserverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for component observers
pub struct ComponentObserverManager {
    observers: HashMap<ObserverId, Box<dyn ComponentObserver>>,
    next_id: u64,
}

impl ComponentObserverManager {
    /// Create a new component observer manager
    pub fn new() -> Self {
        Self {
            observers: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Register a new component observer
    pub fn register_observer(
        &mut self, 
        observer: Box<dyn ComponentObserver>
    ) -> ObserverResult<ObserverId> {
        let id = ObserverId::new(self.next_id);
        self.next_id += 1;
        
        let final_id = if observer.id().0 == 0 { id } else { observer.id() };
        
        self.observers.insert(final_id, observer);
        Ok(final_id)
    }
    
    /// Unregister a component observer
    pub fn unregister_observer(&mut self, id: ObserverId) -> ObserverResult<()> {
        if self.observers.remove(&id).is_some() {
            Ok(())
        } else {
            Err(ObserverError::ObserverNotFound(format!("{:?}", id)))
        }
    }
    
    /// Notify all observers of a component event
    pub fn notify_observers(&mut self, event: &ComponentEvent) -> Vec<ObserverError> {
        let mut errors = Vec::new();
        
        // Get component ID from event
        let component_id = match event {
            ComponentEvent::Created { component_id, .. } => *component_id,
            ComponentEvent::Removed { component_id } => *component_id,
            ComponentEvent::StateChanged { component_id, .. } => *component_id,
            ComponentEvent::InputChanged { component_id, .. } => *component_id,
            ComponentEvent::OutputChanged { component_id, .. } => *component_id,
            ComponentEvent::Reset { component_id } => *component_id,
        };
        
        // Collect observers that are interested in this event and component
        let interested_ids: Vec<ObserverId> = self.observers
            .iter()
            .filter(|(_, observer)| {
                observer.interested_in_component(component_id) && 
                observer.interested_in_event(event)
            })
            .map(|(id, _)| *id)
            .collect();
        
        // Notify interested observers
        for id in interested_ids {
            if let Some(observer) = self.observers.get_mut(&id) {
                if let Err(e) = observer.on_component_event(event) {
                    errors.push(e);
                }
            }
        }
        
        errors
    }
    
    /// Get the number of registered observers
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }
}

impl Default for ComponentObserverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for system observers
pub struct SystemObserverManager {
    observers: HashMap<ObserverId, Box<dyn SystemObserver>>,
    next_id: u64,
}

impl SystemObserverManager {
    /// Create a new system observer manager
    pub fn new() -> Self {
        Self {
            observers: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Register a new system observer
    pub fn register_observer(
        &mut self, 
        observer: Box<dyn SystemObserver>
    ) -> ObserverResult<ObserverId> {
        let id = ObserverId::new(self.next_id);
        self.next_id += 1;
        
        let final_id = if observer.id().0 == 0 { id } else { observer.id() };
        
        self.observers.insert(final_id, observer);
        Ok(final_id)
    }
    
    /// Unregister a system observer
    pub fn unregister_observer(&mut self, id: ObserverId) -> ObserverResult<()> {
        if self.observers.remove(&id).is_some() {
            Ok(())
        } else {
            Err(ObserverError::ObserverNotFound(format!("{:?}", id)))
        }
    }
    
    /// Notify all observers of system initialization
    pub fn notify_system_init(&mut self) -> Vec<ObserverError> {
        let mut errors = Vec::new();
        for observer in self.observers.values_mut() {
            if let Err(e) = observer.on_system_init() {
                errors.push(e);
            }
        }
        errors
    }
    
    /// Notify all observers of system shutdown
    pub fn notify_system_shutdown(&mut self) -> Vec<ObserverError> {
        let mut errors = Vec::new();
        for observer in self.observers.values_mut() {
            if let Err(e) = observer.on_system_shutdown() {
                errors.push(e);
            }
        }
        errors
    }
    
    /// Notify all observers of plugin loading
    pub fn notify_plugin_loaded(&mut self, plugin_name: &str) -> Vec<ObserverError> {
        let mut errors = Vec::new();
        for observer in self.observers.values_mut() {
            if let Err(e) = observer.on_plugin_loaded(plugin_name) {
                errors.push(e);
            }
        }
        errors
    }
    
    /// Notify all observers of plugin unloading
    pub fn notify_plugin_unloaded(&mut self, plugin_name: &str) -> Vec<ObserverError> {
        let mut errors = Vec::new();
        for observer in self.observers.values_mut() {
            if let Err(e) = observer.on_plugin_unloaded(plugin_name) {
                errors.push(e);
            }
        }
        errors
    }
}

impl Default for SystemObserverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A sample observer implementation for debugging/logging
pub struct LoggingObserver {
    id: ObserverId,
    name: String,
}

impl LoggingObserver {
    /// Create a new logging observer
    pub fn new(name: String) -> Self {
        Self {
            id: ObserverId::new(0), // Will be assigned by manager
            name,
        }
    }
}

impl SimulationObserver for LoggingObserver {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_simulation_event(&mut self, event: &SimulationEvent) -> ObserverResult<()> {
        log::info!("[{}] Simulation event: {:?}", self.name, event);
        Ok(())
    }
}

impl ComponentObserver for LoggingObserver {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_component_event(&mut self, event: &ComponentEvent) -> ObserverResult<()> {
        log::info!("[{}] Component event: {:?}", self.name, event);
        Ok(())
    }
}

impl SystemObserver for LoggingObserver {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_system_init(&mut self) -> ObserverResult<()> {
        log::info!("[{}] System initializing", self.name);
        Ok(())
    }
    
    fn on_system_shutdown(&mut self) -> ObserverResult<()> {
        log::info!("[{}] System shutting down", self.name);
        Ok(())
    }
    
    fn on_plugin_loaded(&mut self, plugin_name: &str) -> ObserverResult<()> {
        log::info!("[{}] Plugin loaded: {}", self.name, plugin_name);
        Ok(())
    }
    
    fn on_plugin_unloaded(&mut self, plugin_name: &str) -> ObserverResult<()> {
        log::info!("[{}] Plugin unloaded: {}", self.name, plugin_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::{Timestamp};

    #[test]
    fn test_observer_manager_creation() {
        let manager = SimulationObserverManager::new();
        assert_eq!(manager.observer_count(), 0);
    }

    #[test]
    fn test_observer_registration() {
        let mut manager = SimulationObserverManager::new();
        let observer = Box::new(LoggingObserver::new("test".to_string()));
        
        let result = manager.register_observer(observer);
        assert!(result.is_ok());
        assert_eq!(manager.observer_count(), 1);
    }

    #[test]
    fn test_observer_notification() {
        let mut manager = SimulationObserverManager::new();
        let observer = Box::new(LoggingObserver::new("test".to_string()));
        
        let _id = manager.register_observer(observer).unwrap();
        
        let event = SimulationEvent::Started { timestamp: Timestamp(0) };
        let errors = manager.notify_observers(&event);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_component_observer() {
        let mut manager = ComponentObserverManager::new();
        let observer = Box::new(LoggingObserver::new("test".to_string()));
        
        let _id = manager.register_observer(observer).unwrap();
        
        let event = ComponentEvent::Created {
            component_id: ComponentId::new(1),
            component_type: "Test".to_string(),
        };
        let errors = manager.notify_observers(&event);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_observer_unregistration() {
        let mut manager = SimulationObserverManager::new();
        let observer = Box::new(LoggingObserver::new("test".to_string()));
        
        let id = manager.register_observer(observer).unwrap();
        assert_eq!(manager.observer_count(), 1);
        
        let result = manager.unregister_observer(id);
        assert!(result.is_ok());
        assert_eq!(manager.observer_count(), 0);
    }
}