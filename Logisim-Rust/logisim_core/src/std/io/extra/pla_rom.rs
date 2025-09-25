/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! PLA ROM Component
//!
//! Rust port of `com.cburch.logisim.std.io.extra.PlaRom`
//!
//! Programmable Logic Array ROM with configurable data editor.

use crate::{
    data::{Attribute, BitWidth, Bounds, Direction},
    signal::{Signal, Value},
    util::StringGetter,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// PLA ROM data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaRomData {
    /// Input width (address bits)
    pub input_width: usize,
    /// Output width (data bits)
    pub output_width: usize,
    /// ROM contents - maps input addresses to output values
    pub contents: HashMap<u64, u64>,
    /// Default output value for unmapped addresses
    pub default_value: u64,
}

impl PlaRomData {
    /// Create new PLA ROM data
    pub fn new(input_width: usize, output_width: usize) -> Self {
        Self {
            input_width: input_width.max(1).min(16), // Reasonable limits
            output_width: output_width.max(1).min(32),
            contents: HashMap::new(),
            default_value: 0,
        }
    }

    /// Set data at given address
    pub fn set_data(&mut self, address: u64, data: u64) {
        let address_mask = (1u64 << self.input_width) - 1;
        let data_mask = (1u64 << self.output_width) - 1;
        
        let masked_address = address & address_mask;
        let masked_data = data & data_mask;
        
        if masked_data == self.default_value {
            self.contents.remove(&masked_address);
        } else {
            self.contents.insert(masked_address, masked_data);
        }
    }

    /// Get data at given address
    pub fn get_data(&self, address: u64) -> u64 {
        let address_mask = (1u64 << self.input_width) - 1;
        let masked_address = address & address_mask;
        
        self.contents.get(&masked_address)
            .copied()
            .unwrap_or(self.default_value)
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.contents.clear();
    }

    /// Get number of programmed addresses
    pub fn get_programmed_count(&self) -> usize {
        self.contents.len()
    }

    /// Serialize data to string format (for file storage)
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("input_width:{}\n", self.input_width));
        result.push_str(&format!("output_width:{}\n", self.output_width));
        result.push_str(&format!("default_value:{}\n", self.default_value));
        
        for (&address, &data) in &self.contents {
            result.push_str(&format!("{}:{}\n", address, data));
        }
        
        result
    }

    /// Parse data from string format
    pub fn from_string(s: &str) -> Result<Self, String> {
        let mut input_width = 4;
        let mut output_width = 8;
        let mut default_value = 0;
        let mut contents = HashMap::new();
        
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(colon_pos) = line.find(':') {
                let key = &line[..colon_pos];
                let value_str = &line[colon_pos + 1..];
                
                match key {
                    "input_width" => {
                        input_width = value_str.parse()
                            .map_err(|_| format!("Invalid input_width: {}", value_str))?;
                    }
                    "output_width" => {
                        output_width = value_str.parse()
                            .map_err(|_| format!("Invalid output_width: {}", value_str))?;
                    }
                    "default_value" => {
                        default_value = value_str.parse()
                            .map_err(|_| format!("Invalid default_value: {}", value_str))?;
                    }
                    _ => {
                        // Try to parse as address:data pair
                        let address: u64 = key.parse()
                            .map_err(|_| format!("Invalid address: {}", key))?;
                        let data: u64 = value_str.parse()
                            .map_err(|_| format!("Invalid data: {}", value_str))?;
                        contents.insert(address, data);
                    }
                }
            }
        }
        
        Ok(Self {
            input_width,
            output_width,
            contents,
            default_value,
        })
    }
}

/// PLA ROM component implementation
///
/// Programmable Logic Array ROM that stores and retrieves data based on input addresses.
/// Supports configurable input/output widths and programmable data contents.
#[derive(Debug, Clone)]
pub struct PlaRom {
    /// Component identifier
    id: ComponentId,
    /// ROM data
    data: PlaRomData,
    /// Component attributes
    attributes: HashMap<String, Attribute>,
}

