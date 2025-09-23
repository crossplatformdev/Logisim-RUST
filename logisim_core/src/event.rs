//! Event queue and simulation events.
//!
//! This module implements the event-driven simulation engine, including
//! the priority queue for scheduling events and the event types used
//! throughout the simulation.

use crate::signal::{Signal, Timestamp};
use crate::component::ComponentId;
use crate::netlist::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::fmt;

/// Unique identifier for simulation events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EventId(pub u64);

impl EventId {
    pub fn new(id: u64) -> Self {
        EventId(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for EventId {
    fn from(id: u64) -> Self {
        EventId(id)
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{}", self.0)
    }
}

/// Types of simulation events
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// Signal value change at a node
    SignalChange {
        node_id: NodeId,
        new_signal: Signal,
        source_component: ComponentId,
    },
    /// Clock tick event
    ClockTick,
    /// Component evaluation request
    ComponentUpdate {
        component_id: ComponentId,
    },
    /// Reset signal
    Reset,
}

/// A simulation event with timing information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatorEvent {
    /// Unique identifier for this event
    pub id: EventId,
    /// When this event should be processed
    pub time: Timestamp,
    /// Serial number for ordering events at the same time
    pub serial: u64,
    /// The type and data of this event
    pub event_type: EventType,
}

impl SimulatorEvent {
    /// Create a new signal change event
    pub fn signal_change(
        id: EventId,
        time: Timestamp,
        serial: u64,
        node_id: NodeId,
        new_signal: Signal,
        source_component: ComponentId,
    ) -> Self {
        SimulatorEvent {
            id,
            time,
            serial,
            event_type: EventType::SignalChange {
                node_id,
                new_signal,
                source_component,
            },
        }
    }

    /// Create a new clock tick event
    pub fn clock_tick(id: EventId, time: Timestamp, serial: u64) -> Self {
        SimulatorEvent {
            id,
            time,
            serial,
            event_type: EventType::ClockTick,
        }
    }

    /// Create a new component update event
    pub fn component_update(id: EventId, time: Timestamp, serial: u64, component_id: ComponentId) -> Self {
        SimulatorEvent {
            id,
            time,
            serial,
            event_type: EventType::ComponentUpdate { component_id },
        }
    }

    /// Create a new reset event
    pub fn reset(id: EventId, time: Timestamp, serial: u64) -> Self {
        SimulatorEvent {
            id,
            time,
            serial,
            event_type: EventType::Reset,
        }
    }
}

// Implement ordering for the priority queue (earliest time first, then by serial number)
impl PartialOrd for SimulatorEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimulatorEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // For min-heap with Reverse wrapper, normal ordering gives us min-heap behavior
        self.time.cmp(&other.time)
            .then_with(|| self.serial.cmp(&other.serial))
    }
}

/// Event queue for managing simulation events
#[derive(Debug, Clone)]
pub struct EventQueue {
    /// Priority queue of events (min-heap by time)
    events: BinaryHeap<Reverse<SimulatorEvent>>,
    /// Next event ID to assign
    next_event_id: u64,
    /// Next serial number for events
    next_serial: u64,
    /// Current simulation time
    current_time: Timestamp,
}

impl EventQueue {
    /// Create a new empty event queue
    pub fn new() -> Self {
        EventQueue {
            events: BinaryHeap::new(),
            next_event_id: 1,
            next_serial: 1,
            current_time: Timestamp(0),
        }
    }

    /// Schedule a signal change event
    pub fn schedule_signal_change(
        &mut self,
        time: Timestamp,
        node_id: NodeId,
        new_signal: Signal,
        source_component: ComponentId,
    ) -> EventId {
        let event_id = EventId(self.next_event_id);
        self.next_event_id += 1;
        
        let serial = self.next_serial;
        self.next_serial += 1;

        let event = SimulatorEvent::signal_change(
            event_id,
            time,
            serial,
            node_id,
            new_signal,
            source_component,
        );

        self.events.push(Reverse(event));
        event_id
    }

    /// Schedule a clock tick event
    pub fn schedule_clock_tick(&mut self, time: Timestamp) -> EventId {
        let event_id = EventId(self.next_event_id);
        self.next_event_id += 1;
        
        let serial = self.next_serial;
        self.next_serial += 1;

        let event = SimulatorEvent::clock_tick(event_id, time, serial);
        self.events.push(Reverse(event));
        event_id
    }

    /// Schedule a component update event
    pub fn schedule_component_update(&mut self, time: Timestamp, component_id: ComponentId) -> EventId {
        let event_id = EventId(self.next_event_id);
        self.next_event_id += 1;
        
        let serial = self.next_serial;
        self.next_serial += 1;

        let event = SimulatorEvent::component_update(event_id, time, serial, component_id);
        self.events.push(Reverse(event));
        event_id
    }

    /// Schedule a reset event
    pub fn schedule_reset(&mut self, time: Timestamp) -> EventId {
        let event_id = EventId(self.next_event_id);
        self.next_event_id += 1;
        
        let serial = self.next_serial;
        self.next_serial += 1;

        let event = SimulatorEvent::reset(event_id, time, serial);
        self.events.push(Reverse(event));
        event_id
    }

    /// Get the next event without removing it
    pub fn peek(&self) -> Option<&SimulatorEvent> {
        self.events.peek().map(|Reverse(event)| event)
    }

