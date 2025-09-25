/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Slider Component
//!
//! Rust port of `com.cburch.logisim.std.io.extra.Slider`
//!
//! A slider component that allows variable value input with visual position control.

use crate::{
    component::{Component, ComponentId},
    data::{Attribute, BitWidth, Bounds, Direction, Location, Value},
    signal::Signal,
    util::StringGetter,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum number of bits supported by slider
pub const MAXIMUM_NUMBER_OF_BITS: usize = 8;
/// Maximum slider position value
pub const MAXIMUM_SLIDER_POSITION: i32 = (1 << MAXIMUM_NUMBER_OF_BITS) - 1;

/// Slider component state data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SliderData {
    /// Number of bits for output value
    pub bit_width: usize,
    /// Current position of slider (0 to MAXIMUM_SLIDER_POSITION)
    pub position: i32,
    /// Direction: true for right-to-left, false for left-to-right
    pub right_to_left: bool,
}

impl SliderData {
    /// Create new slider data with default values
    pub fn new() -> Self {
        Self {
            bit_width: MAXIMUM_NUMBER_OF_BITS,
            position: 0,
            right_to_left: false,
        }
    }

    /// Get the current output value based on position and bit width
    pub fn get_current_value(&self) -> i32 {
        let complete_value = if self.right_to_left {
            MAXIMUM_SLIDER_POSITION - self.position
        } else {
            self.position
        };
        complete_value >> (MAXIMUM_NUMBER_OF_BITS - self.bit_width)
    }

    /// Set the slider position (clamped to valid range)
    pub fn set_position(&mut self, position: i32) {
        self.position = position.max(0).min(MAXIMUM_SLIDER_POSITION);
    }

    /// Set the bit width (clamped to valid range)
    pub fn set_bit_width(&mut self, width: usize) {
        if width > 0 && width <= MAXIMUM_NUMBER_OF_BITS {
            self.bit_width = width;
        }
    }

    /// Set the direction
    pub fn set_direction(&mut self, right_to_left: bool) {
        if right_to_left != self.right_to_left {
            self.right_to_left = right_to_left;
            // Invert position when direction changes
            self.position = MAXIMUM_SLIDER_POSITION - self.position;
        }
    }
}

impl Default for SliderData {
    fn default() -> Self {
        Self::new()
    }
}

/// Slider component implementation
///
/// A variable value input component with visual position control.
/// Users can drag the slider to set output values.
#[derive(Debug, Clone)]
pub struct Slider {
    /// Component identifier
    id: ComponentId,
    /// Current slider state
    data: SliderData,
    /// Component attributes
    attributes: HashMap<String, Attribute>,
}

impl Slider {
    /// Create a new slider component
    pub fn new(id: ComponentId) -> Self {
        let mut attributes = HashMap::new();
        
        // Initialize default attributes
        attributes.insert(
            "facing".to_string(),
            Attribute::Direction(Direction::East),
        );
        attributes.insert(
            "width".to_string(),
            Attribute::BitWidth(BitWidth::new(MAXIMUM_NUMBER_OF_BITS)),
        );
        attributes.insert(
            "label".to_string(),
            Attribute::String("".to_string()),
        );
        attributes.insert(
            "direction".to_string(),
            Attribute::String("left_to_right".to_string()),
        );

        Self {
            id,
            data: SliderData::new(),
            attributes,
        }
    }

    /// Get the current slider data
    pub fn get_data(&self) -> &SliderData {
        &self.data
    }

    /// Get mutable reference to slider data
    pub fn get_data_mut(&mut self) -> &mut SliderData {
        &mut self.data
    }

    /// Handle mouse drag to change slider position
    pub fn handle_mouse_drag(&mut self, location: Location, bounds: Bounds) {
        let relative_x = location.x - bounds.x - 10; // Account for slider margin
        let position = relative_x.max(0).min(MAXIMUM_SLIDER_POSITION);
        self.data.set_position(position);
    }

    /// Get the component's display name
    pub fn display_name() -> StringGetter {
        StringGetter::new("Slider")
    }

    /// Get the component's factory ID
    pub fn factory_id() -> &'static str {
        "Slider"
    }
}