impl PlaRom {
    /// Create a new PLA ROM component
    pub fn new(id: ComponentId) -> Self {
        let mut attributes = HashMap::new();
        
        // Initialize default attributes
        attributes.insert(
            "facing".to_string(),
            Attribute::Direction(Direction::East),
        );
        attributes.insert(
            "input_width".to_string(),
            Attribute::BitWidth(BitWidth::new(4)),
        );
        attributes.insert(
            "output_width".to_string(),
            Attribute::BitWidth(BitWidth::new(8)),
        );
        attributes.insert(
            "label".to_string(),
            Attribute::String("".to_string()),
        );
        attributes.insert(
            "contents".to_string(),
            Attribute::String("".to_string()),
        );

        Self {
            id,
            data: PlaRomData::new(4, 8),
            attributes,
        }
    }

    /// Get the current ROM data
    pub fn get_data(&self) -> &PlaRomData {
        &self.data
    }

    /// Get mutable reference to ROM data
    pub fn get_data_mut(&mut self) -> &mut PlaRomData {
        &mut self.data
    }

    /// Update ROM configuration based on attributes
    fn update_configuration(&mut self) {
        let input_width = self.get_attribute("input_width")
            .and_then(|attr| attr.as_bit_width())
            .map(|bw| bw.width())
            .unwrap_or(4);

        let output_width = self.get_attribute("output_width")
            .and_then(|attr| attr.as_bit_width())
            .map(|bw| bw.width())
            .unwrap_or(8);

        // Recreate data if configuration changed
        if input_width != self.data.input_width || output_width != self.data.output_width {
            let old_contents = self.data.contents.clone();
            self.data = PlaRomData::new(input_width, output_width);
            // Preserve compatible data
            for (&address, &data) in &old_contents {
                if address < (1u64 << input_width) && data < (1u64 << output_width) {
                    self.data.contents.insert(address, data);
                }
            }
        }

        // Load contents from attribute
        if let Some(contents_str) = self.get_attribute("contents")
            .and_then(|attr| attr.as_string()) {
            if !contents_str.is_empty() {
                if let Ok(parsed_data) = PlaRomData::from_string(contents_str) {
                    // Use parsed data if compatible
                    if parsed_data.input_width == self.data.input_width &&
                       parsed_data.output_width == self.data.output_width {
                        self.data = parsed_data;
                    }
                }
            }
        }
    }

    /// Handle data editor interaction (placeholder)
    pub fn open_data_editor(&mut self) -> bool {
        // This would open a data editor dialog in a real implementation
        // For now, just return success
        true
    }

    /// Get the component's display name
    pub fn display_name() -> StringGetter {
        StringGetter::new("PlaRomComponent")
    }

    /// Get the component's factory ID
    pub fn factory_id() -> &'static str {
        "PlaRom"
    }
}

impl Component for PlaRom {
    fn get_id(&self) -> ComponentId {
        self.id
    }

