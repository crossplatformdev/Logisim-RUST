/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! FpNegator Implementation (Placeholder)

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpNegator {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl FpNegator {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Input".to_string(), Pin::new_input("Input", BusWidth(32)));
        pins.insert("Output".to_string(), Pin::new_output("Output", BusWidth(32)));
        
        FpNegator { id, pins }
    }
}

impl Component for FpNegator {
    fn id(&self) -> ComponentId { self.id }
    fn name(&self) -> &str { "FpNegator" }
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

