/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component abstractions for Logisim
//!
//! This module provides the core abstractions for digital components in Logisim,
//! migrated from the Java package `com.cburch.logisim.comp`. It includes:
//!
//! - Component traits and base types
//! - Pin abstraction and connection management
//! - Factory patterns for component creation
//! - Event handling for component interactions
//! - Drawing context for component rendering
//!
//! ## Architecture
//!
//! The component abstraction follows the same patterns as the Java version:
//! - `Component` trait defines the basic interface for all components
//! - `ComponentFactory` pattern for creating and managing component types
//! - Event-driven architecture for user interactions and property changes
//! - Separation of logical component behavior from visual representation

pub mod component;
pub mod draw_context;
pub mod event;
pub mod factory;
pub mod pin;

// Re-export core types for convenience
pub use component::{AbstractComponent, Component, ComponentId};
pub use draw_context::{Color, ComponentDrawContext, DrawCommand, GraphicsContext};
pub use event::{ComponentEvent, ComponentListener, ComponentUserEvent};
pub use factory::{AbstractComponentFactory, ComponentFactory};
pub use pin::{EndData, Pin, PinDirection};
