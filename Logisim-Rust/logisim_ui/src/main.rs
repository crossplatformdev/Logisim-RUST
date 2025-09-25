//! Main entry point for Logisim-RUST application
//!
//! This is the Rust equivalent of Java's Main.java class.
//! It handles application startup, theme setup, and command line parsing.

use logisim_core::build_info::BuildInfo;
use logisim_ui::{
    gui::startup::Startup,
    UiResult,
};
use std::env;

// GUI-specific imports
#[cfg(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos")))]
use logisim_core::prefs::AppPreferences;
// Import GUI app functions only if available on the platform
#[cfg(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos")))]
use logisim_ui::gui::app::{run_app, run_app_with_file};
// Fallback imports for unsupported platforms or non-GUI builds
#[cfg(not(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
use logisim_ui::gui::app::{run_app, run_app_with_file};

/// Application entry point - equivalent to Java Main.main()
fn main() -> UiResult<()> {
    // Set application name for the system (equivalent to apple.awt.application.name)
    if cfg!(target_os = "macos") {
        // On macOS, this would set the application name in the menu bar
        // For now, we'll just log it
        log::debug!("Setting application name: {}", BuildInfo::NAME);
    }

    // Initialize logging early
    env_logger::init();

    // Try to set up the look and feel (equivalent to Java's UIManager setup)
    setup_look_and_feel().unwrap_or_else(|e| {
        eprintln!("Failed to set up look and feel: {}", e);
    });

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let startup = Startup::parse_args(&args);

    // Handle startup result
    match startup {
        None => {
            eprintln!("Failed to parse command line arguments");
            std::process::exit(10);
        }
        Some(startup) => {
            if startup.should_quit() {
                std::process::exit(0);
            }

            // Run the application
            match startup.run() {
                Ok(()) => Ok(()),
                Err(e) => {
                    // Show error dialog if possible, otherwise print to stderr
                    eprintln!("Application error: {}", e);

                    // In Java version, this shows a dialog and exits with code 100
                    std::process::exit(100);
                }
            }
        }
    }
}

/// Set up look and feel themes - equivalent to Java's UIManager setup
#[cfg(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos")))]
fn setup_look_and_feel() -> Result<(), Box<dyn std::error::Error>> {
    // Check if we're in headless mode
    let prefs = logisim_core::prefs::get_preferences();
    let is_headless = {
        let prefs = prefs.lock().unwrap();
        prefs.is_headless()
    };

    if is_headless {
        log::debug!("Running in headless mode, skipping look and feel setup");
        return Ok(());
    }

    // In the Java version, this sets up FlatLaf themes:
    // - FlatLightLaf
    // - FlatDarkLaf
    // - FlatIntelliJLaf
    // - FlatDarculaLaf
    // - FlatMacLightLaf
    // - FlatMacDarkLaf

    // For the Rust version using egui, we'll just log the theme setup
    let prefs = prefs.lock().unwrap();
    log::info!("Setting up look and feel: {:?}", prefs.look_and_feel);

    // Set up font scaling if needed
    if prefs.font_scale.scale != 1.0 {
        log::info!("Font scaling enabled: {}x", prefs.font_scale.scale);
    }

    Ok(())
}

/// Headless version of look and feel setup
#[cfg(not(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
fn setup_look_and_feel() -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Running in headless mode, skipping look and feel setup");
    Ok(())
}

/// Check if the application has GUI capability
/// Equivalent to Java's Main.hasGui()
#[cfg(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn has_gui() -> bool {
    let prefs = logisim_core::prefs::get_preferences();
    let prefs = prefs.lock().unwrap();
    prefs.has_gui()
}

/// Headless version always returns false
#[cfg(not(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub fn has_gui() -> bool {
    false
}

/// Global headless flag - equivalent to Java's Main.headless
pub static mut HEADLESS_MODE: bool = false;

/// Set headless mode
#[cfg(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn set_headless(headless: bool) {
    unsafe {
        HEADLESS_MODE = headless;
    }

    // Also update preferences
    let prefs = logisim_core::prefs::get_preferences();
    let mut prefs = prefs.lock().unwrap();
    prefs.set_headless(headless);
}

/// Headless version - always in headless mode
#[cfg(not(all(feature = "gui", any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub fn set_headless(_headless: bool) {
    unsafe {
        HEADLESS_MODE = true;
    }
}

/// Check if running in headless mode
pub fn is_headless() -> bool {
    unsafe { HEADLESS_MODE }
}

/// Dirty marker constant - equivalent to Java's Main.DIRTY_MARKER
pub const DIRTY_MARKER: &str = logisim_core::build_info::constants::DIRTY_MARKER;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headless_mode() {
        set_headless(true);
        assert!(is_headless());
        assert!(!has_gui());

        set_headless(false);
        assert!(!is_headless());
    }

    #[test]
    fn test_dirty_marker() {
        assert_eq!(DIRTY_MARKER, "\u{1F4BE}");
    }
}
