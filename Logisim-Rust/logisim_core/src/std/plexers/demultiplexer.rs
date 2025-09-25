/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Demultiplexer component implementation
//!
//! A demultiplexer (DEMUX) is a data router that routes a single input to one of several outputs
//! based on a selection signal. The selection signal determines which output receives the input data.

use crate::{
    component::{Component, ComponentId, Pin, Propagator, UpdateResult},
    data::{BitWidth, Bounds, Direction, Location},
    signal::{BusWidth, Signal, Timestamp, Value},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Demultiplexer component for data routing
///
/// A demultiplexer routes a single input to one of 2^n outputs based on an n-bit selection signal.
/// Only the selected output receives the input data; other outputs are set to zero or high impedance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demultiplexer {
    /// Unique component identifier
    id: ComponentId,
    /// Component pins (input, outputs, and select)
    pins: HashMap<String, Pin>,
    /// Number of select bits (determines number of outputs)
    select_bits: u8,
    /// Data width in bits
    data_width: BusWidth,
    /// Component facing direction
    facing: Direction,
    /// Whether to use three-state outputs
    tristate: bool,
    /// Whether component has enable input
    enable: bool,
    /// Component bounds for rendering
    bounds: Bounds,
}

impl Demultiplexer {
    /// Create a new demultiplexer with the given ID
    pub fn new(id: ComponentId) -> Self {
        let mut demultiplexer = Demultiplexer {
            id,
            pins: HashMap::new(),
            select_bits: 1,
            data_width: BusWidth(1),
            facing: Direction::East,
            tristate: false,
            enable: false,
            bounds: Bounds::new(0, 0, 40, 30),
        };
        demultiplexer.update_pins();
        demultiplexer
    }

    /// Create a new demultiplexer with specified parameters
    pub fn with_config(
        id: ComponentId,
        select_bits: u8,
        data_width: BusWidth,
        facing: Direction,
    ) -> Self {
        let mut demultiplexer = Demultiplexer {
            id,
            pins: HashMap::new(),
            select_bits: select_bits.clamp(1, 8),
            data_width,
            facing,
            tristate: false,
            enable: false,
            bounds: Bounds::new(0, 0, 40, 30),
        };
        demultiplexer.update_pins();
        demultiplexer
    }

    /// Update pin configuration based on current settings
    fn update_pins(&mut self) {
        self.pins.clear();
        
        let num_outputs = 1 << self.select_bits; // 2^select_bits
        
        // Add input
        let input_pin = Pin::new(
            "Input".to_string(),
            self.data_width,
            crate::component::PinDirection::Input,
            Location::new(0, 15),
        );
        self.pins.insert("input".to_string(), input_pin);
        
        // Add data outputs
        for i in 0..num_outputs {
            let pin_name = format!("output_{}", i);
            let pin = Pin::new(
                format!("Output {}", i),
                self.data_width,
                crate::component::PinDirection::Output,
                Location::new(40, 10 + i * 10),
            );
            self.pins.insert(pin_name, pin);
        }
        
        // Add select input
        let select_pin = Pin::new(
            "Select".to_string(),
            BusWidth(self.select_bits as u32),
            crate::component::PinDirection::Input,
            Location::new(20, 30),
        );
        self.pins.insert("select".to_string(), select_pin);
        
        // Add enable input if enabled
        if self.enable {
            let enable_pin = Pin::new(
                "Enable".to_string(),
                BusWidth(1),
                crate::component::PinDirection::Input,
                Location::new(20, 0),
            );
            self.pins.insert("enable".to_string(), enable_pin);
        }
    }

    /// Set the number of select bits
    pub fn set_select_bits(&mut self, bits: u8) {
        if bits != self.select_bits && bits >= 1 && bits <= 8 {
            self.select_bits = bits;
            self.update_pins();
        }
    }

    /// Get the number of select bits
    pub fn select_bits(&self) -> u8 {
        self.select_bits
    }

