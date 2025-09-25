/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Random component
//!
//! This module implements random number generator functionality equivalent to Random.java.

/// Random component (placeholder for now)
pub struct Random {
    // TODO: Implement Random component
}

impl Random {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_placeholder() {
        let _random = Random::new();
        // TODO: Add real tests when Random is implemented
    }
}