/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Standard Attributes for Instance Components
//!
//! This module provides commonly used attributes for component instances.
//! This is equivalent to Java's `StdAttr` class.

use crate::data::{Attribute, AttributeValue, BitWidth, Direction, Location};
use crate::instance::instance_text_field::FontInfo;

/// Standard attributes commonly used by component instances.
///
/// This struct provides a collection of commonly used attributes that many
/// components share, such as labels, facing direction, bit width, etc.
pub struct StdAttr;

impl StdAttr {
    /// Component label text attribute.
    pub fn label() -> Attribute<String> {
        Attribute::new("label".to_string())
    }

    /// Component label font attribute.
    pub fn label_font() -> Attribute<FontInfo> {
        Attribute::new("labelfont".to_string())
    }

    /// Component label location attribute.
    pub fn label_loc() -> Attribute<Location> {
        Attribute::new("labelloc".to_string())
    }

    /// Component facing direction attribute.
    pub fn facing() -> Attribute<Direction> {
        Attribute::new("facing".to_string())
    }

    /// Component bit width attribute.
    pub fn width() -> Attribute<BitWidth> {
        Attribute::new("width".to_string())
    }

    /// Input/output trigger type attribute.
    pub fn trigger() -> Attribute<TriggerType> {
        Attribute::new("trigger".to_string())
    }

    /// Enable input attribute (for components with enable pins).
    pub fn enable() -> Attribute<bool> {
        Attribute::new("enable".to_string())
    }

    /// Component appearance attribute.
    pub fn appearance() -> Attribute<AppearanceType> {
        Attribute::new("appearance".to_string())
    }
}

/// Trigger types for sequential components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerType {
    /// Rising edge triggered
    RisingEdge,
    /// Falling edge triggered
    FallingEdge,
    /// High level triggered
    HighLevel,
    /// Low level triggered
    LowLevel,
}

impl TriggerType {
    /// Returns all available trigger types.
    pub fn all() -> &'static [TriggerType] {
        &[
            TriggerType::RisingEdge,
            TriggerType::FallingEdge,
            TriggerType::HighLevel,
            TriggerType::LowLevel,
        ]
    }

    /// Returns the display name for this trigger type.
    pub fn display_name(&self) -> &'static str {
        match self {
            TriggerType::RisingEdge => "Rising Edge",
            TriggerType::FallingEdge => "Falling Edge",
            TriggerType::HighLevel => "High Level",
            TriggerType::LowLevel => "Low Level",
        }
    }

    /// Checks if this is an edge-triggered type.
    pub fn is_edge(&self) -> bool {
        matches!(self, TriggerType::RisingEdge | TriggerType::FallingEdge)
    }

    /// Checks if this is a level-triggered type.
    pub fn is_level(&self) -> bool {
        matches!(self, TriggerType::HighLevel | TriggerType::LowLevel)
    }
}

impl Default for TriggerType {
    fn default() -> Self {
        TriggerType::RisingEdge
    }
}

impl AttributeValue for TriggerType {
    fn to_display_string(&self) -> String {
        self.display_name().to_string()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "rising" | "rising edge" => Ok(TriggerType::RisingEdge),
            "falling" | "falling edge" => Ok(TriggerType::FallingEdge),
            "high" | "high level" => Ok(TriggerType::HighLevel),
            "low" | "low level" => Ok(TriggerType::LowLevel),
            _ => Err(format!("Unknown trigger type: {}", s)),
        }
    }
}

/// Appearance types for component rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppearanceType {
    /// Classic/legacy appearance
    Classic,
    /// Modern appearance
    Modern,
    /// Evolution appearance (enhanced modern)
    Evolution,
}

impl AppearanceType {
    /// Returns all available appearance types.
    pub fn all() -> &'static [AppearanceType] {
        &[
            AppearanceType::Classic,
            AppearanceType::Modern,
            AppearanceType::Evolution,
        ]
    }

    /// Returns the display name for this appearance type.
    pub fn display_name(&self) -> &'static str {
        match self {
            AppearanceType::Classic => "Classic",
            AppearanceType::Modern => "Modern", 
            AppearanceType::Evolution => "Evolution",
        }
    }
}

impl Default for AppearanceType {
    fn default() -> Self {
        AppearanceType::Evolution
    }
}

impl AttributeValue for AppearanceType {
    fn to_display_string(&self) -> String {
        self.display_name().to_string()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "classic" => Ok(AppearanceType::Classic),
            "modern" => Ok(AppearanceType::Modern),
            "evolution" => Ok(AppearanceType::Evolution),
            _ => Err(format!("Unknown appearance type: {}", s)),
        }
    }
}