    /// Remove and return the next event
    pub fn pop(&mut self) -> Option<SimulatorEvent> {
        self.events.pop().map(|Reverse(event)| {
            self.current_time = event.time;
            event
        })
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get the number of events in the queue
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Get the current simulation time
    pub fn current_time(&self) -> Timestamp {
        self.current_time
    }

    /// Clear all events from the queue
    pub fn clear(&mut self) {
        self.events.clear();
        self.current_time = Timestamp(0);
    }

    /// Get the time of the next event
    pub fn next_event_time(&self) -> Option<Timestamp> {
        self.peek().map(|event| event.time)
    }

    /// Remove all events scheduled for or after the given time
    pub fn cancel_events_after(&mut self, time: Timestamp) {
        let mut remaining_events = BinaryHeap::new();
        
        while let Some(Reverse(event)) = self.events.pop() {
            if event.time < time {
                remaining_events.push(Reverse(event));
            }
        }
        
        self.events = remaining_events;
    }

    /// Get all events currently in the queue (for debugging)
    pub fn get_all_events(&self) -> Vec<&SimulatorEvent> {
        self.events.iter().map(|Reverse(event)| event).collect()
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EventQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "EventQueue (current_time: {}, {} events):", self.current_time, self.len())?;
        let events: Vec<_> = self.get_all_events();
        for event in events.iter().take(10) { // Show first 10 events
            writeln!(f, "  {}: {:?}", event.time, event.event_type)?;
        }
        if events.len() > 10 {
            writeln!(f, "  ... and {} more events", events.len() - 10)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::NodeId;

    #[test]
    fn test_event_ordering() {
        let event1 = SimulatorEvent::clock_tick(EventId(1), Timestamp(10), 1);
        let event2 = SimulatorEvent::clock_tick(EventId(2), Timestamp(5), 2);
        let event3 = SimulatorEvent::clock_tick(EventId(3), Timestamp(10), 3);

        // Test the basic ordering - event2 (time=5) should be less than event1 (time=10)
        assert!(event2 < event1, "Event with earlier time should be less");
        
        // Same time, lower serial should be less
        assert!(event1 < event3, "Event with same time but lower serial should be less");
        
        // Test the heap behavior by inserting into a priority queue
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Reverse(event1.clone()));
        heap.push(Reverse(event2.clone()));
        heap.push(Reverse(event3.clone()));
        
        // Should pop events in time order
        assert_eq!(heap.pop().unwrap().0.time, Timestamp(5));
        assert_eq!(heap.pop().unwrap().0.time, Timestamp(10));
        assert_eq!(heap.pop().unwrap().0.time, Timestamp(10));
    }

    #[test]
    fn test_event_queue_basic() {
        let mut queue = EventQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);

        // Schedule some events
        let _id1 = queue.schedule_clock_tick(Timestamp(10));
        let _id2 = queue.schedule_clock_tick(Timestamp(5));
        let _id3 = queue.schedule_clock_tick(Timestamp(15));

        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());

        // Events should come out in time order
        let event1 = queue.pop().unwrap();
        assert_eq!(event1.time, Timestamp(5));

        let event2 = queue.pop().unwrap();
        assert_eq!(event2.time, Timestamp(10));

        let event3 = queue.pop().unwrap();
        assert_eq!(event3.time, Timestamp(15));

        assert!(queue.is_empty());
    }

    #[test]
    fn test_event_queue_signal_change() {
        let mut queue = EventQueue::new();
        
        let signal = Signal::new_single(crate::signal::Value::High);
        let _id = queue.schedule_signal_change(
            Timestamp(100),
            NodeId(1),
            signal,
            ComponentId(1),
        );

        let event = queue.pop().unwrap();
        match event.event_type {
            EventType::SignalChange { node_id, source_component, .. } => {
                assert_eq!(node_id, NodeId(1));
                assert_eq!(source_component, ComponentId(1));
            }
            _ => panic!("Expected SignalChange event"),
        }
    }

    #[test]
    fn test_current_time_update() {
        let mut queue = EventQueue::new();
        assert_eq!(queue.current_time(), Timestamp(0));

        queue.schedule_clock_tick(Timestamp(50));
        queue.schedule_clock_tick(Timestamp(25));

        // Current time should update when popping events
        queue.pop();
        assert_eq!(queue.current_time(), Timestamp(25));

        queue.pop();
        assert_eq!(queue.current_time(), Timestamp(50));
    }

    #[test]
    fn test_cancel_events_after() {
        let mut queue = EventQueue::new();
        
        queue.schedule_clock_tick(Timestamp(10));
        queue.schedule_clock_tick(Timestamp(20));
        queue.schedule_clock_tick(Timestamp(30));
        queue.schedule_clock_tick(Timestamp(40));

        assert_eq!(queue.len(), 4);

        // Cancel events at or after time 25
        queue.cancel_events_after(Timestamp(25));
        
        assert_eq!(queue.len(), 2);

        // Remaining events should be before time 25
        let event1 = queue.pop().unwrap();
        assert_eq!(event1.time, Timestamp(10));

        let event2 = queue.pop().unwrap();
        assert_eq!(event2.time, Timestamp(20));

        assert!(queue.is_empty());
    }
}