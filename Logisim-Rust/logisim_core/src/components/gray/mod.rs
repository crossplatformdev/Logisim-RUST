/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Gray code components module
//!
//! This module provides Gray code counter and incrementer components
//! equivalent to the Java com.cburch.gray package.
//! 
//! The Gray code is a binary numeral system where two successive values
//! differ in only one bit. This property makes it useful in digital
//! circuits to minimize glitches and in rotary encoders to reduce errors.
//!
//! ## Components
//!
//! - **GrayIncrementer**: Takes a multibit input and outputs the next Gray code value
//! - **SimpleGrayCounter**: A fixed 4-bit Gray code counter
//! - **GrayCounter**: A configurable Gray code counter with labels and user interaction
//! - **CounterData**: State management for counter components
//! - **CounterPoker**: User interaction handler for direct value editing
//! - **GrayComponents**: Library container for all Gray code tools

mod components;
mod counter_data;
mod counter_poker;
mod gray_counter;
mod gray_incrementer;
mod simple_gray_counter;

// Re-export all public types
pub use components::*;
pub use counter_data::*;
pub use counter_poker::*;
pub use gray_counter::*;
pub use gray_incrementer::*;
pub use simple_gray_counter::*;