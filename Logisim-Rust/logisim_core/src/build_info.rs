//! Build information and application constants
//!
//! This module provides build-time information and application constants,
//! similar to the Java BuildInfo generated class.

/// Application build information
pub struct BuildInfo;

impl BuildInfo {
    /// Application name
    pub const NAME: &'static str = "Logisim-RUST";

    /// Application version
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Build timestamp
    pub const BUILD_TIME: &'static str = "compile-time";

    /// Git commit hash if available
    pub const GIT_HASH: Option<&'static str> = option_env!("GIT_HASH");

    /// Whether this is a debug build
    pub const DEBUG: bool = cfg!(debug_assertions);

    /// Target architecture
    pub const TARGET: &'static str = "unknown";

    /// Application display name
    pub fn display_name() -> String {
        format!("{} v{}", Self::NAME, Self::VERSION)
    }

    /// Full version string including build info
    pub fn full_version() -> String {
        let mut version = format!("{} v{}", Self::NAME, Self::VERSION);

        if let Some(git) = Self::GIT_HASH {
            version.push_str(&format!(" ({})", &git[..8.min(git.len())]));
        }

        if Self::DEBUG {
            version.push_str(" [DEBUG]");
        }

        version
    }

    /// User agent string for network requests
    pub fn user_agent() -> String {
        format!("{}/{} ({})", Self::NAME, Self::VERSION, Self::TARGET)
    }
}

/// Application constants matching Java implementation
pub mod constants {
    /// Unicode floppy disk character used as dirty marker
    pub const DIRTY_MARKER: &str = "\u{1F4BE}";

    /// Default window title
    pub const DEFAULT_TITLE: &str = "Logisim-RUST";

    /// Application organization name for preferences
    pub const ORG_NAME: &str = "logisim-rust";

    /// Application name for preferences
    pub const APP_NAME: &str = "logisim-rust";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_info() {
        assert!(!BuildInfo::NAME.is_empty());
        assert!(!BuildInfo::VERSION.is_empty());
        assert!(!BuildInfo::display_name().is_empty());
        assert!(!BuildInfo::full_version().is_empty());
        assert!(!BuildInfo::user_agent().is_empty());
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::DIRTY_MARKER, "\u{1F4BE}");
        assert!(!constants::DEFAULT_TITLE.is_empty());
    }
}
