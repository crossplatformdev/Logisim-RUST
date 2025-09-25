/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! TTL 74x02: Quad 2-input NOR gate
//! 
//! This is the Rust port of Ttl7402.java, implementing a quad 2-input NOR gate
//! TTL integrated circuit.

use crate::{
    comp::{Component, ComponentId},
    instance::{InstancePainter, InstanceState},
};
use super::abstract_ttl_gate::{AbstractTtlGate, TtlGateImpl};

/// TTL 7402: Quad 2-input NOR gate
/// 
/// The 7402 contains four independent 2-input NOR gates in a 14-pin DIP package.
/// Each gate performs the logical NOR operation: output = NOT(A OR B).
#[derive(Debug, Clone)]
pub struct Ttl7402 {
    impl_data: TtlGateImpl,
}

impl Ttl7402 {
    pub const ID: &'static str = "7402";
    const PIN_COUNT: u8 = 14;
    const OUTPUT_PINS: [u8; 4] = [1, 4, 10, 13];
    
    pub fn new() -> Self {
        Self {
            impl_data: TtlGateImpl::new(
                Self::ID,
                Self::PIN_COUNT,
                Self::OUTPUT_PINS.to_vec(),
                vec!["1Y", "1A", "1B", "2Y", "2A", "2B", "3A", "3B", "3Y", "4B", "4A", "4Y"],
            ),
        }
    }
}

impl Default for Ttl7402 {
    fn default() -> Self {
        Self::new()
    }
}

impl AbstractTtlGate for Ttl7402 {
    fn get_id(&self) -> &'static str {
        self.impl_data.id
    }
    
    fn get_pin_count(&self) -> u8 {
        self.impl_data.pin_count
    }
    
    fn get_output_pins(&self) -> &[u8] {
        &self.impl_data.output_pins
    }
    
    fn get_port_names(&self) -> &[&'static str] {
        &self.impl_data.port_names
    }
    
    fn paint_internal(&self, _painter: &InstancePainter, _x: i32, _y: i32, _height: i32, _up_oriented: bool) {
        // TODO: Implement NOR gate drawing
    }
    
    fn propagate_ttl(&self, state: &mut InstanceState) {
        // TODO: Implement NOR gate logic
        // Gate 1: pins 2,3 -> 1 (NOR)
        // Gate 2: pins 5,6 -> 4 (NOR)  
        // Gate 3: pins 8,9 -> 10 (NOR)
        // Gate 4: pins 11,12 -> 13 (NOR)
    }
}

impl Component for Ttl7402 {
    fn get_id(&self) -> ComponentId {
        ComponentId::new(Self::ID)
    }
    
    fn get_display_name(&self) -> &str {
        "7402"
    }
    
    fn get_description(&self) -> &str {
        "TTL 74x02: Quad 2-input NOR gate"
    }
    
    fn create_instance(&self) -> Box<dyn crate::instance::Instance> {
        todo!("Create TTL 7402 instance")
    }
    
    fn get_bounds(&self, _instance: &dyn crate::instance::Instance) -> crate::data::Bounds {
        crate::data::Bounds::new(0, 0, 120, 60)
    }
    
    fn propagate(&self, state: &mut InstanceState) {
        self.propagate_ttl(state);
    }
}