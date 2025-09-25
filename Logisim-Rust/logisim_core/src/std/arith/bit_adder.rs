/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! BitAdder Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.BitAdder`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Single-bit full adder component
/// 
/// Implements a full adder that takes three single-bit inputs:
/// - A: First input bit
/// - B: Second input bit
/// - CarryIn: Carry input from previous stage
/// 
/// Produces two single-bit outputs:
/// - Sum: XOR of all three inputs
/// - CarryOut: Majority function of the three inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitAdder {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl BitAdder {
    /// Create a new bit adder
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        
        // All pins are single-bit for a bit adder
        let single_bit = BusWidth(1);
        
        // Input pins
        pins.insert("A".to_string(), Pin::new_input("A", single_bit));
        pins.insert("B".to_string(), Pin::new_input("B", single_bit));
        pins.insert("CarryIn".to_string(), Pin::new_input("CarryIn", single_bit));
        
        // Output pins
        pins.insert("Sum".to_string(), Pin::new_output("Sum", single_bit));
        pins.insert("CarryOut".to_string(), Pin::new_output("CarryOut", single_bit));
        
        BitAdder { id, pins }
    }
    
    /// Perform full adder logic
    /// 
    /// Truth table:
    /// A | B | Cin | Sum | Cout
    /// 0 | 0 |  0  |  0  |  0
    /// 0 | 0 |  1  |  1  |  0
    /// 0 | 1 |  0  |  1  |  0
    /// 0 | 1 |  1  |  0  |  1
    /// 1 | 0 |  0  |  1  |  0
    /// 1 | 0 |  1  |  0  |  1
    /// 1 | 1 |  0  |  0  |  1
    /// 1 | 1 |  1  |  1  |  1
    fn compute_outputs(&self, a: Value, b: Value, carry_in: Value) -> (Value, Value) {
        // Handle error and unknown cases
        if !a.is_fully_defined() || !b.is_fully_defined() || !carry_in.is_fully_defined() {
            let error_val = if matches!(a, Value::Error) || matches!(b, Value::Error) || matches!(carry_in, Value::Error) {
                Value::Error
            } else {
                Value::Unknown
            };
            return (error_val, error_val);
        }
        
        // Sum = A XOR B XOR Carry_In
        let sum = a.xor(b).xor(carry_in);
        
        // Carry_Out = (A AND B) OR (A AND Carry_In) OR (B AND Carry_In)
        // This is the majority function - true when at least 2 of the 3 inputs are true
        let carry_out = (a.and(b)).or(a.and(carry_in)).or(b.and(carry_in));
        
        (sum, carry_out)
    }
}

impl Component for BitAdder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "BitAdder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        // Get input values
        let a = self.pins.get("A").map(|p| *p.get_signal().value()).unwrap_or(Value::Unknown);
        let b = self.pins.get("B").map(|p| *p.get_signal().value()).unwrap_or(Value::Unknown);
        let carry_in = self.pins.get("CarryIn").map(|p| *p.get_signal().value()).unwrap_or(Value::Low);
        
        // Compute outputs
        let (sum, carry_out) = self.compute_outputs(a, b, carry_in);
        
        // Update output pins
        let mut changed = false;
        
        if let Some(pin) = self.pins.get_mut("Sum") {
            if *pin.get_signal().value() != sum {
                let _ = pin.set_signal(Signal::new(sum, current_time));
                changed = true;
            }
        }
        
        if let Some(pin) = self.pins.get_mut("CarryOut") {
            if *pin.get_signal().value() != carry_out {
                let _ = pin.set_signal(Signal::new(carry_out, current_time));
                changed = true;
            }
        }
        
        if changed {
            UpdateResult::changed()
        } else {
            UpdateResult::no_change()
        }
    }

    fn reset(&mut self) {
        // Reset all pins to their default states
        for pin in self.pins.values_mut() {
            pin.reset();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_adder_creation() {
        let bit_adder = BitAdder::new(ComponentId(1));
        assert_eq!(bit_adder.id(), ComponentId(1));
        assert_eq!(bit_adder.name(), "BitAdder");
        assert_eq!(bit_adder.pins().len(), 5);
        
        // All pins should be single-bit
        for pin in bit_adder.pins().values() {
            assert_eq!(pin.width(), BusWidth(1));
        }
    }

    #[test]
    fn test_full_adder_truth_table() {
        let bit_adder = BitAdder::new(ComponentId(1));
        
        // Test all 8 combinations of the truth table
        let test_cases = [
            // A, B, Cin, expected Sum, expected Carry_Out
            (Value::Low, Value::Low, Value::Low, Value::Low, Value::Low),
            (Value::Low, Value::Low, Value::High, Value::High, Value::Low),
            (Value::Low, Value::High, Value::Low, Value::High, Value::Low),
            (Value::Low, Value::High, Value::High, Value::Low, Value::High),
            (Value::High, Value::Low, Value::Low, Value::High, Value::Low),
            (Value::High, Value::Low, Value::High, Value::Low, Value::High),
            (Value::High, Value::High, Value::Low, Value::Low, Value::High),
            (Value::High, Value::High, Value::High, Value::High, Value::High),
        ];
        
        for (a, b, cin, expected_sum, expected_carry) in test_cases {
            let (sum, carry_out) = bit_adder.compute_outputs(a, b, cin);
            assert_eq!(sum, expected_sum, "Sum failed for A={:?}, B={:?}, Cin={:?}", a, b, cin);
            assert_eq!(carry_out, expected_carry, "Carry_Out failed for A={:?}, B={:?}, Cin={:?}", a, b, cin);
        }
    }

    #[test]
    fn test_component_reset() {
        let mut bit_adder = BitAdder::new(ComponentId(1));
        bit_adder.reset();
        // Should not panic and should reset all pins
        assert_eq!(bit_adder.pins().len(), 5);
    }
}