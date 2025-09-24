//! Core data structures for Logisim
//! 
//! This module contains the fundamental data types used throughout Logisim,
//! including geometric types, attributes, and value representations.

pub mod location;
pub mod direction;
pub mod bounds;
pub mod bit_width;

// Re-export commonly used types
pub use location::*;
pub use direction::*;
pub use bounds::*;
pub use bit_width::*;