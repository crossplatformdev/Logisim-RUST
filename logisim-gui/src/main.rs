//! Logisim-RUST GUI Application
//!
//! Entry point for the graphical user interface built with egui/eframe.

mod app;
mod canvas;
mod component_panel;
mod dialogs;
mod state;
mod toolbar;

use app::LogisimApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Logisim-RUST")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Logisim-RUST",
        native_options,
        Box::new(|cc| Ok(Box::new(LogisimApp::new(cc)))),
    )
}
