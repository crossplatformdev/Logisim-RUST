//! VHDL Entity Component
//!
//! VHDL entity component implementation.
//! This module ports functionality from Java VhdlEntityComponent.

use crate::component::{Component, Pin, UpdateResult};
use crate::hdl::parsers::VhdlContentComponent;
use crate::{ComponentId, Timestamp};
use std::collections::HashMap;

/// VHDL Entity Component
/// 
/// Represents a VHDL entity as a component that can be instantiated in circuits.
/// Equivalent to Java VhdlEntityComponent.
#[derive(Debug, Clone)]
pub struct VhdlEntityComponent {
    id: ComponentId,
    content: VhdlContentComponent,
    pins: HashMap<String, Pin>,
}

impl VhdlEntityComponent {
    /// Create a new VHDL entity component
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            content: VhdlContentComponent::create(),
            pins: HashMap::new(),
        }
    }

    /// Get the VHDL content
    pub fn get_content(&self) -> &VhdlContentComponent {
        &self.content
    }

    /// Set the VHDL content
    pub fn set_content(&mut self, content: VhdlContentComponent) {
        self.content = content;
        self.update_pins_from_content();
    }

    /// Update pins based on VHDL content
    fn update_pins_from_content(&mut self) {
        self.pins.clear();

        // Create input pins
        for input in self.content.get_inputs() {
            let width = crate::BusWidth(if input.get_width_int() > 0 { input.get_width_int() as u32 } else { 1 });
            let pin = Pin::new_input(input.get_name(), width);
            self.pins.insert(input.get_name().to_string(), pin);
        }

        // Create output pins
        for output in self.content.get_outputs() {
            let width = crate::BusWidth(if output.get_width_int() > 0 { output.get_width_int() as u32 } else { 1 });
            let pin = Pin::new_output(output.get_name(), width);
            self.pins.insert(output.get_name().to_string(), pin);
        }
    }
}

impl Component for VhdlEntityComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "VHDL Entity"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // VHDL entities are handled externally by HDL simulation
        // For now, just return success with empty output changes
        UpdateResult::new()
    }

    fn reset(&mut self) {
        // Reset all output pins to unknown state
        for pin in self.pins.values_mut() {
            if pin.is_output() {
                pin.signal = crate::Signal::unknown(pin.width);
            }
        }
    }
}