/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! BCD to Seven Segment Display Decoder Component
//!
//! This module implements a BCD to 7-segment display decoder that converts
//! 4-bit BCD input to seven individual segment outputs for driving a
//! 7-segment display.

use crate::data::{Attribute, AttributeSet, BitWidth, Bounds, Direction, Value, Location};
use crate::instance::{InstanceFactory, InstancePainter, InstanceState, Port, Instance, StdAttr};
use crate::tools::Tool;
use std::fmt::Debug;

/// BCD to Seven Segment Display Decoder
///
/// Converts 4-bit BCD (Binary Coded Decimal) input to seven segment outputs
/// for driving a 7-segment display. Each segment output is a single bit that
/// controls whether that segment should be illuminated.
///
/// ## Segments
///
/// ```text
///  AAA
/// F   B
/// F   B  
///  GGG
/// E   C
/// E   C
///  DDD
/// ```
///
/// ## Ports
///
/// - **Input**: 4-bit BCD value (0-9, values 10-15 display as blank)
/// - **Outputs**: 7 single-bit segment outputs (A, B, C, D, E, F, G)
///
/// ## Truth Table
///
/// | BCD | A | B | C | D | E | F | G | Display |
/// |-----|---|---|---|---|---|---|---|---------|
/// |  0  | 1 | 1 | 1 | 1 | 1 | 1 | 0 |    0    |
/// |  1  | 0 | 1 | 1 | 0 | 0 | 0 | 0 |    1    |
/// |  2  | 1 | 1 | 0 | 1 | 1 | 0 | 1 |    2    |
/// |  3  | 1 | 1 | 1 | 1 | 0 | 0 | 1 |    3    |
/// |  4  | 0 | 1 | 1 | 0 | 0 | 1 | 1 |    4    |
/// |  5  | 1 | 0 | 1 | 1 | 0 | 1 | 1 |    5    |
/// |  6  | 1 | 0 | 1 | 1 | 1 | 1 | 1 |    6    |
/// |  7  | 1 | 1 | 1 | 0 | 0 | 0 | 0 |    7    |
/// |  8  | 1 | 1 | 1 | 1 | 1 | 1 | 1 |    8    |
/// |  9  | 1 | 1 | 1 | 1 | 0 | 1 | 1 |    9    |
/// | >9  | X | X | X | X | X | X | X |  blank  |
#[derive(Debug, Clone)]
pub struct BcdToSevenSegmentDisplay;

impl BcdToSevenSegmentDisplay {
    /// Unique identifier of the tool, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "BCD_to_7_Segment_decoder";

    /// Propagation delay in time units
    pub const PROPAGATION_DELAY: i32 = 1;

    // Port indices
    pub const SEGMENT_A: usize = 0;
    pub const SEGMENT_B: usize = 1;
    pub const SEGMENT_C: usize = 2;
    pub const SEGMENT_D: usize = 3;
    pub const SEGMENT_E: usize = 4;
    pub const SEGMENT_F: usize = 5;
    pub const SEGMENT_G: usize = 6;
    pub const BCD_INPUT: usize = 7;

    /// Creates a new BCD to Seven Segment Display decoder
    pub fn new() -> Self {
        Self
    }

    /// Get the 7-segment pattern for a given BCD digit
    /// Returns a 7-bit value where bit 0 = segment A, bit 6 = segment G
    fn get_segment_pattern(bcd_value: u8) -> u8 {
        match bcd_value {
            0 => 0b0111111, // 0: A,B,C,D,E,F on
            1 => 0b0000110, // 1: B,C on
            2 => 0b1011011, // 2: A,B,D,E,G on
            3 => 0b1001111, // 3: A,B,C,D,G on
            4 => 0b1100110, // 4: B,C,F,G on
            5 => 0b1101101, // 5: A,C,D,F,G on
            6 => 0b1111101, // 6: A,C,D,E,F,G on
            7 => 0b0000111, // 7: A,B,C on
            8 => 0b1111111, // 8: All segments on
            9 => 0b1101111, // 9: A,B,C,D,F,G on
            _ => 0b0000000, // Invalid/blank: All segments off
        }
    }

    /// Set all segment outputs to known values based on the pattern
    fn set_segments_known(&self, state: &mut InstanceState, pattern: u8) {
        for segment in 0..7 {
            let bit_value = (pattern >> segment) & 1;
            let value = Value::create_known(BitWidth::create(1), bit_value as i64);
            state.set_port(segment, value, Self::PROPAGATION_DELAY);
        }
    }

