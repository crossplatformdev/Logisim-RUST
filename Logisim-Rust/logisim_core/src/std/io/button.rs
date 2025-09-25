/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Push Button Component
//!
//! Implements a clickable button component that generates digital signals
//! when pressed. Equivalent to the Java Button class.

use crate::{
    data::{AttributeSet, Bounds, Location},
    instance::{InstanceFactory, InstancePainter, InstanceState, Port},
};
use std::any::Any;

/// Unique identifier for the Button component
pub const BUTTON_ID: &str = "Button";

/// Button component factory
#[derive(Debug)]
pub struct Button {
    ports: Vec<Port>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            ports: vec![Port::new(20, 0, Port::OUTPUT, 1)],
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl InstanceFactory for Button {
    fn get_name(&self) -> &str {
        BUTTON_ID
    }
    
    fn create_attribute_set(&self) -> AttributeSet {
        AttributeSet::new()
    }
    
    fn get_display_name(&self) -> String {
        "Button".to_string()
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
        // TODO: Implement Button rendering
    }
    
    fn propagate(&self, _state: &mut dyn InstanceState) {
        // TODO: Implement Button signal propagation
    }
}