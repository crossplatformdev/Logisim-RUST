//! Main chronogram panel - equivalent to ChronoPanel.java
//!
//! This module provides the main chronogram user interface panel that
//! coordinates between the timeline, signal list, and waveform display.

use crate::gui::chronogram::{
    constants::*,
    model::{ChronogramModel, SignalInfo},
    timeline::Timeline,
    waveform::{Waveform, WaveformColors},
};
use logisim_core::{signal::Timestamp, Simulation};
use egui::{Color32, Pos2, Rect, ScrollArea, Ui, Vec2};
use std::collections::HashMap;

/// Main chronogram panel containing all UI elements
#[derive(Debug)]
pub struct ChronogramPanel {
    /// The chronogram data model
    model: ChronogramModel,
    /// Timeline component for time navigation
    timeline: Timeline,
    /// Individual waveform renderers for each signal
    waveforms: HashMap<String, Waveform>,
    /// Current split position between signal names and waveforms
    split_position: f32,
    /// Whether the panel is currently recording simulation data
    recording: bool,
    /// Colors for the chronogram display
    colors: WaveformColors,
    /// Scroll position for the signal list
    scroll_position: Vec2,
    /// Whether to show the signal selection dialog
    show_signal_selection: bool,
}

impl Default for ChronogramPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ChronogramPanel {
    /// Create a new chronogram panel
    pub fn new() -> Self {
        Self {
            model: ChronogramModel::new(),
            timeline: Timeline::new(),
            waveforms: HashMap::new(),
            split_position: INITIAL_SPLIT,
            recording: false,
            colors: WaveformColors::default(),
            scroll_position: Vec2::ZERO,
            show_signal_selection: false,
        }
    }
    
    /// Start recording simulation data
    pub fn start_recording(&mut self, simulation: &Simulation) {
        self.recording = true;
        self.model.clear();
        
        // Setup signals from simulation netlist
        self.setup_signals_from_simulation(simulation);
        
        // Register callback for signal changes (we can't actually do this due to borrowing issues,
        // but we'll simulate the updates in update_from_simulation)
        // simulation.add_signal_callback(Box::new(move |node_id, time, signal| {
        //     // This would require self to be accessible, which creates borrowing issues
        // }));
    }
    
