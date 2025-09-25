/*
 * Logisim-evolution - digital logic design tool and simulator  
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Display decoder utilities for TTL components
//! 
//! This is the Rust port of DisplayDecoder.java, providing utilities for
//! seven-segment display decoders like the TTL 7447.

use crate::{
    data::Value,
    instance::InstanceState,
};

/// Utilities for seven-segment display decoder TTL components
/// 
/// This provides the logic for components like the 7447 BCD-to-seven-segment decoder.
pub struct DisplayDecoder;

impl DisplayDecoder {
    /// Seven-segment display patterns for digits 0-9
    /// 
    /// Each pattern represents which segments should be active for each digit.
    /// Segments are ordered as: a, b, c, d, e, f, g
    const SEGMENT_PATTERNS: [[bool; 7]; 16] = [
        // 0: a, b, c, d, e, f, -g
        [true, true, true, true, true, true, false],
        // 1: -a, b, c, -d, -e, -f, -g
        [false, true, true, false, false, false, false],
        // 2: a, b, -c, d, e, -f, g
        [true, true, false, true, true, false, true],
        // 3: a, b, c, d, -e, -f, g
        [true, true, true, true, false, false, true],
        // 4: -a, b, c, -d, -e, f, g
        [false, true, true, false, false, true, true],
        // 5: a, -b, c, d, -e, f, g
        [true, false, true, true, false, true, true],
        // 6: a, -b, c, d, e, f, g
        [true, false, true, true, true, true, true],
        // 7: a, b, c, -d, -e, -f, -g
        [true, true, true, false, false, false, false],
        // 8: a, b, c, d, e, f, g
        [true, true, true, true, true, true, true],
        // 9: a, b, c, d, -e, f, g
        [true, true, true, true, false, true, true],
        // A (10): a, b, c, -d, e, f, g
        [true, true, true, false, true, true, true],
        // b (11): -a, -b, c, d, e, f, g
        [false, false, true, true, true, true, true],
        // C (12): a, -b, -c, d, e, f, -g
        [true, false, false, true, true, true, false],
        // d (13): -a, b, c, d, e, -f, g
        [false, true, true, true, true, false, true],
        // E (14): a, -b, -c, d, e, f, g
        [true, false, false, true, true, true, true],
        // F (15): a, -b, -c, -d, e, f, g
        [true, false, false, false, true, true, true],
    ];
    
    /// Get the decimal value from BCD input ports
    /// 
    /// # Arguments
    /// * `state` - Instance state containing port values
    /// * `active_low` - Whether the inputs are active low
    /// * `offset` - Port offset for input ports
    /// * `port_a` - Port index for A input (LSB)
    /// * `port_b` - Port index for B input
    /// * `port_c` - Port index for C input
    /// * `port_d` - Port index for D input (MSB)
    /// 
    /// # Returns
    /// The decimal value (0-15) represented by the BCD inputs
    pub fn get_decimal_value(
        state: &InstanceState,
        active_low: bool,
        offset: usize,
        port_a: usize,
        port_b: usize,
        port_c: usize,
        port_d: usize,
    ) -> u8 {
        let a = state.get_port_value(port_a + offset);
        let b = state.get_port_value(port_b + offset);
        let c = state.get_port_value(port_c + offset);
        let d = state.get_port_value(port_d + offset);
        
        let mut value = 0u8;
        
        if (a == Value::TRUE) != active_low {
            value |= 1;
        }
        if (b == Value::TRUE) != active_low {
            value |= 2;
        }
        if (c == Value::TRUE) != active_low {
            value |= 4;
        }
        if (d == Value::TRUE) != active_low {
            value |= 8;
        }
        
        value
    }
    
    /// Compute seven-segment display decoder outputs
    /// 
    /// This method implements the logic for BCD-to-seven-segment decoders
    /// like the TTL 7447, including lamp test and blanking functionality.
    /// 
    /// # Arguments
    /// * `state` - Mutable instance state for setting outputs
    /// * `decimal_value` - BCD input value (0-15)
    /// * `port_qa` - Port index for segment a output
    /// * `port_qb` - Port index for segment b output
    /// * `port_qc` - Port index for segment c output
    /// * `port_qd` - Port index for segment d output
    /// * `port_qe` - Port index for segment e output
    /// * `port_qf` - Port index for segment f output
    /// * `port_qg` - Port index for segment g output
    /// * `port_lt` - Port index for lamp test input
    /// * `port_bi` - Port index for blanking input
    /// * `port_rbi` - Port index for ripple blanking input
    pub fn compute_display_decoder_outputs(
        state: &mut InstanceState,
        decimal_value: u8,
        port_qa: usize,
        port_qb: usize,
        port_qc: usize,
        port_qd: usize,
        port_qe: usize,
        port_qf: usize,
        port_qg: usize,
        port_lt: usize,
        port_bi: usize,
        port_rbi: usize,
    ) {
        let lamp_test = state.get_port_value(port_lt) == Value::FALSE;
        let blanking_input = state.get_port_value(port_bi) == Value::FALSE;
        let ripple_blanking = state.get_port_value(port_rbi) == Value::FALSE;
        
        // Determine if display should be blanked
        let blank_display = blanking_input || (ripple_blanking && decimal_value == 0);
        
        let segments = if lamp_test {
            // Lamp test: all segments ON
            [true; 7]
        } else if blank_display {
            // Blanked: all segments OFF
            [false; 7]
        } else if decimal_value < 16 {
            // Normal operation: use pattern lookup
            Self::SEGMENT_PATTERNS[decimal_value as usize]
        } else {
            // Invalid BCD code: all segments OFF
            [false; 7]
        };
        
        // Set output ports (active low for 7447)
        state.set_port(port_qa, if segments[0] { Value::FALSE } else { Value::TRUE }, 1);
        state.set_port(port_qb, if segments[1] { Value::FALSE } else { Value::TRUE }, 1);
        state.set_port(port_qc, if segments[2] { Value::FALSE } else { Value::TRUE }, 1);
        state.set_port(port_qd, if segments[3] { Value::FALSE } else { Value::TRUE }, 1);
        state.set_port(port_qe, if segments[4] { Value::FALSE } else { Value::TRUE }, 1);
        state.set_port(port_qf, if segments[5] { Value::FALSE } else { Value::TRUE }, 1);
        state.set_port(port_qg, if segments[6] { Value::FALSE } else { Value::TRUE }, 1);
    }
    
    /// Check if a BCD value is valid (0-9)
    pub fn is_valid_bcd(value: u8) -> bool {
        value <= 9
    }
    
    /// Get segment pattern for a given decimal value
    pub fn get_segment_pattern(value: u8) -> Option<[bool; 7]> {
        if (value as usize) < Self::SEGMENT_PATTERNS.len() {
            Some(Self::SEGMENT_PATTERNS[value as usize])
        } else {
            None
        }
    }
}