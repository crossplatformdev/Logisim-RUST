/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! TTL 74x00: Quad 2-input NAND gate
//! 
//! This is the Rust port of Ttl7400.java, implementing a quad 2-input NAND gate
//! TTL integrated circuit.

use crate::{
    comp::{Component, ComponentId, Pin, UpdateResult},
    data::{Bounds, Direction, Location},
    instance::{Instance, InstancePainter, InstanceState},
    signal::Value,
};
use super::{
    abstract_ttl_gate::{AbstractTtlGate, TtlGateImpl},
    drawgates::Drawgates,
};

/// TTL 7400: Quad 2-input NAND gate
/// 
/// The 7400 contains four independent 2-input NAND gates in a 14-pin DIP package.
/// Each gate performs the logical NAND operation: output = NOT(A AND B).
/// 
/// Pin Configuration (14-pin DIP):
/// ```
/// 1A  1  14  VCC
/// 1B  2  13  4B  
/// 1Y  3  12  4A
/// 2A  4  11  4Y
/// 2B  5  10  3B
/// 2Y  6   9  3A
/// GND 7   8  3Y
/// ```
#[derive(Debug, Clone)]
pub struct Ttl7400 {
    impl_data: TtlGateImpl,
    pins: std::collections::HashMap<String, Pin>,
}

impl Ttl7400 {
    /// Unique identifier for the TTL 7400 component
    /// 
    /// This MUST match the Java implementation to maintain project file compatibility.
    pub const ID: &'static str = "7400";
    
    /// Pin count for 14-pin DIP package
    const PIN_COUNT: u8 = 14;
    
    /// Output pin numbers (pins 3, 6, 8, 11)
    const OUTPUT_PINS: [u8; 4] = [3, 6, 8, 11];
    
    /// Port names for the 7400 NAND gates
    const PORT_NAMES: [&'static str; 12] = [
        "1A", "1B", "1Y", // Gate 1
        "2A", "2B", "2Y", // Gate 2  
        "3Y", "3A", "3B", // Gate 3
        "4Y", "4A", "4B", // Gate 4
    ];
    
    /// Create a new TTL 7400 component
    pub fn new() -> Self {
        let mut pins = std::collections::HashMap::new();
        
        // Initialize pins based on TTL 7400 configuration
        // Add input pins
        pins.insert("1A".to_string(), Pin::new_input("1A"));
        pins.insert("1B".to_string(), Pin::new_input("1B"));
        pins.insert("2A".to_string(), Pin::new_input("2A"));
        pins.insert("2B".to_string(), Pin::new_input("2B"));
        pins.insert("3A".to_string(), Pin::new_input("3A"));
        pins.insert("3B".to_string(), Pin::new_input("3B"));
        pins.insert("4A".to_string(), Pin::new_input("4A"));
        pins.insert("4B".to_string(), Pin::new_input("4B"));
        
        // Add output pins
        pins.insert("1Y".to_string(), Pin::new_output("1Y"));
        pins.insert("2Y".to_string(), Pin::new_output("2Y"));
        pins.insert("3Y".to_string(), Pin::new_output("3Y"));
        pins.insert("4Y".to_string(), Pin::new_output("4Y"));
        
        Self {
            impl_data: TtlGateImpl::new(
                Self::ID,
                Self::PIN_COUNT,
                Self::OUTPUT_PINS.to_vec(),
                Self::PORT_NAMES.to_vec(),
            ),
            pins,
        }
    }
    
    /// Create a new TTL 7400 component with custom name
    pub fn new_with_name(name: &'static str) -> Self {
        let mut instance = Self::new();
        instance.impl_data.id = name;
        instance
    }
}

impl Default for Ttl7400 {
    fn default() -> Self {
        Self::new()
    }
}

impl AbstractTtlGate for Ttl7400 {
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
    
    fn paint_internal(&self, painter: &InstancePainter, x: i32, y: i32, height: i32, up_oriented: bool) {
        let port_width = 19;
        let port_height = 15;
        let y_output = y + if up_oriented { 20 } else { 40 };
        
        // Draw NAND gate symbol
        Drawgates::paint_and(painter, x + 40, y_output, port_width - 4, port_height, true);
        
        // Draw output line
        Drawgates::paint_output_gate(painter, x + 50, y, x + 44, y_output, up_oriented, height);
        
        // Draw input lines
        Drawgates::paint_double_input_gate(
            painter,
            x + 30,
            y,
            x + 44 - port_width,
            y_output,
            port_height,
            up_oriented,
            false,
            height,
        );
    }
    
    fn propagate_ttl(&self, state: &mut dyn InstanceState) {
        // Gate 1: pins 1,2 -> 3 (ports 0,1 -> 2)
        let gate1_output = !(state.get_port_value(0) & state.get_port_value(1));
        state.set_port(2, gate1_output, 1);
        
        // Gate 2: pins 4,5 -> 6 (ports 3,4 -> 5)  
        let gate2_output = !(state.get_port_value(3) & state.get_port_value(4));
        state.set_port(5, gate2_output, 1);
        
        // Gate 3: pins 9,10 -> 8 (ports 8,7 -> 6)
        let gate3_output = !(state.get_port_value(8) & state.get_port_value(7));
        state.set_port(6, gate3_output, 1);
        
        // Gate 4: pins 12,13 -> 11 (ports 10,11 -> 9)
        let gate4_output = !(state.get_port_value(10) & state.get_port_value(11));
        state.set_port(9, gate4_output, 1);
    }
}

impl Component for Ttl7400 {
    fn id(&self) -> ComponentId {
        ComponentId::new(Self::ID)
    }
    
    fn name(&self) -> &str {
        "7400"
    }
    
    fn pins(&self) -> &std::collections::HashMap<String, Pin> {
        &self.pins
    }
    
    fn pins_mut(&mut self) -> &mut std::collections::HashMap<String, Pin> {
        &mut self.pins
    }
    
    fn update(&mut self, current_time: crate::signal::Timestamp) -> UpdateResult {
        // TTL components typically don't need time-based updates
        UpdateResult::Continue
    }
    
    fn reset(&mut self) {
        // Reset TTL 7400 to initial state
        for pin in self.pins.values_mut() {
            pin.set_signal(crate::signal::Signal::new_single(crate::signal::Value::Unknown));
        }
    }
}

impl Ttl7400 {
    /// Convert pin number to port index
    /// 
    /// This maps physical pin numbers to logical port indices,
    /// accounting for VCC/GND pins and unused pins.
    fn pin_to_port_index(&self, pin: u8) -> usize {
        // This is a simplified mapping - the actual implementation would
        // need to handle the complex pin-to-port mapping logic
        match pin {
            3 => 2,  // 1Y
            6 => 5,  // 2Y
            8 => 6,  // 3Y  
            11 => 9, // 4Y
            _ => 0,
        }
    }
}