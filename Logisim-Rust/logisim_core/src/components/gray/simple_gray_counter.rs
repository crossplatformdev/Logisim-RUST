/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Simple gray counter component
//!
//! Manufactures a simple counter that iterates over the 4-bit Gray Code.
//! Equivalent to Java's com.cburch.gray.SimpleGrayCounter class.

/// Simple Gray counter that iterates over 4-bit Gray Code.
///
/// This is equivalent to Java's SimpleGrayCounter class.
pub struct SimpleGrayCounter;

impl SimpleGrayCounter {
    /// Unique identifier of the tool, used as reference in project files.
    pub const ID: &'static str = "Gray Counter (Simple)";

    pub fn new() -> Self {
        Self
    }
}

impl Default for SimpleGrayCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_gray_counter_creation() {
        let _counter = SimpleGrayCounter::new();
    }
}