/// Helper functions for working with standard attributes.
impl StdAttr {
    /// Gets the default label location for a component.
    ///
    /// # Arguments
    ///
    /// * `facing` - The component's facing direction
    /// * `bounds` - The component's bounds
    ///
    /// # Returns
    ///
    /// A suitable default location for the label.
    pub fn default_label_location(facing: Direction, bounds: crate::data::Bounds) -> Location {
        match facing {
            Direction::North => Location::new(
                bounds.get_x() + bounds.get_width() / 2,
                bounds.get_y() - 5,
            ),
            Direction::South => Location::new(
                bounds.get_x() + bounds.get_width() / 2,
                bounds.get_y() + bounds.get_height() + 15,
            ),
            Direction::East => Location::new(
                bounds.get_x() + bounds.get_width() + 5,
                bounds.get_y() + bounds.get_height() / 2,
            ),
            Direction::West => Location::new(
                bounds.get_x() - 5,
                bounds.get_y() + bounds.get_height() / 2,
            ),
        }
    }

    /// Checks if a facing direction is vertical.
    pub fn is_vertical(facing: Direction) -> bool {
        matches!(facing, Direction::North | Direction::South)
    }

    /// Checks if a facing direction is horizontal.
    pub fn is_horizontal(facing: Direction) -> bool {
        matches!(facing, Direction::East | Direction::West)
    }

    /// Gets the opposite direction.
    pub fn opposite(facing: Direction) -> Direction {
        match facing {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Bounds;

    #[test]
    fn test_trigger_type_properties() {
        assert!(TriggerType::RisingEdge.is_edge());
        assert!(!TriggerType::RisingEdge.is_level());
        
        assert!(TriggerType::FallingEdge.is_edge());
        assert!(!TriggerType::FallingEdge.is_level());
        
        assert!(!TriggerType::HighLevel.is_edge());
        assert!(TriggerType::HighLevel.is_level());
        
        assert!(!TriggerType::LowLevel.is_edge());
        assert!(TriggerType::LowLevel.is_level());
    }

    #[test]
    fn test_trigger_type_default() {
        assert_eq!(TriggerType::default(), TriggerType::RisingEdge);
    }

    #[test]
    fn test_appearance_type_default() {
        assert_eq!(AppearanceType::default(), AppearanceType::Evolution);
    }

    #[test]
    fn test_direction_helpers() {
        assert!(StdAttr::is_vertical(Direction::North));
        assert!(StdAttr::is_vertical(Direction::South));
        assert!(!StdAttr::is_vertical(Direction::East));
        assert!(!StdAttr::is_vertical(Direction::West));

        assert!(StdAttr::is_horizontal(Direction::East));
        assert!(StdAttr::is_horizontal(Direction::West));
        assert!(!StdAttr::is_horizontal(Direction::North));
        assert!(!StdAttr::is_horizontal(Direction::South));
    }

    #[test]
    fn test_opposite_direction() {
        assert_eq!(StdAttr::opposite(Direction::North), Direction::South);
        assert_eq!(StdAttr::opposite(Direction::South), Direction::North);
        assert_eq!(StdAttr::opposite(Direction::East), Direction::West);
        assert_eq!(StdAttr::opposite(Direction::West), Direction::East);
    }

    #[test]
    fn test_default_label_location() {
        let bounds = Bounds::create(10, 20, 30, 40);
        
        let north_loc = StdAttr::default_label_location(Direction::North, bounds);
        assert_eq!(north_loc.x, 25); // center x: 10 + 30/2
        assert_eq!(north_loc.y, 15); // above: 20 - 5
        
        let south_loc = StdAttr::default_label_location(Direction::South, bounds);
        assert_eq!(south_loc.x, 25); // center x: 10 + 30/2
        assert_eq!(south_loc.y, 75); // below: 20 + 40 + 15
        
        let east_loc = StdAttr::default_label_location(Direction::East, bounds);
        assert_eq!(east_loc.x, 45); // right: 10 + 30 + 5
        assert_eq!(east_loc.y, 40); // center y: 20 + 40/2
        
        let west_loc = StdAttr::default_label_location(Direction::West, bounds);
        assert_eq!(west_loc.x, 5); // left: 10 - 5
        assert_eq!(west_loc.y, 40); // center y: 20 + 40/2
    }
}