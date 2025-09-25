/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Negator Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Negator`

use crate::component::{Component, ComponentId, Pin, Propagator, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Negator {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
}

impl Negator {
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, BusWidth(8))
    }
    
    pub fn new_with_width(id: ComponentId, bit_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Input".to_string(), Pin::new_input("Input", bit_width));
        pins.insert("Output".to_string(), Pin::new_output("Output", bit_width));
        
        Negator { id, pins, bit_width }
    }
    
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
}

impl Component for Negator {
    fn id(&self) -> ComponentId { self.id }
    fn name(&self) -> &str { "Negator" }
    fn pins(&self) -> &HashMap<String, Pin> { &self.pins }
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> { &mut self.pins }
    
    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        let input = self.pins.get("Input").map(|p| p.signal().value()).unwrap_or(Value::Unknown);
        
        let output = if input.is_fully_defined() {
            // Two's complement negation: ~input + 1
            let val = input.to_long_value();
            let negated = (!val).wrapping_add(1);
            Value::from_long(negated, self.bit_width)
        } else {
            Value::Unknown
        };
        
        let mut changed = false;
        if let Some(pin) = self.pins.get_mut("Output") {
            if pin.signal().value() != output {
                pin.set_signal(Signal::new(output, current_time));
                changed = true;
            }
        }
        
        if changed { UpdateResult::Changed } else { UpdateResult::NoChange }
    }
    
    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.reset();
        }
    }
}

impl Propagator for Negator {
    fn propagate(&mut self, current_time: Timestamp) {
        let delay = self.bit_width.0 + 2;
        self.update(current_time + delay as u64);
    }
}