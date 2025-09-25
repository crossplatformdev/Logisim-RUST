/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! NOT Gate Implementation
//!
//! Rust port of `com.cburch.logisim.std.gates.NotGate`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NOT Gate (Inverter) implementation
///
/// Performs logical NOT operation on its input. The output is the inverse
/// of the input value. Supports configurable bit width.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl NotGate {
    /// Create a new NOT gate
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        NotGate { id, pins }
    }

    /// Create a new NOT gate with configurable bit width
    pub fn new_with_width(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", width));
        pins.insert("Y".to_string(), Pin::new_output("Y", width));

        NotGate { id, pins }
    }
}

impl Component for NotGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "NOT"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.not();
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

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
        1 // 1 time unit for NOT gate
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_gate_creation() {
        let gate = NotGate::new(ComponentId(1));
        assert_eq!(gate.id(), ComponentId(1));
        assert_eq!(gate.name(), "NOT");
        assert_eq!(gate.pins().len(), 2); // A, Y
    }

    #[test]
    fn test_not_gate_truth_table() {
        let mut gate = NotGate::new(ComponentId(1));

        // Test all combinations
        let test_cases = [
            (Value::Low, Value::High),
            (Value::High, Value::Low),
            (Value::Unknown, Value::Unknown),
        ];

        for (input, expected) in test_cases {
            gate.get_pin_mut("A")
                .unwrap()
                .set_signal(Signal::new_single(input))
                .unwrap();

            let result = gate.update(Timestamp(0));
            let outputs = result.get_outputs();

            if let Some(output_signal) = outputs.get("Y") {
                let output_value = output_signal.as_single().unwrap();
                assert_eq!(
                    output_value, expected,
                    "NOT({:?}) should be {:?}, got {:?}",
                    input, expected, output_value
                );
            } else {
                panic!("No output signal found");
            }
        }
    }

    #[test]
    fn test_not_gate_with_width() {
        let gate = NotGate::new_with_width(ComponentId(1), BusWidth(4));
        assert_eq!(gate.pins().get("A").unwrap().width, BusWidth(4));
        assert_eq!(gate.pins().get("Y").unwrap().width, BusWidth(4));
    }
}
