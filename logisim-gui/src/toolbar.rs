//! Toolbar: tool selection buttons.

use crate::state::{AppState, Tool};
use egui::Ui;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Toolbar
    }

    pub fn show(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            tool_button(ui, state, "⬆ Select", Tool::Select);
            tool_button(ui, state, "✏ Wire", Tool::Wire);
            tool_button(ui, state, "👆 Poke", Tool::Poke);
            tool_button(ui, state, "T Label", Tool::Text);

            ui.separator();

            let run_label = if state.running { "⏹ Stop" } else { "▶ Run" };
            if ui.button(run_label).clicked() {
                state.running = !state.running;
                state.status = if state.running {
                    "Simulation running".to_string()
                } else {
                    "Simulation stopped".to_string()
                };
            }

            if ui.button("⏭ Step").clicked() {
                state.step_requested = true;
                state.status = "Stepped".to_string();
            }

            ui.separator();

            if ui.button("🔍+ Zoom In").clicked() {
                state.zoom = (state.zoom * 1.25).min(8.0);
            }
            if ui.button("🔍- Zoom Out").clicked() {
                state.zoom = (state.zoom / 1.25).max(0.25);
            }
            if ui.button("1:1").clicked() {
                state.zoom = 1.0;
                state.pan = egui::Vec2::ZERO;
            }
        });
    }
}

fn tool_button(ui: &mut Ui, state: &mut AppState, label: &str, tool: Tool) {
    let active = state.tool == tool;
    if ui.selectable_label(active, label).clicked() {
        state.tool = tool;
        state.wire_start = None;
    }
}
