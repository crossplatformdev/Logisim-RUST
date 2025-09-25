/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! XNOR Gate Implementation
//!
//! Rust port of `com.cburch.logisim.std.gates.XnorGate`

use crate::component::{Component, ComponentId, Pin, Propagator, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// XNOR Gate implementation
///
/// Performs logical XNOR operation on its inputs. The output is high when
/// an even number of inputs are high. Equivalent to NOT(XOR(inputs)).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XnorGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl XnorGate {
    /// Create a new 2-input XNOR gate
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        XnorGate { id, pins }
    }

    /// Create a new XNOR gate with configurable number of inputs
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

        XnorGate { id, pins }
    }
}

impl Component for XnorGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "XNOR"
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

        // Compute XNOR of all inputs (even parity / NOT(XOR))
        let mut high_count = 0;
        let mut has_unknown = false;

        for &input in &inputs {
            match input {
                Value::High => high_count += 1,
                Value::Unknown | Value::Error => has_unknown = true,
                Value::Low => {} // No effect on XOR
            }
        }

        let output = if has_unknown {
            Value::Unknown
        } else if high_count % 2 == 0 {
            Value::High // Even parity for XNOR
        } else {
            Value::Low
        };

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
        3 // 3 time units for XNOR gate
    }
}

impl Propagator for XnorGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xnor_gate_creation() {
        let gate = XnorGate::new(ComponentId(1));
        assert_eq!(gate.id(), ComponentId(1));
        assert_eq!(gate.name(), "XNOR");
        assert_eq!(gate.pins().len(), 3); // A, B, Y
    }

    #[test]
    fn test_xnor_gate_truth_table() {
        let mut gate = XnorGate::new(ComponentId(1));

        // Test all combinations
        let test_cases = [
            (Value::Low, Value::Low, Value::High),   // 0 highs = even
            (Value::Low, Value::High, Value::Low),   // 1 high = odd
            (Value::High, Value::Low, Value::Low),   // 1 high = odd
            (Value::High, Value::High, Value::High), // 2 highs = even
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
                    "XNOR({:?}, {:?}) should be {:?}, got {:?}",
                    a, b, expected, output_value
                );
            } else {
                panic!("No output signal found");
            }
        }
    }

    #[test]
    fn test_xnor_gate_multi_input() {
        let mut gate = XnorGate::new_with_inputs(ComponentId(1), 3);

        // Test 3-input XNOR gate - even number of highs should give high
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
            .set_signal(Signal::new_single(Value::Low))
            .unwrap();

        let result = gate.update(Timestamp(0));
        let outputs = result.get_outputs();

        if let Some(output_signal) = outputs.get("Y") {
            let output_value = output_signal.as_single().unwrap();
            assert_eq!(output_value, Value::High); // 2 highs = even
        }

        // Test with odd number of highs
        gate.get_pin_mut("I2")
            .unwrap()
            .set_signal(Signal::new_single(Value::High))
            .unwrap();
        let result = gate.update(Timestamp(0));
        let outputs = result.get_outputs();

        if let Some(output_signal) = outputs.get("Y") {
            let output_value = output_signal.as_single().unwrap();
            assert_eq!(output_value, Value::Low); // 3 highs = odd
        }
    }
}
