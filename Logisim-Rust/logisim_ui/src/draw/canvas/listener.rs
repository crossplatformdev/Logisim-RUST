//! Canvas listener trait

/// Listener for canvas events
pub trait CanvasListener: Send + Sync {
    /// Called when the canvas repaints
    fn canvas_repainted(&mut self);
}