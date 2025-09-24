//! GUI module containing all user interface components

pub mod app;
pub mod frame;

#[cfg(feature = "gui")]
pub mod canvas;
#[cfg(feature = "gui")]
pub mod chronogram;
#[cfg(feature = "gui")]
pub mod menu;
#[cfg(feature = "gui")]
pub mod project_explorer;
#[cfg(feature = "gui")]
pub mod toolbox;

// Always available modules (non-GUI dependent)
pub mod edit_handler;
pub mod selection;

#[cfg(test)]
pub mod tests;

// Common GUI utilities and types
#[cfg(feature = "gui")]
pub mod common {
    use egui::{Color32, Stroke};

    // Color constants matching the Java implementation
    pub const HALO_COLOR: Color32 = Color32::from_rgb(255, 0, 255);
    pub const DEFAULT_ERROR_COLOR: Color32 = Color32::from_rgb(192, 0, 0);
    pub const TICK_RATE_COLOR: Color32 = Color32::from_rgba_unmultiplied(0, 0, 92, 92);
    pub const SINGLE_STEP_MSG_COLOR: Color32 = Color32::BLUE;

    // Grid and canvas constants
    pub const GRID_SIZE: f32 = 10.0;
    pub const CANVAS_MARGIN: f32 = 50.0;

    // Zoom constants
    pub const MIN_ZOOM: f32 = 0.25;
    pub const MAX_ZOOM: f32 = 4.0;
    pub const DEFAULT_ZOOM: f32 = 1.0;
    pub const ZOOM_STEP: f32 = 1.2;

    /// Snap coordinate to grid
    pub fn snap_to_grid(coord: f32) -> f32 {
        if coord < 0.0 {
            -((-coord + GRID_SIZE / 2.0) / GRID_SIZE).floor() * GRID_SIZE
        } else {
            ((coord + GRID_SIZE / 2.0) / GRID_SIZE).floor() * GRID_SIZE
        }
    }

    /// Default stroke for drawing components
    pub fn default_stroke() -> Stroke {
        Stroke::new(1.0, Color32::BLACK)
    }

    /// Selected stroke for highlighting
    pub fn selected_stroke() -> Stroke {
        Stroke::new(2.0, HALO_COLOR)
    }
}

// Common non-GUI utilities
pub mod core_common {
    // Grid and canvas constants (non-GUI)
    pub const GRID_SIZE: f32 = 10.0;
    pub const CANVAS_MARGIN: f32 = 50.0;

    // Zoom constants
    pub const MIN_ZOOM: f32 = 0.25;
    pub const MAX_ZOOM: f32 = 4.0;
    pub const DEFAULT_ZOOM: f32 = 1.0;
    pub const ZOOM_STEP: f32 = 1.2;

    /// Snap coordinate to grid
    pub fn snap_to_grid(coord: f32) -> f32 {
        if coord < 0.0 {
            -((-coord + GRID_SIZE / 2.0) / GRID_SIZE).floor() * GRID_SIZE
        } else {
            ((coord + GRID_SIZE / 2.0) / GRID_SIZE).floor() * GRID_SIZE
        }
    }
}
