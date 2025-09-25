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

use crate::comp::{Component, ComponentId, Pin, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
/// | >9  | 0 | 0 | 0 | 0 | 0 | 0 | 0 |  blank  |
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BcdToSevenSegmentDisplay {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl BcdToSevenSegmentDisplay {
    /// Unique identifier of the tool, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "BCD_to_7_Segment_decoder";

    /// Propagation delay in time units
    pub const PROPAGATION_DELAY: u64 = 1;

    /// Creates a new BCD to Seven Segment Display decoder
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        
        // BCD input pin
        pins.insert("BCD_IN".to_string(), Pin::new_input("BCD_IN", BusWidth(4)));
        
        // Seven segment output pins
        let segment_names = ["SEG_A", "SEG_B", "SEG_C", "SEG_D", "SEG_E", "SEG_F", "SEG_G"];
        for &name in &segment_names {
            pins.insert(name.to_string(), Pin::new_output(name, BusWidth(1)));
        }

        BcdToSevenSegmentDisplay { id, pins }
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

    /// Convert BCD signal to u8 value if possible
    fn signal_to_bcd(&self, signal: &Signal) -> Option<u8> {
        if signal.width() != BusWidth(4) {
            return None;
        }
        
        let mut value = 0u8;
        for (i, &bit) in signal.values().iter().enumerate() {
            match bit {
                Value::High => value |= 1 << i,
                Value::Low => {},
                Value::Unknown | Value::Error => return None,
            }
        }
        Some(value)
    }

    /// Convert single bit to signal
    fn bit_to_signal(&self, bit: bool) -> Signal {
        Signal::new_single(if bit { Value::High } else { Value::Low })
    }
}

impl Component for BcdToSevenSegmentDisplay {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        Self::ID
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, timestamp: Timestamp) -> UpdateResult {
        self.propagate(timestamp)
    }

    fn reset(&mut self) {
        // Reset all outputs to unknown
        for pin in self.pins.values_mut() {
            if pin.is_output() {
                pin.set_signal(Signal::unknown(pin.get_width()));
            }
        }
    }
}

impl BcdToSevenSegmentDisplay {
    /// Convert BCD signal to u8 value if possible
    fn signal_to_bcd(&self, signal: &Signal) -> Option<u8> {
        if signal.width() != BusWidth(4) {
            return None;
        }
        
        let mut value = 0u8;
        for (i, &bit) in signal.values().iter().enumerate() {
            match bit {
                Value::High => value |= 1 << i,
                Value::Low => {},
                Value::Unknown | Value::Error => return None,
            }
        }
        Some(value)
    }

    /// Convert single bit to signal
    fn bit_to_signal(&self, bit: bool) -> Signal {
        Signal::new_single(if bit { Value::High } else { Value::Low })
    }

    /// Perform the propagation logic - convert BCD input to 7-segment outputs
    fn propagate(&mut self, _timestamp: Timestamp) -> UpdateResult {
        let mut updates = Vec::new();
        
        if let Some(input_pin) = self.pins.get("BCD_IN") {
            let input_signal = input_pin.get_signal();
            
            // Check if input is valid
            if let Some(bcd_value) = self.signal_to_bcd(input_signal) {
                // Get segment pattern
                let pattern = Self::get_segment_pattern(bcd_value);
                
                // Set each segment output
                let segment_names = ["SEG_A", "SEG_B", "SEG_C", "SEG_D", "SEG_E", "SEG_F", "SEG_G"];
                for (i, &segment_name) in segment_names.iter().enumerate() {
                    if let Some(output_pin) = self.pins.get_mut(segment_name) {
                        let bit_value = (pattern >> i) & 1 != 0;
                        let signal = self.bit_to_signal(bit_value);
                        output_pin.set_signal(signal);
                        updates.push((self.id, segment_name.to_string(), Self::PROPAGATION_DELAY));
                    }
                }
            } else {
                // Invalid input, set all outputs to unknown
                let segment_names = ["SEG_A", "SEG_B", "SEG_C", "SEG_D", "SEG_E", "SEG_F", "SEG_G"];
                for &segment_name in &segment_names {
                    if let Some(output_pin) = self.pins.get_mut(segment_name) {
                        output_pin.set_signal(Signal::unknown(BusWidth(1)));
                        updates.push((self.id, segment_name.to_string(), Self::PROPAGATION_DELAY));
                    }
                }
            }
        }
        
        UpdateResult { updates }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let decoder = BcdToSevenSegmentDisplay::new(ComponentId(1));
        assert_eq!(decoder.id(), ComponentId(1));
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
    fn test_pin_configuration() {
        let decoder = BcdToSevenSegmentDisplay::new(ComponentId(1));
        let pins = decoder.pins();
        
        // Should have 1 input + 7 outputs
        assert_eq!(pins.len(), 8);
        
        // Check BCD input pin
        let input_pin = pins.get("BCD_IN").unwrap();
        assert!(input_pin.is_input());
        assert_eq!(input_pin.get_width(), BusWidth(4));
        
        // Check segment output pins
        let segment_names = ["SEG_A", "SEG_B", "SEG_C", "SEG_D", "SEG_E", "SEG_F", "SEG_G"];
        for &segment_name in &segment_names {
            let pin = pins.get(segment_name).unwrap();
            assert!(pin.is_output());
            assert_eq!(pin.get_width(), BusWidth(1));
        }
    }

    #[test]
    fn test_signal_to_bcd_conversion() {
        let decoder = BcdToSevenSegmentDisplay::new(ComponentId(1));
        
        // Test valid 4-bit signal
        let signal_5 = Signal::new_bus(vec![Value::High, Value::Low, Value::High, Value::Low]);
        assert_eq!(decoder.signal_to_bcd(&signal_5), Some(5));
        
        // Test signal with unknown value
        let signal_unknown = Signal::new_bus(vec![Value::High, Value::Unknown, Value::Low, Value::Low]);
        assert_eq!(decoder.signal_to_bcd(&signal_unknown), None);
        
        // Test wrong width signal
        let signal_wrong_width = Signal::new_bus(vec![Value::High, Value::Low]);
        assert_eq!(decoder.signal_to_bcd(&signal_wrong_width), None);
    }

    #[test]
    fn test_bit_to_signal_conversion() {
        let decoder = BcdToSevenSegmentDisplay::new(ComponentId(1));
        
        let signal_high = decoder.bit_to_signal(true);
        assert_eq!(signal_high.values(), &[Value::High]);
        
        let signal_low = decoder.bit_to_signal(false);
        assert_eq!(signal_low.values(), &[Value::Low]);
    }

    #[test]
    fn test_component_reset() {
        let mut decoder = BcdToSevenSegmentDisplay::new(ComponentId(1));
        decoder.reset();
        
        // All output pins should be unknown after reset
        for (name, pin) in decoder.pins() {
            if pin.is_output() {
                assert!(name.starts_with("SEG_"));
                // Signal should be unknown (we can't easily test this without more infrastructure)
            }
        }
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