/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Text Attributes Implementation
//!
//! Rust port of `com.cburch.logisim.std.base.TextAttributes`

use serde::{Deserialize, Serialize};

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center, 
    Right,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::Left
    }
}

/// Vertical text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Top
    }
}

/// Font style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontStyle {
    Plain,
    Bold,
    Italic,
    BoldItalic,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Plain
    }
}

/// Color representation (RGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
    
    /// Black color
    pub const BLACK: Color = Color::new(0, 0, 0);
    
    /// White color
    pub const WHITE: Color = Color::new(255, 255, 255);
    
    /// Red color
    pub const RED: Color = Color::new(255, 0, 0);
    
    /// Green color
    pub const GREEN: Color = Color::new(0, 255, 0);
    
    /// Blue color
    pub const BLUE: Color = Color::new(0, 0, 255);
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

/// Text attributes for text components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAttributes {
    /// The text content
    pub text: String,
    
    /// Font family name
    pub font_family: String,
    
    /// Font size in points
    pub font_size: u32,
    
    /// Font style
    pub font_style: FontStyle,
    
    /// Text color
    pub color: Color,
    
    /// Horizontal alignment
    pub horizontal_alignment: TextAlignment,
    
    /// Vertical alignment
    pub vertical_alignment: VerticalAlignment,
}

impl TextAttributes {
    /// Create new default text attributes
    pub fn new() -> Self {
        TextAttributes {
            text: String::new(),
            font_family: "SansSerif".to_string(),
            font_size: 12,
            font_style: FontStyle::Plain,
            color: Color::BLACK,
            horizontal_alignment: TextAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }
    
    /// Create text attributes with specified text
    pub fn with_text(text: String) -> Self {
        let mut attrs = Self::new();
        attrs.text = text;
        attrs
    }
    
    /// Set the text content
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }
    
    /// Set the font size
    pub fn set_font_size(&mut self, size: u32) -> &mut Self {
        self.font_size = size;
        self
    }
    
    /// Set the font style
    pub fn set_font_style(&mut self, style: FontStyle) -> &mut Self {
        self.font_style = style;
        self
    }
    
    /// Set the text color
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
    
    /// Set horizontal alignment
    pub fn set_horizontal_alignment(&mut self, alignment: TextAlignment) -> &mut Self {
        self.horizontal_alignment = alignment;
        self
    }
    
    /// Set vertical alignment
    pub fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) -> &mut Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl Default for TextAttributes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_attributes_creation() {
        let attrs = TextAttributes::new();
        assert_eq!(attrs.text, "");
        assert_eq!(attrs.font_family, "SansSerif");
        assert_eq!(attrs.font_size, 12);
        assert_eq!(attrs.font_style, FontStyle::Plain);
        assert_eq!(attrs.color, Color::BLACK);
        assert_eq!(attrs.horizontal_alignment, TextAlignment::Left);
        assert_eq!(attrs.vertical_alignment, VerticalAlignment::Top);
    }

    #[test]
    fn test_text_attributes_with_text() {
        let attrs = TextAttributes::with_text("Hello World".to_string());
        assert_eq!(attrs.text, "Hello World");
    }

    #[test]
    fn test_text_attributes_modification() {
        let mut attrs = TextAttributes::new();
        
        attrs.set_text("Test".to_string())
             .set_font_size(16)
             .set_font_style(FontStyle::Bold)
             .set_color(Color::RED)
             .set_horizontal_alignment(TextAlignment::Center)
             .set_vertical_alignment(VerticalAlignment::Middle);
        
        assert_eq!(attrs.text, "Test");
        assert_eq!(attrs.font_size, 16);
        assert_eq!(attrs.font_style, FontStyle::Bold);
        assert_eq!(attrs.color, Color::RED);
        assert_eq!(attrs.horizontal_alignment, TextAlignment::Center);
        assert_eq!(attrs.vertical_alignment, VerticalAlignment::Middle);
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::BLACK, Color::new(0, 0, 0));
        assert_eq!(Color::WHITE, Color::new(255, 255, 255));
        assert_eq!(Color::RED, Color::new(255, 0, 0));
        assert_eq!(Color::GREEN, Color::new(0, 255, 0));
        assert_eq!(Color::BLUE, Color::new(0, 0, 255));
    }
}