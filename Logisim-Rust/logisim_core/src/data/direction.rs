/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Direction enumeration for component orientation
//! 
//! Rust port of Direction.java

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the four cardinal directions for component orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

impl Direction {
    /// All cardinal directions in order
    pub const CARDINALS: [Direction; 4] = [
        Direction::East,
        Direction::North,
        Direction::West,
        Direction::South,
    ];

    /// Parse a direction from a string
    pub fn parse(s: &str) -> Result<Direction, String> {
        match s.to_lowercase().as_str() {
            "east" => Ok(Direction::East),
            "north" => Ok(Direction::North),
            "west" => Ok(Direction::West),
            "south" => Ok(Direction::South),
            _ => Err(format!("Invalid direction: '{}'", s)),
        }
    }

    /// Get the direction to the left (counterclockwise)
    pub fn get_left(self) -> Direction {
        Self::CARDINALS[(self as usize + 1) % 4]
    }

    /// Get the direction to the right (clockwise)
    pub fn get_right(self) -> Direction {
        Self::CARDINALS[(self as usize + 3) % 4]
    }

    /// Get the reverse direction
    pub fn reverse(self) -> Direction {
        Self::CARDINALS[(self as usize + 2) % 4]
    }

    /// Convert to degrees (0=East, 90=North, 180=West, 270=South)
    pub fn to_degrees(self) -> i32 {
        (self as i32) * 90
    }

    /// Convert to radians
    pub fn to_radians(self) -> f64 {
        (self as i32 as f64) * std::f64::consts::PI / 2.0
    }

    /// Get a display string for the direction
    pub fn to_display_string(self) -> &'static str {
        match self {
            Direction::East => "East",
            Direction::North => "North",
            Direction::West => "West",
            Direction::South => "South",
        }
    }

    /// Get a vertical display string for the direction
    pub fn to_vertical_display_string(self) -> &'static str {
        match self {
            Direction::East => "→",
            Direction::North => "↑",
            Direction::West => "←",
            Direction::South => "↓",
        }
    }

    /// Check if this is a horizontal direction (East or West)
    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction::East | Direction::West)
    }

    /// Check if this is a vertical direction (North or South)
    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::North | Direction::South)
    }

    /// Get the unit vector for this direction
    pub fn to_unit_vector(self) -> (i32, i32) {
        match self {
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::East => "east",
            Direction::North => "north",
            Direction::West => "west",
            Direction::South => "south",
        })
    }
}

impl From<usize> for Direction {
    fn from(id: usize) -> Self {
        Self::CARDINALS[id % 4]
    }
}

impl From<Direction> for usize {
    fn from(dir: Direction) -> usize {
        dir as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_basic() {
        assert_eq!(Direction::East as usize, 0);
        assert_eq!(Direction::North as usize, 1);
        assert_eq!(Direction::West as usize, 2);
        assert_eq!(Direction::South as usize, 3);
    }

    #[test]
    fn test_direction_parse() {
        assert_eq!(Direction::parse("east").unwrap(), Direction::East);
        assert_eq!(Direction::parse("NORTH").unwrap(), Direction::North);
        assert_eq!(Direction::parse("West").unwrap(), Direction::West);
        assert_eq!(Direction::parse("south").unwrap(), Direction::South);
        
        assert!(Direction::parse("invalid").is_err());
    }

    #[test]
    fn test_direction_rotation() {
        assert_eq!(Direction::East.get_left(), Direction::North);
        assert_eq!(Direction::North.get_left(), Direction::West);
        assert_eq!(Direction::West.get_left(), Direction::South);
        assert_eq!(Direction::South.get_left(), Direction::East);

        assert_eq!(Direction::East.get_right(), Direction::South);
        assert_eq!(Direction::South.get_right(), Direction::West);
        assert_eq!(Direction::West.get_right(), Direction::North);
        assert_eq!(Direction::North.get_right(), Direction::East);

        assert_eq!(Direction::East.reverse(), Direction::West);
        assert_eq!(Direction::North.reverse(), Direction::South);
    }

    #[test]
    fn test_direction_degrees() {
        assert_eq!(Direction::East.to_degrees(), 0);
        assert_eq!(Direction::North.to_degrees(), 90);
        assert_eq!(Direction::West.to_degrees(), 180);
        assert_eq!(Direction::South.to_degrees(), 270);
    }

    #[test]
    fn test_direction_properties() {
        assert!(Direction::East.is_horizontal());
        assert!(Direction::West.is_horizontal());
        assert!(!Direction::North.is_horizontal());
        assert!(!Direction::South.is_horizontal());

        assert!(Direction::North.is_vertical());
        assert!(Direction::South.is_vertical());
        assert!(!Direction::East.is_vertical());
        assert!(!Direction::West.is_vertical());
    }

    #[test]
    fn test_unit_vectors() {
        assert_eq!(Direction::East.to_unit_vector(), (1, 0));
        assert_eq!(Direction::North.to_unit_vector(), (0, -1));
        assert_eq!(Direction::West.to_unit_vector(), (-1, 0));
        assert_eq!(Direction::South.to_unit_vector(), (0, 1));
    }

    #[test]
    fn test_display() {
        assert_eq!(Direction::East.to_string(), "east");
        assert_eq!(Direction::North.to_display_string(), "North");
        assert_eq!(Direction::East.to_vertical_display_string(), "→");
    }

    #[test]
    fn test_from_conversions() {
        assert_eq!(Direction::from(0), Direction::East);
        assert_eq!(Direction::from(1), Direction::North);
        assert_eq!(Direction::from(4), Direction::East); // Wraps around
        
        assert_eq!(usize::from(Direction::West), 2);
    }
}