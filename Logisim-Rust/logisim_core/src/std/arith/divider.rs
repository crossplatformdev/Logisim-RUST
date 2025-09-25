/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Divider Implementation
//!
//! Rust port of `com.cburch.logisim.std.arith.Divider`

use crate::comp::{Component, ComponentId, Pin, Propagator, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divider {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: BusWidth,
}

impl Divider {
    pub fn new(id: ComponentId) -> Self {
        Self::new_with_width(id, BusWidth(8))
    }
    
    pub fn new_with_width(id: ComponentId, bit_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Dividend".to_string(), Pin::new_input("Dividend", bit_width));
        pins.insert("Divisor".to_string(), Pin::new_input("Divisor", bit_width));
        pins.insert("Quotient".to_string(), Pin::new_output("Quotient", bit_width));
        pins.insert("Remainder".to_string(), Pin::new_output("Remainder", bit_width));
        
        Divider { id, pins, bit_width }
    }
    
    pub fn bit_width(&self) -> BusWidth {
        self.bit_width
    }
}

impl Component for Divider {
    fn id(&self) -> ComponentId { self.id }
    fn name(&self) -> &str { "Divider" }
    fn pins(&self) -> &HashMap<String, Pin> { &self.pins }
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> { &mut self.pins }
    
    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        let dividend = self.pins.get("Dividend").map(|p| p.signal().value()).unwrap_or(Value::Unknown);
        let divisor = self.pins.get("Divisor").map(|p| p.signal().value()).unwrap_or(Value::Unknown);
        
        let (quotient, remainder) = if dividend.is_fully_defined() && divisor.is_fully_defined() {
            let div_val = divisor.to_long_value();
            if div_val == 0 {
                (Value::Error, Value::Error)
            } else {
                let div_result = dividend.to_long_value() / div_val;
                let rem_result = dividend.to_long_value() % div_val;
                (
                    Value::from_long(div_result, self.bit_width),
                    Value::from_long(rem_result, self.bit_width)
                )
            }
        } else {
            (Value::Unknown, Value::Unknown)
        };
        
        let mut changed = false;
        if let Some(pin) = self.pins.get_mut("Quotient") {
            if pin.signal().value() != quotient {
                pin.set_signal(Signal::new(quotient, current_time));
                changed = true;
            }
        }
        if let Some(pin) = self.pins.get_mut("Remainder") {
            if pin.signal().value() != remainder {
                pin.set_signal(Signal::new(remainder, current_time));
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

impl Propagator for Divider {
    fn propagate(&mut self, current_time: Timestamp) {
        let delay = self.bit_width.0 * 2;
        self.update(current_time + delay as u64);
    }
}