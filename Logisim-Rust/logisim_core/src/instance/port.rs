/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component Port Definitions
//!
//! This module provides the `Port` struct and related types for defining component
//! connection points (pins). This is equivalent to Java's `Port` class.

use crate::data::{Attribute, AttributeSet, BitWidth, Location};
use std::fmt;

/// Port type enumeration defining the direction and behavior of a component port.
///
/// This corresponds to the Java constants `INPUT`, `OUTPUT`, and `INOUT` in the Port class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortType {
    /// Input-only port - receives signals from other components
    Input,
    /// Output-only port - drives signals to other components  
    Output,
    /// Bidirectional port - can both receive and drive signals
    InOut,
}

impl PortType {
    /// Converts a string representation to a PortType.
    ///
    /// # Arguments
    ///
    /// * `s` - String representation ("input", "output", or "inout")
    ///
    /// # Returns
    ///
    /// The corresponding PortType, or an error if the string is not recognized.
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "input" => Ok(PortType::Input),
            "output" => Ok(PortType::Output), 
            "inout" => Ok(PortType::InOut),
            _ => Err(format!("Unrecognized port type: {}", s)),
        }
    }

    /// Returns the string representation of this port type.
    pub fn as_str(&self) -> &'static str {
        match self {
            PortType::Input => "input",
            PortType::Output => "output",
            PortType::InOut => "inout",
        }
    }

    /// Returns the default exclusion behavior for this port type.
    ///
    /// Input and InOut ports are shared by default, Output ports are exclusive.
    pub fn default_exclusive(&self) -> bool {
        match self {
            PortType::Input | PortType::InOut => false, // SHARED
            PortType::Output => true, // EXCLUSIVE
        }
    }
}

impl fmt::Display for PortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Port width specification - either fixed or determined by an attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum PortWidth {
    /// Fixed bit width known at compile time
    Fixed(BitWidth),
    /// Variable bit width determined by component attributes at runtime
    Attribute(Attribute<BitWidth>),
}

impl PortWidth {
    /// Creates a fixed-width port specification.
    pub fn fixed(width: BitWidth) -> Self {
        PortWidth::Fixed(width)
    }

    /// Creates a fixed-width port specification from a bit count.
    pub fn fixed_bits(bits: u32) -> Self {
        PortWidth::Fixed(BitWidth::new(bits))
    }

    /// Creates an attribute-based width specification.
    pub fn attribute(attr: Attribute<BitWidth>) -> Self {
        PortWidth::Attribute(attr)
    }

    /// Resolves the actual bit width given an attribute set.
    ///
    /// # Arguments
    ///
    /// * `attrs` - The attribute set to query for variable widths
    ///
    /// # Returns
    ///
    /// The resolved BitWidth, or an error if attribute resolution fails.
    pub fn resolve(&self, attrs: &AttributeSet) -> Result<BitWidth, String> {
        match self {
            PortWidth::Fixed(width) => Ok(*width),
            PortWidth::Attribute(attr) => {
                attrs.get_value(attr)
                    .copied()
                    .ok_or_else(|| format!("Width attribute {:?} not found", attr))
            }
        }
    }
}

/// Defines a connection point (pin) on a component.
///
/// This struct is equivalent to Java's `Port` class and specifies the location,
/// type, bit width, and other properties of a component port.
///
/// # Example
///
/// ```rust
/// use logisim_core::instance::{Port, PortType, PortWidth};
/// use logisim_core::BitWidth;
///
/// // Create a simple input port
/// let input_port = Port::new(-30, -10, PortType::Input, PortWidth::fixed_bits(1));
///
/// // Create an output port with variable width
/// let output_port = Port::builder(-30, 0, PortType::Output)
///     .width(PortWidth::fixed_bits(8))
///     .tooltip("Data output")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Port {
    /// X offset from component center
    dx: i32,
    /// Y offset from component center  
    dy: i32,
    /// Port type (input, output, inout)
    port_type: PortType,
    /// Bit width specification
    width: PortWidth,
    /// Whether this port excludes others from the same connection point
    exclusive: bool,
    /// Optional tooltip text
    tooltip: Option<String>,
}

impl Port {
    /// Creates a new port with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `dx` - X offset from component center
    /// * `dy` - Y offset from component center
    /// * `port_type` - Type of port (input, output, inout)
    /// * `width` - Bit width specification
    ///
    /// # Returns
    ///
    /// A new Port instance with default exclusion behavior.
    pub fn new(dx: i32, dy: i32, port_type: PortType, width: PortWidth) -> Self {
        Self {
            dx,
            dy,
            port_type,
            width,
            exclusive: port_type.default_exclusive(),
            tooltip: None,
        }
    }

    /// Creates a builder for constructing ports with optional parameters.
    pub fn builder(dx: i32, dy: i32, port_type: PortType) -> PortBuilder {
        PortBuilder::new(dx, dy, port_type)
    }

    /// Returns the X offset from component center.
    pub fn dx(&self) -> i32 {
        self.dx
    }

    /// Returns the Y offset from component center.
    pub fn dy(&self) -> i32 {
        self.dy
    }

    /// Returns the port type.
    pub fn port_type(&self) -> PortType {
        self.port_type
    }

