//! Core data structures for Logisim
//! 
//! This module contains the fundamental data types used throughout Logisim,
//! including geometric types, attributes, and value representations.

pub mod location;
pub mod direction;

// Re-export commonly used types
pub use location::*;
pub use direction::*;