    /// Set the data width
    pub fn set_data_width(&mut self, width: BusWidth) {
        if width != self.data_width {
            self.data_width = width;
            self.update_pins();
        }
    }

    /// Get the data width
    pub fn data_width(&self) -> BusWidth {
        self.data_width
    }

    /// Set the facing direction
    pub fn set_facing(&mut self, facing: Direction) {
        if facing != self.facing {
            self.facing = facing;
            self.update_pins();
        }
    }

    /// Get the facing direction
    pub fn facing(&self) -> Direction {
        self.facing
    }

    /// Set tristate output mode
    pub fn set_tristate(&mut self, tristate: bool) {
        self.tristate = tristate;
    }

    /// Check if tristate output is enabled
    pub fn tristate(&self) -> bool {
        self.tristate
    }

    /// Set enable input
    pub fn set_enable(&mut self, enable: bool) {
        if enable != self.enable {
            self.enable = enable;
            self.update_pins();
        }
    }

    /// Check if enable input is present
    pub fn enable(&self) -> bool {
        self.enable
    }

    /// Calculate the number of outputs based on select bits
    pub fn num_outputs(&self) -> usize {
        1 << self.select_bits
    }
}

impl Component for Demultiplexer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Demultiplexer"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Get input signal
        let input_value = if let Some(input_pin) = self.pins.get("input") {
            match &input_pin.signal {
                Some(signal) => signal.value().clone(),
                None => Value::HighImpedance, // No input signal
            }
        } else {
            return UpdateResult::NoChange;
        };

        // Get select signal
        let select_value = if let Some(select_pin) = self.pins.get("select") {
            match &select_pin.signal {
                Some(signal) => signal.value(),
                None => return UpdateResult::NoChange, // No select signal
            }
        } else {
            return UpdateResult::NoChange;
        };

        // Check if enabled (if enable pin exists)
        if self.enable {
            if let Some(enable_pin) = self.pins.get("enable") {
                match &enable_pin.signal {
                    Some(signal) => {
                        if signal.value() == Value::Zero {
                            // Component is disabled, set all outputs to high impedance or zero
                            let disabled_value = if self.tristate {
                                Value::HighImpedance
                            } else {
                                Value::Zero
                            };
                            
                            for i in 0..self.num_outputs() {
                                let output_pin_name = format!("output_{}", i);
                                if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                                    let output_signal = Signal::new(self.data_width, disabled_value.clone());
                                    output_pin.signal = Some(output_signal);
                                }
                            }
                            return UpdateResult::Changed;
                        }
                    }
                    None => return UpdateResult::NoChange,
                }
            }
        }

        // Convert select value to index
        let select_index = match select_value {
            Value::Zero => 0,
            Value::One => 1,
            Value::Binary(bits) => {
                // Convert binary representation to index
                bits.iter().enumerate().fold(0usize, |acc, (i, &bit)| {
                    acc + if bit { 1 << i } else { 0 }
                })
            }
            Value::HighImpedance | Value::Error => {
                // Invalid select signal - set all outputs to error
                for i in 0..self.num_outputs() {
                    let output_pin_name = format!("output_{}", i);
                    if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                        let output_signal = Signal::new(self.data_width, Value::Error);
                        output_pin.signal = Some(output_signal);
                    }
                }
                return UpdateResult::Changed;
            }
        };

        // Ensure select index is within valid range
        if select_index >= self.num_outputs() {
            // Invalid select index - set all outputs to error
            for i in 0..self.num_outputs() {
                let output_pin_name = format!("output_{}", i);
                if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                    let output_signal = Signal::new(self.data_width, Value::Error);
                    output_pin.signal = Some(output_signal);
                }
            }
            return UpdateResult::Changed;
        }

        // Set outputs: selected output gets input value, others get zero/high-impedance
        let mut changed = false;
        for i in 0..self.num_outputs() {
            let output_pin_name = format!("output_{}", i);
            if let Some(output_pin) = self.pins.get_mut(&output_pin_name) {
                let output_value = if i == select_index {
                    input_value.clone()
                } else if self.tristate {
                    Value::HighImpedance
                } else {
                    Value::Zero
                };
                
                let output_signal = Signal::new(self.data_width, output_value);
                output_pin.signal = Some(output_signal);
                changed = true;
            }
        }

        if changed {
            UpdateResult::Changed
        } else {
            UpdateResult::NoChange
        }
    }

    fn reset(&mut self) {
        // Clear all pin signals
        for pin in self.pins.values_mut() {
            pin.signal = None;
        }
    }

    fn propagation_delay(&self) -> u64 {
        super::plexers_library::PlexersLibrary::DELAY
    }
}

