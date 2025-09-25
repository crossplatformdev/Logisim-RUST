//! Utility modules for Logisim core functionality
//! 
//! This module contains utility functions, data structures, and helpers
//! that are used throughout the Logisim core system.

pub mod string_util;
pub mod collection_util;
pub mod event_support;
pub mod xml_util;
pub mod cache;
pub mod dag;

// Re-export commonly used utilities
pub use string_util::*;
pub use collection_util::*;