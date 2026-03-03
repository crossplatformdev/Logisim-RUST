//! Application state: selected tool, active circuit, clipboard, etc.

use logisim_core::{
    component::{ComponentId, ComponentKind},
    project::Project,
    simulation::Simulator,
    value::BitWidth,
};
use std::path::PathBuf;

/// The currently selected interaction tool.
#[derive(Clone, PartialEq, Debug)]
pub enum Tool {
    /// Selection / pointer tool.
    Select,
    /// Wire drawing tool.
    Wire,
    /// Place a component (holds the kind to be placed).
    Place(ComponentKind),
    /// Poke tool (interact with inputs at run-time).
    Poke,
    /// Text/label tool.
    Text,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Select
    }
}

/// Global application state.
pub struct AppState {
    /// The open project.
    pub project: Project,
    /// The simulator.
    pub simulator: Simulator,
    /// Currently active circuit name.
    pub active_circuit: String,
    /// Currently selected tool.
    pub tool: Tool,
    /// Selected component IDs in the active circuit.
    pub selected: Vec<ComponentId>,
    /// Whether the simulation is running (continuous clock).
    pub running: bool,
    /// Path to the currently open file (if any).
    pub file_path: Option<PathBuf>,
    /// Whether there are unsaved changes.
    pub modified: bool,
    /// Canvas zoom level (1.0 = 100%).
    pub zoom: f32,
    /// Canvas pan offset (pixels).
    pub pan: egui::Vec2,
    /// Show the grid.
    pub show_grid: bool,
    /// Simulation speed (Hz).
    pub sim_hz: f32,
    /// Wire currently being drawn: start point in grid coords.
    pub wire_start: Option<(i32, i32)>,
    /// Status bar message.
    pub status: String,
    /// Clipboard (copied components).
    pub clipboard: Vec<logisim_core::component::Component>,
    /// Pending single-step request.
    pub step_requested: bool,
}

impl AppState {
    pub fn new() -> Self {
        let project = Project::new("Untitled");
        let simulator = Simulator::new(project.clone());
        AppState {
            project,
            simulator,
            active_circuit: String::new(),
            tool: Tool::Select,
            selected: Vec::new(),
            running: false,
            file_path: None,
            modified: false,
            zoom: 1.0,
            pan: egui::Vec2::ZERO,
            show_grid: true,
            sim_hz: 1.0,
            wire_start: None,
            status: "Ready".to_string(),
            clipboard: Vec::new(),
            step_requested: false,
        }
    }

    /// Grid size in screen pixels at current zoom.
    pub fn grid_px(&self) -> f32 {
        10.0 * self.zoom
    }

    /// Convert screen position to grid coordinates.
    pub fn screen_to_grid(&self, screen_pos: egui::Pos2, canvas_origin: egui::Pos2) -> (i32, i32) {
        let gx = ((screen_pos.x - canvas_origin.x - self.pan.x) / self.grid_px()).round() as i32;
        let gy = ((screen_pos.y - canvas_origin.y - self.pan.y) / self.grid_px()).round() as i32;
        (gx, gy)
    }

    /// Convert grid coordinates to screen position.
    pub fn grid_to_screen(&self, gx: i32, gy: i32, canvas_origin: egui::Pos2) -> egui::Pos2 {
        egui::Pos2::new(
            canvas_origin.x + self.pan.x + gx as f32 * self.grid_px(),
            canvas_origin.y + self.pan.y + gy as f32 * self.grid_px(),
        )
    }

    /// Sync the simulator with the current project state.
    pub fn sync_simulator(&mut self) {
        self.simulator = Simulator::new(self.project.clone());
    }
}
