/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Tests for TTL components
//! 
//! This module contains unit tests for TTL integrated circuit implementations,
//! ensuring they match the behavior of the original Java components.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Value;
    use crate::instance::InstanceState;

    #[test]
    fn test_ttl7400_creation() {
        let ttl7400 = Ttl7400::new();
        assert_eq!(ttl7400.get_id(), "7400");
        assert_eq!(ttl7400.get_pin_count(), 14);
        assert_eq!(ttl7400.get_output_pins(), &[3, 6, 8, 11]);
    }

    #[test]
    fn test_ttl7400_nand_logic() {
        // This test would verify the NAND gate logic
        // TODO: Implement when InstanceState is available
        // let mut state = create_test_instance_state();
        // let ttl7400 = Ttl7400::new();
        // 
        // // Test all NAND truth table combinations
        // // A=0, B=0 -> Y=1
        // state.set_port_value(0, Value::FALSE);
        // state.set_port_value(1, Value::FALSE);
        // ttl7400.propagate_ttl(&mut state);
        // assert_eq!(state.get_port_value(2), Value::TRUE);
    }

    #[test]
    fn test_display_decoder_patterns() {
        use super::display_decoder::DisplayDecoder;
        
        // Test that digit 0 produces correct segment pattern
        let pattern = DisplayDecoder::get_segment_pattern(0).unwrap();
        assert_eq!(pattern, [true, true, true, true, true, true, false]);
        
        // Test that digit 8 produces all segments on
        let pattern = DisplayDecoder::get_segment_pattern(8).unwrap();
        assert_eq!(pattern, [true, true, true, true, true, true, true]);
    }

    #[test]
    fn test_ttl_library_creation() {
        let library = TtlLibrary::new();
        assert_eq!(library.get_id(), "TTL");
        assert!(!library.get_tools().is_empty());
    }

    #[test]
    fn test_bcd_value_validation() {
        use super::display_decoder::DisplayDecoder;
        
        assert!(DisplayDecoder::is_valid_bcd(0));
        assert!(DisplayDecoder::is_valid_bcd(9));
        assert!(!DisplayDecoder::is_valid_bcd(10));
        assert!(!DisplayDecoder::is_valid_bcd(15));
    }
}