    /// Set all segment outputs to unknown
    fn set_segments_unknown(&self, state: &mut InstanceState) {
        for segment in 0..7 {
            let value = Value::create_unknown(BitWidth::create(1));
            state.set_port(segment, value, Self::PROPAGATION_DELAY);
        }
    }
}

impl Default for BcdToSevenSegmentDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for BcdToSevenSegmentDisplay {
    fn get_name(&self) -> &str {
        "BCD2SevenSegment"
    }

    fn get_display_name(&self) -> &str {
        "BCD to 7-Segment Display"
    }

    fn get_description(&self) -> Option<String> {
        Some("Converts 4-bit BCD input to seven segment display outputs".to_string())
    }

    fn clone_tool(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }
}

impl InstanceFactory for BcdToSevenSegmentDisplay {
    fn create_instance(&self, id: crate::ComponentId, location: Location) -> Instance {
        let mut instance = Instance::new(id, Self::ID.to_string(), location);
        
        // Set default attributes (using dummy attribute as in Java)
        let mut attrs = AttributeSet::new();
        attrs.set_value(StdAttr::DUMMY, "".to_string());
        instance.set_attribute_set(attrs);
        
        // Create ports
        let mut ports = Vec::with_capacity(8);
        
        // Segment output ports (A through G)
        ports.push(Port::new(
            Location::create(20, 0),
            Port::OUTPUT,
            BitWidth::create(1),
        ).with_tooltip("Segment A"));

        ports.push(Port::new(
            Location::create(30, 0),
            Port::OUTPUT,
            BitWidth::create(1),
        ).with_tooltip("Segment B"));

        ports.push(Port::new(
            Location::create(20, 60),
            Port::OUTPUT,
            BitWidth::create(1),
        ).with_tooltip("Segment C"));

        ports.push(Port::new(
            Location::create(10, 60),
            Port::OUTPUT,
            BitWidth::create(1),
        ).with_tooltip("Segment D"));

        ports.push(Port::new(
            Location::create(0, 60),
            Port::OUTPUT,
            BitWidth::create(1),
        ).with_tooltip("Segment E"));

        ports.push(Port::new(
            Location::create(10, 0),
            Port::OUTPUT,
            BitWidth::create(1),
        ).with_tooltip("Segment F"));

        ports.push(Port::new(
            Location::create(0, 0),
            Port::OUTPUT,
            BitWidth::create(1),
        ).with_tooltip("Segment G"));

        // BCD input port
        ports.push(Port::new(
            Location::create(10, 80),
            Port::INPUT,
            BitWidth::create(4),
        ).with_tooltip("BCD Value"));
        
        instance.set_ports(ports);
        instance
    }

    fn paint_instance(&self, painter: &mut InstancePainter) {
        let bounds = painter.get_bounds();
        
        // Draw component body
        painter.draw_rectangle(bounds, "");
        
        // Draw BCD input port
        painter.draw_port(Self::BCD_INPUT, "BCD", Direction::South);
        
        // Draw segment output ports (no labels needed, just port indicators)
        for i in 0..7 {
            painter.draw_port(i, "", Direction::North);
        }
        
        // Draw internal 7-segment display representation
        let inner_x = bounds.x() + 5;
        let inner_y = bounds.y() + 20;
        let inner_width = bounds.width() - 10;
        let inner_height = bounds.height() - 40;
        
        painter.draw_rectangle(
            Bounds::create(inner_x, inner_y, inner_width, inner_height),
            "",
        );
    }

    fn get_offset_bounds(&self, _attrs: &AttributeSet) -> Bounds {
        // Fixed size component as defined in Java
        Bounds::create(-10, -20, 50, 100)
    }

    fn propagate(&self, state: &mut InstanceState) {
        let input_value = state.get_port_value(Self::BCD_INPUT);
        
        // Check if input is valid
        if input_value.is_fully_defined() 
            && !input_value.is_error_value() 
            && !input_value.is_unknown() {
            
            let bcd_digit = input_value.to_long_value().unwrap_or(0) as u8;
            let segment_pattern = Self::get_segment_pattern(bcd_digit);
            self.set_segments_known(state, segment_pattern);
        } else {
            // Input is invalid, set all segments to unknown
            self.set_segments_unknown(state);
        }
    }

    fn configure_new_instance(&self, _instance: &mut Instance) {
        // No special configuration needed
    }

    fn instance_attribute_changed(&self, _instance: &mut Instance, _attr: &dyn std::any::Any) {
        // No attributes to handle
    }

