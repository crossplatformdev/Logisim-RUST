//! Drawing attributes for shapes
//!
//! This module corresponds to the Java DrawAttr class.

use logisim_core::data::Attribute;

/// Standard drawing attributes used by shapes
pub struct DrawAttr;

impl DrawAttr {
    /// Stroke width attribute
    pub fn stroke_width() -> &'static Attribute<i32> {
        static STROKE_WIDTH: Attribute<i32> = Attribute::new("stroke_width", 1);
        &STROKE_WIDTH
    }
    
    /// Stroke color attribute  
    pub fn stroke_color() -> &'static Attribute<egui::Color32> {
        static STROKE_COLOR: Attribute<egui::Color32> = Attribute::new("stroke_color", egui::Color32::BLACK);
        &STROKE_COLOR
    }
    
    /// Fill color attribute
    pub fn fill_color() -> &'static Attribute<Option<egui::Color32>> {
        static FILL_COLOR: Attribute<Option<egui::Color32>> = Attribute::new("fill_color", None);
        &FILL_COLOR
    }
    
    /// Font family attribute
    pub fn font_family() -> &'static Attribute<String> {
        static FONT_FAMILY: Attribute<String> = Attribute::new("font_family", "SansSerif".to_string());
        &FONT_FAMILY
    }
    
    /// Font size attribute
    pub fn font_size() -> &'static Attribute<i32> {
        static FONT_SIZE: Attribute<i32> = Attribute::new("font_size", 12);
        &FONT_SIZE
    }
    
    /// Text alignment attribute
    pub fn text_align() -> &'static Attribute<TextAlign> {
        static TEXT_ALIGN: Attribute<TextAlign> = Attribute::new("text_align", TextAlign::Center);
        &TEXT_ALIGN
    }
}

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Center
    }
}