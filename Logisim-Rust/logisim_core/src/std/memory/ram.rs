/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! RAM (Random Access Memory) component
//!
//! This module implements RAM functionality equivalent to Ram.java.
//! RAM provides read/write memory storage.

/// RAM component (placeholder for now)
pub struct Ram {
    // TODO: Implement RAM component
}

impl Ram {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_placeholder() {
        let _ram = Ram::new();
        // TODO: Add real tests when RAM is implemented
    }
}