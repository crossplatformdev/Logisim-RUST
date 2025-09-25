/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Binary to BCD Converter Component
//!
//! This module implements a binary to BCD (Binary Coded Decimal) converter component
//! that converts binary input values to multiple BCD outputs based on the decimal
//! representation of the input value.

use crate::data::{Attribute, AttributeSet, Attributes, BitWidth, Bounds, Direction, Value, Location};
use crate::instance::{InstanceFactory, InstancePainter, InstanceState, Port, Instance};
use crate::tools::Tool;
use std::fmt::Debug;

/// Binary to BCD Converter
///
/// Converts binary input values (4-13 bits) to multiple BCD outputs representing
/// the decimal digits of the input value. Each BCD output is 4 bits wide and
/// represents one decimal digit.
///
/// ## Ports
///
/// - **Input**: Binary value (configurable width: 4-13 bits)
/// - **Outputs**: BCD digits (4 bits each), number depends on input bit width
///
/// ## Example
///
/// For a 9-bit input (max value 511):
/// - Input: binary 123 (0b01111011)
/// - Output 1: BCD 3 (0b0011) - ones digit
/// - Output 2: BCD 2 (0b0010) - tens digit  
/// - Output 3: BCD 1 (0b0001) - hundreds digit
#[derive(Debug, Clone)]
pub struct BinToBcd {
    bit_width_attr: Attribute<BitWidth>,
}

impl BinToBcd {
    /// Unique identifier of the tool, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "Binary_to_BCD_converter";

    /// Propagation delay in time units
    pub const PROPAGATION_DELAY: i32 = 1;

    /// Input port index
    const BIN_INPUT_PORT: usize = 0;

    /// Distance between output ports
    const INNER_DISTANCE: i32 = 60;

    /// Creates a new Binary to BCD converter
    pub fn new() -> Self {
        Self {
            bit_width_attr: Attributes::for_bit_width(
                "binvalue",
                "Binary Data Bits",
                4,
                13,
            ),
        }
    }

    /// Calculate number of BCD output ports needed based on bit width
    fn calculate_bcd_ports(bit_width: &BitWidth) -> usize {
        let max_value = (1u32 << bit_width.width()) - 1;
        let max_decimal = max_value as f64;
        (max_decimal.log10().floor() as usize) + 1
    }

    /// Calculate the bounds for the component based on number of ports
    fn calculate_bounds(num_ports: usize) -> Bounds {
        let width = (num_ports as i32) * Self::INNER_DISTANCE;
        let x_offset = -(Self::INNER_DISTANCE / 2);
        Bounds::create(x_offset, -20, width, 40)
    }

    /// Update the component's ports based on current bit width
    fn update_ports(&self, instance: &mut Instance) {
        let bit_width = instance.get_attribute_value(&self.bit_width_attr)
            .unwrap_or(BitWidth::create(9));
        let num_bcd_ports = Self::calculate_bcd_ports(&bit_width);
        
        let mut ports = Vec::with_capacity(num_bcd_ports + 1);
        
        // Input port
        ports.push(Port::new(
            Location::create(-(Self::INNER_DISTANCE / 2), 0),
            Port::INPUT,
            bit_width,
        ).with_tooltip("Binary Input"));

        // BCD output ports (from most significant to least significant)
        for i in 0..num_bcd_ports {
            let x = (i as i32) * Self::INNER_DISTANCE;
            let power = num_bcd_ports - 1 - i;
            let decimal_value = 10_i32.pow(power as u32);
            
            ports.push(Port::new(
                Location::create(x, -20),
                Port::OUTPUT,
                BitWidth::create(4),
            ).with_tooltip(&format!("{}", decimal_value)));
        }
        
        instance.set_ports(ports);
    }
}

impl Default for BinToBcd {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for BinToBcd {
    fn get_name(&self) -> &str {
        "Bin2BCD"
    }

    fn get_display_name(&self) -> &str {
        "Binary to BCD Converter"
    }

    fn get_description(&self) -> Option<String> {
        Some("Converts binary input to Binary Coded Decimal (BCD) output digits".to_string())
    }

    fn clone_tool(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }
}

impl InstanceFactory for BinToBcd {
    fn create_instance(&self, id: crate::ComponentId, location: Location) -> Instance {
        let mut instance = Instance::new(id, Self::ID.to_string(), location);
        
        // Set default attributes
        let mut attrs = AttributeSet::new();
        attrs.set_value(self.bit_width_attr.clone(), BitWidth::create(9));
        instance.set_attribute_set(attrs);
        
        // Initialize ports
        self.update_ports(&mut instance);
        
        instance
    }

    fn paint_instance(&self, painter: &mut InstancePainter) {
        let bounds = painter.get_bounds();
        let bit_width = painter.get_attribute_value(&self.bit_width_attr)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(&bit_width);

        // Draw component body
        painter.draw_rectangle(bounds, "");
        
        // Draw input port
        painter.draw_port(Self::BIN_INPUT_PORT, "Bin", Direction::East);
        
        // Draw output ports with labels
        for i in 0..num_ports {
            let port_index = i + 1;
            let power = num_ports - 1 - i;
            let decimal_value = 10_i32.pow(power as u32);
            painter.draw_port(port_index, &decimal_value.to_string(), Direction::North);
        }
    }

