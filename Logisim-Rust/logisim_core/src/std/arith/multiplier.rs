/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Multiplier Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Multiplier`

use crate::component::{Component, ComponentId, Pin, Propagator, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multi-bit Multiplier component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiplier {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
}

impl Multiplier {
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, BusWidth(8))
    }
    
    pub fn new_with_width(id: ComponentId, bit_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", bit_width));
        pins.insert("B".to_string(), Pin::new_input("B", bit_width));
        pins.insert("Product".to_string(), Pin::new_output("Product", BusWidth(bit_width.0 * 2)));
        
        Multiplier { id, pins, bit_width }
    }
    
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
}

impl Component for Multiplier {
    fn id(&self) -> ComponentId { self.id }
    fn name(&self) -> &str { "Multiplier" }
    fn pins(&self) -> &HashMap<String, Pin> { &self.pins }
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> { &mut self.pins }
    
    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        let value_a = self.pins.get("A").map(|p| p.signal().value()).unwrap_or(Value::Unknown);
        let value_b = self.pins.get("B").map(|p| p.signal().value()).unwrap_or(Value::Unknown);
        
        let product = if value_a.is_fully_defined() && value_b.is_fully_defined() {
            let result = value_a.to_long_value() * value_b.to_long_value();
            Value::from_long(result, BusWidth(self.bit_width.0 * 2))
        } else {
            Value::Unknown
        };
        
        let mut changed = false;
        if let Some(pin) = self.pins.get_mut("Product") {
            if pin.signal().value() != product {
                pin.set_signal(Signal::new(product, current_time));
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

impl Propagator for Multiplier {
    fn propagate(&mut self, current_time: Timestamp) {
        let delay = self.bit_width.0 * self.bit_width.0; // Rough approximation
        self.update(current_time + delay as u64);
    }
}