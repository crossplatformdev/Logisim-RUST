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

use crate::{AttributeSet, BitWidth, Bounds, Location};
use crate::instance::{Instance, InstanceFactory, InstancePainter, InstanceState, Port, PortType, PortWidth};
use crate::std::memory::{MemContents, MemState};
use crate::std::memory::mem::{MemFactory, MemoryComponent};
use std::sync::{Arc, Mutex};

/// ROM component factory
#[derive(Debug)]
pub struct RomFactory {
    base: MemFactory,
}

impl RomFactory {
    pub fn new() -> Self {
        Self {
            base: MemFactory::new(
                "ROM".to_string(),
                "ROM Component".to_string(),
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

    fn get_ports(&self) -> &[Port] {
        &[]
    }

    fn get_offset_bounds(&self, _attrs: &AttributeSet) -> Bounds {
        Bounds::create(0, 0, 60, 60)
    }

    fn create_component(&self, _location: Location, _attrs: AttributeSet) -> Box<dyn std::any::Any> {
        Box::new(())
    }

    fn paint_instance(&self, _painter: &mut InstancePainter) {
        // TODO: Implement ROM drawing
    }

    fn propagate(&self, _state: &mut dyn InstanceState) {
        // TODO: Implement ROM logic
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
            0, 20,
            PortType::Input,
            PortWidth::fixed(BitWidth::create(addr_bits as u32).unwrap_or_default()),
        ));
        
        // Clock input (for compatibility, though ROM doesn't need it)
        ports.push(Port::new(
            0, 40,
            PortType::Input,
            PortWidth::fixed(BitWidth::ONE),
        ));
        
        // Data output port
        ports.push(Port::new(
            60, 30,
            PortType::Output,
            PortWidth::fixed(BitWidth::create(data_bits as u32).unwrap_or_default()),
        ));
        
        // Set ports for the instance (ports are managed internally)
        // instance.set_ports(&ports); // TODO: Replace with proper port management
    }

    /// Get or create memory state for this ROM instance
    fn get_or_create_mem_state(&self, _state: &dyn InstanceState, addr_bits: i32, data_bits: i32) -> MemState {
        // Try to get existing state
        if let Some(mem_state) = self.base.get_mem_state(_state) {
            return mem_state;
        }
        
        // Create new ROM memory state
        let mem_state = MemState::new(addr_bits, data_bits);
        
        // Initialize with default ROM contents
        // TODO: Load from file or attributes if specified
        
        mem_state
    }

    /// Paint the base ROM component
    fn paint_base(&self, _painter: &mut InstancePainter) {
        // TODO: Implement ROM painting when graphics system is complete
        // Get component bounds
        // let bounds = painter.get_bounds();
        // let g = painter.get_graphics();
        
        // Draw ROM rectangle and label
    }
}

impl MemoryComponent for RomFactory {
    fn get_contents(&self, _state: &dyn InstanceState) -> Option<Arc<Mutex<MemContents>>> {
        // TODO: Implement when state system is complete
        None
    }

    fn set_contents(&self, _state: &mut dyn InstanceState, _contents: MemContents) {
        // TODO: Implement when state system is complete
    }

    fn get_addr_bits(&self, _attrs: &AttributeSet) -> i32 {
        // TODO: Get from attributes when system is complete
        8
    }

    fn get_data_bits(&self, _attrs: &AttributeSet) -> i32 {
        // TODO: Get from attributes when system is complete  
        8
    }
}

/// ROM contents attribute for storing ROM data
#[derive(Clone)]
pub struct RomContentsAttribute {
    name: String,
    display_name: String,
}

impl RomContentsAttribute {
    pub fn new() -> Self {
        Self {
            name: "contents".to_string(),
            display_name: "ROM Contents".to_string(),
        }
    }
}

impl Default for RomContentsAttribute {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Implement proper Attribute trait when the attribute system is complete

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