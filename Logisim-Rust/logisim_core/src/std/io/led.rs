/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! LED (Light Emitting Diode) Component
//!
//! Implements an LED component that displays the state of digital signals.
//! Equivalent to the Java Led class.

use crate::{
    data::{AttributeSet, Bounds, Location},
    instance::{InstanceFactory, InstancePainter, InstanceState, Port},
};
use std::any::Any;

/// Unique identifier for the LED component
pub const LED_ID: &str = "LED";

/// LED component factory
#[derive(Debug)]
pub struct Led {
    ports: Vec<Port>,
}

impl Led {
    pub fn new() -> Self {
        Self {
            ports: vec![Port::new(0, 0, Port::INPUT, 1)],
        }
    }
}

impl Default for Led {
    fn default() -> Self {
        Self::new()
    }
}

impl InstanceFactory for Led {
    fn get_name(&self) -> &str {
        LED_ID
    }
    
    fn create_attribute_set(&self) -> AttributeSet {
        AttributeSet::new()
    }
    
    fn get_display_name(&self) -> String {
        "LED".to_string()
    }
    
    fn get_ports(&self) -> &[Port] {
        &self.ports
    }
    
    fn get_offset_bounds(&self, _attrs: &AttributeSet) -> Bounds {
        Bounds::create(-10, -10, 20, 20)
    }
    
    fn create_component(&self, _location: Location, _attrs: AttributeSet) -> Box<dyn Any> {
        Box::new(()) // Placeholder for component instance
    }
    
    fn paint_instance(&self, _painter: &mut InstancePainter) {
        // TODO: Implement LED rendering
    }
    
    fn propagate(&self, _state: &mut dyn InstanceState) {
        // TODO: Implement LED signal propagation
    }
}