    /// Returns the port width specification.
    pub fn width(&self) -> &PortWidth {
        &self.width
    }

    /// Returns whether this port is exclusive.
    pub fn is_exclusive(&self) -> bool {
        self.exclusive
    }

    /// Returns the tooltip text if available.
    pub fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    /// Calculates the absolute location of this port given a component location.
    ///
    /// # Arguments
    ///
    /// * `component_loc` - Location of the component center
    ///
    /// # Returns  
    ///
    /// The absolute location of this port.
    pub fn location(&self, component_loc: Location) -> Location {
        Location::new(
            component_loc.x + self.dx,
            component_loc.y + self.dy,
        )
    }

    /// Resolves the bit width of this port given an attribute set.
    ///
    /// # Arguments
    ///
    /// * `attrs` - Attribute set for width resolution
    ///
    /// # Returns
    ///
    /// The resolved bit width.
    pub fn resolve_width(&self, attrs: &AttributeSet) -> Result<BitWidth, String> {
        self.width.resolve(attrs)
    }

    /// Sets the exclusion behavior of this port.
    pub fn set_exclusive(&mut self, exclusive: bool) {
        self.exclusive = exclusive;
    }

    /// Sets the tooltip for this port.
    pub fn set_tooltip(&mut self, tooltip: String) {
        self.tooltip = Some(tooltip);
    }
}

/// Builder pattern for constructing Port instances with optional parameters.
pub struct PortBuilder {
    dx: i32,
    dy: i32,
    port_type: PortType,
    width: Option<PortWidth>,
    exclusive: Option<bool>,
    tooltip: Option<String>,
}

impl PortBuilder {
    /// Creates a new port builder.
    fn new(dx: i32, dy: i32, port_type: PortType) -> Self {
        Self {
            dx,
            dy,
            port_type,
            width: None,
            exclusive: None,
            tooltip: None,
        }
    }

    /// Sets the port width.
    pub fn width(mut self, width: PortWidth) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the port width to a fixed bit count.
    pub fn width_bits(self, bits: u32) -> Self {
        self.width(PortWidth::fixed_bits(bits))
    }

    /// Sets the exclusion behavior.
    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    /// Sets the tooltip text.
    pub fn tooltip(mut self, tooltip: &str) -> Self {
        self.tooltip = Some(tooltip.to_string());
        self
    }

    /// Builds the Port instance.
    ///
    /// # Panics
    ///
    /// Panics if width was not specified.
    pub fn build(self) -> Port {
        let width = self.width.expect("Port width must be specified");
        let exclusive = self.exclusive.unwrap_or_else(|| self.port_type.default_exclusive());

        Port {
            dx: self.dx,
            dy: self.dy,
            port_type: self.port_type,
            width,
            exclusive,
            tooltip: self.tooltip,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{AttributeSet, Location};

    #[test]
    fn test_port_type_from_str() {
        assert_eq!(PortType::from_str("input").unwrap(), PortType::Input);
        assert_eq!(PortType::from_str("INPUT").unwrap(), PortType::Input);
        assert_eq!(PortType::from_str("output").unwrap(), PortType::Output);
        assert_eq!(PortType::from_str("inout").unwrap(), PortType::InOut);
        assert!(PortType::from_str("invalid").is_err());
    }

    #[test]
    fn test_port_type_default_exclusive() {
        assert!(!PortType::Input.default_exclusive());
        assert!(PortType::Output.default_exclusive());
        assert!(!PortType::InOut.default_exclusive());
    }

    #[test]
    fn test_port_width_fixed() {
        let width = PortWidth::fixed_bits(8);
        let attrs = AttributeSet::new();
        assert_eq!(width.resolve(&attrs).unwrap(), BitWidth::new(8));
    }

    #[test]
    fn test_port_creation() {
        let port = Port::new(
            -30, -10,
            PortType::Input,
            PortWidth::fixed_bits(1),
        );

        assert_eq!(port.dx(), -30);
        assert_eq!(port.dy(), -10);
        assert_eq!(port.port_type(), PortType::Input);
        assert!(!port.is_exclusive()); // Input ports are shared by default
    }

    #[test]
    fn test_port_builder() {
        let port = Port::builder(-20, 5, PortType::Output)
            .width_bits(4)
            .exclusive(false)
            .tooltip("Test output")
            .build();

        assert_eq!(port.dx(), -20);
        assert_eq!(port.dy(), 5);
        assert_eq!(port.port_type(), PortType::Output);
        assert!(!port.is_exclusive());
        assert_eq!(port.tooltip(), Some("Test output"));
    }

    #[test]
    fn test_port_location() {
        let port = Port::new(
            10, -5,
            PortType::Output,
            PortWidth::fixed_bits(1),
        );

        let component_loc = Location::new(100, 200);
        let port_loc = port.location(component_loc);

        assert_eq!(port_loc.x(), 110);
        assert_eq!(port_loc.y(), 195);
    }

    #[test]
    fn test_port_resolve_width() {
        let port = Port::new(
            0, 0,
            PortType::Input,
            PortWidth::fixed_bits(16),
        );

        let attrs = AttributeSet::new();
        let width = port.resolve_width(&attrs).unwrap();
        assert_eq!(width, BitWidth::new(16));
    }
}