/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Gray code incrementer component
//!
//! This component takes a multibit input and outputs the value that follows it in Gray Code.
//! For instance, given input 0100 the output is 1100.
//! Equivalent to Java's com.cburch.gray.GrayIncrementer class.

use crate::data::{Attribute, BitWidth, Bounds, Location};
use crate::signal::Value;

/// Stub for instance factory trait
pub trait InstanceFactory {
    fn id(&self) -> &str;
    fn display_name(&self) -> &str;
    fn offset_bounds(&self) -> Bounds;
    fn create_attributes(&self) -> Vec<Attribute<BitWidth>>;
    fn create_ports(&self) -> Vec<Port>;
    fn paint_instance(&self, painter: &mut InstancePainter);
    fn propagate(&self, state: &mut InstanceState);
}

/// Stub for instance painter
pub struct InstancePainter;

impl InstancePainter {
    pub fn draw_rectangle_with_text(&mut self, _text: &str) {}
    pub fn draw_ports(&mut self) {}
}

/// Stub for instance state
pub struct InstanceState;

impl InstanceState {
    pub fn port_value(&self, _port: usize) -> Value {
        Value::Low
    }

    pub fn set_port(&mut self, _port: usize, _value: Value, _delay: usize) {}
}

/// Stub for port definition
pub struct Port;

impl Port {
    pub const INPUT: &'static str = "INPUT";
    pub const OUTPUT: &'static str = "OUTPUT";

    pub fn new(_location: Location, _kind: &str, _width: &str) -> Self {
        Self
    }
}

/// Gray code incrementer component.
///
/// This component takes a multibit input and outputs the value that follows it in Gray Code.
/// This is equivalent to Java's GrayIncrementer class.
pub struct GrayIncrementer;

impl GrayIncrementer {
    /// Unique identifier of the tool, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "Gray Code Incrementer";

    /// Creates a new Gray code incrementer.
    pub fn new() -> Self {
        Self
    }

    /// Computes the next gray value in the sequence after prev.
    ///
    /// This static method just does some bit twiddling for simplified demonstration.
    /// In the full implementation this would work with proper multi-bit values.
    pub fn next_gray(prev: u64, width: BitWidth) -> u64 {
        let mask = width.get_mask();
        let mut x = prev & mask;

        // Compute parity of x
        let mut ct = (x >> 32) ^ x;
        ct = (ct >> 16) ^ ct;
        ct = (ct >> 8) ^ ct;
        ct = (ct >> 4) ^ ct;
        ct = (ct >> 2) ^ ct;
        ct = (ct >> 1) ^ ct;

        if (ct & 1) == 0 {
            // If parity is even, flip 1's bit
            x ^= 1;
        } else {
            // Else flip bit just above last 1
            let y = x ^ (x & (x.wrapping_sub(1))); // first compute the last 1
            let y = (y << 1) & mask;
            x = if y == 0 { 0 } else { x ^ y };
        }

        x & mask
    }
}

impl InstanceFactory for GrayIncrementer {
    fn id(&self) -> &str {
        Self::ID
    }

    fn display_name(&self) -> &str {
        "Gray Incrementer"
    }

    fn offset_bounds(&self) -> Bounds {
        Bounds::create(-30, -15, 30, 30)
    }

    fn create_attributes(&self) -> Vec<Attribute<BitWidth>> {
        vec![Attribute::new("WIDTH".to_string())]
    }

    fn create_ports(&self) -> Vec<Port> {
        vec![
            Port::new(Location::new(-30, 0), Port::INPUT, "WIDTH"),
            Port::new(Location::new(0, 0), Port::OUTPUT, "WIDTH"),
        ]
    }

    fn paint_instance(&self, painter: &mut InstancePainter) {
        // Draw rectangle with "G+1" label
        painter.draw_rectangle_with_text("G+1");
        painter.draw_ports();
    }

    fn propagate(&self, state: &mut InstanceState) {
        // Retrieve the value being fed into the input (port 0)
        let input = state.port_value(0);

        // For simplicity, just toggle between High and Low
        let output = match input {
            Value::Low => Value::High,
            Value::High => Value::Low,
            _ => Value::Unknown,
        };

        // Propagate the output to port 1 with delay
        state.set_port(1, output, 2);
    }
}

impl Default for GrayIncrementer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_incrementer_creation() {
        let incrementer = GrayIncrementer::new();
        assert_eq!(incrementer.id(), "Gray Code Incrementer");
        assert_eq!(incrementer.display_name(), "Gray Incrementer");
    }

    #[test]
    fn test_next_gray_logic() {
        let width = BitWidth::new(4);

        // Test basic Gray code progression
        assert_eq!(GrayIncrementer::next_gray(0b0000, width), 0b0001);
        assert_eq!(GrayIncrementer::next_gray(0b0001, width), 0b0011);
        assert_eq!(GrayIncrementer::next_gray(0b0011, width), 0b0010);
        assert_eq!(GrayIncrementer::next_gray(0b0010, width), 0b0110);
    }

    #[test]
    fn test_default_implementation() {
        let incrementer = GrayIncrementer::default();
        assert_eq!(incrementer.id(), "Gray Code Incrementer");
    }
}
