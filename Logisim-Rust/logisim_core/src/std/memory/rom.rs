/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! ROM (Read-Only Memory) component
//!
//! This module implements ROM functionality equivalent to Rom.java.
//! ROM provides read-only memory storage that can be programmed with initial values.

use crate::{Attribute, AttributeSet, BitWidth, Bounds, Direction, Location, StdAttr, Value};
use crate::instance::{Instance, InstanceFactory, InstancePainter, InstanceState, Port, PortType};
use crate::std::memory::{MemContents, MemState};
use crate::std::memory::mem::{MemFactory, MemoryComponent};
use crate::util::StringGetter;
use std::sync::{Arc, Mutex};

/// ROM component factory
pub struct RomFactory {
    base: MemFactory,
}

impl RomFactory {
    pub fn new() -> Self {
        Self {
            base: MemFactory::new(
                "ROM".to_string(),
                StringGetter::new("romComponent"),
                0, // No extra ports
                false, // No label needed
            ),
        }
    }
}

impl Default for RomFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl InstanceFactory for RomFactory {
    fn get_name(&self) -> &str {
        "ROM"
    }

    fn get_display_name(&self) -> String {
        "ROM".to_string()
    }

    fn create_attribute_set(&self) -> AttributeSet {
        // TODO: Implement proper attributes when attribute system is complete
        AttributeSet::new()
    }
}

impl RomFactory {
    const DELAY: i32 = 5;

    /// Configure ROM-specific ports
    fn configure_ports(&self, instance: &mut Instance) {
        // TODO: Implement proper port configuration when attribute system is complete
        // For now, use default values
        let addr_bits = 8;
        let data_bits = 8;

        let mut ports = Vec::new();
        
        // Address input port
        ports.push(Port::new(
            Location::new(0, 20),
            PortType::Input,
            BitWidth::create(addr_bits),
        ));
        
        // Clock input (for compatibility, though ROM doesn't need it)
        ports.push(Port::new(
            Location::new(0, 40),
            PortType::Input,
            BitWidth::ONE,
        ));
        
        // Data output port
        ports.push(Port::new(
            Location::new(60, 30),
            PortType::Output,
            BitWidth::create(data_bits),
        ));
        
        instance.set_ports(&ports);
    }

    /// Get or create memory state for this ROM instance
    fn get_or_create_mem_state(&self, state: &InstanceState, addr_bits: i32, data_bits: i32) -> MemState {
        // Try to get existing state
        if let Some(mem_state) = self.base.get_mem_state(state) {
            return mem_state;
        }
        
        // Create new ROM memory state
        let mem_state = MemState::new(addr_bits, data_bits);
        
        // Initialize with default ROM contents
        // TODO: Load from file or attributes if specified
        
        mem_state
    }

    /// Paint the base ROM component
    fn paint_base(&self, painter: &mut InstancePainter) {
        // Get component bounds
        let bounds = painter.get_bounds();
        let g = painter.get_graphics();
        
        // Draw ROM rectangle
        g.set_color(painter.get_attribute_value(&StdAttr::FILL_COLOR).unwrap_or_default());
        g.fill_rect(bounds.get_x(), bounds.get_y(), bounds.get_width(), bounds.get_height());
        
        g.set_color(painter.get_stroke_color());
        g.draw_rect(bounds.get_x(), bounds.get_y(), bounds.get_width(), bounds.get_height());
        
        // Draw "ROM" label
        g.set_color(painter.get_text_color());
        let center_x = bounds.get_x() + bounds.get_width() / 2;
        let center_y = bounds.get_y() + bounds.get_height() / 2;
        g.draw_string_centered("ROM", center_x, center_y);
        
        // Draw ports
        painter.draw_ports();
    }
}

impl MemoryComponent for RomFactory {
    fn get_contents(&self, state: &InstanceState) -> Option<Arc<Mutex<MemContents>>> {
        self.base.get_mem_state(state)
            .map(|mem_state| mem_state.get_contents())
    }

    fn set_contents(&self, state: &mut InstanceState, contents: MemContents) {
        if let Some(mut mem_state) = self.base.get_mem_state(state) {
            mem_state.set_contents(contents);
            self.base.set_mem_state(state, mem_state);
        }
    }

    fn get_addr_bits(&self, attrs: &AttributeSet) -> i32 {
        // TODO: Get from attributes when system is complete
        8
    }

    fn get_data_bits(&self, attrs: &AttributeSet) -> i32 {
        // TODO: Get from attributes when system is complete  
        8
    }
}

/// ROM contents attribute for storing ROM data
#[derive(Clone)]
pub struct RomContentsAttribute {
    name: String,
    display_name: StringGetter,
}

impl RomContentsAttribute {
    pub fn new() -> Self {
        Self {
            name: "contents".to_string(),
            display_name: StringGetter::new("romContentsAttr"),
        }
    }
}

impl Default for RomContentsAttribute {
    fn default() -> Self {
        Self::new()
    }
}

impl Attribute<MemContents> for RomContentsAttribute {
    fn name(&self) -> &str {
        &self.name
    }

    fn display_name(&self) -> String {
        self.display_name.get()
    }

    fn parse(&self, value: &str) -> Result<MemContents, String> {
        // TODO: Implement ROM contents parsing from string
        // This would parse hex files or other ROM data formats
        Ok(MemContents::create(8, 8, false))
    }

    fn to_display_string(&self, value: &MemContents) -> String {
        format!("ROM Contents ({}x{} bits)", 
                1 << value.get_log_length(), 
                value.get_width())
    }

    fn to_standard_string(&self, value: &MemContents) -> String {
        // TODO: Implement proper ROM contents serialization
        format!("addr/data: {} {}\n", 
                value.get_log_length(), 
                value.get_width())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom_factory_creation() {
        let factory = RomFactory::new();
        assert_eq!(factory.get_name(), "ROM");
        assert_eq!(factory.get_display_name(), "ROM");
    }

    #[test]
    fn test_rom_factory_attributes() {
        let factory = RomFactory::new();
        let attrs = factory.get_default_attribute_set();
        
        // Should be able to create attributes without error
        // TODO: Add specific tests when attribute system is complete
    }

    #[test]
    fn test_rom_factory_bounds() {
        let factory = RomFactory::new();
        let attrs = factory.get_default_attribute_set();
        let bounds = factory.create_bounds(&attrs);
        
        assert_eq!(bounds.get_width(), 60);
        assert_eq!(bounds.get_height(), 60);
    }

    #[test]
    fn test_rom_factory_as_memory_component() {
        let factory = RomFactory::new();
        let attrs = factory.get_default_attribute_set();
        
        assert_eq!(factory.get_addr_bits(&attrs), 8);
        assert_eq!(factory.get_data_bits(&attrs), 8);
        assert_eq!(factory.get_extra_ports(), 0);
        assert!(!factory.needs_label());
    }

    #[test]
    fn test_rom_contents_attribute() {
        let attr = RomContentsAttribute::new();
        assert_eq!(attr.name(), "contents");
        
        let contents = MemContents::create(8, 8, false);
        let display = attr.to_display_string(&contents);
        assert!(display.contains("ROM Contents"));
        assert!(display.contains("256x8"));
    }
}