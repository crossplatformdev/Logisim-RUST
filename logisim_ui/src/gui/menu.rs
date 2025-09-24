//! Menu bar implementation

use eframe::egui::{self, Ui};

/// Main menu bar
pub struct MenuBar {
    /// Show about dialog flag
    show_about: bool,
}

impl MenuBar {
    /// Create a new menu bar
    pub fn new() -> Self {
        Self {
            show_about: false,
        }
    }
    
    /// Show the menu bar UI
    pub fn show(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            // File menu
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    // TODO: New file
                    ui.close_menu();
                }
                if ui.button("Open...").clicked() {
                    // TODO: Open file dialog
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Save").clicked() {
                    // TODO: Save file
                    ui.close_menu();
                }
                if ui.button("Save As...").clicked() {
                    // TODO: Save as dialog
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Export Image...").clicked() {
                    // TODO: Export image
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    // TODO: Exit application
                    ui.close_menu();
                }
            });
            
            // Edit menu
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    // TODO: Undo
                    ui.close_menu();
                }
                if ui.button("Redo").clicked() {
                    // TODO: Redo
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Cut").clicked() {
                    // TODO: Cut
                    ui.close_menu();
                }
                if ui.button("Copy").clicked() {
                    // TODO: Copy
                    ui.close_menu();
                }
                if ui.button("Paste").clicked() {
                    // TODO: Paste
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Select All").clicked() {
                    // TODO: Select all
                    ui.close_menu();
                }
                if ui.button("Delete").clicked() {
                    // TODO: Delete selected
                    ui.close_menu();
                }
            });
            
            // Project menu
            ui.menu_button("Project", |ui| {
                if ui.button("Add Circuit...").clicked() {
                    // TODO: Add circuit
                    ui.close_menu();
                }
                if ui.button("Load Library...").clicked() {
                    // TODO: Load library
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Project Options...").clicked() {
                    // TODO: Project options
                    ui.close_menu();
                }
            });
            
            // Simulate menu
            ui.menu_button("Simulate", |ui| {
                if ui.button("Reset Simulation").clicked() {
                    // TODO: Reset simulation
                    ui.close_menu();
                }
                if ui.button("Step Simulation").clicked() {
                    // TODO: Step simulation
                    ui.close_menu();
                }
                if ui.button("Start/Stop Simulation").clicked() {
                    // TODO: Toggle simulation
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Enable Ticks").clicked() {
                    // TODO: Enable ticks
                    ui.close_menu();
                }
                if ui.button("Tick Once").clicked() {
                    // TODO: Tick once
                    ui.close_menu();
                }
            });
            
            // Window menu
            ui.menu_button("Window", |ui| {
                if ui.button("Minimize").clicked() {
                    // TODO: Minimize window
                    ui.close_menu();
                }
                if ui.button("Zoom In").clicked() {
                    // TODO: Zoom in
                    ui.close_menu();
                }
                if ui.button("Zoom Out").clicked() {
                    // TODO: Zoom out
                    ui.close_menu();
                }
                if ui.button("Zoom to Fit").clicked() {
                    // TODO: Zoom to fit
                    ui.close_menu();
                }
            });
            
            // Help menu
            ui.menu_button("Help", |ui| {
                if ui.button("User's Guide").clicked() {
                    // TODO: Open user guide
                    ui.close_menu();
                }
                if ui.button("Library Reference").clicked() {
                    // TODO: Open library reference
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("About").clicked() {
                    self.show_about = true;
                    ui.close_menu();
                }
            });
        });
        
        // Show about dialog if requested
        if self.show_about {
            self.show_about_dialog(ui.ctx());
        }
    }
    
    /// Show the about dialog
    fn show_about_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("About Logisim-RUST")
            .open(&mut self.show_about)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Logisim-RUST");
                    ui.label("Digital Logic Designer and Simulator");
                    ui.label("");
                    ui.label("A Rust port of Logisim-Evolution");
                    ui.label("Maintaining full compatibility with .circ files");
                    ui.label("");
                    ui.label("© 2024 Logisim-RUST Contributors");
                    ui.label("Licensed under GPL-3.0-or-later");
                });
            });
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}