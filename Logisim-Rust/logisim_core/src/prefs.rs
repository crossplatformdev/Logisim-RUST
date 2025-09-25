//! Application preferences and settings
//!
//! This module provides application preferences management similar to the Java AppPreferences class.
//! It handles user settings, look-and-feel preferences, locale settings, and other configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Look and feel theme options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LookAndFeel {
    /// System native look and feel
    #[default]
    System,
    /// Light theme
    Light,
    /// Dark theme  
    Dark,
    /// High contrast theme
    HighContrast,
}

impl LookAndFeel {
    /// Get all available look and feel options
    pub fn all() -> &'static [LookAndFeel] {
        &[
            LookAndFeel::System,
            LookAndFeel::Light,
            LookAndFeel::Dark,
            LookAndFeel::HighContrast,
        ]
    }

    /// Get display name for the look and feel
    pub fn display_name(&self) -> &'static str {
        match self {
            LookAndFeel::System => "System",
            LookAndFeel::Light => "Light",
            LookAndFeel::Dark => "Dark",
            LookAndFeel::HighContrast => "High Contrast",
        }
    }
}

/// Font scaling preferences
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FontScale {
    /// Font scale factor (1.0 = normal)
    pub scale: f32,
}

impl Default for FontScale {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

impl FontScale {
    /// Create new font scale
    pub fn new(scale: f32) -> Self {
        Self {
            scale: scale.clamp(0.5, 3.0), // Clamp to reasonable range
        }
    }

    /// Get scaled font size
    pub fn scale_size(&self, base_size: f32) -> f32 {
        base_size * self.scale
    }

    /// Get scaled integer font size
    pub fn scale_size_int(&self, base_size: i32) -> i32 {
        (base_size as f32 * self.scale).round() as i32
    }
}

/// Application preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPreferences {
    /// Look and feel theme
    pub look_and_feel: LookAndFeel,

    /// Font scaling
    pub font_scale: FontScale,

    /// Locale/language setting
    pub locale: String,

    /// Recently opened files
    pub recent_files: Vec<PathBuf>,

    /// Maximum number of recent files to track
    pub max_recent_files: usize,

    /// Window geometry and state
    pub window_geometry: Option<WindowGeometry>,

    /// Custom key bindings
    pub key_bindings: HashMap<String, String>,

    /// Headless mode flag
    pub headless: bool,

    /// Enable high DPI scaling
    pub high_dpi: bool,

    /// Custom preferences for extensions
    pub custom_prefs: HashMap<String, String>,
}

/// Window geometry and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            look_and_feel: LookAndFeel::default(),
            font_scale: FontScale::default(),
            locale: "en".to_string(),
            recent_files: Vec::new(),
            max_recent_files: 10,
            window_geometry: None,
            key_bindings: HashMap::new(),
            headless: false,
            high_dpi: true,
            custom_prefs: HashMap::new(),
        }
    }
}

impl AppPreferences {
    /// Create new preferences with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Load preferences from system
    pub fn load() -> Result<Self, PreferencesError> {
        // In a full implementation, this would load from:
        // - Windows: Registry
        // - macOS: ~/Library/Preferences
        // - Linux: ~/.config or XDG config dirs

        // For now, return defaults with a warning
        log::warn!("Preferences loading not implemented, using defaults");
        Ok(Self::default())
    }

    /// Save preferences to system
    pub fn save(&self) -> Result<(), PreferencesError> {
        // In a full implementation, this would save to system-specific locations
        log::warn!("Preferences saving not implemented");
        Ok(())
    }

