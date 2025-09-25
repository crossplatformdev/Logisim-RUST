/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Painter for Component Rendering
//!
//! This module provides the `InstancePainter` struct for rendering component instances.
//! This is equivalent to Java's `InstancePainter` class.

use crate::data::{AttributeSet, Bounds, Direction};
use crate::{Value};
use crate::instance::{Instance, InstanceFactory, InstanceState, Port};

/// Drawing context and utilities for rendering component instances.
///
/// This struct provides a high-level interface for component rendering operations,
/// abstracting over the underlying graphics system. It is equivalent to Java's
/// `InstancePainter` class.
///
/// # Design
///
/// The InstancePainter serves multiple roles:
/// - Drawing context with access to graphics primitives
/// - Component state accessor (implements InstanceState)
/// - Rendering utilities specific to digital logic components
/// - Bridge between the instance system and the graphics backend
///
/// # Example Usage
///
/// ```rust
/// use logisim_core::instance::InstancePainter;
/// use logisim_core::{Bounds, Location, Value};
///
/// fn paint_and_gate(painter: &mut InstancePainter) {
///     let bounds = painter.get_bounds();
///     
///     // Draw the gate body
///     painter.draw_rectangle(bounds.x(), bounds.y(), bounds.width()/2, bounds.height());
///     painter.draw_arc(bounds.x() + bounds.width()/2, bounds.y(), 
///                     bounds.width()/2, bounds.height(), 90, 180);
///     
///     // Draw input/output pins
///     painter.draw_pin(0, bounds.x(), bounds.y() + bounds.height()/3);
///     painter.draw_pin(1, bounds.x(), bounds.y() + 2*bounds.height()/3);  
///     painter.draw_pin(2, bounds.x() + bounds.width(), bounds.y() + bounds.height()/2);
/// }
/// ```
#[derive(Debug)]
pub struct InstancePainter {
    /// Reference to the instance being painted
    instance: Option<Instance>,
    /// Current graphics bounds for drawing operations
    bounds: Bounds,
    /// Whether we're in icon painting mode
    is_icon_mode: bool,
    /// Whether we're in ghost/preview mode
    is_ghost_mode: bool,
}

impl InstancePainter {
    /// Creates a new instance painter.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The drawing bounds for the component
    ///
    /// # Returns
    ///
    /// A new InstancePainter ready for drawing operations.
    pub fn new(bounds: Bounds) -> Self {
        Self {
            instance: None,
            bounds,
            is_icon_mode: false,
            is_ghost_mode: false,
        }
    }

    /// Sets the instance being painted.
    ///
    /// # Arguments
    ///
    /// * `instance` - The component instance to paint
    pub fn set_instance(&mut self, instance: Instance) {
        self.instance = Some(instance);
    }

    /// Sets icon painting mode.
    ///
    /// In icon mode, components should render a simplified version
    /// suitable for toolbars and component palettes.
    pub fn set_icon_mode(&mut self, is_icon: bool) {
        self.is_icon_mode = is_icon;
    }

    /// Sets ghost/preview painting mode.
    ///
    /// In ghost mode, components should render in a translucent or
    /// outline style for placement previews.
    pub fn set_ghost_mode(&mut self, is_ghost: bool) {
        self.is_ghost_mode = is_ghost;
    }

    /// Returns the current drawing bounds.
    pub fn get_bounds(&self) -> Bounds {
        self.bounds
    }

    /// Checks if we're in icon painting mode.
    pub fn is_icon_mode(&self) -> bool {
        self.is_icon_mode
    }

    /// Checks if we're in ghost painting mode.
    pub fn is_ghost_mode(&self) -> bool {
        self.is_ghost_mode
    }

    /// Returns the instance being painted, if available.
    pub fn get_instance(&self) -> Option<&Instance> {
        self.instance.as_ref()
    }

    // Graphics primitives - these would interface with the actual graphics backend

    /// Draws a rectangle.
    ///
    /// # Arguments
    ///
    /// * `x`, `y` - Top-left corner coordinates
    /// * `width`, `height` - Rectangle dimensions
    pub fn draw_rectangle(&self, x: i32, y: i32, width: u32, height: u32) {
        // In a full implementation, this would call the graphics backend
        // For now, this is a stub for compilation
        let _ = (x, y, width, height);
    }

    /// Draws a filled rectangle.
    ///
    /// # Arguments
    ///
    /// * `x`, `y` - Top-left corner coordinates  
    /// * `width`, `height` - Rectangle dimensions
    pub fn fill_rectangle(&self, x: i32, y: i32, width: u32, height: u32) {
        let _ = (x, y, width, height);
    }

