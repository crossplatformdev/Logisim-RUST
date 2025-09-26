/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! FpAdder Implementation (Placeholder)

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpAdder {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl FpAdder {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(32)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(32)));
        pins.insert("Sum".to_string(), Pin::new_output("Sum", BusWidth(32)));
        pins.insert("Error".to_string(), Pin::new_output("Error", BusWidth(1)));
        
        FpAdder { id, pins }
    }
}

impl Component for FpAdder {
    fn id(&self) -> ComponentId { self.id }
    fn name(&self) -> &str { "FpAdder" }
    fn pins(&self) -> &HashMap<String, Pin> { &self.pins }
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> { &mut self.pins }
    
    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        UpdateResult::new() // Placeholder
    }
    
    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

