/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Core component traits and types
//!
//! This module contains the fundamental component abstractions equivalent to
//! Java's `Component` and `AbstractComponent` classes. These define the
//! basic interface that all digital components must implement.

use crate::data::{AttributeSet, Bounds, Location};
use crate::signal::{Signal, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use super::pin::Pin;

/// Unique identifier for a component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ComponentId(pub u64);

impl ComponentId {
    /// Create a new component ID
    pub fn new(id: u64) -> Self {
        ComponentId(id)
    }

    /// Get the ID as u64
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for ComponentId {
    fn from(id: u64) -> Self {
        ComponentId(id)
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}", self.0)
    }
}

/// Result of a component update
#[derive(Debug, Clone)]
pub struct UpdateResult {
    /// New output signals to propagate
    pub outputs: HashMap<String, Signal>,
    /// Propagation delay for these outputs
    pub delay: u64,
    /// Whether the component state changed
    pub state_changed: bool,
}

impl UpdateResult {
    /// Create a new update result with no outputs
    pub fn new() -> Self {
        UpdateResult {
            outputs: HashMap::new(),
            delay: 0,
            state_changed: false,
        }
    }

    /// Create an update result with outputs
    pub fn with_outputs(outputs: HashMap<String, Signal>, delay: u64) -> Self {
        UpdateResult {
            outputs,
            delay,
            state_changed: true,
        }
    }

    /// Add an output signal
    pub fn add_output(&mut self, pin_name: String, signal: Signal) {
        self.outputs.insert(pin_name, signal);
        self.state_changed = true;
    }

    /// Set the propagation delay
    pub fn set_delay(&mut self, delay: u64) {
        self.delay = delay;
    }
}

impl Default for UpdateResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Clock edge types for sequential components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClockEdge {
    /// Rising edge (low to high)
    Rising,
    /// Falling edge (high to low)
    Falling,
}

/// Trait that all simulation components must implement
///
/// This is the core interface equivalent to Java's `Component` interface.
/// All digital logic components must implement this trait to participate
/// in the simulation.
pub trait Component: std::fmt::Debug + Send + Sync {
    /// Get the unique identifier for this component
    fn id(&self) -> ComponentId;

    /// Get the name/type of this component
    fn name(&self) -> &str;

    /// Get all pins on this component
    fn pins(&self) -> &HashMap<String, Pin>;

    /// Get all pins on this component (mutable)
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin>;

    /// Get a specific pin by name
    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins().get(name)
    }

    /// Get a specific pin by name (mutable)
    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins_mut().get_mut(name)
    }

    /// Update the component's outputs based on current inputs
    /// This is called when input signals change
    fn update(&mut self, current_time: Timestamp) -> UpdateResult;

    /// Reset the component to its initial state
    fn reset(&mut self);

    /// Get the typical propagation delay for this component
    fn propagation_delay(&self) -> u64 {
        1 // Default 1 time unit delay
    }

    /// Get the location of this component (if positioned)
    fn location(&self) -> Option<Location> {
        None // Default: no position
    }

    /// Get the bounds of this component for drawing/hit-testing
    fn bounds(&self) -> Option<Bounds> {
        None // Default: no bounds
    }

    /// Get the attribute set for this component
    fn attribute_set(&self) -> Option<&AttributeSet> {
        None // Default: no attributes
    }

    /// Check if a point is contained within this component
    fn contains(&self, _location: Location) -> bool {
        false // Default: no containment
    }

    /// Check if this component has an endpoint at the given location
    fn ends_at(&self, _location: Location) -> bool {
        // Check if any pin is at this location
        // This is a simplified version - actual implementation would consider pin positions
        self.pins().values().any(|_pin| {
            // TODO: Implement proper pin position checking
            false
        })
    }

    /// Check if this component is sequential (has state that depends on clock)
    fn is_sequential(&self) -> bool {
        false // Default: most components are combinational
    }

    /// Handle a clock edge if this is a sequential component
    fn clock_edge(&mut self, _edge: ClockEdge, _current_time: Timestamp) -> UpdateResult {
        UpdateResult::new() // Default: no response to clock edges
    }
}

/// Abstract base implementation providing common component functionality
///
/// This provides default implementations for common component operations,
/// equivalent to Java's `AbstractComponent` class.
pub trait AbstractComponent: Component {
    /// Default contains implementation using bounds
    fn contains_default(&self, pt: Location) -> bool {
        if let Some(bounds) = self.bounds() {
            bounds.contains(pt.x, pt.y)
        } else {
            false
        }
    }

    /// Default ends_at implementation checking all pin locations
    fn ends_at_default(&self, _pt: Location) -> bool {
        // In a full implementation, this would check actual pin positions
        // For now, we provide a basic implementation
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::Value;

    #[test]
    fn test_component_id() {
        let id = ComponentId::new(42);
        assert_eq!(id.as_u64(), 42);
        assert_eq!(format!("{}", id), "C42");

        let id2: ComponentId = 123.into();
        assert_eq!(id2.as_u64(), 123);
    }

    #[test]
    fn test_update_result() {
        let mut result = UpdateResult::new();
        assert!(!result.state_changed);
        assert_eq!(result.outputs.len(), 0);
        assert_eq!(result.delay, 0);

        result.add_output("OUT".to_string(), Signal::new_single(Value::High));
        assert!(result.state_changed);
        assert_eq!(result.outputs.len(), 1);

        result.set_delay(5);
        assert_eq!(result.delay, 5);
    }

    #[test]
    fn test_update_result_with_outputs() {
        let mut outputs = HashMap::new();
        outputs.insert("Y".to_string(), Signal::new_single(Value::Low));

        let result = UpdateResult::with_outputs(outputs, 3);
        assert!(result.state_changed);
        assert_eq!(result.outputs.len(), 1);
        assert_eq!(result.delay, 3);
    }
}