    fn get_type_name(&self) -> &'static str {
        "PlaRom"
    }

    fn get_bounds(&self) -> Bounds {
        // ROM bounds: width depends on bit widths
        let facing = self.get_attribute("facing")
            .and_then(|attr| attr.as_direction())
            .unwrap_or(Direction::East);
            
        let width = (self.data.input_width.max(self.data.output_width) * 10 + 40).max(60);
        let height = 40;
        
        match facing {
            Direction::East => Bounds::new(-width as i32, -height / 2, width as i32, height),
            Direction::West => Bounds::new(0, -height / 2, width as i32, height),
            Direction::North => Bounds::new(-width as i32 / 2, 0, width as i32, height),
            Direction::South => Bounds::new(-width as i32 / 2, -height, width as i32, height),
        }
    }

    fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    fn set_attribute(&mut self, name: String, value: Attribute) {
        self.attributes.insert(name, value);
        self.update_configuration();
    }

    fn get_input_count(&self) -> usize {
        1 // Address input
    }

    fn get_output_count(&self) -> usize {
        1 // Data output
    }

    fn propagate(&mut self, inputs: &[Signal]) -> Vec<Signal> {
        let output_width = BitWidth::new(self.data.output_width);
        
        if inputs.is_empty() {
            return vec![Signal::new(Value::unknown(output_width))];
        }

        let address = inputs[0].value.as_u64().unwrap_or(0);
        let data = self.data.get_data(address);
        let output_value = Value::known(data, output_width);

        vec![Signal::new(output_value)]
    }

    fn is_interactive(&self) -> bool {
        true // Can open data editor
    }

    fn clone_component(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pla_rom_creation() {
        let rom = PlaRom::new(ComponentId::new(1));
        assert_eq!(rom.get_id(), ComponentId::new(1));
        assert_eq!(rom.get_type_name(), "PlaRom");
        assert!(rom.is_interactive());
        assert_eq!(rom.get_input_count(), 1);
        assert_eq!(rom.get_output_count(), 1);
    }

    #[test]
    fn test_pla_rom_data() {
        let mut data = PlaRomData::new(4, 8);
        assert_eq!(data.input_width, 4);
        assert_eq!(data.output_width, 8);
        assert_eq!(data.get_programmed_count(), 0);

        // Test setting and getting data
        data.set_data(5, 0xAB);
        assert_eq!(data.get_data(5), 0xAB);
        assert_eq!(data.get_data(6), 0); // Default value
        assert_eq!(data.get_programmed_count(), 1);

        // Test address masking
        data.set_data(0x15, 0xCD); // Address 0x15 -> 0x5 (4-bit mask)
        assert_eq!(data.get_data(0x5), 0xCD);
    }

    #[test]
    fn test_pla_rom_serialization() {
        let mut data = PlaRomData::new(4, 8);
        data.set_data(1, 0x42);
        data.set_data(3, 0x84);
        data.default_value = 0xFF;

        let serialized = data.to_string();
        let deserialized = PlaRomData::from_string(&serialized).unwrap();
        
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_pla_rom_propagation() {
        let mut rom = PlaRom::new(ComponentId::new(1));
        
        // Program some data
        rom.get_data_mut().set_data(0, 0x11);
        rom.get_data_mut().set_data(1, 0x22);
        rom.get_data_mut().set_data(2, 0x33);
        
        // Test address 0
        let inputs = vec![Signal::new(Value::known(0, BitWidth::new(4)))];
        let outputs = rom.propagate(&inputs);
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].value.as_u64().unwrap(), 0x11);
        
        // Test address 1
        let inputs = vec![Signal::new(Value::known(1, BitWidth::new(4)))];
        let outputs = rom.propagate(&inputs);
        assert_eq!(outputs[0].value.as_u64().unwrap(), 0x22);
        
        // Test unprogrammed address (should return default)
        let inputs = vec![Signal::new(Value::known(5, BitWidth::new(4)))];
        let outputs = rom.propagate(&inputs);
        assert_eq!(outputs[0].value.as_u64().unwrap(), 0);
    }

    #[test]
    fn test_pla_rom_configuration() {
        let mut rom = PlaRom::new(ComponentId::new(1));
        
        // Change input width
        rom.set_attribute("input_width".to_string(), Attribute::BitWidth(BitWidth::new(6)));
        assert_eq!(rom.get_data().input_width, 6);
        
        // Change output width
        rom.set_attribute("output_width".to_string(), Attribute::BitWidth(BitWidth::new(12)));
        assert_eq!(rom.get_data().output_width, 12);
    }

    #[test]
    fn test_data_bounds_checking() {
        let mut data = PlaRomData::new(2, 2); // 2-bit address, 2-bit data
        
        // Test address masking (only lower 2 bits should be used)
        data.set_data(0b1111, 0b11); // Address 15 -> 3, Data 3 -> 3
        assert_eq!(data.get_data(0b11), 0b11);
        assert_eq!(data.get_data(0b1111), 0b11); // Same result due to masking
        
        // Test data masking
        data.set_data(0, 0b1111); // Data 15 -> 3 (2-bit mask)
        assert_eq!(data.get_data(0), 0b11);
    }

    #[test]
    fn test_data_string_parsing() {
        let data_str = r#"
            input_width:3
            output_width:4
            default_value:7
            0:1
            1:2
            2:4
        "#;
        
        let data = PlaRomData::from_string(data_str).unwrap();
        assert_eq!(data.input_width, 3);
        assert_eq!(data.output_width, 4);
        assert_eq!(data.default_value, 7);
        assert_eq!(data.get_data(0), 1);
        assert_eq!(data.get_data(1), 2);
        assert_eq!(data.get_data(2), 4);
        assert_eq!(data.get_data(3), 7); // Default value
    }
}