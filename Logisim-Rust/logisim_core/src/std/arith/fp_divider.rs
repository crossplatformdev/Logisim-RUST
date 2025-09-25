/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! FpDivider Implementation (Placeholder)

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpDivider {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl FpDivider {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Dividend".to_string(), Pin::new_input("Dividend", BusWidth(32)));
        pins.insert("Divisor".to_string(), Pin::new_input("Divisor", BusWidth(32)));
        pins.insert("Quotient".to_string(), Pin::new_output("Quotient", BusWidth(32)));
        
        FpDivider { id, pins }
    }
}

impl Component for FpDivider {
    fn id(&self) -> ComponentId { self.id }
    fn name(&self) -> &str { "FpDivider" }
    fn pins(&self) -> &HashMap<String, Pin> { &self.pins }
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> { &mut self.pins }
    
    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        UpdateResult::NoChange // Placeholder
    }
    
    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.reset();
        }
    }
}

