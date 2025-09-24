//! Main application frame - equivalent to the Java Frame class

#[cfg(feature = "gui")]
use eframe::egui::{self, CentralPanel, Context, ScrollArea, SidePanel, TopBottomPanel};
use logisim_core::Simulation;

#[cfg(feature = "gui")]
use super::{canvas::Canvas, chronogram::ChronogramPanel, menu::MenuBar, project_explorer::ProjectExplorer, toolbox::Toolbox};

/// Main application frame containing all UI components
pub struct MainFrame {
    /// The main canvas for schematic editing
    #[cfg(feature = "gui")]
    canvas: Canvas,

    /// Component toolbox
    #[cfg(feature = "gui")]
    toolbox: Toolbox,

    /// Main menu bar
    #[cfg(feature = "gui")]
    menu_bar: MenuBar,

    /// Project explorer showing circuit hierarchy
    #[cfg(feature = "gui")]
    project_explorer: ProjectExplorer,

    /// Chronogram panel for timing diagram display
    #[cfg(feature = "gui")]
    chronogram_panel: ChronogramPanel,

    /// Current simulation instance
    simulation: Option<Simulation>,

    /// Selected tab in the left panel (toolbox vs explorer)
    #[cfg(feature = "gui")]
    left_tab_selected: LeftTab,

    /// Whether to show the chronogram window
    #[cfg(feature = "gui")]
    show_chronogram: bool,

    /// Zoom level
    #[allow(dead_code)] // Used only with GUI feature
    zoom_level: f32,

    /// Grid visibility
    #[allow(dead_code)] // Used only with GUI feature
    show_grid: bool,
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone, Copy, PartialEq)]
enum LeftTab {
    Toolbox,
    Explorer,
}

impl MainFrame {
    /// Create a new main frame
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "gui")]
            canvas: Canvas::new(),
            #[cfg(feature = "gui")]
            toolbox: Toolbox::new(),
            #[cfg(feature = "gui")]
            menu_bar: MenuBar::new(),
            #[cfg(feature = "gui")]
            project_explorer: ProjectExplorer::new(),
            #[cfg(feature = "gui")]
            chronogram_panel: ChronogramPanel::new(),
            simulation: None,
            #[cfg(feature = "gui")]
            left_tab_selected: LeftTab::Toolbox,
            #[cfg(feature = "gui")]
            show_chronogram: false,
            zoom_level: 1.0,
            show_grid: true,
        }
    }

    /// Set the current simulation
    pub fn set_simulation(&mut self, simulation: Simulation) {
        #[cfg(feature = "gui")]
        {
            self.canvas.set_simulation(&simulation);
            self.project_explorer.set_simulation(&simulation);
            // Initialize chronogram with simulation if it's being shown
            if self.show_chronogram {
                self.chronogram_panel.start_recording(&simulation);
            }
        }
        self.simulation = Some(simulation);
    }

    /// Update the main frame UI
    #[cfg(feature = "gui")]
    pub fn update(&mut self, ctx: &Context) {
        // Top menu bar
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu_bar.show(ui);
            
            // Check if chronogram should be shown (triggered from menu)
            if ui.button("📊 Chronogram").clicked() {
                self.show_chronogram = !self.show_chronogram;
                
                // Start/stop recording based on chronogram visibility
                if let Some(simulation) = &self.simulation {
                    if self.show_chronogram {
                        self.chronogram_panel.start_recording(simulation);
                    } else {
                        self.chronogram_panel.stop_recording();
                    }
                }
            }
        });

        // Show chronogram window if requested
        if self.show_chronogram {
            egui::Window::new("Chronogram")
                .resizable(true)
                .default_width(800.0)
                .default_height(400.0)
                .open(&mut self.show_chronogram)
                .show(ctx, |ui| {
                    self.chronogram_panel.render(ui);
                });
        }

        // Left side panel with toolbox and explorer
        SidePanel::left("left_panel")
            .default_width(250.0)
            .min_width(200.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                self.show_left_panel(ui);
            });

        // Bottom panel for attributes and zoom controls
        TopBottomPanel::bottom("bottom_panel")
            .default_height(150.0)
            .min_height(100.0)
            .max_height(300.0)
            .show(ctx, |ui| {
                self.show_bottom_panel(ui);
            });

        // Central canvas area
        CentralPanel::default().show(ctx, |ui| {
            self.canvas.show(ui, self.zoom_level, self.show_grid);
        });

        // Update chronogram with current simulation data if recording
        if self.show_chronogram && self.chronogram_panel.is_recording() {
            if let Some(simulation) = &self.simulation {
                self.chronogram_panel.update_from_simulation(simulation);
            }
        }
    }

    /// Show the left panel with tabs for toolbox and explorer
    #[cfg(feature = "gui")]
    fn show_left_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.left_tab_selected, LeftTab::Toolbox, "Toolbox");
            ui.selectable_value(&mut self.left_tab_selected, LeftTab::Explorer, "Explorer");
        });

        ui.separator();

        ScrollArea::vertical().show(ui, |ui| match self.left_tab_selected {
            LeftTab::Toolbox => {
                self.toolbox.show(ui);
            }
            LeftTab::Explorer => {
                self.project_explorer.show(ui);
            }
        });
    }

    /// Show the bottom panel with attributes and controls
    #[cfg(feature = "gui")]
    fn show_bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Zoom controls
            ui.label("Zoom:");
            if ui.button("−").clicked() {
                self.zoom_level = (self.zoom_level / 1.2).max(0.1);
            }
            ui.label(format!("{:.0}%", self.zoom_level * 100.0));
            if ui.button("+").clicked() {
                self.zoom_level = (self.zoom_level * 1.2).min(4.0);
            }

            ui.separator();

            // Grid toggle
            ui.checkbox(&mut self.show_grid, "Show Grid");

            ui.separator();

            // Simulation controls
            if let Some(_simulation) = &self.simulation {
                if ui.button("Reset").clicked() {
                    // TODO: Reset simulation
                }
                if ui.button("Step").clicked() {
                    // TODO: Step simulation
                }
                if ui.button("Run/Stop").clicked() {
                    // TODO: Toggle simulation running
                }
            }
        });

        ui.separator();

        // Attributes table area
        ScrollArea::vertical().show(ui, |ui| {
            ui.label("Component Attributes");
            // TODO: Show selected component attributes
            ui.label("No component selected");
        });
    }

    /// Get current simulation (for headless access)
    pub fn simulation(&self) -> Option<&Simulation> {
        self.simulation.as_ref()
    }
}

impl Default for MainFrame {
    fn default() -> Self {
        Self::new()
    }
}
