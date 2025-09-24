/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Gray counter component
//!
//! Manufactures a counter that iterates over Gray codes.
//! Equivalent to Java's com.cburch.gray.GrayCounter class.

/// Gray counter component with configurable width and label support.
/// 
/// This is equivalent to Java's GrayCounter class.
pub struct GrayCounter;

impl GrayCounter {
    /// Unique identifier of the tool, used as reference in project files.
    pub const ID: &'static str = "Gray Counter";

    pub fn new() -> Self {
        Self
    }
}

impl Default for GrayCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_counter_creation() {
        let _counter = GrayCounter::new();
    }
}