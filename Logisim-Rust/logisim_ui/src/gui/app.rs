//! Main application structure and entry point

use crate::{UiError, UiResult};
#[cfg(feature = "gui")]
use eframe::egui;
use logisim_core::{circ_format::CircIntegration, Simulation};
use std::path::PathBuf;

#[cfg(feature = "gui")]
use super::frame::MainFrame;

/// Main Logisim application struct - equivalent to the Java Frame class
pub struct LogisimApp {
    /// The main frame containing all UI elements
    #[cfg(feature = "gui")]
    main_frame: MainFrame,

    /// Currently loaded project file path
    current_file: Option<PathBuf>,

    /// Application title for the window
    title: String,

    /// Current simulation (headless mode)
    #[cfg(not(feature = "gui"))]
    simulation: Option<Simulation>,
}

impl LogisimApp {
    /// Create a new Logisim application
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "gui")]
            main_frame: MainFrame::new(),
            current_file: None,
            title: "Logisim-RUST".to_string(),
            #[cfg(not(feature = "gui"))]
            simulation: None,
        }
    }

    /// Load a circuit file
    pub fn load_circuit_file(&mut self, path: PathBuf) -> UiResult<()> {
        let simulation = CircIntegration::load_into_simulation(&path)
            .map_err(|e| UiError::FileError(format!("Failed to load circuit file: {}", e)))?;

        #[cfg(feature = "gui")]
        self.main_frame.set_simulation(simulation);

        #[cfg(not(feature = "gui"))]
        {
            self.simulation = Some(simulation);
        }

        self.current_file = Some(path.clone());

        // Update title to include filename
        let filename = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("Unknown");
        self.title = format!("Logisim-RUST - {}", filename);

        Ok(())
    }

    /// Get the current window title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Check if there are unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        // TODO: Track modification state
        false
    }

    /// Get the current simulation (for headless mode)
    #[cfg(not(feature = "gui"))]
    pub fn simulation(&self) -> Option<&Simulation> {
        self.simulation.as_ref()
    }
}

impl Default for LogisimApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "gui")]
impl eframe::App for LogisimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the main frame UI
        self.main_frame.update(ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // TODO: Save application state/preferences
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // TODO: Handle cleanup on exit
    }
}

/// Launch the Logisim application
#[cfg(feature = "gui")]
pub fn run_app() -> UiResult<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        // TODO: Add proper icon when IconData is available
        // .with_icon(
        //     eframe::IconData::default(),
        // ),
        ..Default::default()
    };

    let app = LogisimApp::new();

    eframe::run_native(
        &app.title(),
        options,
        Box::new(|_cc| {
            // Set up custom fonts if needed
            // let mut fonts = egui::FontDefinitions::default();
            // Load custom fonts here if needed
            // cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(app))
        }),
    )
    .map_err(|e| UiError::GuiInitError(e.to_string()))?;

    Ok(())
}

/// Launch the Logisim application with a circuit file
#[cfg(feature = "gui")]
pub fn run_app_with_file(file_path: PathBuf) -> UiResult<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let mut app = LogisimApp::new();
    app.load_circuit_file(file_path)?;

    eframe::run_native(
        &app.title(),
        options,
        Box::new(move |_cc| Ok(Box::new(app))),
    )
    .map_err(|e| UiError::GuiInitError(e.to_string()))?;

    Ok(())
}

/// Run in headless mode (for testing and non-GUI environments)
#[cfg(not(feature = "gui"))]
pub fn run_app() -> UiResult<()> {
    println!("Logisim-RUST running in headless mode");
    Ok(())
}

/// Run with a file in headless mode
#[cfg(not(feature = "gui"))]
pub fn run_app_with_file(file_path: PathBuf) -> UiResult<()> {
    let mut app = LogisimApp::new();
    app.load_circuit_file(file_path)?;

    println!("Loaded circuit file: {}", app.title());
    if let Some(sim) = app.simulation() {
        let stats = sim.netlist().stats();
        println!(
            "Circuit has {} nets and {} nodes",
            stats.net_count, stats.node_count
        );
    }

    Ok(())
}