    /// Stop recording simulation data
    pub fn stop_recording(&mut self) {
        self.recording = false;
    }
    
    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.recording
    }
    
    /// Update with simulation data (called during simulation)
    pub fn update_from_simulation(&mut self, simulation: &Simulation) {
        if !self.recording {
            return;
        }
        
        let current_time = simulation.current_time();
        
        // TODO: Extract current signal values from simulation
        // This would involve getting the current state of all tracked nodes
        // For now, we'll use placeholder logic
        self.record_simulation_state(simulation, current_time);
    }
    
    /// Render the chronogram panel
    pub fn render(&mut self, ui: &mut Ui) {
        let available_rect = ui.available_rect_before_wrap();
        
        // Top toolbar
        self.render_toolbar(ui);
        
        let remaining_rect = ui.available_rect_before_wrap();
        
        // Split panel between signal names (left) and waveforms (right)
        ui.allocate_ui_at_rect(remaining_rect, |ui| {
            ui.horizontal(|ui| {
                // Left panel - signal names and controls
                let left_width = self.split_position;
                let left_rect = Rect::from_min_size(
                    remaining_rect.min,
                    Vec2::new(left_width, remaining_rect.height()),
                );
                self.render_signal_list(ui, left_rect);
                
                // Splitter (draggable divider)
                ui.separator();
                
                // Right panel - timeline and waveforms
                let right_rect = Rect::from_min_size(
                    Pos2::new(remaining_rect.min.x + left_width + 5.0, remaining_rect.min.y),
                    Vec2::new(remaining_rect.width() - left_width - 5.0, remaining_rect.height()),
                );
                self.render_waveform_area(ui, right_rect);
            });
        });
        
        // Signal selection dialog
        if self.show_signal_selection {
            self.render_signal_selection_dialog(ui);
        }
    }
    
    /// Render the toolbar with controls
    fn render_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Recording controls
            if self.recording {
                if ui.button("⏹ Stop").clicked() {
                    self.stop_recording();
                }
                ui.label("Recording...");
            } else {
                if ui.button("⏺ Record").clicked() {
                    // TODO: Start recording - would need access to simulation
                }
            }
            
            ui.separator();
            
            // Signal selection
            if ui.button("📊 Add/Remove Signals").clicked() {
                self.show_signal_selection = true;
            }
            
            ui.separator();
            
            // Timeline controls
            if ui.button("🔍+").clicked() {
                self.timeline.set_zoom(self.timeline.zoom() * 1.2);
            }
            if ui.button("🔍-").clicked() {
                self.timeline.set_zoom(self.timeline.zoom() / 1.2);
            }
            
            ui.separator();
            
            // Export controls
            if ui.button("💾 Export").clicked() {
                // TODO: Implement export functionality
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(cursor_time) = self.timeline.cursor_time() {
                    ui.label(format!("Time: {}", cursor_time.as_u64()));
                }
            });
        });
    }
    
    /// Render the signal list on the left side
    fn render_signal_list(&mut self, ui: &mut Ui, rect: Rect) {
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical(|ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.label("Signal");
                    ui.separator();
                    ui.label("Value");
                });
                ui.separator();
                
                // Signal list
                ScrollArea::vertical()
                    .id_source("signal_list")
                    .show(ui, |ui| {
                        for (i, signal_info) in self.model.signals().iter().enumerate() {
                            self.render_signal_row(ui, i, signal_info);
                        }
                    });
            });
        });
    }
    
    /// Render a single signal row
    fn render_signal_row(&mut self, ui: &mut Ui, index: usize, signal_info: &SignalInfo) {
        let row_height = SIGNAL_HEIGHT;
        let row_rect = Rect::from_min_size(
            ui.cursor().min,
            Vec2::new(ui.available_width(), row_height),
        );
        
        let response = ui.allocate_rect(row_rect, egui::Sense::click());
        
        // Background color based on selection
        let bg_color = if signal_info.selected {
            Color32::from_rgba_unmultiplied(200, 200, 255, 100)
        } else if index % 2 == 0 {
            Color32::from_rgba_unmultiplied(240, 240, 240, 100)
        } else {
            Color32::TRANSPARENT
        };
        
        if bg_color != Color32::TRANSPARENT {
            ui.painter().rect_filled(row_rect, 0.0, bg_color);
        }
        
        ui.allocate_ui_at_rect(row_rect, |ui| {
            ui.horizontal(|ui| {
                // Signal name
                ui.set_min_width(100.0);
                ui.label(&signal_info.name);
                
                ui.separator();
                
                // Current value
                if let Some(signal_data) = self.model.get_signal_data(signal_info.id) {
                    let cursor_time = self.timeline.cursor_time().unwrap_or(self.model.end_time());
                    if let Some(current_signal) = signal_data.get_value_at(cursor_time) {
                        let waveform = Waveform::new();
                        let value_text = waveform.format_signal_value(current_signal);
                        ui.label(&value_text);
                    } else {
                        ui.label("-");
                    }
                } else {
                    ui.label("-");
                }
            });
        });
        
        // Handle clicks
        if response.clicked() {
            // Toggle selection
            let mut model = std::mem::replace(&mut self.model, ChronogramModel::new());
            model.set_signal_selected(signal_info.id, !signal_info.selected);
            
            // Update waveform selection
            if let Some(waveform) = self.waveforms.get_mut(&signal_info.name) {
                waveform.set_selected(!signal_info.selected);
            }
            
            self.model = model;
        }
    }
    
    /// Render the waveform area on the right side
    fn render_waveform_area(&mut self, ui: &mut Ui, rect: Rect) {
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical(|ui| {
                // Timeline header
                let header_rect = Rect::from_min_size(
                    rect.min,
                    Vec2::new(rect.width(), HEADER_HEIGHT),
                );
                self.timeline.set_width(rect.width());
                let timeline_response = self.timeline.render_header(ui, header_rect);
                
                // Handle timeline interactions
                if let Some(hover_pos) = timeline_response.hover_pos() {
                    if ui.input(|i| i.scroll_delta().y != 0.0) {
                        let scroll_delta = ui.input(|i| i.scroll_delta().y);
                        self.timeline.handle_zoom(scroll_delta, hover_pos.x - header_rect.left());
                    }
                }
                
                // Waveform area
                let waveform_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x, rect.min.y + HEADER_HEIGHT),
                    Vec2::new(rect.width(), rect.height() - HEADER_HEIGHT),
                );
                
                ScrollArea::vertical()
                    .id_source("waveforms")
                    .show_viewport(ui, |ui, viewport| {
                        let mut y_offset = 0.0;
                        
                        for signal_info in self.model.signals() {
                            let signal_rect = Rect::from_min_size(
                                Pos2::new(waveform_rect.min.x, waveform_rect.min.y + y_offset),
                                Vec2::new(waveform_rect.width(), SIGNAL_HEIGHT),
                            );
                            
                            // Only render if visible
                            if signal_rect.intersects(viewport) {
                                if let Some(signal_data) = self.model.get_signal_data(signal_info.id) {
                                    let waveform = self.waveforms
                                        .entry(signal_info.name.clone())
                                        .or_insert_with(|| {
                                            let mut wf = Waveform::new();
                                            wf.set_selected(signal_info.selected);
                                            wf.set_colors(self.colors.clone());
                                            wf
                                        });
                                    
                                    waveform.render(ui, signal_rect, signal_data, &self.timeline);
                                }
                            }
                            
                            y_offset += SIGNAL_HEIGHT;
                        }
                        
                        // Set content size for scrolling
                        let content_height = self.model.signal_count() as f32 * SIGNAL_HEIGHT;
                        ui.allocate_space(Vec2::new(waveform_rect.width(), content_height));
                    });
            });
        });
    }
    
    /// Render signal selection dialog
    fn render_signal_selection_dialog(&mut self, ui: &mut Ui) {
        egui::Window::new("Signal Selection")
            .resizable(true)
            .default_width(400.0)
            .default_height(300.0)
            .show(ui.ctx(), |ui| {
                ui.label("Select signals to display in the chronogram:");
                
                ui.separator();
                
                // TODO: Show available signals from the simulation
                // For now, show a placeholder
                ui.label("No signals available. Load a simulation first.");
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        self.show_signal_selection = false;
                    }
                });
            });
    }
    
    /// Setup default signals from simulation (placeholder implementation)
    fn setup_default_signals(&mut self, _simulation: &Simulation) {
        // TODO: This should examine the simulation's netlist and create
        // SignalInfo objects for nodes that should be tracked
        // For now, we'll create some placeholder signals
        
        use logisim_core::netlist::NodeId;
        use logisim_core::signal::BusWidth;
        
        // Add a system clock signal (required by Java implementation)
        let sysclk_info = SignalInfo::new(
            NodeId(0),
            "sysclk".to_string(),
            BusWidth(1),
            0,
        );
        self.model.add_signal(sysclk_info);
    }
    
    /// Setup signals from simulation netlist
    fn setup_signals_from_simulation(&mut self, simulation: &Simulation) {
        let node_ids = simulation.get_all_node_ids();
        
        for (index, node_id) in node_ids.iter().enumerate() {
            // Create a signal info for each node
            // In a real implementation, we'd want to get the actual name and width
            // from the netlist, but for now we'll generate placeholder names
            let name = if index == 0 {
                "sysclk".to_string() // Required system clock
            } else {
                format!("node_{}", node_id.as_u64())
            };
            
            let width = BusWidth(1); // Default to single bit for now
            
            let info = SignalInfo::new(*node_id, name, width, index);
            self.model.add_signal(info);
        }
        
        // If no nodes exist, add a default system clock
        if node_ids.is_empty() {
            self.setup_default_signals(simulation);
        }
    }
    
    /// Record current simulation state (enhanced implementation)
    fn record_simulation_state(&mut self, simulation: &Simulation, time: Timestamp) {
        // Extract current values of all tracked signals from the simulation
        for signal_info in self.model.signals() {
            if let Some(current_signal) = simulation.get_node_signal(signal_info.id) {
                self.model.record_signal_change(signal_info.id, time, current_signal);
            }
        }
        
        // If no real signals, add a placeholder clock signal for demo purposes
        if self.model.signal_count() == 1 && self.model.signals()[0].name == "sysclk" {
            use logisim_core::{netlist::NodeId, signal::{Signal, Value}};
            
            // Generate a simple clock signal
            let clock_value = if (time.as_u64() / 10) % 2 == 0 {
                Value::Low
            } else {
                Value::High
            };
            let clock_signal = Signal::new_single(clock_value);
            self.model.record_signal_change(NodeId(0), time, clock_signal);
        }
    }
    
    /// Get the chronogram model (read-only access)
    pub fn model(&self) -> &ChronogramModel {
        &self.model
    }
    
    /// Get the timeline (read-only access)
    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }
    
    /// Export chronogram data to text format
    pub fn export_to_text(&self) -> String {
        let mut output = String::new();
        
        // Header
        output.push_str("Logisim-RUST Chronogram Export\n");
        output.push_str("==============================\n\n");
        
        // Time range
        output.push_str(&format!("Time Range: {} - {}\n", 
            self.model.start_time().as_u64(), 
            self.model.end_time().as_u64()
        ));
        output.push_str(&format!("Signals: {}\n\n", self.model.signal_count()));
        
        // Signal data
        for signal_info in self.model.signals() {
            output.push_str(&format!("Signal: {} (width: {})\n", 
                signal_info.name, 
                signal_info.width.as_u32()
            ));
            
            if let Some(signal_data) = self.model.get_signal_data(signal_info.id) {
                for (time, signal) in signal_data.iter() {
                    let waveform = Waveform::new();
                    let value = waveform.format_signal_value(signal);
                    output.push_str(&format!("  {} = {}\n", time.as_u64(), value));
                }
            }
            output.push('\n');
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logisim_core::{netlist::NodeId, signal::BusWidth};

    #[test]
    fn test_chronogram_panel_creation() {
        let panel = ChronogramPanel::new();
        assert!(!panel.is_recording());
        assert_eq!(panel.model().signal_count(), 0);
    }

    #[test]
    fn test_recording_control() {
        let mut panel = ChronogramPanel::new();
        let simulation = Simulation::new();
        
        panel.start_recording(&simulation);
        assert!(panel.is_recording());
        
        panel.stop_recording();
        assert!(!panel.is_recording());
    }

    #[test]
    fn test_export_to_text() {
        let panel = ChronogramPanel::new();
        let output = panel.export_to_text();
        
        assert!(output.contains("Logisim-RUST Chronogram Export"));
        assert!(output.contains("Time Range:"));
        assert!(output.contains("Signals: 0"));
    }
}