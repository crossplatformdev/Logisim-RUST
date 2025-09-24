/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Counter poker for user interaction
//!
//! When the user clicks a counter using the Poke Tool, a CounterPoker object is created,
//! and that object will handle all user events.
//! Equivalent to Java's com.cburch.gray.CounterPoker class.

/// Counter poker for handling user interaction with Gray counters.
/// 
/// This is equivalent to Java's CounterPoker class.
pub struct CounterPoker;

impl CounterPoker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CounterPoker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_poker_creation() {
        let _poker = CounterPoker::new();
    }
}