    /// Draws an oval/ellipse.
    ///
    /// # Arguments
    ///
    /// * `x`, `y` - Bounding box top-left coordinates
    /// * `width`, `height` - Bounding box dimensions
    pub fn draw_oval(&self, x: i32, y: i32, width: u32, height: u32) {
        let _ = (x, y, width, height);
    }

    /// Draws a filled oval/ellipse.
    pub fn fill_oval(&self, x: i32, y: i32, width: u32, height: u32) {
        let _ = (x, y, width, height);
    }

    /// Draws an arc.
    ///
    /// # Arguments
    ///
    /// * `x`, `y` - Bounding box top-left coordinates
    /// * `width`, `height` - Bounding box dimensions
    /// * `start_angle` - Starting angle in degrees
    /// * `arc_angle` - Arc span in degrees
    pub fn draw_arc(&self, x: i32, y: i32, width: u32, height: u32, start_angle: i32, arc_angle: i32) {
        let _ = (x, y, width, height, start_angle, arc_angle);
    }

    /// Draws a line.
    ///
    /// # Arguments
    ///
    /// * `x1`, `y1` - Starting point coordinates
    /// * `x2`, `y2` - Ending point coordinates
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let _ = (x1, y1, x2, y2);
    }

    /// Draws text at the specified location.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to draw
    /// * `x`, `y` - Text position coordinates
    pub fn draw_text(&self, text: &str, x: i32, y: i32) {
        let _ = (text, x, y);
    }

    // Digital logic specific drawing utilities

    /// Draws a component pin/port.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Index of the port being drawn
    /// * `x`, `y` - Pin coordinates
    pub fn draw_pin(&self, port_index: usize, x: i32, y: i32) {
        let _ = (port_index, x, y);
        // In a full implementation, this would:
        // 1. Get the port definition
        // 2. Draw the pin based on port type and current value
        // 3. Add any necessary visual indicators (direction, width, etc.)
    }

    /// Draws a clock input symbol at the specified port.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Index of the clock port
    /// * `direction` - Direction the clock symbol faces
    pub fn draw_clock(&self, port_index: usize, direction: Direction) {
        let _ = (port_index, direction);
        // Draws the triangular clock input symbol
    }

    /// Draws the component's bounding box (for debugging).
    pub fn draw_bounds(&self) {
        let bounds = self.bounds;
        self.draw_rectangle(bounds.get_x(), bounds.get_y(), bounds.get_width() as u32, bounds.get_height() as u32);
    }

    /// Draws a connection dot at the specified location.
    ///
    /// # Arguments
    ///
    /// * `x`, `y` - Dot center coordinates
    pub fn draw_dot(&self, x: i32, y: i32) {
        let _ = (x, y);
        // Typically a small filled circle indicating a connection point
    }

    /// Sets the drawing color.
    ///
    /// # Arguments
    ///
    /// * `red`, `green`, `blue` - RGB color components (0-255)
    pub fn set_color(&self, red: u8, green: u8, blue: u8) {
        let _ = (red, green, blue);
    }

    /// Sets the line width for drawing operations.
    ///
    /// # Arguments
    ///
    /// * `width` - Line width in pixels
    pub fn set_line_width(&self, width: u32) {
        let _ = width;
    }
}

// Implement InstanceState for InstancePainter to provide component context
impl InstanceState for InstancePainter {
    fn fire_invalidated(&mut self) {
        if let Some(instance) = &self.instance {
            instance.fire_invalidated();
        }
    }

    fn get_attribute_set(&self) -> &AttributeSet {
        self.instance
            .as_ref()
            .map(|i| i.attribute_set())
            .expect("No instance set for painter")
    }

    fn get_attribute_value_erased(&self, attr: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        // Would need proper attribute access through instance
        let _ = attr;
        None
    }

    fn get_data(&self) -> Option<&dyn crate::instance::InstanceData> {
        // Would need access to simulation context
        None
    }

    fn get_data_mut(&mut self) -> Option<&mut (dyn crate::instance::InstanceData + '_)> {
        // Would need access to simulation context
        None
    }

    fn get_factory(&self) -> &dyn InstanceFactory {
        self.instance
            .as_ref()
            .map(|i| i.factory())
            .expect("No instance set for painter")
    }

    fn get_instance(&self) -> &Instance {
        self.instance
            .as_ref()
            .expect("No instance set for painter")
    }

