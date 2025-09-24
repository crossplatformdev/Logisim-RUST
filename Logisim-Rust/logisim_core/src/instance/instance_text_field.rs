/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Text Field Support
//!
//! This module provides text field support for component labels and text elements.

use crate::data::{Attribute, Bounds, Location};

/// Text field alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center, 
    Right,
}

/// Vertical text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

/// Font information for text rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct FontInfo {
    pub family: String,
    pub size: u32,
    pub bold: bool,
    pub italic: bool,
}

impl Default for FontInfo {
    fn default() -> Self {
        Self {
            family: "Sans-serif".to_string(),
            size: 12,
            bold: false,
            italic: false,
        }
    }
}

/// Text field for component labels and text display.
///
/// This struct manages the display and editing of text associated with components,
/// particularly for labels and other textual elements.
#[derive(Debug, Clone)]
pub struct InstanceTextField {
    /// Text content
    text: String,
    /// Position of the text field
    location: Location,
    /// Text alignment
    align: TextAlign,
    /// Vertical alignment
    vertical_align: VerticalAlign,
    /// Font information
    font: FontInfo,
    /// Text attribute reference
    text_attr: Option<Attribute<String>>,
    /// Font attribute reference
    font_attr: Option<Attribute<FontInfo>>,
}

impl InstanceTextField {
    /// Creates a new text field.
    pub fn new(
        text: String,
        location: Location,
        align: TextAlign,
        vertical_align: VerticalAlign,
    ) -> Self {
        Self {
            text,
            location,
            align,
            vertical_align,
            font: FontInfo::default(),
            text_attr: None,
            font_attr: None,
        }
    }

    /// Creates a text field bound to attributes.
    pub fn from_attributes(
        text_attr: Attribute<String>,
        font_attr: Option<Attribute<FontInfo>>,
        location: Location,
        align: TextAlign,
        vertical_align: VerticalAlign,
    ) -> Self {
        Self {
            text: String::new(), // Will be filled from attributes
            location,
            align,
            vertical_align,
            font: FontInfo::default(),
            text_attr: Some(text_attr),
            font_attr,
        }
    }

    /// Gets the current text content.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the text content.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    /// Gets the text location.
    pub fn location(&self) -> Location {
        self.location
    }

    /// Sets the text location.
    pub fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    /// Gets the text alignment.
    pub fn align(&self) -> TextAlign {
        self.align
    }

    /// Sets the text alignment.
    pub fn set_align(&mut self, align: TextAlign) {
        self.align = align;
    }

    /// Gets the vertical alignment.
    pub fn vertical_align(&self) -> VerticalAlign {
        self.vertical_align
    }

    /// Sets the vertical alignment.
    pub fn set_vertical_align(&mut self, vertical_align: VerticalAlign) {
        self.vertical_align = vertical_align;
    }

    /// Gets the font information.
    pub fn font(&self) -> &FontInfo {
        &self.font
    }

    /// Sets the font information.
    pub fn set_font(&mut self, font: FontInfo) {
        self.font = font;
    }

    /// Updates the text field from component attributes.
    pub fn update_from_attributes(&mut self, attrs: &crate::data::AttributeSet) {
        if let Some(text_attr) = &self.text_attr {
            if let Some(text) = attrs.get_value(text_attr) {
                self.text = text.clone();
            }
        }

        if let Some(font_attr) = &self.font_attr {
            if let Some(font) = attrs.get_value(font_attr) {
                self.font = font.clone();
            }
        }
    }

    /// Calculates the bounding box for this text field.
    ///
    /// This is a simplified implementation that estimates bounds based on
    /// font size and text length. A full implementation would use actual
    /// text measurement APIs.
    pub fn get_bounds(&self) -> Bounds {
        let char_width = self.font.size as i32 * 6 / 10; // Rough estimate
        let char_height = self.font.size as i32;
        
        let text_width = self.text.len() as i32 * char_width;
        let text_height = char_height;

        let x = match self.align {
            TextAlign::Left => self.location.x(),
            TextAlign::Center => self.location.x() - text_width / 2,
            TextAlign::Right => self.location.x() - text_width,
        };

        let y = match self.vertical_align {
            VerticalAlign::Top => self.location.y(),
            VerticalAlign::Center => self.location.y() - text_height / 2,
            VerticalAlign::Bottom => self.location.y() - text_height,
        };

        Bounds::new(x, y, text_width, text_height)
    }

    /// Checks if a point is within the text field bounds.
    pub fn contains(&self, point: Location) -> bool {
        self.get_bounds().contains(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_field_creation() {
        let field = InstanceTextField::new(
            "Test Label".to_string(),
            Location::new(10, 20),
            TextAlign::Center,
            VerticalAlign::Center,
        );

        assert_eq!(field.text(), "Test Label");
        assert_eq!(field.location(), Location::new(10, 20));
        assert_eq!(field.align(), TextAlign::Center);
        assert_eq!(field.vertical_align(), VerticalAlign::Center);
    }

    #[test]
    fn test_text_field_modification() {
        let mut field = InstanceTextField::new(
            "Original".to_string(),
            Location::new(0, 0),
            TextAlign::Left,
            VerticalAlign::Top,
        );

        field.set_text("Modified".to_string());
        field.set_location(Location::new(5, 10));
        field.set_align(TextAlign::Right);
        field.set_vertical_align(VerticalAlign::Bottom);

        assert_eq!(field.text(), "Modified");
        assert_eq!(field.location(), Location::new(5, 10));
        assert_eq!(field.align(), TextAlign::Right);
        assert_eq!(field.vertical_align(), VerticalAlign::Bottom);
    }

    #[test]
    fn test_bounds_calculation() {
        let field = InstanceTextField::new(
            "ABC".to_string(), // 3 characters
            Location::new(50, 100),
            TextAlign::Left,
            VerticalAlign::Top,
        );

        let bounds = field.get_bounds();
        
        // With default font size 12, expect roughly 3 * 7 = 21 width, 12 height
        assert_eq!(bounds.x(), 50);
        assert_eq!(bounds.y(), 100);
        assert!(bounds.width() > 0);
        assert!(bounds.height() > 0);
    }

    #[test]
    fn test_alignment_affects_bounds() {
        let text = "Test".to_string();
        let location = Location::new(50, 100);

        let left_field = InstanceTextField::new(
            text.clone(), location, TextAlign::Left, VerticalAlign::Top
        );
        let center_field = InstanceTextField::new(
            text.clone(), location, TextAlign::Center, VerticalAlign::Top
        );
        let right_field = InstanceTextField::new(
            text, location, TextAlign::Right, VerticalAlign::Top
        );

        let left_bounds = left_field.get_bounds();
        let center_bounds = center_field.get_bounds();
        let right_bounds = right_field.get_bounds();

        // Left alignment: text starts at location
        assert_eq!(left_bounds.x(), 50);
        
        // Center alignment: text centered on location
        assert!(center_bounds.x() < 50);
        
        // Right alignment: text ends at location
        assert!(right_bounds.x() < center_bounds.x());
    }
}