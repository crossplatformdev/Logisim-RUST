//! Main entry point for Logisim-RUST application
//!
//! This binary provides both GUI and headless modes depending on compile-time features.

use logisim_ui::{
    gui::app::{run_app, run_app_with_file},
    UiResult,
};
use std::env;

fn main() -> UiResult<()> {
    // Initialize logging
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments - run the main application
            run_app()
        }
        2 => {
            // One argument - treat as circuit file path
            let file_path = std::path::PathBuf::from(&args[1]);
            run_app_with_file(file_path)
        }
        _ => {
            eprintln!("Usage: {} [circuit_file.circ]", args[0]);
            eprintln!();
            eprintln!("Arguments:");
            eprintln!("  circuit_file.circ  Optional circuit file to load");
            eprintln!();
            eprintln!("Environment:");
            #[cfg(feature = "gui")]
            eprintln!("  GUI mode enabled - requires display server");
            #[cfg(not(feature = "gui"))]
            eprintln!("  Headless mode - no GUI dependencies");

            std::process::exit(1);
        }
    }
}
