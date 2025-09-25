/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Input/Output Components
//!
//! Rust port of `com.cburch.logisim.std.io` package containing
//! input/output components for interfacing with users.

pub mod extra;

// Re-export extra IO components
pub use extra::*;

use crate::data::{Attribute, AttributeSet};
use crate::comp::Color;

/// Default background color (transparent white)
pub const DEFAULT_BACKGROUND: Color = Color::new(255, 255, 255, 0);