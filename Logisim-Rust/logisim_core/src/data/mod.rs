//! Core data structures for Logisim
//!
//! This module contains the fundamental data types used throughout Logisim,
//! including geometric types, attributes, and value representations.

pub mod attributes;
pub mod bit_width;
pub mod bounds;
pub mod direction;
pub mod location;

// Re-export commonly used types
pub use attributes::*;
pub use bit_width::*;
pub use bounds::*;
pub use direction::*;
pub use location::*;