    fn get_hdl_name(&self, _attrs: &AttributeSet) -> String {
        "BCD_to_7_Segment_decoder".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ComponentId;

    #[test]
    fn test_component_creation() {
        let decoder = BcdToSevenSegmentDisplay::new();
        assert_eq!(decoder.get_name(), "BCD2SevenSegment");
        assert_eq!(decoder.get_display_name(), "BCD to 7-Segment Display");
    }

    #[test]
    fn test_id_constant() {
        // Ensure ID never changes for .circ file compatibility
        assert_eq!(BcdToSevenSegmentDisplay::ID, "BCD_to_7_Segment_decoder");
    }

    #[test]
    fn test_segment_patterns() {
        // Test all valid BCD digits
        let expected_patterns = [
            (0, 0b0111111), // 0
            (1, 0b0000110), // 1
            (2, 0b1011011), // 2
            (3, 0b1001111), // 3
            (4, 0b1100110), // 4
            (5, 0b1101101), // 5
            (6, 0b1111101), // 6
            (7, 0b0000111), // 7
            (8, 0b1111111), // 8
            (9, 0b1101111), // 9
        ];

        for (digit, expected) in expected_patterns {
            let pattern = BcdToSevenSegmentDisplay::get_segment_pattern(digit);
            assert_eq!(pattern, expected, "Pattern mismatch for digit {}", digit);
        }
    }

    #[test]
    fn test_invalid_bcd_values() {
        // Test invalid BCD values (should return blank pattern)
        for invalid_digit in 10..16 {
            let pattern = BcdToSevenSegmentDisplay::get_segment_pattern(invalid_digit);
            assert_eq!(pattern, 0b0000000, "Invalid digit {} should be blank", invalid_digit);
        }
    }

    #[test]
    fn test_instance_creation() {
        let decoder = BcdToSevenSegmentDisplay::new();
        let instance = decoder.create_instance(
            ComponentId(1), 
            Location::create(100, 100)
        );
        
        assert_eq!(instance.get_factory_id(), BcdToSevenSegmentDisplay::ID);
        assert_eq!(instance.get_location(), Location::create(100, 100));
        
        // Should have 8 ports (7 segments + 1 input)
        assert_eq!(instance.get_ports().len(), 8);
    }

    #[test]
    fn test_port_configuration() {
        let decoder = BcdToSevenSegmentDisplay::new();
        let instance = decoder.create_instance(
            ComponentId(1), 
            Location::create(0, 0)
        );
        
        let ports = instance.get_ports();
        
        // Check BCD input port
        let bcd_port = &ports[BcdToSevenSegmentDisplay::BCD_INPUT];
        assert_eq!(bcd_port.get_type(), Port::INPUT);
        assert_eq!(bcd_port.get_width(), BitWidth::create(4));
        
        // Check segment output ports
        for i in 0..7 {
            let segment_port = &ports[i];
            assert_eq!(segment_port.get_type(), Port::OUTPUT);
            assert_eq!(segment_port.get_width(), BitWidth::create(1));
        }
    }

    #[test]
    fn test_bounds() {
        let decoder = BcdToSevenSegmentDisplay::new();
        let attrs = AttributeSet::new();
        let bounds = decoder.get_offset_bounds(&attrs);
        
        assert_eq!(bounds.x(), -10);
        assert_eq!(bounds.y(), -20);
        assert_eq!(bounds.width(), 50);
        assert_eq!(bounds.height(), 100);
    }

    #[test]
    fn test_hdl_name() {
        let decoder = BcdToSevenSegmentDisplay::new();
        let attrs = AttributeSet::new();
        let hdl_name = decoder.get_hdl_name(&attrs);
        
        assert_eq!(hdl_name, "BCD_to_7_Segment_decoder");
    }

    #[test]
    fn test_segment_bit_mapping() {
        // Verify that our bit mapping matches the expected layout
        // Pattern 0b1011011 for digit 2 should have:
        // A(0)=1, B(1)=1, C(2)=0, D(3)=1, E(4)=1, F(5)=0, G(6)=1
        let pattern = BcdToSevenSegmentDisplay::get_segment_pattern(2);
        
        assert_eq!((pattern >> 0) & 1, 1); // Segment A
        assert_eq!((pattern >> 1) & 1, 1); // Segment B
        assert_eq!((pattern >> 2) & 1, 0); // Segment C
        assert_eq!((pattern >> 3) & 1, 1); // Segment D
        assert_eq!((pattern >> 4) & 1, 1); // Segment E
        assert_eq!((pattern >> 5) & 1, 0); // Segment F
        assert_eq!((pattern >> 6) & 1, 1); // Segment G
    }
}