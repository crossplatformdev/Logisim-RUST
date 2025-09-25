/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Comparator Implementation (Placeholder)

use crate::component::{Component, ComponentId, Pin, Propagator, UpdateResult};
use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparator {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl Comparator {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(8)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(8)));
        pins.insert("Equal".to_string(), Pin::new_output("Equal", BusWidth(1)));
        pins.insert("Less".to_string(), Pin::new_output("Less", BusWidth(1)));
        pins.insert("Greater".to_string(), Pin::new_output("Greater", BusWidth(1)));
        
        Comparator { id, pins }
    }
}

impl Component for Comparator {
    fn id(&self) -> ComponentId { self.id }
    fn name(&self) -> &str { "Comparator" }
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

impl Propagator for Comparator {
    fn propagate(&mut self, current_time: Timestamp) {
        self.update(current_time + 1);
    }
}