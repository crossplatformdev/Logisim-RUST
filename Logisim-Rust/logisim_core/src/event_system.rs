//! Event System and Observer Pattern Implementation
//!
//! This module provides a comprehensive event system for circuit simulation
//! and UI updates, supporting both synchronous and asynchronous event handling.
//! 
//! # Stability
//! 
//! **⚠️ UNSTABLE API**: The event system APIs are experimental and subject to change
//! in future versions. Breaking changes may occur without major version increments.

use crate::{ComponentId, Signal, Location};
use std::collections::HashMap;
use std::any::Any;
use std::sync::{Arc, Weak, Mutex};
use thiserror::Error;

/// Event system errors
#[derive(Error, Debug)]
pub enum EventError {
    #[error("Observer not found")]
    ObserverNotFound,
    #[error("Event type not registered: {0}")]
    EventTypeNotRegistered(String),
    #[error("Failed to deliver event: {0}")]
    DeliveryFailed(String),
}

/// Event system result type
pub type EventResult<T> = Result<T, EventError>;

/// Unique identifier for event observers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObserverId(u64);

impl ObserverId {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// Base trait for all events in the system
/// 
/// **⚠️ UNSTABLE API**: Event traits may change structure in future versions
pub trait Event: Any + Send + Sync + std::fmt::Debug {
    /// Get event type name for debugging
    fn event_type(&self) -> &'static str;
    
    /// Get event timestamp
    fn timestamp(&self) -> u64;
    
    /// Check if event should be processed synchronously
    fn is_synchronous(&self) -> bool {
        true
    }
}

/// Circuit-related events
#[derive(Debug, Clone)]
pub enum CircuitEvent {
    /// Component added to circuit
    ComponentAdded {
        component_id: ComponentId,
        location: Location,
        timestamp: u64,
    },
    /// Component removed from circuit
    ComponentRemoved {
        component_id: ComponentId,
        timestamp: u64,
    },
    /// Component moved in circuit
    ComponentMoved {
        component_id: ComponentId,
        old_location: Location,
        new_location: Location,
        timestamp: u64,
    },
    /// Component properties changed
    ComponentPropertiesChanged {
        component_id: ComponentId,
        properties: HashMap<String, String>,
        timestamp: u64,
    },
    /// Wire added to circuit
    WireAdded {
        start: Location,
        end: Location,
        timestamp: u64,
    },
    /// Wire removed from circuit
    WireRemoved {
        start: Location,
        end: Location,
        timestamp: u64,
    },
}

impl Event for CircuitEvent {
    fn event_type(&self) -> &'static str {
        match self {
            CircuitEvent::ComponentAdded { .. } => "ComponentAdded",
            CircuitEvent::ComponentRemoved { .. } => "ComponentRemoved",
            CircuitEvent::ComponentMoved { .. } => "ComponentMoved",
            CircuitEvent::ComponentPropertiesChanged { .. } => "ComponentPropertiesChanged",
            CircuitEvent::WireAdded { .. } => "WireAdded",
            CircuitEvent::WireRemoved { .. } => "WireRemoved",
        }
    }
    
    fn timestamp(&self) -> u64 {
        match self {
            CircuitEvent::ComponentAdded { timestamp, .. } => *timestamp,
            CircuitEvent::ComponentRemoved { timestamp, .. } => *timestamp,
            CircuitEvent::ComponentMoved { timestamp, .. } => *timestamp,
            CircuitEvent::ComponentPropertiesChanged { timestamp, .. } => *timestamp,
            CircuitEvent::WireAdded { timestamp, .. } => *timestamp,
            CircuitEvent::WireRemoved { timestamp, .. } => *timestamp,
        }
    }
}

/// Simulation-related events
#[derive(Debug, Clone)]
pub enum SimulationEvent {
    /// Simulation started
    SimulationStarted {
        timestamp: u64,
    },
    /// Simulation stopped
    SimulationStopped {
        timestamp: u64,
    },
    /// Simulation step completed
    StepCompleted {
        step_count: u64,
        timestamp: u64,
    },
    /// Signal value changed
    SignalChanged {
        component_id: ComponentId,
        signal: Signal,
        timestamp: u64,
    },
    /// Clock tick occurred
    ClockTick {
        clock_name: String,
        rising_edge: bool,
        timestamp: u64,
    },
}

