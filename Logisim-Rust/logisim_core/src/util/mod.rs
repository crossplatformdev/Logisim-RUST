//! Utility modules for Logisim core functionality
//!
//! This module contains utility functions, data structures, and helpers
//! that are used throughout the Logisim core system.

pub mod cache;
pub mod collection_util;
pub mod file_util;
pub mod locale_manager;
pub mod string_util;

// Re-export commonly used utilities
pub use cache::*;
pub use collection_util::*;
pub use file_util::*;
pub use locale_manager::*;
pub use string_util::*;
