//! Canvas tool trait

use crate::draw::model::DrawingContext;

/// Trait for canvas tools
pub trait CanvasTool: Send + Sync {
    /// Get the tool name
    fn name(&self) -> &str;
    
    /// Paint tool-specific graphics
    fn paint(&self, g: &mut dyn DrawingContext);
}