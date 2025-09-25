/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component drawing context abstractions
//!
//! This module provides the drawing context for component rendering,
//! equivalent to Java's `ComponentDrawContext` class. It manages
//! graphics state and provides helper methods for drawing components.

use super::component::Component;
use crate::data::{Bounds, Direction, Location};
use serde::{Deserialize, Serialize};

/// Color representation for component drawing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Create a color with full opacity
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(r, g, b, 255)
    }

    /// Common colors
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const GRAY: Color = Color {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
}

/// Drawing command for component rendering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DrawCommand {
    /// Draw a line from one point to another
    DrawLine { x1: i32, y1: i32, x2: i32, y2: i32 },
    /// Draw a rectangle
    DrawRect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Fill a rectangle
    FillRect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Draw an oval
    DrawOval {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Fill an oval
    FillOval {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Draw text at a location
    DrawText { text: String, x: i32, y: i32 },
    /// Set the drawing color
    SetColor { color: Color },
    /// Set the line width
    SetLineWidth { width: u32 },
}

/// Graphics context for component drawing
///
/// This provides a simplified graphics interface equivalent to Java's
/// Graphics2D for component rendering. Commands are recorded and can
/// be played back by the UI layer.
#[derive(Debug, Clone)]
pub struct GraphicsContext {
    commands: Vec<DrawCommand>,
    current_color: Color,
    current_line_width: u32,
}

impl GraphicsContext {
    /// Create a new graphics context
    pub fn new() -> Self {
        GraphicsContext {
            commands: Vec::new(),
            current_color: Color::BLACK,
            current_line_width: 1,
        }
    }

    /// Get all drawing commands
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Clear all drawing commands
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Set the drawing color
    pub fn set_color(&mut self, color: Color) {
        if color != self.current_color {
            self.current_color = color;
            self.commands.push(DrawCommand::SetColor { color });
        }
    }

    /// Set the line width
    pub fn set_line_width(&mut self, width: u32) {
        if width != self.current_line_width {
            self.current_line_width = width;
            self.commands.push(DrawCommand::SetLineWidth { width });
        }
    }

    /// Draw a line
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.commands.push(DrawCommand::DrawLine { x1, y1, x2, y2 });
    }

    /// Draw a rectangle
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.commands.push(DrawCommand::DrawRect {
            x,
            y,
            width,
            height,
        });
    }

    /// Fill a rectangle
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.commands.push(DrawCommand::FillRect {
            x,
            y,
            width,
            height,
        });
    }

    /// Draw an oval
    pub fn draw_oval(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.commands.push(DrawCommand::DrawOval {
            x,
            y,
            width,
            height,
        });
    }

    /// Fill an oval
    pub fn fill_oval(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.commands.push(DrawCommand::FillOval {
            x,
            y,
            width,
            height,
        });
    }

    /// Draw text
    pub fn draw_text(&mut self, text: String, x: i32, y: i32) {
        self.commands.push(DrawCommand::DrawText { text, x, y });
    }
}

impl Default for GraphicsContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Drawing context for component rendering
///
/// This is equivalent to Java's `ComponentDrawContext` and provides
/// the context and helper methods for drawing components.
#[derive(Debug)]
pub struct ComponentDrawContext {
    graphics: GraphicsContext,
    show_state: bool,
    show_color: bool,
    print_view: bool,
    circuit_bounds: Option<Bounds>,
}

impl ComponentDrawContext {
    /// Create a new component draw context
    pub fn new() -> Self {
        ComponentDrawContext {
            graphics: GraphicsContext::new(),
            show_state: true,
            show_color: true,
            print_view: false,
            circuit_bounds: None,
        }
    }

    /// Create a draw context for printing
    pub fn new_for_print() -> Self {
        ComponentDrawContext {
            graphics: GraphicsContext::new(),
            show_state: false,
            show_color: false,
            print_view: true,
            circuit_bounds: None,
        }
    }

    /// Get the graphics context
    pub fn graphics(&mut self) -> &mut GraphicsContext {
        &mut self.graphics
    }

    /// Get drawing commands
    pub fn commands(&self) -> &[DrawCommand] {
        self.graphics.commands()
    }

    /// Clear all drawing commands
    pub fn clear(&mut self) {
        self.graphics.clear();
    }

    /// Set whether to show component state
    pub fn set_show_state(&mut self, show: bool) {
        self.show_state = show;
    }

    /// Get whether to show component state
    pub fn show_state(&self) -> bool {
        self.show_state
    }

    /// Set whether to show colors
    pub fn set_show_color(&mut self, show: bool) {
        self.show_color = show;
    }

    /// Get whether to show colors
    pub fn show_color(&self) -> bool {
        self.show_color
    }

    /// Check if this is a print view
    pub fn is_print_view(&self) -> bool {
        self.print_view
    }

