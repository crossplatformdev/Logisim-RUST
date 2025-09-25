/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * This is free software released under GNU GPLv3 license
 */

//! TTL 74x04: Hex inverter
//! 
//! This is the Rust port of Ttl7404.java, implementing a hex inverter
//! TTL integrated circuit.

use crate::{
    comp::{Component, ComponentId},
    instance::{InstancePainter, InstanceState},
};
use super::abstract_ttl_gate::{AbstractTtlGate, TtlGateImpl};

/// TTL 7404: Hex inverter
/// 
/// The 7404 contains six independent inverters in a 14-pin DIP package.
/// Each inverter performs the logical NOT operation: output = NOT(input).
#[derive(Debug, Clone)]
pub struct Ttl7404 {
    impl_data: TtlGateImpl,
}

impl Ttl7404 {
    pub const ID: &'static str = "7404";
    const PIN_COUNT: u8 = 14;
    const OUTPUT_PINS: [u8; 6] = [2, 4, 6, 8, 10, 12];
    
    pub fn new() -> Self {
        Self {
            impl_data: TtlGateImpl::new(
                Self::ID,
                Self::PIN_COUNT,
                Self::OUTPUT_PINS.to_vec(),
                vec!["1A", "1Y", "2A", "2Y", "3A", "3Y", "4Y", "4A", "5Y", "5A", "6Y", "6A"],
            ),
        }
    }
}

impl Default for Ttl7404 {
    fn default() -> Self {
        Self::new()
    }
}

impl AbstractTtlGate for Ttl7404 {
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
        // TODO: Implement inverter gate drawing
    }
    
    fn propagate_ttl(&self, state: &mut InstanceState) {
        // TODO: Implement inverter logic
        // Six inverters: pins 1->2, 3->4, 5->6, 9->8, 11->10, 13->12
    }
}

impl Component for Ttl7404 {
    fn get_id(&self) -> ComponentId {
        ComponentId::new(Self::ID)
    }
    
    fn get_display_name(&self) -> &str {
        "7404"
    }
    
    fn get_description(&self) -> &str {
        "TTL 74x04: Hex inverter"
    }
    
    fn create_instance(&self) -> Box<dyn crate::instance::Instance> {
        todo!("Create TTL 7404 instance")
    }
    
    fn get_bounds(&self, _instance: &dyn crate::instance::Instance) -> crate::data::Bounds {
        crate::data::Bounds::new(0, 0, 120, 60)
    }
    
    fn propagate(&self, state: &mut InstanceState) {
        self.propagate_ttl(state);
    }
}