    /// Add a file to recent files list
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);

        // Add to front
        self.recent_files.insert(0, path);

        // Trim to max size
        self.recent_files.truncate(self.max_recent_files);
    }

    /// Remove a file from recent files list
    pub fn remove_recent_file(&mut self, path: &PathBuf) {
        self.recent_files.retain(|p| p != path);
    }

    /// Clear recent files list
    pub fn clear_recent_files(&mut self) {
        self.recent_files.clear();
    }

    /// Get scaled font size
    pub fn scaled_font_size(&self, base_size: i32) -> i32 {
        self.font_scale.scale_size_int(base_size)
    }

    /// Set custom preference
    pub fn set_custom(&mut self, key: String, value: String) {
        self.custom_prefs.insert(key, value);
    }

    /// Get custom preference
    pub fn get_custom(&self, key: &str) -> Option<&String> {
        self.custom_prefs.get(key)
    }

    /// Check if GUI is available (not headless)
    pub fn has_gui(&self) -> bool {
        !self.headless && !cfg!(target_os = "linux") || std::env::var("DISPLAY").is_ok()
    }

    /// Check if we're running in headless mode
    pub fn is_headless(&self) -> bool {
        self.headless
    }

    /// Set headless mode
    pub fn set_headless(&mut self, headless: bool) {
        self.headless = headless;
    }
}

/// Global preferences instance
static GLOBAL_PREFS: once_cell::sync::Lazy<Arc<Mutex<AppPreferences>>> =
    once_cell::sync::Lazy::new(|| {
        let prefs = AppPreferences::load().unwrap_or_default();
        Arc::new(Mutex::new(prefs))
    });

/// Get global preferences instance
pub fn get_preferences() -> Arc<Mutex<AppPreferences>> {
    GLOBAL_PREFS.clone()
}

/// Preferences error types
#[derive(Debug, thiserror::Error)]
pub enum PreferencesError {
    #[error("Failed to load preferences: {0}")]
    LoadError(String),

    #[error("Failed to save preferences: {0}")]
    SaveError(String),

    #[error("Invalid preference value: {0}")]
    InvalidValue(String),

    #[error("Preference not found: {0}")]
    NotFound(String),
}

/// Utility functions matching Java AppPreferences static methods
impl AppPreferences {
    /// Get scaled size based on current font scale
    pub fn get_scaled(base_size: i32) -> i32 {
        let prefs = get_preferences();
        let prefs = prefs.lock().unwrap();
        prefs.scaled_font_size(base_size)
    }

    /// Check if high DPI scaling is enabled
    pub fn is_high_dpi() -> bool {
        let prefs = get_preferences();
        let prefs = prefs.lock().unwrap();
        prefs.high_dpi
    }

    /// Get current locale
    pub fn get_locale() -> String {
        let prefs = get_preferences();
        let prefs = prefs.lock().unwrap();
        prefs.locale.clone()
    }

    /// Set locale
    pub fn set_locale(locale: String) {
        let prefs = get_preferences();
        let mut prefs = prefs.lock().unwrap();
        prefs.locale = locale;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_feel() {
        let laf = LookAndFeel::Dark;
        assert_eq!(laf.display_name(), "Dark");
        assert!(LookAndFeel::all().contains(&laf));
    }

    #[test]
    fn test_font_scale() {
        let scale = FontScale::new(1.5);
        assert_eq!(scale.scale_size_int(12), 18);
        assert_eq!(scale.scale_size(12.0), 18.0);
    }

    #[test]
    fn test_preferences() {
        let mut prefs = AppPreferences::new();

        // Test recent files
        let path = PathBuf::from("test.circ");
        prefs.add_recent_file(path.clone());
        assert_eq!(prefs.recent_files.len(), 1);
        assert_eq!(prefs.recent_files[0], path);

        prefs.remove_recent_file(&path);
        assert_eq!(prefs.recent_files.len(), 0);

        // Test custom preferences
        prefs.set_custom("test_key".to_string(), "test_value".to_string());
        assert_eq!(
            prefs.get_custom("test_key"),
            Some(&"test_value".to_string())
        );
    }

    #[test]
    fn test_headless_mode() {
        let mut prefs = AppPreferences::new();
        assert!(!prefs.is_headless());

        prefs.set_headless(true);
        assert!(prefs.is_headless());
        assert!(!prefs.has_gui());
    }
}
