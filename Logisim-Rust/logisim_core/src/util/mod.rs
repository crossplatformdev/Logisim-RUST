//! Utility modules for Logisim core functionality
//! 
//! This module contains utility functions, data structures, and helpers
//! that are used throughout the Logisim core system.

pub mod string_util;
pub mod collection_util;
pub mod cache;
pub mod file_util;
pub mod locale_manager;

// Re-export commonly used utilities
pub use string_util::*;
pub use collection_util::*;
pub use cache::*;
pub use file_util::*;
pub use locale_manager::*;