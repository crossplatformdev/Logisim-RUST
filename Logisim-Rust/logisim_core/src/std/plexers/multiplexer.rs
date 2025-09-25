/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Multiplexer component implementation
//!
//! A multiplexer (MUX) is a data selector that routes one of several inputs to a single output
//! based on a selection signal. The selection signal determines which input is connected to the output.

use crate::{
    component::{Component, ComponentId, Pin, Propagator, UpdateResult},
    data::{BitWidth, Bounds, Direction, Location},
    signal::{BusWidth, Signal, Timestamp, Value},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multiplexer component for data selection
///
/// A multiplexer selects one of 2^n inputs based on an n-bit selection signal.
/// The selected input is routed to the output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiplexer {
    /// Unique component identifier
    id: ComponentId,
    /// Component pins (inputs, output, and select)
    pins: HashMap<String, Pin>,
    /// Number of select bits (determines number of inputs)
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

impl Multiplexer {
    /// Create a new multiplexer with the given ID
    pub fn new(id: ComponentId) -> Self {
        let mut multiplexer = Multiplexer {
            id,
            pins: HashMap::new(),
            select_bits: 1,
            data_width: BusWidth(1),
            facing: Direction::East,
            tristate: false,
            enable: false,
            bounds: Bounds::create(0, 0, 40, 30),
        };
        multiplexer.update_pins();
        multiplexer
    }

    /// Create a new multiplexer with specified parameters
    pub fn with_config(
        id: ComponentId,
        select_bits: u8,
        data_width: BusWidth,
        facing: Direction,
    ) -> Self {
        let mut multiplexer = Multiplexer {
            id,
            pins: HashMap::new(),
            select_bits: select_bits.clamp(1, 8),
            data_width,
            facing,
            tristate: false,
            enable: false,
            bounds: Bounds::new(0, 0, 40, 30),
        };
        multiplexer.update_pins();
        multiplexer
    }

    /// Update pin configuration based on current settings
    fn update_pins(&mut self) {
        self.pins.clear();
        
        let num_inputs = 1 << self.select_bits; // 2^select_bits
        
        // Add data inputs
        for i in 0..num_inputs {
            let pin_name = format!("input_{}", i);
            let pin = Pin::new(
                format!("Input {}", i),
                self.data_width,
                crate::component::PinDirection::Input,
                Location::new(0, 10 + i * 10),
            );
            self.pins.insert(pin_name, pin);
        }
        
        // Add output
        let output_pin = Pin::new(
            "Output".to_string(),
            self.data_width,
            crate::component::PinDirection::Output,
            Location::new(40, 15),
        );
        self.pins.insert("output".to_string(), output_pin);
        
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

    /// Calculate the number of inputs based on select bits
    pub fn num_inputs(&self) -> usize {
        1 << self.select_bits
    }
}

impl Component for Multiplexer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Multiplexer"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
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
                            // Component is disabled, set output to high impedance or zero
                            if let Some(output_pin) = self.pins.get_mut("output") {
                                let output_value = if self.tristate {
                                    Value::HighImpedance
                                } else {
                                    Value::Zero
                                };
                                let output_signal = Signal::new(self.data_width, output_value);
                                output_pin.signal = Some(output_signal);
                                return UpdateResult::Changed;
                            }
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
                // Invalid select signal
                if let Some(output_pin) = self.pins.get_mut("output") {
                    let output_signal = Signal::new(self.data_width, Value::Error);
                    output_pin.signal = Some(output_signal);
                }
                return UpdateResult::Changed;
            }
        };

        // Ensure select index is within valid range
        if select_index >= self.num_inputs() {
            if let Some(output_pin) = self.pins.get_mut("output") {
                let output_signal = Signal::new(self.data_width, Value::Error);
                output_pin.signal = Some(output_signal);
            }
            return UpdateResult::Changed;
        }

        // Get the selected input signal
        let input_pin_name = format!("input_{}", select_index);
        let input_value = if let Some(input_pin) = self.pins.get(&input_pin_name) {
            match &input_pin.signal {
                Some(signal) => signal.value().clone(),
                None => Value::HighImpedance, // No input signal
            }
        } else {
            Value::Error // Should not happen
        };

        // Set output to selected input value
        if let Some(output_pin) = self.pins.get_mut("output") {
            let output_signal = Signal::new(self.data_width, input_value);
            output_pin.signal = Some(output_signal);
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

impl Propagator for Multiplexer {
    fn propagate(&mut self, timestamp: Timestamp) -> UpdateResult {
        self.update(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplexer_creation() {
        let mux = Multiplexer::new(ComponentId(1));
        assert_eq!(mux.id(), ComponentId(1));
        assert_eq!(mux.name(), "Multiplexer");
        assert_eq!(mux.select_bits(), 1);
        assert_eq!(mux.num_inputs(), 2);
        assert_eq!(mux.data_width(), BusWidth(1));
    }

    #[test]
    fn test_multiplexer_configuration() {
        let mut mux = Multiplexer::new(ComponentId(1));
        
        // Test setting select bits
        mux.set_select_bits(2);
        assert_eq!(mux.select_bits(), 2);
        assert_eq!(mux.num_inputs(), 4);
        
        // Test setting data width
        mux.set_data_width(BusWidth(8));
        assert_eq!(mux.data_width(), BusWidth(8));
        
        // Test setting facing direction
        mux.set_facing(Direction::North);
        assert_eq!(mux.facing(), Direction::North);
        
        // Test enabling tristate
        mux.set_tristate(true);
        assert!(mux.tristate());
        
        // Test enabling enable input
        mux.set_enable(true);
        assert!(mux.enable());
    }

    #[test]
    fn test_multiplexer_pins() {
        let mux = Multiplexer::new(ComponentId(1));
        let pins = mux.pins();
        
        // Should have 2 inputs + 1 output + 1 select = 4 pins
        assert_eq!(pins.len(), 4);
        assert!(pins.contains_key("input_0"));
        assert!(pins.contains_key("input_1"));
        assert!(pins.contains_key("output"));
        assert!(pins.contains_key("select"));
        
        // Test with more select bits
        let mut mux = Multiplexer::new(ComponentId(2));
        mux.set_select_bits(2);
        let pins = mux.pins();
        
        // Should have 4 inputs + 1 output + 1 select = 6 pins
        assert_eq!(pins.len(), 6);
        assert!(pins.contains_key("input_0"));
        assert!(pins.contains_key("input_1"));
        assert!(pins.contains_key("input_2"));
        assert!(pins.contains_key("input_3"));
    }

    #[test]
    fn test_multiplexer_with_enable() {
        let mut mux = Multiplexer::new(ComponentId(1));
        mux.set_enable(true);
        let pins = mux.pins();
        
        // Should have 2 inputs + 1 output + 1 select + 1 enable = 5 pins
        assert_eq!(pins.len(), 5);
        assert!(pins.contains_key("enable"));
    }

    #[test]
    fn test_multiplexer_reset() {
        let mut mux = Multiplexer::new(ComponentId(1));
        
        // Set some pin signals
        if let Some(pin) = mux.pins.get_mut("input_0") {
            pin.signal = Some(Signal::new(BusWidth(1), Value::One));
        }
        
        // Reset should clear all signals
        mux.reset();
        
        for pin in mux.pins().values() {
            assert!(pin.signal.is_none());
        }
    }

    #[test]
    fn test_multiplexer_propagation_delay() {
        let mux = Multiplexer::new(ComponentId(1));
        assert_eq!(mux.propagation_delay(), 3);
    }
}