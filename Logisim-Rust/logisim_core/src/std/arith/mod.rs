/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Arithmetic component library
//!
//! This module provides the Rust port of Java's `com.cburch.logisim.std.arith` package,
//! containing arithmetic and mathematical components for digital circuits.
//!
//! ## Components
//!
//! ### Basic Integer Arithmetic
//! - [`Adder`] - Multi-bit integer addition with carry
//! - [`Subtractor`] - Multi-bit integer subtraction with borrow
//! - [`Multiplier`] - Multi-bit integer multiplication
//! - [`Divider`] - Multi-bit integer division
//! - [`Negator`] - Two's complement negation
//! - [`Comparator`] - Integer comparison operations
//!
//! ### Bit Operations
//! - [`Shifter`] - Logical and arithmetic shift operations
//! - [`BitAdder`] - Single-bit full adder component
//! - [`BitFinder`] - Find first/last set bit operations
//!
//! ### Floating-Point Arithmetic (IEEE 754)
//! - [`FpAdder`] - IEEE 754 floating-point addition
//! - [`FpSubtractor`] - IEEE 754 floating-point subtraction
//! - [`FpMultiplier`] - IEEE 754 floating-point multiplication
//! - [`FpDivider`] - IEEE 754 floating-point division
//! - [`FpNegator`] - IEEE 754 floating-point negation
//! - [`FpComparator`] - IEEE 754 floating-point comparison
//! - [`FpToInt`] - IEEE 754 to integer conversion
//! - [`IntToFp`] - Integer to IEEE 754 conversion
//!
//! ## Architecture
//!
//! All arithmetic components follow the standard Logisim component architecture:
//! - Implement the [`Component`] trait for circuit integration
//! - Support configurable bit widths where applicable
//! - Handle 4-value logic (High, Low, Unknown, Error)
//! - Provide proper propagation delays
//! - Support serialization for circuit file format
//!
//! The arithmetic library maintains behavioral compatibility with the Java
//! implementation while leveraging Rust's type safety and performance benefits.

pub mod arithmetic_library;
pub mod adder;
pub mod subtractor;
pub mod multiplier;
pub mod divider;
pub mod negator;
pub mod comparator;
pub mod shifter;
pub mod bit_adder;
pub mod bit_finder;
pub mod fp_adder;
pub mod fp_subtractor;
pub mod fp_multiplier;
pub mod fp_divider;
pub mod fp_negator;
pub mod fp_comparator;
pub mod fp_to_int;
pub mod int_to_fp;

// Re-export all components for easy usage
pub use arithmetic_library::ArithmeticLibrary;
pub use adder::Adder;
pub use subtractor::Subtractor;
pub use multiplier::Multiplier;
pub use divider::Divider;
pub use negator::Negator;
pub use comparator::Comparator;
pub use shifter::Shifter;
pub use bit_adder::BitAdder;
pub use bit_finder::BitFinder;
pub use fp_adder::FpAdder;
pub use fp_subtractor::FpSubtractor;
pub use fp_multiplier::FpMultiplier;
pub use fp_divider::FpDivider;
pub use fp_negator::FpNegator;
pub use fp_comparator::FpComparator;
pub use fp_to_int::FpToInt;
pub use int_to_fp::IntToFp;