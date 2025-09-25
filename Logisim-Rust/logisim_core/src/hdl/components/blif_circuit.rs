//! BLIF Circuit Component
//!
//! BLIF circuit component implementation.
//! This module ports functionality from Java BlifCircuitComponent.

use crate::comp::{Component, ComponentId, Pin, UpdateResult};
use crate::hdl::parsers::BlifContentComponent;
use crate::{Timestamp};
use std::collections::HashMap;

/// BLIF Circuit Component
/// 
/// Represents a BLIF circuit as a component that can be instantiated in circuits.
/// Equivalent to Java BlifCircuitComponent.
#[derive(Debug, Clone)]
pub struct BlifCircuitComponent {
    id: ComponentId,
    content: BlifContentComponent,
    pins: HashMap<String, Pin>,
}

impl BlifCircuitComponent {
    /// Create a new BLIF circuit component
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            content: BlifContentComponent::create(),
            pins: HashMap::new(),
        }
    }

    /// Get the BLIF content
    pub fn get_content(&self) -> &BlifContentComponent {
        &self.content
    }

    /// Set the BLIF content
    pub fn set_content(&mut self, content: BlifContentComponent) {
        self.content = content;
        self.update_pins_from_content();
    }

    /// Update pins based on BLIF content
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

impl Component for BlifCircuitComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "BLIF Circuit"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // BLIF circuits are handled externally by HDL simulation
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