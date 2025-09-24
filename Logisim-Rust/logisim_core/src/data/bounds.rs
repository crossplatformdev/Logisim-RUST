/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Bounds - immutable rectangular bounding box
//!
//! Rust port of Bounds.java

use super::{Direction, Location};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::fmt;

/// Represents an immutable rectangular bounding box
///
/// This is analogous to java.awt's Rectangle class but immutable and cached
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bounds {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Bounds {
    /// The empty bounds constant
    pub const EMPTY_BOUNDS: Bounds = Bounds {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    /// Create a new bounds rectangle
    pub fn create(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut w = width;
        let mut h = height;
        let mut adj_x = x;
        let mut adj_y = y;

        // Handle negative dimensions
        if w < 0 {
            adj_x += w / 2;
            w = 0;
        }
        if h < 0 {
            adj_y += h / 2;
            h = 0;
        }

        Bounds {
            x: adj_x,
            y: adj_y,
            width: w,
            height: h,
        }
    }

    /// Create bounds from a location (1x1 bounds)
    pub fn create_from_location(location: Location) -> Self {
        Self::create(location.get_x(), location.get_y(), 1, 1)
    }

    /// Get X coordinate
    pub fn get_x(self) -> i32 {
        self.x
    }

    /// Get Y coordinate
    pub fn get_y(self) -> i32 {
        self.y
    }

    /// Get width
    pub fn get_width(self) -> i32 {
        self.width
    }

    /// Get height
    pub fn get_height(self) -> i32 {
        self.height
    }

    /// Get center X coordinate
    pub fn get_center_x(self) -> i32 {
        self.x + self.width / 2
    }

    /// Get center Y coordinate
    pub fn get_center_y(self) -> i32 {
        self.y + self.height / 2
    }

    /// Check if this bounds contains a point
    pub fn contains(self, x: i32, y: i32) -> bool {
        self.contains_with_error(x, y, 0)
    }

    /// Check if this bounds contains a point with allowed error
    pub fn contains_with_error(self, x: i32, y: i32, allowed_error: i32) -> bool {
        x >= self.x - allowed_error
            && x < self.x + self.width + allowed_error
            && y >= self.y - allowed_error
            && y < self.y + self.height + allowed_error
    }

    /// Check if this bounds contains a location
    pub fn contains_location(self, location: Location) -> bool {
        self.contains(location.get_x(), location.get_y())
    }

    /// Check if this bounds contains a location with allowed error
    pub fn contains_location_with_error(self, location: Location, allowed_error: i32) -> bool {
        self.contains_with_error(location.get_x(), location.get_y(), allowed_error)
    }

    /// Check if this bounds completely contains another bounds
    pub fn contains_bounds(self, other: Bounds) -> bool {
        if other.width <= 0 || other.height <= 0 {
            return self.contains(other.x, other.y);
        }
        let other_right = other.x + other.width - 1;
        let other_bottom = other.y + other.height - 1;
        self.contains(other.x, other.y) && self.contains(other_right, other_bottom)
    }

    /// Check if a point is on the border of this bounds
    pub fn border_contains(self, x: i32, y: i32, fudge: i32) -> bool {
        let right = self.x + self.width - 1;
        let bottom = self.y + self.height - 1;

        if (x - self.x).abs() <= fudge || (x - right).abs() <= fudge {
            // On east or west border
            return y >= self.y - fudge && y <= bottom + fudge;
        }
        if (y - self.y).abs() <= fudge || (y - bottom).abs() <= fudge {
            // On north or south border
            return x >= self.x - fudge && x <= right + fudge;
        }
        false
    }

    /// Check if a location is on the border of this bounds
    pub fn border_contains_location(self, location: Location, fudge: i32) -> bool {
        self.border_contains(location.get_x(), location.get_y(), fudge)
    }

    /// Add another bounds to this one (union)
    pub fn add_bounds(self, other: Bounds) -> Bounds {
        if self == Self::EMPTY_BOUNDS {
            return other;
        }
        if other == Self::EMPTY_BOUNDS {
            return self;
        }

        let ret_x = min(self.x, other.x);
        let ret_y = min(self.y, other.y);
        let ret_width = max(self.x + self.width, other.x + other.width) - ret_x;
        let ret_height = max(self.y + self.height, other.y + other.height) - ret_y;

        if ret_x == self.x
            && ret_y == self.y
            && ret_width == self.width
            && ret_height == self.height
        {
            self
        } else if ret_x == other.x
            && ret_y == other.y
            && ret_width == other.width
            && ret_height == other.height
        {
            other
        } else {
            Self::create(ret_x, ret_y, ret_width, ret_height)
        }
    }

    /// Add a point to this bounds (expand to include point)
    pub fn add_point(self, x: i32, y: i32) -> Bounds {
        if self == Self::EMPTY_BOUNDS {
            return Self::create(x, y, 1, 1);
        }
        if self.contains(x, y) {
            return self;
        }

        let mut new_x = self.x;
        let mut new_width = self.width;
        let mut new_y = self.y;
        let mut new_height = self.height;

        if x < self.x {
            new_x = x;
            new_width = (self.x + self.width) - x;
        } else if x >= self.x + self.width {
            new_width = x - self.x + 1;
        }

        if y < self.y {
            new_y = y;
            new_height = (self.y + self.height) - y;
        } else if y >= self.y + self.height {
            new_height = y - self.y + 1;
        }

        Self::create(new_x, new_y, new_width, new_height)
    }

    /// Add a location to this bounds
    pub fn add_location(self, location: Location) -> Bounds {
        self.add_point(location.get_x(), location.get_y())
    }

    /// Add a rectangle area to this bounds
    pub fn add_rectangle(self, x: i32, y: i32, width: i32, height: i32) -> Bounds {
        if self == Self::EMPTY_BOUNDS {
            return Self::create(x, y, width, height);
        }

        let ret_x = min(x, self.x);
        let ret_y = min(y, self.y);
        let ret_width = max(x + width, self.x + self.width) - ret_x;
        let ret_height = max(y + height, self.y + self.height) - ret_y;

        if ret_x == self.x
            && ret_y == self.y
            && ret_width == self.width
            && ret_height == self.height
        {
            self
        } else {
            Self::create(ret_x, ret_y, ret_width, ret_height)
        }
    }

    /// Expand bounds by d pixels in each direction
    pub fn expand(self, d: i32) -> Bounds {
        if self == Self::EMPTY_BOUNDS {
            return self;
        }
        if d == 0 {
            return self;
        }
        Self::create(
            self.x - d,
            self.y - d,
            self.width + 2 * d,
            self.height + 2 * d,
        )
    }

    /// Translate bounds by offset
    pub fn translate(self, dx: i32, dy: i32) -> Bounds {
        if self == Self::EMPTY_BOUNDS {
            return self;
        }
        if dx == 0 && dy == 0 {
            return self;
        }
        Self::create(self.x + dx, self.y + dy, self.width, self.height)
    }

    /// Intersect with another bounds
    pub fn intersect(self, other: Bounds) -> Bounds {
        let x0 = max(self.x, other.x);
        let y0 = max(self.y, other.y);
        let x1 = min(self.x + self.width, other.x + other.width);
        let y1 = min(self.y + self.height, other.y + other.height);

        if x1 < x0 || y1 < y0 {
            Self::EMPTY_BOUNDS
        } else {
            Self::create(x0, y0, x1 - x0, y1 - y0)
        }
    }

    /// Rotate bounds around a center point
    pub fn rotate(self, from: Direction, to: Direction, center_x: i32, center_y: i32) -> Bounds {
        let mut degrees = to.to_degrees() - from.to_degrees();
        while degrees >= 360 {
            degrees -= 360;
        }
        while degrees < 0 {
            degrees += 360;
        }

        let dx = self.x - center_x;
        let dy = self.y - center_y;

        match degrees {
            90 => Self::create(
                center_x + dy,
                center_y - dx - self.width,
                self.height,
                self.width,
            ),
            180 => Self::create(
                center_x - dx - self.width,
                center_y - dy - self.height,
                self.width,
                self.height,
            ),
            270 => Self::create(
                center_x - dy - self.height,
                center_y + dx,
                self.height,
                self.width,
            ),
            _ => self,
        }
    }

    /// Check if bounds is empty
    pub fn is_empty(self) -> bool {
        self.width <= 0 || self.height <= 0
    }
}

impl fmt::Display for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}): {}x{}", self.x, self.y, self.width, self.height)
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self::EMPTY_BOUNDS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_creation() {
        let bounds = Bounds::create(10, 20, 30, 40);
        assert_eq!(bounds.get_x(), 10);
        assert_eq!(bounds.get_y(), 20);
        assert_eq!(bounds.get_width(), 30);
        assert_eq!(bounds.get_height(), 40);
    }

    #[test]
    fn test_bounds_center() {
        let bounds = Bounds::create(10, 20, 30, 40);
        assert_eq!(bounds.get_center_x(), 25);
        assert_eq!(bounds.get_center_y(), 40);
    }

    #[test]
    fn test_bounds_contains() {
        let bounds = Bounds::create(10, 20, 30, 40);

        assert!(bounds.contains(10, 20));
        assert!(bounds.contains(25, 35));
        assert!(bounds.contains(39, 59));
        assert!(!bounds.contains(40, 60));
        assert!(!bounds.contains(9, 19));
    }

    #[test]
    fn test_bounds_contains_location() {
        let bounds = Bounds::create(10, 20, 30, 40);
        let inside = Location::new(25, 35);
        let outside = Location::new(50, 70);

        assert!(bounds.contains_location(inside));
        assert!(!bounds.contains_location(outside));
    }

    #[test]
    fn test_bounds_contains_bounds() {
        let outer = Bounds::create(10, 20, 30, 40);
        let inner = Bounds::create(15, 25, 10, 15);
        let overlapping = Bounds::create(35, 55, 10, 15);

        assert!(outer.contains_bounds(inner));
        assert!(!outer.contains_bounds(overlapping));
    }

    #[test]
    fn test_bounds_add_bounds() {
        let b1 = Bounds::create(10, 20, 30, 40);
        let b2 = Bounds::create(50, 70, 20, 25);
        let union = b1.add_bounds(b2);

        assert_eq!(union.get_x(), 10);
        assert_eq!(union.get_y(), 20);
        assert_eq!(union.get_width(), 60);
        assert_eq!(union.get_height(), 75);
    }

    #[test]
    fn test_bounds_add_point() {
        let bounds = Bounds::create(10, 20, 30, 40);

        // Point inside - no change
        let same = bounds.add_point(25, 35);
        assert_eq!(same, bounds);

        // Point outside - expand
        let expanded = bounds.add_point(50, 70);
        assert_eq!(expanded.get_x(), 10);
        assert_eq!(expanded.get_y(), 20);
        assert_eq!(expanded.get_width(), 41);
        assert_eq!(expanded.get_height(), 51);
    }

    #[test]
    fn test_bounds_expand() {
        let bounds = Bounds::create(10, 20, 30, 40);
        let expanded = bounds.expand(5);

        assert_eq!(expanded.get_x(), 5);
        assert_eq!(expanded.get_y(), 15);
        assert_eq!(expanded.get_width(), 40);
        assert_eq!(expanded.get_height(), 50);
    }

    #[test]
    fn test_bounds_translate() {
        let bounds = Bounds::create(10, 20, 30, 40);
        let translated = bounds.translate(5, -3);

        assert_eq!(translated.get_x(), 15);
        assert_eq!(translated.get_y(), 17);
        assert_eq!(translated.get_width(), 30);
        assert_eq!(translated.get_height(), 40);
    }

    #[test]
    fn test_bounds_intersect() {
        let b1 = Bounds::create(10, 20, 30, 40);
        let b2 = Bounds::create(25, 35, 30, 40);
        let intersection = b1.intersect(b2);

        assert_eq!(intersection.get_x(), 25);
        assert_eq!(intersection.get_y(), 35);
        assert_eq!(intersection.get_width(), 15);
        assert_eq!(intersection.get_height(), 25);

        // Non-intersecting bounds
        let b3 = Bounds::create(100, 100, 10, 10);
        let no_intersection = b1.intersect(b3);
        assert_eq!(no_intersection, Bounds::EMPTY_BOUNDS);
    }

    #[test]
    fn test_bounds_rotation() {
        let bounds = Bounds::create(1, 0, 2, 3);
        let center_x = 0;
        let center_y = 0;

        // 90-degree rotation
        let rotated = bounds.rotate(Direction::East, Direction::North, center_x, center_y);
        assert_eq!(rotated.get_x(), 0);
        assert_eq!(rotated.get_y(), -3);
        assert_eq!(rotated.get_width(), 3);
        assert_eq!(rotated.get_height(), 2);
    }

    #[test]
    fn test_bounds_empty() {
        assert!(Bounds::EMPTY_BOUNDS.is_empty());
        assert!(!Bounds::create(10, 20, 30, 40).is_empty());

        let zero_width = Bounds::create(10, 20, 0, 40);
        assert!(zero_width.is_empty());
    }

    #[test]
    fn test_bounds_border_contains() {
        let bounds = Bounds::create(10, 10, 20, 20);

        // Points on borders
        assert!(bounds.border_contains(10, 15, 0)); // Left border
        assert!(bounds.border_contains(29, 15, 0)); // Right border
        assert!(bounds.border_contains(15, 10, 0)); // Top border
        assert!(bounds.border_contains(15, 29, 0)); // Bottom border

        // Points inside
        assert!(!bounds.border_contains(15, 15, 0));

        // Points outside
        assert!(!bounds.border_contains(5, 15, 0));
    }

    #[test]
    fn test_bounds_display() {
        let bounds = Bounds::create(10, 20, 30, 40);
        assert_eq!(bounds.to_string(), "(10,20): 30x40");
    }

    #[test]
    fn test_bounds_from_location() {
        let location = Location::new(10, 20);
        let bounds = Bounds::create_from_location(location);

        assert_eq!(bounds.get_x(), 10);
        assert_eq!(bounds.get_y(), 20);
        assert_eq!(bounds.get_width(), 1);
        assert_eq!(bounds.get_height(), 1);
    }
}
