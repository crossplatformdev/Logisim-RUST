/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Pin and connection abstractions for components
//!
//! This module contains the pin abstraction system equivalent to Java's
//! `EndData` and related classes. It defines connection points on components
//! and how signals flow through them.

use crate::data::Location;
use crate::signal::{BusWidth, Signal};
use serde::{Deserialize, Serialize};

/// Direction of a pin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PinDirection {
    /// Input pin
    Input,
    /// Output pin
    Output,
    /// Bidirectional pin
    InOut,
}

/// Represents a connection point on a component
/// 
/// Equivalent to Java's pin/port concept, this defines where signals
/// can be connected to a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pin {
    /// Name of this pin
    pub name: String,
    /// Direction of this pin
    pub direction: PinDirection,
    /// Bus width of this pin
    pub width: BusWidth,
    /// Current signal on this pin
    pub signal: Signal,
}

impl Pin {
    /// Create a new input pin
    pub fn new_input(name: impl Into<String>, width: BusWidth) -> Self {
        Pin {
            name: name.into(),
            direction: PinDirection::Input,
            width,
            signal: Signal::unknown(width),
        }
    }

    /// Create a new output pin
    pub fn new_output(name: impl Into<String>, width: BusWidth) -> Self {
        Pin {
            name: name.into(),
            direction: PinDirection::Output,
            width,
            signal: Signal::unknown(width),
        }
    }

    /// Create a new bidirectional pin
    pub fn new_inout(name: impl Into<String>, width: BusWidth) -> Self {
        Pin {
            name: name.into(),
            direction: PinDirection::InOut,
            width,
            signal: Signal::unknown(width),
        }
    }

    /// Check if this is an input pin
    pub fn is_input(&self) -> bool {
        matches!(self.direction, PinDirection::Input | PinDirection::InOut)
    }

    /// Check if this is an output pin
    pub fn is_output(&self) -> bool {
        matches!(self.direction, PinDirection::Output | PinDirection::InOut)
    }

    /// Update the signal on this pin
    pub fn set_signal(&mut self, signal: Signal) -> Result<(), &'static str> {
        if signal.width() != self.width {
            return Err("Signal width mismatch");
        }
        self.signal = signal;
        Ok(())
    }

    /// Get the current signal
    pub fn get_signal(&self) -> &Signal {
        &self.signal
    }
}

/// EndData represents connection information for a component pin
/// 
/// This is equivalent to Java's `EndData` class and provides information
/// about where and how a pin can be connected in a circuit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndData {
    /// Location of this connection point
    pub location: Location,
    /// The pin this end data refers to
    pub pin_name: String,
    /// Bus width at this connection point
    pub width: BusWidth,
    /// Whether this is an input, output, or bidirectional connection
    pub direction: PinDirection,
    /// Whether this end point is exclusive (only one connection allowed)
    pub exclusive: bool,
}

impl EndData {
    /// Create new end data for a connection point
    pub fn new(
        location: Location,
        pin_name: String,
        width: BusWidth,
        direction: PinDirection,
    ) -> Self {
        EndData {
            location,
            pin_name,
            width,
            direction,
            exclusive: true, // Default to exclusive connections
        }
    }

    /// Create end data that allows multiple connections
    pub fn new_shared(
        location: Location,
        pin_name: String,
        width: BusWidth,
        direction: PinDirection,
    ) -> Self {
        EndData {
            location,
            pin_name,
            width,
            direction,
            exclusive: false,
        }
    }

    /// Get the location of this connection point
    pub fn location(&self) -> Location {
        self.location
    }

    /// Get the pin name this end data refers to
    pub fn pin_name(&self) -> &str {
        &self.pin_name
    }

    /// Get the bus width at this connection point
    pub fn width(&self) -> BusWidth {
        self.width
    }

    /// Get the direction of this connection point
    pub fn direction(&self) -> PinDirection {
        self.direction
    }

    /// Check if this connection point is exclusive
    pub fn is_exclusive(&self) -> bool {
        self.exclusive
    }

    /// Check if this is an input connection
    pub fn is_input(&self) -> bool {
        matches!(self.direction, PinDirection::Input | PinDirection::InOut)
    }

    /// Check if this is an output connection
    pub fn is_output(&self) -> bool {
        matches!(self.direction, PinDirection::Output | PinDirection::InOut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::{Value, BusWidth};

    #[test]
    fn test_pin_creation() {
        let pin = Pin::new_input("A", BusWidth(1));
        assert_eq!(pin.name, "A");
        assert!(pin.is_input());
        assert!(!pin.is_output());
        assert_eq!(pin.width, BusWidth(1));
    }

    #[test]
    fn test_pin_directions() {
        let input = Pin::new_input("IN", BusWidth(1));
        let output = Pin::new_output("OUT", BusWidth(1));
        let inout = Pin::new_inout("IO", BusWidth(1));

        assert!(input.is_input());
        assert!(!input.is_output());

        assert!(!output.is_input());
        assert!(output.is_output());

        assert!(inout.is_input());
        assert!(inout.is_output());
    }

    #[test]
    fn test_pin_signal_update() {
        let mut pin = Pin::new_input("A", BusWidth(1));
        let signal = Signal::new_single(Value::High);
        
        assert!(pin.set_signal(signal.clone()).is_ok());
        assert_eq!(pin.get_signal(), &signal);

        // Test width mismatch
        let wrong_signal = Signal::new_bus(vec![Value::High, Value::Low]);
        assert!(pin.set_signal(wrong_signal).is_err());
    }

    #[test]
    fn test_end_data() {
        let location = Location::new(10, 20);
        let end_data = EndData::new(
            location,
            "INPUT".to_string(),
            BusWidth(4),
            PinDirection::Input,
        );

        assert_eq!(end_data.location(), location);
        assert_eq!(end_data.pin_name(), "INPUT");
        assert_eq!(end_data.width(), BusWidth(4));
        assert_eq!(end_data.direction(), PinDirection::Input);
        assert!(end_data.is_exclusive());
        assert!(end_data.is_input());
        assert!(!end_data.is_output());
    }

    #[test]
    fn test_end_data_shared() {
        let location = Location::new(5, 15);
        let end_data = EndData::new_shared(
            location,
            "CLOCK".to_string(),
            BusWidth(1),
            PinDirection::Input,
        );

        assert!(!end_data.is_exclusive());
    }
}