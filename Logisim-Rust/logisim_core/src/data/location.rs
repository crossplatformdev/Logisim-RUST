/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Location and positioning types
//!
//! Rust port of Location.java

use super::Direction;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents an immutable 2D point/location
///
/// This is analogous to Java's Point class but immutable and cached
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    pub x: i32,
    pub y: i32,
    snap_to_grid: bool,
}

impl Location {
    /// Create a new location, optionally snapping to grid
    pub fn create(x: i32, y: i32, snap_to_grid: bool) -> Self {
        let (x_snapped, y_snapped) = if snap_to_grid {
            // Round to 5-unit grid (half-grid base)
            (
                (x as f32 / 5.0).round() as i32 * 5,
                (y as f32 / 5.0).round() as i32 * 5,
            )
        } else {
            (x, y)
        };

        Location {
            x: x_snapped,
            y: y_snapped,
            snap_to_grid,
        }
    }

    /// Create a new location without grid snapping
    pub fn new(x: i32, y: i32) -> Self {
        Self::create(x, y, false)
    }

    /// Create a new location with grid snapping
    pub fn new_snapped(x: i32, y: i32) -> Self {
        Self::create(x, y, true)
    }

    /// Parse a location from a string like "(x,y)" or "x,y"
    pub fn parse(value: &str) -> Result<Location, String> {
        let value = value.trim();

        // Handle parentheses
        let value = if value.starts_with('(') && value.ends_with(')') {
            &value[1..value.len() - 1]
        } else {
            value
        };

        // Find comma or space separator
        let separator_pos = value
            .find(',')
            .or_else(|| value.find(' '))
            .ok_or_else(|| format!("Invalid location format: '{}'", value))?;

        let x_str = value[..separator_pos].trim();
        let y_str = value[separator_pos + 1..].trim();

        let x = x_str
            .parse::<i32>()
            .map_err(|_| format!("Invalid x coordinate: '{}'", x_str))?;
        let y = y_str
            .parse::<i32>()
            .map_err(|_| format!("Invalid y coordinate: '{}'", y_str))?;

        Ok(Location::create(x, y, true))
    }

    /// Get the X coordinate
    pub fn get_x(self) -> i32 {
        self.x
    }

    /// Get the Y coordinate
    pub fn get_y(self) -> i32 {
        self.y
    }

    /// Calculate Manhattan distance to another point
    pub fn manhattan_distance_to(self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }

    /// Calculate Manhattan distance to another location
    pub fn manhattan_distance_to_location(self, other: Location) -> i32 {
        self.manhattan_distance_to(other.x, other.y)
    }

    /// Translate by a given offset
    pub fn translate(self, dx: i32, dy: i32) -> Location {
        if dx == 0 && dy == 0 {
            self
        } else {
            Location::create(self.x + dx, self.y + dy, self.snap_to_grid)
        }
    }

    /// Translate in a given direction
    pub fn translate_direction(self, dir: Direction, dist: i32) -> Location {
        self.translate_direction_with_offset(dir, dist, 0)
    }

    /// Translate in a given direction with perpendicular offset
    pub fn translate_direction_with_offset(
        self,
        dir: Direction,
        dist: i32,
        right: i32,
    ) -> Location {
        if dist == 0 && right == 0 {
            return self;
        }

        match dir {
            Direction::East => Location::create(self.x + dist, self.y + right, self.snap_to_grid),
            Direction::West => Location::create(self.x - dist, self.y - right, self.snap_to_grid),
            Direction::South => Location::create(self.x - right, self.y + dist, self.snap_to_grid),
            Direction::North => Location::create(self.x + right, self.y - dist, self.snap_to_grid),
        }
    }

    /// Rotate this location around a center point
    pub fn rotate(self, from: Direction, to: Direction, center_x: i32, center_y: i32) -> Location {
        let mut degrees = to.to_degrees() - from.to_degrees();
        while degrees >= 360 {
            degrees -= 360;
        }
        while degrees < 0 {
            degrees += 360;
        }

        let dx = self.x - center_x;
        let dy = self.y - center_y;

        let (new_dx, new_dy) = match degrees {
            90 => (dy, -dx),
            180 => (-dx, -dy),
            270 => (-dy, dx),
            _ => (dx, dy), // No rotation or invalid angle
        };

        Location::create(center_x + new_dx, center_y + new_dy, self.snap_to_grid)
    }