    fn get_port_index(&self, port: &Port) -> Option<usize> {
        self.instance
            .as_ref()?
            .ports()
            .iter()
            .position(|p| std::ptr::eq(p, port))
    }

    fn get_port_value(&self, _port_index: usize) -> Value {
        // Would need access to simulation context
        Value::Unknown
    }

    fn get_port_net(&self, _port_index: usize) -> Option<crate::netlist::NetId> {
        // Would need access to simulation context
        None
    }

    fn get_tick_count(&self) -> u64 {
        // Would need access to simulation context
        0
    }

    fn get_timestamp(&self) -> crate::signal::Timestamp {
        // Would need access to simulation context
        crate::signal::Timestamp::new(0)
    }

    fn is_circuit_root(&self) -> bool {
        // Would need access to simulation context
        true
    }

    fn is_port_connected(&self, _port_index: usize) -> bool {
        // Would need access to simulation context
        false
    }

    fn set_data(&mut self, _data: Box<dyn crate::instance::InstanceData>) {
        // Would need access to simulation context
        panic!("Cannot set data from painter context");
    }

    fn set_port_value(&mut self, _port_index: usize, _value: Value, _delay: u32) {
        // Would need access to simulation context
        panic!("Cannot set port values from painter context");
    }

    fn schedule_evaluation(&mut self, _delay: u32) {
        // Would need access to simulation context
        panic!("Cannot schedule evaluation from painter context");
    }

    fn get_port(&self, index: usize) -> Option<&Port> {
        self.instance
            .as_ref()
            .and_then(|i| i.get_port(index))
    }

    fn get_port_count(&self) -> usize {
        self.instance
            .as_ref()
            .map(|i| i.ports().len())
            .unwrap_or(0)
    }

    fn is_input_port(&self, port_index: usize) -> bool {
        self.get_port(port_index)
            .map(|p| matches!(p.port_type(), crate::instance::PortType::Input | crate::instance::PortType::InOut))
            .unwrap_or(false)
    }

    fn is_output_port(&self, port_index: usize) -> bool {
        self.get_port(port_index)
            .map(|p| matches!(p.port_type(), crate::instance::PortType::Output | crate::instance::PortType::InOut))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_painter_creation() {
        let bounds = Bounds::new(10, 20, 30, 40);
        let painter = InstancePainter::new(bounds);

        assert_eq!(painter.get_bounds(), bounds);
        assert!(!painter.is_icon_mode());
        assert!(!painter.is_ghost_mode());
        assert!(painter.get_instance().is_none());
    }

    #[test]
    fn test_painter_modes() {
        let bounds = Bounds::new(0, 0, 100, 100);
        let mut painter = InstancePainter::new(bounds);

        // Test icon mode
        painter.set_icon_mode(true);
        assert!(painter.is_icon_mode());
        assert!(!painter.is_ghost_mode());

        // Test ghost mode
        painter.set_ghost_mode(true);
        assert!(painter.is_icon_mode());
        assert!(painter.is_ghost_mode());

        // Turn off icon mode
        painter.set_icon_mode(false);
        assert!(!painter.is_icon_mode());
        assert!(painter.is_ghost_mode());
    }

    #[test]
    fn test_graphics_primitives() {
        let bounds = Bounds::new(0, 0, 50, 50);
        let painter = InstancePainter::new(bounds);

        // These should not panic (just stub implementations)
        painter.draw_rectangle(0, 0, 10, 10);
        painter.fill_rectangle(5, 5, 20, 20);
        painter.draw_oval(10, 10, 15, 15);
        painter.fill_oval(20, 20, 10, 10);
        painter.draw_arc(0, 0, 30, 30, 0, 90);
        painter.draw_line(0, 0, 25, 25);
        painter.draw_text("Test", 5, 15);
        painter.draw_pin(0, 0, 0);
        painter.draw_bounds();
        painter.draw_dot(25, 25);
        painter.set_color(255, 0, 0);
        painter.set_line_width(2);
    }

    #[test]
    fn test_instance_state_interface() {
        let bounds = Bounds::new(0, 0, 100, 100);
        let painter = InstancePainter::new(bounds);

        // Without an instance set, most operations should return defaults
        assert_eq!(painter.get_port_count(), 0);
        assert!(painter.get_port(0).is_none());
        assert_eq!(painter.get_port_value(0), Value::Unknown);
        assert_eq!(painter.get_tick_count(), 0);
        assert!(painter.is_circuit_root());
        assert!(!painter.is_port_connected(0));
        assert!(!painter.is_input_port(0));
        assert!(!painter.is_output_port(0));
    }
}