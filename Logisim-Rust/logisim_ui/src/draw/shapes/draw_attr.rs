//! Drawing attributes for shapes
//!
//! This module corresponds to the Java DrawAttr class.

use crate::draw::model::canvas_object::Color32;

/// Standard drawing attributes used by shapes
pub struct DrawAttr;

impl DrawAttr {
    /// Stroke width attribute name
    pub const STROKE_WIDTH: &'static str = "stroke_width";
    
    /// Stroke color attribute name  
    pub const STROKE_COLOR: &'static str = "stroke_color";
    
    /// Fill color attribute name
    pub const FILL_COLOR: &'static str = "fill_color";
    
    /// Font family attribute name
    pub const FONT_FAMILY: &'static str = "font_family";
    
    /// Font size attribute name
    pub const FONT_SIZE: &'static str = "font_size";
    
    /// Text alignment attribute name
    pub const TEXT_ALIGN: &'static str = "text_align";
    
    /// Default stroke width
    pub const DEFAULT_STROKE_WIDTH: i32 = 1;
    
    /// Default stroke color
    pub const DEFAULT_STROKE_COLOR: Color32 = Color32::BLACK;
    
    /// Default font family
    pub const DEFAULT_FONT_FAMILY: &'static str = "SansSerif";
    
    /// Default font size
    pub const DEFAULT_FONT_SIZE: i32 = 12;
    
    /// Default text alignment
    pub const DEFAULT_TEXT_ALIGN: TextAlign = TextAlign::Center;
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