impl Event for SimulationEvent {
    fn event_type(&self) -> &'static str {
        match self {
            SimulationEvent::SimulationStarted { .. } => "SimulationStarted",
            SimulationEvent::SimulationStopped { .. } => "SimulationStopped",
            SimulationEvent::StepCompleted { .. } => "StepCompleted",
            SimulationEvent::SignalChanged { .. } => "SignalChanged",
            SimulationEvent::ClockTick { .. } => "ClockTick",
        }
    }
    
    fn timestamp(&self) -> u64 {
        match self {
            SimulationEvent::SimulationStarted { timestamp } => *timestamp,
            SimulationEvent::SimulationStopped { timestamp } => *timestamp,
            SimulationEvent::StepCompleted { timestamp, .. } => *timestamp,
            SimulationEvent::SignalChanged { timestamp, .. } => *timestamp,
            SimulationEvent::ClockTick { timestamp, .. } => *timestamp,
        }
    }
    
    /// Signal changes should be processed asynchronously for performance
    fn is_synchronous(&self) -> bool {
        !matches!(self, SimulationEvent::SignalChanged { .. })
    }
}

/// Observer trait for handling events
/// 
/// **⚠️ UNSTABLE API**: Observer interface may be extended with additional methods
pub trait Observer<E: Event>: Send + Sync {
    /// Handle an event
    fn on_event(&mut self, event: &E) -> EventResult<()>;
    
    /// Get observer name for debugging
    fn name(&self) -> &str {
        "UnnamedObserver"
    }
    
    /// Check if observer should receive this specific event
    fn should_handle(&self, _event: &E) -> bool {
        true
    }
}

/// Weak reference to an observer to prevent circular references
type WeakObserver<E> = Weak<Mutex<dyn Observer<E>>>;

/// Event dispatcher for managing observers and delivering events
/// 
/// **⚠️ UNSTABLE API**: EventDispatcher interface is experimental
pub struct EventDispatcher<E: Event> {
    observers: HashMap<ObserverId, WeakObserver<E>>,
    event_queue: Vec<E>,
    is_processing: bool,
}

impl<E: Event> EventDispatcher<E> {
    /// Create a new event dispatcher
    pub fn new() -> Self {
        Self {
            observers: HashMap::new(),
            event_queue: Vec::new(),
            is_processing: false,
        }
    }
    
    /// Register an observer
    pub fn register_observer(&mut self, observer: Arc<Mutex<dyn Observer<E>>>) -> ObserverId {
        let id = ObserverId::new();
        self.observers.insert(id, Arc::downgrade(&observer));
        id
    }
    
    /// Unregister an observer
    pub fn unregister_observer(&mut self, id: ObserverId) -> EventResult<()> {
        self.observers.remove(&id)
            .ok_or(EventError::ObserverNotFound)
            .map(|_| ())
    }
    
    /// Emit an event to all registered observers
    pub fn emit(&mut self, event: E) -> EventResult<()> {
        if event.is_synchronous() {
            self.deliver_event(&event)
        } else {
            self.event_queue.push(event);
            Ok(())
        }
    }
    
    /// Process queued asynchronous events
    pub fn process_queue(&mut self) -> EventResult<()> {
        if self.is_processing {
            return Ok(());
        }
        
        self.is_processing = true;
        let events = std::mem::take(&mut self.event_queue);
        
        for event in events {
            if let Err(e) = self.deliver_event(&event) {
                log::warn!("Failed to deliver queued event: {}", e);
            }
        }
        
        self.is_processing = false;
        Ok(())
    }
    
    /// Deliver event to all observers
    fn deliver_event(&mut self, event: &E) -> EventResult<()> {
        // Clean up dead weak references
        self.observers.retain(|_, weak| weak.strong_count() > 0);
        
        let mut delivery_errors = Vec::new();
        
        for (observer_id, weak_observer) in &self.observers {
            if let Some(observer) = weak_observer.upgrade() {
                match observer.lock() {
                    Ok(mut obs) => {
                        if obs.should_handle(event) {
                            if let Err(e) = obs.on_event(event) {
                                delivery_errors.push(format!("Observer {}: {}", obs.name(), e));
                            }
                        }
                    }
                    Err(_) => {
                        delivery_errors.push(format!("Observer {:?}: mutex poisoned", observer_id));
                    }
                }
            }
        }
        
        if !delivery_errors.is_empty() {
            return Err(EventError::DeliveryFailed(delivery_errors.join("; ")));
        }
        
        Ok(())
    }
    
    /// Get number of active observers
    pub fn observer_count(&self) -> usize {
        self.observers.values().filter(|weak| weak.strong_count() > 0).count()
    }
    
    /// Get number of queued events
    pub fn queue_length(&self) -> usize {
        self.event_queue.len()
    }
}

