/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Text Component Implementation
//!
//! Rust port of `com.cburch.logisim.std.base.Text`

use crate::comp::{Component, ComponentId, Pin, UpdateResult};
use crate::signal::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Text component for circuit annotations
///
/// A text component allows users to add text annotations to their circuits
/// for documentation purposes. It has no electrical properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    text: String,
    font_size: u32,
    // Color and other attributes would be added here
}

impl Text {
    /// Unique identifier for the text component
    pub const ID: &'static str = "Text";

    /// Create a new text component
    pub fn new(id: ComponentId) -> Self {
        Text {
            id,
            pins: HashMap::new(), // Text has no pins
            text: String::new(),
            font_size: 12,
        }
    }

    /// Create a new text component with specified text
    pub fn new_with_text(id: ComponentId, text: String) -> Self {
        Text {
            id,
            pins: HashMap::new(),
            text,
            font_size: 12,
        }
    }

    /// Get the text content
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Set the text content
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    /// Get the font size
    pub fn get_font_size(&self) -> u32 {
        self.font_size
    }

    /// Set the font size
    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size;
    }
}

impl Component for Text {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Text"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Text components don't have electrical behavior
        UpdateResult::new()
    }

    fn reset(&mut self) {
        // Text components don't need resetting
    }

    fn propagation_delay(&self) -> u64 {
        0 // Text has no propagation delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let text = Text::new(ComponentId(1));
        assert_eq!(text.id(), ComponentId(1));
        assert_eq!(text.name(), "Text");
        assert_eq!(text.get_text(), "");
        assert_eq!(text.get_font_size(), 12);
        assert_eq!(text.pins().len(), 0); // No pins
    }

    #[test]
    fn test_text_creation_with_content() {
        let text = Text::new_with_text(ComponentId(1), "Hello World".to_string());
        assert_eq!(text.get_text(), "Hello World");
    }

    #[test]
    fn test_text_modification() {
        let mut text = Text::new(ComponentId(1));

        text.set_text("Test Text".to_string());
        assert_eq!(text.get_text(), "Test Text");

        text.set_font_size(16);
        assert_eq!(text.get_font_size(), 16);
    }

    #[test]
    fn test_text_update() {
        let mut text = Text::new(ComponentId(1));
        let result = text.update(Timestamp(0));

        // Text components should not produce any outputs
        assert_eq!(result.get_outputs().len(), 0);
        assert_eq!(result.get_delay(), 0);
    }
}
