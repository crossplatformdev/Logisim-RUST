/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! NAND Gate Implementation
//!
//! Rust port of `com.cburch.logisim.std.gates.NandGate`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NAND Gate implementation
///
/// Performs logical NAND operation on its inputs. The output is low only when
/// all inputs are high. Equivalent to NOT(AND(inputs)).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NandGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl NandGate {
    /// Create a new 2-input NAND gate
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        NandGate { id, pins }
    }

    /// Create a new NAND gate with configurable number of inputs
    pub fn new_with_inputs(id: ComponentId, num_inputs: usize) -> Self {
        let mut pins = HashMap::new();

        // Add input pins
        for i in 0..num_inputs {
            let pin_name = if i == 0 {
                "A".to_string()
            } else if i == 1 {
                "B".to_string()
            } else {
                format!("I{}", i)
            };
            pins.insert(pin_name.clone(), Pin::new_input(&pin_name, BusWidth(1)));
        }

        // Add output pin
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        NandGate { id, pins }
    }
}

impl Component for NandGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "NAND"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Get all input values
        let mut inputs = Vec::new();
        for (name, pin) in &self.pins {
            if name != "Y" {
                // Skip output pin
                let value = pin.signal.as_single().unwrap_or(Value::Unknown);
                inputs.push(value);
            }
        }

        // Compute NAND of all inputs (NOT(AND(inputs)))
        let mut and_result = Value::High;
        for &input in &inputs {
            and_result = and_result.and(input);
            if and_result == Value::Low {
                break; // Short circuit - if any input is low, AND is low
            }
        }

        let output = and_result.not(); // Invert the AND result
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        // Update internal pin state
        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        2 // 2 time units for NAND gate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_gate_creation() {
        let gate = NandGate::new(ComponentId(1));
        assert_eq!(gate.id(), ComponentId(1));
        assert_eq!(gate.name(), "NAND");
        assert_eq!(gate.pins().len(), 3); // A, B, Y
    }

    #[test]
    fn test_nand_gate_truth_table() {
        let mut gate = NandGate::new(ComponentId(1));

        // Test all combinations
        let test_cases = [
            (Value::Low, Value::Low, Value::High),
            (Value::Low, Value::High, Value::High),
            (Value::High, Value::Low, Value::High),
            (Value::High, Value::High, Value::Low),
        ];

        for (a, b, expected) in test_cases {
            gate.get_pin_mut("A")
                .unwrap()
                .set_signal(Signal::new_single(a))
                .unwrap();
            gate.get_pin_mut("B")
                .unwrap()
                .set_signal(Signal::new_single(b))
                .unwrap();

            let result = gate.update(Timestamp(0));
            let outputs = result.get_outputs();

            if let Some(output_signal) = outputs.get("Y") {
                let output_value = output_signal.as_single().unwrap();
                assert_eq!(
                    output_value, expected,
                    "NAND({:?}, {:?}) should be {:?}, got {:?}",
                    a, b, expected, output_value
                );
            } else {
                panic!("No output signal found");
            }
        }
    }

    #[test]
    fn test_nand_gate_multi_input() {
        let mut gate = NandGate::new_with_inputs(ComponentId(1), 3);

        // Test 3-input NAND gate - all high should give low
        gate.get_pin_mut("A")
            .unwrap()
            .set_signal(Signal::new_single(Value::High))
            .unwrap();
        gate.get_pin_mut("B")
            .unwrap()
            .set_signal(Signal::new_single(Value::High))
            .unwrap();
        gate.get_pin_mut("I2")
            .unwrap()
            .set_signal(Signal::new_single(Value::High))
            .unwrap();

        let result = gate.update(Timestamp(0));
        let outputs = result.get_outputs();

        if let Some(output_signal) = outputs.get("Y") {
            let output_value = output_signal.as_single().unwrap();
            assert_eq!(output_value, Value::Low);
        }

        // Test with one input low - should give high
        gate.get_pin_mut("B")
            .unwrap()
            .set_signal(Signal::new_single(Value::Low))
            .unwrap();
        let result = gate.update(Timestamp(0));
        let outputs = result.get_outputs();

        if let Some(output_signal) = outputs.get("Y") {
            let output_value = output_signal.as_single().unwrap();
            assert_eq!(output_value, Value::High);
        }
    }
}
