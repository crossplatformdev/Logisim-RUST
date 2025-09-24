//! Chronogram (waveform/timing view) module for displaying signal states over time.
//!
//! This module provides the chronogram functionality equivalent to the Java implementation,
//! allowing users to visualize signal changes and timing behavior in digital circuits.

pub mod model;
pub mod panel;
pub mod timeline;
pub mod waveform;

pub use model::{ChronogramModel, SignalData, SignalInfo};
pub use panel::ChronogramPanel;
pub use timeline::Timeline;
pub use waveform::Waveform;

/// Configuration constants for chronogram display
pub mod constants {
    /// Height of each signal waveform in pixels
    pub const SIGNAL_HEIGHT: f32 = 30.0;

    /// Height of the timeline header in pixels
    pub const HEADER_HEIGHT: f32 = 20.0;

    /// Gap between signal traces
    pub const GAP: f32 = 2.0;

    /// Initial split pane position
    pub const INITIAL_SPLIT: f32 = 150.0;

    /// Height of waveform area
    pub const WAVE_HEIGHT: f32 = SIGNAL_HEIGHT;

    /// Extra space for timeline
    pub const EXTRA_SPACE: f32 = 40.0;

    /// Cursor gap in pixels
    pub const CURSOR_GAP: f32 = 20.0;

    /// Timeline spacing
    pub const TIMELINE_SPACING: f32 = 80.0;

    /// Default tick width for time scaling
    pub const DEFAULT_TICK_WIDTH: f32 = 10.0;
}
