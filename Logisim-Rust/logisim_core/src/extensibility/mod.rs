//! Extensibility and plugin system
//! 
//! This module provides the extensibility framework for Logisim-RUST, including
//! dynamic component registration, plugin interfaces, and extension points.
//!
//! **API Stability: UNSTABLE** - These APIs are subject to change in future versions.

pub mod registry;
pub mod extension_points;
pub mod example_plugin;

pub use registry::*;
pub use extension_points::*;
pub use example_plugin::*;