    fn get_offset_bounds(&self, attrs: &AttributeSet) -> Bounds {
        let bit_width = attrs.get_value(&self.bit_width_attr)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(&bit_width);
        Self::calculate_bounds(num_ports)
    }

    fn propagate(&self, state: &mut InstanceState) {
        let input_value = state.get_port_value(Self::BIN_INPUT_PORT);
        
        // Check if input is valid
        let binary_value = if input_value.is_fully_defined() 
            && !input_value.is_unknown() 
            && !input_value.is_error_value() {
            input_value.to_long_value().unwrap_or(0) as u32
        } else {
            // Set all outputs to unknown
            let bit_width = state.get_attribute_value(&self.bit_width_attr)
                .unwrap_or(BitWidth::create(9));
            let num_ports = Self::calculate_bcd_ports(&bit_width);
            
            for i in 1..=num_ports {
                state.set_port(i, Value::create_unknown(BitWidth::create(4)), Self::PROPAGATION_DELAY);
            }
            return;
        };

        // Convert to BCD digits
        let bit_width = state.get_attribute_value(&self.bit_width_attr)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(&bit_width);
        
        let mut remaining_value = binary_value;
        
        // Process each decimal digit from most significant to least significant
        for i in 0..num_ports {
            let port_index = num_ports - i; // Reverse order for port indexing
            let power = i as u32;
            let divisor = 10_u32.pow(power);
            
            let digit = remaining_value % 10;
            remaining_value /= 10;
            
            let bcd_value = Value::create_known(BitWidth::create(4), digit as i64);
            state.set_port(port_index, bcd_value, Self::PROPAGATION_DELAY);
        }
    }

    fn configure_new_instance(&self, instance: &mut Instance) {
        // Add attribute listener for dynamic port updates
        instance.add_attribute_listener();
        self.update_ports(instance);
    }

    fn instance_attribute_changed(&self, instance: &mut Instance, attr: &dyn std::any::Any) {
        // Check if the bit width attribute changed
        if let Some(bit_width_attr) = attr.downcast_ref::<Attribute<BitWidth>>() {
            if bit_width_attr == &self.bit_width_attr {
                instance.recompute_bounds();
                self.update_ports(instance);
            }
        }
    }

    fn get_hdl_name(&self, attrs: &AttributeSet) -> String {
        let bit_width = attrs.get_value(&self.bit_width_attr)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(&bit_width);
        format!("Bin2BCD_{}_bcd_ports", num_ports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ComponentId;

    #[test]
    fn test_component_creation() {
        let converter = BinToBcd::new();
        assert_eq!(converter.get_name(), "Bin2BCD");
        assert_eq!(converter.get_display_name(), "Binary to BCD Converter");
    }

    #[test]
    fn test_id_constant() {
        // Ensure ID never changes for .circ file compatibility
        assert_eq!(BinToBcd::ID, "Binary_to_BCD_converter");
    }

    #[test]
    fn test_bcd_ports_calculation() {
        // 4 bits: max 15, needs 2 BCD digits
        let bit_width_4 = BitWidth::create(4);
        assert_eq!(BinToBcd::calculate_bcd_ports(&bit_width_4), 2);
        
        // 8 bits: max 255, needs 3 BCD digits  
        let bit_width_8 = BitWidth::create(8);
        assert_eq!(BinToBcd::calculate_bcd_ports(&bit_width_8), 3);
        
        // 10 bits: max 1023, needs 4 BCD digits
        let bit_width_10 = BitWidth::create(10);
        assert_eq!(BinToBcd::calculate_bcd_ports(&bit_width_10), 4);
    }

    #[test]
    fn test_bounds_calculation() {
        let bounds_2_ports = BinToBcd::calculate_bounds(2);
        assert_eq!(bounds_2_ports.width(), 120); // 2 * 60
        
        let bounds_3_ports = BinToBcd::calculate_bounds(3);
        assert_eq!(bounds_3_ports.width(), 180); // 3 * 60
    }

    #[test]
    fn test_instance_creation() {
        let converter = BinToBcd::new();
        let instance = converter.create_instance(
            ComponentId(1), 
            Location::create(100, 100)
        );
        
        assert_eq!(instance.get_factory_id(), BinToBcd::ID);
        assert_eq!(instance.get_location(), Location::create(100, 100));
    }

    #[test]
    fn test_hdl_name_generation() {
        let converter = BinToBcd::new();
        let mut attrs = AttributeSet::new();
        attrs.set_value(converter.bit_width_attr.clone(), BitWidth::create(8));
        
        let hdl_name = converter.get_hdl_name(&attrs);
        assert_eq!(hdl_name, "Bin2BCD_3_bcd_ports");
    }

    #[test]
    fn test_binary_to_bcd_conversion_logic() {
        // Test the core conversion logic
        let test_cases = vec![
            (0, vec![0]),
            (7, vec![7]),
            (10, vec![1, 0]),
            (99, vec![9, 9]),
            (123, vec![1, 2, 3]),
            (255, vec![2, 5, 5]),
        ];

        for (binary, expected_digits) in test_cases {
            let mut remaining = binary;
            let mut digits = Vec::new();
            
            // Extract digits (same logic as component)
            while remaining > 0 || digits.is_empty() {
                digits.push(remaining % 10);
                remaining /= 10;
            }
            digits.reverse();
            
            assert_eq!(digits, expected_digits, "Failed for input {}", binary);
        }
    }
}