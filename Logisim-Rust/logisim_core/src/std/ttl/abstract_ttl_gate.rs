/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Abstract base for TTL gate components
//! 
//! This is the Rust port of AbstractTtlGate.java, providing the common
//! functionality for all TTL integrated circuit components.

use crate::{
    comp::{Component, ComponentId, Pin},
    data::{AttributeSet, Bounds, Direction, Location},
    signal::Value,
    instance::{Instance, InstanceFactory, InstancePainter, InstanceState},
    signal::Signal,
};
use std::collections::HashSet;

/// Abstract base trait for TTL gate components
/// 
/// This trait provides the common functionality for all TTL ICs, including:
/// - Pin management and configuration
/// - VCC/GND power supply handling
/// - Internal structure drawing
/// - Port propagation logic
pub trait AbstractTtlGate: Component {
    /// Get the unique identifier for this TTL component
    fn get_id(&self) -> &'static str;
    
    /// Get the total number of pins (including VCC and GND)
    fn get_pin_count(&self) -> u8;
    
    /// Get the output pin numbers
    fn get_output_pins(&self) -> &[u8];
    
    /// Get the unused pin numbers (if any)
    fn get_unused_pins(&self) -> &[u8] {
        &[]
    }
    
    /// Get the port names for labeling
    fn get_port_names(&self) -> &[&'static str];
    
    /// Paint the internal structure of the TTL component
    /// 
    /// # Arguments
    /// * `painter` - The instance painter for drawing
    /// * `x` - X coordinate
    /// * `y` - Y coordinate  
    /// * `height` - Height of the component
    /// * `up_oriented` - Whether the component is oriented upward
    fn paint_internal(&self, painter: &InstancePainter, x: i32, y: i32, height: i32, up_oriented: bool);
    
    /// Propagate TTL-specific logic
    /// 
    /// This method implements the actual logic behavior of the TTL component.
    /// It should read input values and compute outputs according to the IC's specification.
    fn propagate_ttl(&self, state: &mut InstanceState);
}

/// Standard TTL gate implementation structure
/// 
/// This struct provides a concrete implementation of common TTL functionality
/// that can be used by specific TTL components.
#[derive(Debug, Clone)]
pub struct TtlGateImpl {
    pub id: &'static str,
    pub pin_count: u8,
    pub output_pins: Vec<u8>,
    pub unused_pins: Vec<u8>,
    pub port_names: Vec<&'static str>,
    pub height: i32,
    pub draw_gates_count: u8,
}

impl TtlGateImpl {
    /// Create a new TTL gate implementation
    pub fn new(
        id: &'static str,
        pin_count: u8,
        output_pins: Vec<u8>,
        port_names: Vec<&'static str>,
    ) -> Self {
        Self {
            id,
            pin_count,
            output_pins,
            unused_pins: Vec::new(),
            port_names,
            height: 60,
            draw_gates_count: 0,
        }
    }
    
    /// Create a new TTL gate implementation with unused pins
    pub fn new_with_unused(
        id: &'static str,
        pin_count: u8,
        output_pins: Vec<u8>,
        unused_pins: Vec<u8>,
        port_names: Vec<&'static str>,
    ) -> Self {
        Self {
            id,
            pin_count,
            output_pins,
            unused_pins,
            port_names,
            height: 60,
            draw_gates_count: 0,
        }
    }
    
    /// Check if VCC/GND power supply is properly connected
    pub fn check_power_supply(&self, state: &InstanceState, vcc_gnd_enabled: bool) -> bool {
        if !vcc_gnd_enabled {
            return true;
        }
        
        let unused_count = self.unused_pins.len();
        let vcc_pin_index = self.pin_count as usize - 2 - unused_count;
        let gnd_pin_index = self.pin_count as usize - 1 - unused_count;
        
        // Check VCC (should be TRUE) and GND (should be FALSE)
        state.get_port_value(vcc_pin_index) == Value::TRUE && 
        state.get_port_value(gnd_pin_index) == Value::FALSE
    }
    
    /// Update port configuration based on instance attributes
    pub fn update_ports(&self, instance: &mut Instance) {
        let facing = instance.get_attribute_value("facing")
            .unwrap_or(Direction::East);
        
        let mut ports = Vec::new();
        let mut port_index = 0;
        
        for pin in 1..=self.pin_count {
            let is_output = self.output_pins.contains(&pin);
            let is_unused = self.unused_pins.contains(&pin);
            
            if !is_unused && pin != (self.pin_count / 2) {
                let (dx, dy) = self.calculate_pin_position(pin, facing);
                ports.push(Pin::new(
                    port_index,
                    if is_output { "output" } else { "input" }.to_string(),
                    Location::new(dx, dy),
                ));
                port_index += 1;
            }
        }
        
        instance.set_ports(ports);
    }
    
    /// Calculate pin position based on pin number and facing direction
    fn calculate_pin_position(&self, pin: u8, facing: Direction) -> (i32, i32) {
        let width = 120; // Standard TTL width
        let height = self.height;
        
        let (mut dx, mut dy) = if pin <= self.pin_count / 2 {
            // Left side pins (1 to n/2)
            let pin_pos = (pin - 1) as i32;
            match facing {
                Direction::East => (pin_pos * 20 + 10, height - 30),
                Direction::West => (-10 - 20 * pin_pos, 30 - height),
                Direction::North => (width - 30, -10 - 20 * pin_pos),
                Direction::South => (30 - width, 10 + 20 * pin_pos),
            }
        } else {
            // Right side pins (n/2+1 to n)
            let pin_pos = (self.pin_count - pin) as i32;
            match facing {
                Direction::East => (width - 10 - 20 * pin_pos, 30),
                Direction::West => (10 + 20 * pin_pos, height - 30),
                Direction::North => (30, 10 + 20 * pin_pos),
                Direction::South => (width - 30, height - 10 - 20 * pin_pos),
            }
        };
        
        (dx, dy)
    }
}

/// TTL-specific attributes for component configuration
pub mod ttl_attributes {
    use crate::data::Attribute;
    
    /// Enable VCC/GND power supply pins
    pub fn vcc_gnd() -> Attribute<bool> {
        Attribute::new("VccGndPorts", "VccGndPorts", false)
    }
    
    /// Show internal structure of TTL component
    pub fn draw_internal_structure() -> Attribute<bool> {
        Attribute::new("ShowInternalStructure", "ShowInternalStructure", false)
    }
}