    /// Set the circuit bounds
    pub fn set_circuit_bounds(&mut self, bounds: Bounds) {
        self.circuit_bounds = Some(bounds);
    }

    /// Get the circuit bounds
    pub fn circuit_bounds(&self) -> Option<Bounds> {
        self.circuit_bounds
    }

    /// Draw component bounds for debugging
    pub fn draw_bounds(&mut self, component: &dyn Component) {
        if let Some(bounds) = component.bounds() {
            self.graphics.set_line_width(2);
            self.graphics.draw_rect(
                bounds.get_x(),
                bounds.get_y(),
                bounds.get_width(),
                bounds.get_height(),
            );
            self.graphics.set_line_width(1);
        }
    }

    /// Draw a clock symbol at a pin location
    pub fn draw_clock(&mut self, location: Location, direction: Direction) {
        self.graphics.set_line_width(2);

        let x = location.x;
        let y = location.y;
        let clk_sz = 4;
        let clk_szd = clk_sz - 1;

        match direction {
            Direction::North => {
                self.graphics.draw_line(x - clk_szd, y - 1, x, y - clk_sz);
                self.graphics.draw_line(x + clk_szd, y - 1, x, y - clk_sz);
            }
            Direction::South => {
                self.graphics.draw_line(x - clk_szd, y + 1, x, y + clk_sz);
                self.graphics.draw_line(x + clk_szd, y + 1, x, y + clk_sz);
            }
            Direction::East => {
                self.graphics.draw_line(x + 1, y - clk_szd, x + clk_sz, y);
                self.graphics.draw_line(x + 1, y + clk_szd, x + clk_sz, y);
            }
            Direction::West => {
                self.graphics.draw_line(x - 1, y - clk_szd, x - clk_sz, y);
                self.graphics.draw_line(x - 1, y + clk_szd, x - clk_sz, y);
            }
        }

        self.graphics.set_line_width(1);
    }

    /// Draw a pin at a location
    pub fn draw_pin(&mut self, location: Location, is_input: bool) {
        let pin_rad = 4;
        let x = location.x - pin_rad / 2;
        let y = location.y - pin_rad / 2;

        if is_input {
            self.graphics.set_color(Color::GREEN);
            self.graphics.fill_oval(x, y, pin_rad, pin_rad);
        } else {
            self.graphics.set_color(Color::RED);
            self.graphics.fill_oval(x, y, pin_rad, pin_rad);
        }

        // Draw outline
        self.graphics.set_color(Color::BLACK);
        self.graphics.draw_oval(x, y, pin_rad, pin_rad);
    }

    /// Draw a wire connection point
    pub fn draw_wire_dot(&mut self, location: Location) {
        let dot_rad = 3;
        let x = location.x - dot_rad / 2;
        let y = location.y - dot_rad / 2;

        self.graphics.set_color(Color::BLACK);
        self.graphics.fill_oval(x, y, dot_rad, dot_rad);
    }
}

impl Default for ComponentDrawContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 64, 200);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 200);

        let rgb_color = Color::rgb(100, 150, 200);
        assert_eq!(rgb_color.a, 255);
    }

    #[test]
    fn test_graphics_context() {
        let mut ctx = GraphicsContext::new();
        assert_eq!(ctx.commands().len(), 0);

        ctx.set_color(Color::RED);
        ctx.draw_line(0, 0, 10, 10);
        ctx.draw_rect(5, 5, 20, 20);

        assert_eq!(ctx.commands().len(), 3);

        match &ctx.commands()[0] {
            DrawCommand::SetColor { color } => assert_eq!(*color, Color::RED),
            _ => panic!("Expected SetColor command"),
        }

        match &ctx.commands()[1] {
            DrawCommand::DrawLine { x1, y1, x2, y2 } => {
                assert_eq!(*x1, 0);
                assert_eq!(*y1, 0);
                assert_eq!(*x2, 10);
                assert_eq!(*y2, 10);
            }
            _ => panic!("Expected DrawLine command"),
        }
    }

    #[test]
    fn test_component_draw_context() {
        let mut ctx = ComponentDrawContext::new();
        assert!(ctx.show_state());
        assert!(ctx.show_color());
        assert!(!ctx.is_print_view());

        ctx.set_show_state(false);
        assert!(!ctx.show_state());

        let print_ctx = ComponentDrawContext::new_for_print();
        assert!(!print_ctx.show_state());
        assert!(!print_ctx.show_color());
        assert!(print_ctx.is_print_view());
    }

    #[test]
    fn test_drawing_operations() {
        let mut ctx = ComponentDrawContext::new();
        let location = Location::new(10, 20);

        ctx.draw_clock(location, Direction::North);
        ctx.draw_pin(location, true);
        ctx.draw_wire_dot(location);

        // Should have multiple drawing commands
        assert!(ctx.commands().len() > 0);
    }
}
