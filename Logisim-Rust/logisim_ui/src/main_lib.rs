//! Main application functionality
//!
//! This module contains the main application functions that would be in main.rs
//! but are needed as library functions for the startup system.

/// Check if the application has GUI capability
/// Equivalent to Java's Main.hasGui()
pub fn has_gui() -> bool {
    let prefs = logisim_core::prefs::get_preferences();
    let prefs = prefs.lock().unwrap();
    prefs.has_gui()
}

/// Global headless flag - equivalent to Java's Main.headless
pub static mut HEADLESS_MODE: bool = false;

/// Set headless mode
pub fn set_headless(headless: bool) {
    unsafe {
        HEADLESS_MODE = headless;
    }

    // Also update preferences
    let prefs = logisim_core::prefs::get_preferences();
    let mut prefs = prefs.lock().unwrap();
    prefs.set_headless(headless);
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