impl Propagator for Demultiplexer {
    fn propagate(&mut self, timestamp: Timestamp) -> UpdateResult {
        self.update(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demultiplexer_creation() {
        let demux = Demultiplexer::new(ComponentId(1));
        assert_eq!(demux.id(), ComponentId(1));
        assert_eq!(demux.name(), "Demultiplexer");
        assert_eq!(demux.select_bits(), 1);
        assert_eq!(demux.num_outputs(), 2);
        assert_eq!(demux.data_width(), BusWidth(1));
    }

    #[test]
    fn test_demultiplexer_configuration() {
        let mut demux = Demultiplexer::new(ComponentId(1));
        
        // Test setting select bits
        demux.set_select_bits(2);
        assert_eq!(demux.select_bits(), 2);
        assert_eq!(demux.num_outputs(), 4);
        
        // Test setting data width
        demux.set_data_width(BusWidth(8));
        assert_eq!(demux.data_width(), BusWidth(8));
        
        // Test setting facing direction
        demux.set_facing(Direction::North);
        assert_eq!(demux.facing(), Direction::North);
        
        // Test enabling tristate
        demux.set_tristate(true);
        assert!(demux.tristate());
        
        // Test enabling enable input
        demux.set_enable(true);
        assert!(demux.enable());
    }

    #[test]
    fn test_demultiplexer_pins() {
        let demux = Demultiplexer::new(ComponentId(1));
        let pins = demux.pins();
        
        // Should have 1 input + 2 outputs + 1 select = 4 pins
        assert_eq!(pins.len(), 4);
        assert!(pins.contains_key("input"));
        assert!(pins.contains_key("output_0"));
        assert!(pins.contains_key("output_1"));
        assert!(pins.contains_key("select"));
        
        // Test with more select bits
        let mut demux = Demultiplexer::new(ComponentId(2));
        demux.set_select_bits(2);
        let pins = demux.pins();
        
        // Should have 1 input + 4 outputs + 1 select = 6 pins
        assert_eq!(pins.len(), 6);
        assert!(pins.contains_key("output_0"));
        assert!(pins.contains_key("output_1"));
        assert!(pins.contains_key("output_2"));
        assert!(pins.contains_key("output_3"));
    }

    #[test]
    fn test_demultiplexer_with_enable() {
        let mut demux = Demultiplexer::new(ComponentId(1));
        demux.set_enable(true);
        let pins = demux.pins();
        
        // Should have 1 input + 2 outputs + 1 select + 1 enable = 5 pins
        assert_eq!(pins.len(), 5);
        assert!(pins.contains_key("enable"));
    }

    #[test]
    fn test_demultiplexer_reset() {
        let mut demux = Demultiplexer::new(ComponentId(1));
        
        // Set some pin signals
        if let Some(pin) = demux.pins.get_mut("input") {
            pin.signal = Some(Signal::new(BusWidth(1), Value::One));
        }
        
        // Reset should clear all signals
        demux.reset();
        
        for pin in demux.pins().values() {
            assert!(pin.signal.is_none());
        }
    }

    #[test]
    fn test_demultiplexer_propagation_delay() {
        let demux = Demultiplexer::new(ComponentId(1));
        assert_eq!(demux.propagation_delay(), 3);
    }
}