impl<E: Event> Default for EventDispatcher<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Global event system manager
/// 
/// **⚠️ UNSTABLE API**: Global event system design is experimental
pub struct EventSystem {
    circuit_dispatcher: EventDispatcher<CircuitEvent>,
    simulation_dispatcher: EventDispatcher<SimulationEvent>,
}

impl EventSystem {
    /// Create a new event system
    pub fn new() -> Self {
        Self {
            circuit_dispatcher: EventDispatcher::new(),
            simulation_dispatcher: EventDispatcher::new(),
        }
    }
    
    /// Get circuit event dispatcher
    pub fn circuit_events(&mut self) -> &mut EventDispatcher<CircuitEvent> {
        &mut self.circuit_dispatcher
    }
    
    /// Get simulation event dispatcher
    pub fn simulation_events(&mut self) -> &mut EventDispatcher<SimulationEvent> {
        &mut self.simulation_dispatcher
    }
    
    /// Process all queued events
    pub fn process_all_queues(&mut self) -> EventResult<()> {
        self.circuit_dispatcher.process_queue()?;
        self.simulation_dispatcher.process_queue()?;
        Ok(())
    }
    
    /// Get system statistics
    pub fn stats(&self) -> EventSystemStats {
        EventSystemStats {
            circuit_observers: self.circuit_dispatcher.observer_count(),
            simulation_observers: self.simulation_dispatcher.observer_count(),
            circuit_queue_length: self.circuit_dispatcher.queue_length(),
            simulation_queue_length: self.simulation_dispatcher.queue_length(),
        }
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Event system statistics
#[derive(Debug, Clone)]
pub struct EventSystemStats {
    pub circuit_observers: usize,
    pub simulation_observers: usize,
    pub circuit_queue_length: usize,
    pub simulation_queue_length: usize,
}

/// Utility functions for creating events with current timestamp
pub mod event_utils {
    use super::*;
    
    /// Get current timestamp for events
    pub fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
    
    /// Create a component added event
    pub fn component_added(component_id: ComponentId, location: Location) -> CircuitEvent {
        CircuitEvent::ComponentAdded {
            component_id,
            location,
            timestamp: current_timestamp(),
        }
    }
    
    /// Create a signal changed event
    pub fn signal_changed(component_id: ComponentId, signal: Signal) -> SimulationEvent {
        SimulationEvent::SignalChanged {
            component_id,
            signal,
            timestamp: current_timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    #[derive(Debug)]
    struct TestEvent {
        name: String,
        timestamp: u64,
    }
    
    impl Event for TestEvent {
        fn event_type(&self) -> &'static str {
            "TestEvent"
        }
        
        fn timestamp(&self) -> u64 {
            self.timestamp
        }
    }
    
    struct TestObserver {
        name: String,
        events_received: Arc<AtomicUsize>,
    }
    
    impl Observer<TestEvent> for TestObserver {
        fn on_event(&mut self, _event: &TestEvent) -> EventResult<()> {
            self.events_received.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
        
        fn name(&self) -> &str {
            &self.name
        }
    }
    
    #[test]
    fn test_event_dispatcher_registration() {
        let mut dispatcher = EventDispatcher::<TestEvent>::new();
        let counter = Arc::new(AtomicUsize::new(0));
        
        let observer = Arc::new(Mutex::new(TestObserver {
            name: "test".to_string(),
            events_received: counter.clone(),
        }));
        
        let id = dispatcher.register_observer(observer);
        assert_eq!(dispatcher.observer_count(), 1);
        
        dispatcher.unregister_observer(id).unwrap();
        dispatcher.observers.retain(|_, weak| weak.strong_count() > 0);
        assert_eq!(dispatcher.observer_count(), 0);
    }
    
    #[test]
    fn test_event_delivery() {
        let mut dispatcher = EventDispatcher::<TestEvent>::new();
        let counter = Arc::new(AtomicUsize::new(0));
        
        let observer = Arc::new(Mutex::new(TestObserver {
            name: "test".to_string(),
            events_received: counter.clone(),
        }));
        
        dispatcher.register_observer(observer);
        
        let event = TestEvent {
            name: "test_event".to_string(),
            timestamp: event_utils::current_timestamp(),
        };
        
        dispatcher.emit(event).unwrap();
        assert_eq!(counter.load(Ordering::Relaxed), 1);
    }
    
    #[test]
    fn test_circuit_event_creation() {
        let event = event_utils::component_added(ComponentId::new(), Location::new(10, 20));
        assert_eq!(event.event_type(), "ComponentAdded");
        assert!(event.timestamp() > 0);
    }
}