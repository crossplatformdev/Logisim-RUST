//! Shape implementations for drawable objects
//!
//! This module corresponds to the Java com.cburch.draw.shapes package.

pub mod draw_attr;
pub mod line;
pub mod rectangle;
pub mod oval;
pub mod text;
pub mod curve;
pub mod poly;
pub mod fillable;

// Utility modules
pub mod line_util;
pub mod poly_util;
pub mod curve_util;
pub mod svg;

// Re-export key types
pub use draw_attr::DrawAttr;
pub use line::Line;
pub use rectangle::Rectangle;
pub use oval::Oval;
pub use text::Text;
pub use curve::Curve;
pub use poly::Poly;
pub use fillable::FillableCanvasObject;