impl Component for Slider {
    fn get_id(&self) -> ComponentId {
        self.id
    }

    fn get_type_name(&self) -> &'static str {
        "Slider"
    }

    fn get_bounds(&self) -> Bounds {
        // Slider bounds: 275x30 pixels (wide component)
        let facing = self.get_attribute("facing")
            .and_then(|attr| attr.as_direction())
            .unwrap_or(Direction::East);
            
        let width = MAXIMUM_SLIDER_POSITION + 20;
        let height = 30;
        
        match facing {
            Direction::East => Bounds::new(-width, -height / 2, width, height),
            Direction::West => Bounds::new(0, -height / 2, width, height),
            Direction::North => Bounds::new(-width / 2, 0, width, height),
            Direction::South => Bounds::new(-width / 2, -height, width, height),
        }
    }

    fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    fn set_attribute(&mut self, name: String, value: Attribute) {
        if name == "width" {
            if let Some(bit_width) = value.as_bit_width() {
                self.data.set_bit_width(bit_width.width());
            }
        } else if name == "direction" {
            if let Some(dir_str) = value.as_string() {
                let right_to_left = dir_str == "right_to_left";
                self.data.set_direction(right_to_left);
            }
        }
        self.attributes.insert(name, value);
    }

    fn get_input_count(&self) -> usize {
        0
    }

    fn get_output_count(&self) -> usize {
        1
    }

    fn propagate(&mut self, _inputs: &[Signal]) -> Vec<Signal> {
        let bit_width = self.get_attribute("width")
            .and_then(|attr| attr.as_bit_width())
            .unwrap_or(BitWidth::new(MAXIMUM_NUMBER_OF_BITS));

        let value = Value::known(self.data.get_current_value() as u64, bit_width);
        vec![Signal::new(value)]
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn clone_component(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_creation() {
        let slider = Slider::new(ComponentId::new(1));
        assert_eq!(slider.get_id(), ComponentId::new(1));
        assert_eq!(slider.get_type_name(), "Slider");
        assert!(slider.is_interactive());
        assert_eq!(slider.get_input_count(), 0);
        assert_eq!(slider.get_output_count(), 1);
    }

    #[test]
    fn test_slider_data() {
        let mut data = SliderData::new();
        assert_eq!(data.bit_width, MAXIMUM_NUMBER_OF_BITS);
        assert_eq!(data.position, 0);
        assert!(!data.right_to_left);

        // Test position setting
        data.set_position(100);
        assert_eq!(data.position, 100);

        // Test clamping
        data.set_position(-10);
        assert_eq!(data.position, 0);
        data.set_position(1000);
        assert_eq!(data.position, MAXIMUM_SLIDER_POSITION);
    }

    #[test]
    fn test_slider_value_calculation() {
        let mut data = SliderData::new();
        data.set_position(255); // Maximum position for 8-bit
        assert_eq!(data.get_current_value(), 255);

        // Test with 4-bit width
        data.set_bit_width(4);
        data.set_position(240); // Should give 15 for 4-bit
        assert_eq!(data.get_current_value(), 15);
    }

    #[test]
    fn test_slider_direction() {
        let mut data = SliderData::new();
        data.set_position(100);
        let original_value = data.get_current_value();

        // Change direction - position should invert
        data.set_direction(true);
        assert!(data.right_to_left);
        assert_eq!(data.position, MAXIMUM_SLIDER_POSITION - 100);

        // Value calculation should account for direction
        let new_value = data.get_current_value();
        assert_ne!(original_value, new_value);
    }

    #[test]
    fn test_slider_propagation() {
        let mut slider = Slider::new(ComponentId::new(1));
        slider.get_data_mut().set_position(128);

        let outputs = slider.propagate(&[]);
        assert_eq!(outputs.len(), 1);
        assert!(!outputs[0].value.is_unknown());
    }

    #[test]
    fn test_slider_bounds() {
        let slider = Slider::new(ComponentId::new(1));
        let bounds = slider.get_bounds();
        
        // Should be wide component
        assert!(bounds.width > 200);
        assert_eq!(bounds.height, 30);
    }
}