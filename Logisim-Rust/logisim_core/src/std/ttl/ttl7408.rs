/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! TTL 74x08: Quad 2-input AND gate
//! 
//! This is the Rust port of Ttl7408.java, implementing a quad 2-input AND gate
//! TTL integrated circuit.

use crate::{
    component::{Component, ComponentId},
    instance::{InstancePainter, InstanceState},
};
use super::abstract_ttl_gate::{AbstractTtlGate, TtlGateImpl};

/// TTL 7408: Quad 2-input AND gate
/// 
/// The 7408 contains four independent 2-input AND gates in a 14-pin DIP package.
/// Each gate performs the logical AND operation: output = A AND B.
#[derive(Debug, Clone)]
pub struct Ttl7408 {
    impl_data: TtlGateImpl,
}

impl Ttl7408 {
    pub const ID: &'static str = "7408";
    const PIN_COUNT: u8 = 14;
    const OUTPUT_PINS: [u8; 4] = [3, 6, 8, 11];
    
    pub fn new() -> Self {
        Self {
            impl_data: TtlGateImpl::new(
                Self::ID,
                Self::PIN_COUNT,
                Self::OUTPUT_PINS.to_vec(),
                vec!["1A", "1B", "1Y", "2A", "2B", "2Y", "3Y", "3A", "3B", "4Y", "4A", "4B"],
            ),
        }
    }
}

impl Default for Ttl7408 {
    fn default() -> Self {
        Self::new()
    }
}

impl AbstractTtlGate for Ttl7408 {
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
        // TODO: Implement AND gate drawing
    }
    
    fn propagate_ttl(&self, state: &mut InstanceState) {
        // TODO: Implement AND gate logic
        // Gate 1: pins 1,2 -> 3 (AND)
        // Gate 2: pins 4,5 -> 6 (AND)
        // Gate 3: pins 9,10 -> 8 (AND)
        // Gate 4: pins 12,13 -> 11 (AND)
    }
}

impl Component for Ttl7408 {
    fn get_id(&self) -> ComponentId {
        ComponentId::new(Self::ID)
    }
    
    fn get_display_name(&self) -> &str {
        "7408"
    }
    
    fn get_description(&self) -> &str {
        "TTL 74x08: Quad 2-input AND gate"
    }
    
    fn create_instance(&self) -> Box<dyn crate::instance::Instance> {
        todo!("Create TTL 7408 instance")
    }
    
    fn get_bounds(&self, _instance: &dyn crate::instance::Instance) -> crate::data::Bounds {
        crate::data::Bounds::new(0, 0, 120, 60)
    }
    
    fn propagate(&self, state: &mut InstanceState) {
        self.propagate_ttl(state);
    }
}