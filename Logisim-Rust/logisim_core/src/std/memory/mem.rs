/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Base memory component implementation
//!
//! This module provides the base functionality for memory components,
//! equivalent to the Java Mem.java abstract class.

use crate::{Attribute, AttributeSet, BitWidth, Bounds, Direction, Location, StdAttr};
use crate::instance::{Instance, InstanceFactory, InstanceState, Port, PortType};
use crate::std::memory::{MemContents, MemState};
use crate::util::StringGetter;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Weak, Mutex};

/// Memory enable modes
#[derive(Debug, Clone, PartialEq)]
pub enum EnableMode {
    ByteEnables,
    LineEnables,
}

/// Base trait for memory components
pub trait MemoryComponent: Send + Sync {
    /// Get the memory contents
    fn get_contents(&self, state: &InstanceState) -> Option<Arc<Mutex<MemContents>>>;
    
    /// Set the memory contents
    fn set_contents(&self, state: &mut InstanceState, contents: MemContents);
    
    /// Get the address bit width
    fn get_addr_bits(&self, attrs: &AttributeSet) -> i32;
    
    /// Get the data bit width  
    fn get_data_bits(&self, attrs: &AttributeSet) -> i32;
    
    /// Get extra port count beyond standard memory ports
    fn get_extra_ports(&self) -> i32 { 0 }
    
    /// Check if this component needs a label
    fn needs_label(&self) -> bool { false }
}

/// Base memory component factory
pub struct MemFactory {
    name: String,
    description: crate::util::ConstantStringGetter,
    extra_ports: i32,
    needs_label: bool,
    current_files: HashMap<u32, PathBuf>, // Instance ID -> File path
}

impl MemFactory {
    /// Create a new memory factory
    pub fn new(
        name: String,
        description: StringGetter,
        extra_ports: i32,
        needs_label: bool,
    ) -> Self {
        Self {
            name,
            description,
            extra_ports,
            needs_label,
            current_files: HashMap::new(),
        }
    }

    /// Memory delay constant
    pub const DELAY: i32 = 10;

    /// Create common memory attributes
    pub fn create_attributes() -> AttributeSet {
        // TODO: Implement proper attribute creation when attribute system is complete
        AttributeSet::new()
    }

    /// Configure ports for a memory component
    pub fn configure_ports(&self, instance: &mut Instance, addr_bits: i32, data_bits: i32) {
        let mut ports = Vec::new();
        
        // Address port
        ports.push(Port::new(
            Location::new(0, 10),
            PortType::Input,
            BitWidth::create(addr_bits),
        ));
        
        // Data input port  
        ports.push(Port::new(
            Location::new(0, 20),
            PortType::Input,
            BitWidth::create(data_bits),
        ));
        
        // Data output port
        ports.push(Port::new(
            Location::new(60, 20),
            PortType::Output, 
            BitWidth::create(data_bits),
        ));
        
        // Clock port
        ports.push(Port::new(
            Location::new(0, 30),
            PortType::Input,
            BitWidth::ONE,
        ));
        
        // Write enable port
        ports.push(Port::new(
            Location::new(0, 40),
            PortType::Input,
            BitWidth::ONE,
        ));
        
        // Add extra ports
        for i in 0..self.extra_ports {
            ports.push(Port::new(
                Location::new(0, 50 + i * 10),
                PortType::Input,
                BitWidth::ONE,
            ));
        }
        
        instance.set_ports(&ports);
    }

    /// Get or create memory state for an instance
    pub fn get_mem_state(&self, state: &InstanceState) -> Option<MemState> {
        // TODO: Implement proper instance data integration
        // This would retrieve MemState from the instance's data store
        None
    }

    /// Set memory state for an instance
    pub fn set_mem_state(&self, state: &mut InstanceState, mem_state: MemState) {
        // TODO: Implement proper instance data integration
        // This would store MemState in the instance's data store
    }

    /// Get the current file associated with an instance
    pub fn get_current_file(&self, instance_id: u32) -> Option<&PathBuf> {
        self.current_files.get(&instance_id)
    }

    /// Set the current file for an instance
    pub fn set_current_file(&mut self, instance_id: u32, file_path: PathBuf) {
        self.current_files.insert(instance_id, file_path);
    }

    /// Remove file association for an instance
    pub fn remove_current_file(&mut self, instance_id: u32) {
        self.current_files.remove(&instance_id);
    }
}

impl InstanceFactory for MemFactory {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_display_name(&self) -> String {
        self.description.get()
    }

    fn create_attribute_set(&self) -> AttributeSet {
        Self::create_attributes()
    }
}

/// Memory component bounds helper
pub struct MemBounds;

impl MemBounds {
    /// Calculate bounds for a memory component
    pub fn calculate(extra_ports: i32, has_label: bool) -> Bounds {
        let width = 60;
        let base_height = 60;
        let extra_height = extra_ports * 10;
        let label_height = if has_label { 20 } else { 0 };
        
        Bounds::new(0, 0, width, base_height + extra_height + label_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::StringGetter;

    #[test]
    fn test_enable_mode() {
        assert_eq!(EnableMode::ByteEnables, EnableMode::ByteEnables);
        assert_ne!(EnableMode::ByteEnables, EnableMode::LineEnables);
    }

    #[test]
    fn test_mem_factory_creation() {
        let factory = MemFactory::new(
            "TestMem".to_string(),
            StringGetter::new("Test Memory"),
            2,
            true,
        );
        
        assert_eq!(factory.name, "TestMem");
        assert_eq!(factory.extra_ports, 2);
        assert!(factory.needs_label);
    }

    #[test]
    fn test_mem_factory_file_management() {
        let mut factory = MemFactory::new(
            "TestMem".to_string(),
            StringGetter::new("Test Memory"),
            0,
            false,
        );
        
        let path = PathBuf::from("/test/path");
        factory.set_current_file(123, path.clone());
        
        assert_eq!(factory.get_current_file(123), Some(&path));
        
        factory.remove_current_file(123);
        assert_eq!(factory.get_current_file(123), None);
    }

    #[test]
    fn test_mem_bounds_calculation() {
        let bounds1 = MemBounds::calculate(0, false);
        assert_eq!(bounds1.get_width(), 60);
        assert_eq!(bounds1.get_height(), 60);
        
        let bounds2 = MemBounds::calculate(2, true);
        assert_eq!(bounds2.get_width(), 60);
        assert_eq!(bounds2.get_height(), 100); // 60 + 20 (extra ports) + 20 (label)
    }

    #[test]
    fn test_mem_attributes() {
        let attrs = MemFactory::create_attributes();
        
        // Should be able to create attributes without error
        // TODO: Add more specific tests when attribute system is complete
    }
}