    /// Check if grid snapping is enabled
    pub fn has_snap_to_grid(self) -> bool {
        self.snap_to_grid
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            other => other,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/// Trait for objects that have a location
pub trait HasLocation {
    fn get_location(&self) -> Location;
}

/// Utilities for sorting objects by location
pub struct LocationSorting;

impl LocationSorting {
    /// Sort by horizontal position (left to right, then top to bottom)
    pub fn sort_horizontal<T: HasLocation>(items: &mut [T]) {
        items.sort_by(|a, b| {
            let loc_a = a.get_location();
            let loc_b = b.get_location();

            match loc_a.x.cmp(&loc_b.x) {
                Ordering::Equal => loc_a.y.cmp(&loc_b.y),
                other => other,
            }
        });
    }

    /// Sort by vertical position (top to bottom, then left to right)
    pub fn sort_vertical<T: HasLocation>(items: &mut [T]) {
        items.sort_by(|a, b| {
            let loc_a = a.get_location();
            let loc_b = b.get_location();

            match loc_a.y.cmp(&loc_b.y) {
                Ordering::Equal => loc_a.x.cmp(&loc_b.x),
                other => other,
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_creation() {
        let loc = Location::new(10, 20);
        assert_eq!(loc.x, 10);
        assert_eq!(loc.y, 20);
        assert_eq!(loc.get_x(), 10);
        assert_eq!(loc.get_y(), 20);
        assert!(!loc.has_snap_to_grid());
    }

    #[test]
    fn test_location_snapping() {
        let loc = Location::new_snapped(7, 13);
        assert_eq!(loc.x, 5); // Snapped to nearest 5
        assert_eq!(loc.y, 15); // Snapped to nearest 5
        assert!(loc.has_snap_to_grid());
    }

    #[test]
    fn test_location_parse() {
        assert_eq!(
            Location::parse("(10,20)").unwrap(),
            Location::new_snapped(10, 20)
        );
        assert_eq!(
            Location::parse("10,20").unwrap(),
            Location::new_snapped(10, 20)
        );
        assert_eq!(
            Location::parse("10 20").unwrap(),
            Location::new_snapped(10, 20)
        );

        assert!(Location::parse("invalid").is_err());
        assert!(Location::parse("(10,20").is_err());
    }

    #[test]
    fn test_manhattan_distance() {
        let loc = Location::new(0, 0);
        assert_eq!(loc.manhattan_distance_to(3, 4), 7);

        let other = Location::new(3, 4);
        assert_eq!(loc.manhattan_distance_to_location(other), 7);
    }

    #[test]
    fn test_translation() {
        let loc = Location::new(10, 20);
        let translated = loc.translate(5, -3);

        assert_eq!(translated.x, 15);
        assert_eq!(translated.y, 17);

        // No-op translation
        let same = loc.translate(0, 0);
        assert_eq!(same, loc);
    }

    #[test]
    fn test_direction_translation() {
        let loc = Location::new(10, 10);

        assert_eq!(
            loc.translate_direction(Direction::East, 5),
            Location::new(15, 10)
        );
        assert_eq!(
            loc.translate_direction(Direction::West, 5),
            Location::new(5, 10)
        );
        assert_eq!(
            loc.translate_direction(Direction::North, 5),
            Location::new(10, 5)
        );
        assert_eq!(
            loc.translate_direction(Direction::South, 5),
            Location::new(10, 15)
        );

        // With offset
        assert_eq!(
            loc.translate_direction_with_offset(Direction::East, 5, 3),
            Location::new(15, 13)
        );
    }

    #[test]
    fn test_rotation() {
        let loc = Location::new(1, 0);
        let center_x = 0;
        let center_y = 0;

        // 90-degree rotation
        let rotated = loc.rotate(Direction::East, Direction::North, center_x, center_y);
        assert_eq!(rotated, Location::new(0, -1));

        // 180-degree rotation
        let rotated = loc.rotate(Direction::East, Direction::West, center_x, center_y);
        assert_eq!(rotated, Location::new(-1, 0));

        // 270-degree rotation
        let rotated = loc.rotate(Direction::East, Direction::South, center_x, center_y);
        assert_eq!(rotated, Location::new(0, 1));
    }

    #[test]
    fn test_comparison() {
        let loc1 = Location::new(10, 20);
        let loc2 = Location::new(10, 20);
        let loc3 = Location::new(15, 20);
        let loc4 = Location::new(10, 25);

        assert_eq!(loc1, loc2);
        assert!(loc1 < loc3); // Same y, smaller x
        assert!(loc1 < loc4); // Same x, smaller y
        assert!(loc3 > loc1);
    }

    #[test]
    fn test_display() {
        let loc = Location::new(10, 20);
        assert_eq!(loc.to_string(), "(10,20)");
    }

    struct TestLocationItem {
        location: Location,
    }

    impl HasLocation for TestLocationItem {
        fn get_location(&self) -> Location {
            self.location
        }
    }

    #[test]
    fn test_sorting() {
        let mut items = vec![
            TestLocationItem {
                location: Location::new(20, 10),
            },
            TestLocationItem {
                location: Location::new(10, 20),
            },
            TestLocationItem {
                location: Location::new(10, 10),
            },
            TestLocationItem {
                location: Location::new(20, 20),
            },
        ];

        // Test horizontal sorting
        LocationSorting::sort_horizontal(&mut items);
        assert_eq!(items[0].location, Location::new(10, 10));
        assert_eq!(items[1].location, Location::new(10, 20));
        assert_eq!(items[2].location, Location::new(20, 10));
        assert_eq!(items[3].location, Location::new(20, 20));

        // Test vertical sorting
        LocationSorting::sort_vertical(&mut items);
        assert_eq!(items[0].location, Location::new(10, 10));
        assert_eq!(items[1].location, Location::new(20, 10));
        assert_eq!(items[2].location, Location::new(10, 20));
        assert_eq!(items[3].location, Location::new(20, 20));
    }
}

impl super::AttributeValue for Location {
    fn to_display_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err("Location must be in format (x, y)".to_string());
        }
        
        let inner = &s[1..s.len()-1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 2 {
            return Err("Location must have exactly two coordinates".to_string());
        }
        
        let x = parts[0].trim().parse::<i32>().map_err(|_| "Invalid x coordinate")?;
        let y = parts[1].trim().parse::<i32>().map_err(|_| "Invalid y coordinate")?;
        
        Ok(Location::new(x, y))
    }
}
