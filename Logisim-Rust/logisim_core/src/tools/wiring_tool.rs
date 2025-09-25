/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 *
 * Ported to Rust by the Logisim-RUST project
 * https://github.com/crossplatformdev/Logisim-RUST
 */

//! WiringTool for creating wire connections between components
//!
//! The WiringTool allows users to create electrical connections (wires) between
//! component pins. It supports both horizontal and vertical wire segments and
//! provides visual feedback during wire placement.

use crate::{
    component::{Component, ComponentId},
    data::{Direction, Location},
    tools::{
        tool::{
            Canvas, ComponentDrawContext, CursorType, KeyEvent, KeyModifiers, LogisimVersion,
            MouseButton, MouseEvent, Tool,
        },
        ToolResult,
    },
};
use std::collections::HashSet;

/// Direction constraint for wire placement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WireDirection {
    /// No direction constraint yet
    None,
    /// Wire must be horizontal
    Horizontal,
    /// Wire must be vertical
    Vertical,
}

/// State of wire placement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WiringState {
    /// No active wire placement
    Idle,
    /// Wire placement in progress
    Placing,
    /// Wire repair operation
    Repairing,
}

/// Represents a wire segment being placed
#[derive(Debug, Clone)]
struct WireSegment {
    /// Starting location of the wire
    start: Location,
    /// Current end location of the wire
    end: Location,
    /// Direction constraint
    direction: WireDirection,
    /// Wire width (for bus support)
    width: i32,
}

impl WireSegment {
    /// Create a new wire segment
    fn new(start: Location) -> Self {
        Self {
            start,
            end: start,
            direction: WireDirection::None,
            width: 1, // Default to single bit
        }
    }

    /// Update the end location, respecting direction constraints
    fn update_end(&mut self, new_end: Location) -> bool {
        // Check if location actually changed
        if self.end == new_end {
            return false;
        }

        let old_end = self.end;

        // Determine or maintain direction constraint
        match self.direction {
            WireDirection::None => {
                // First movement determines direction
                if new_end.x != self.start.x {
                    self.direction = WireDirection::Horizontal;
                    self.end = Location::new(new_end.x, self.start.y);
                } else if new_end.y != self.start.y {
                    self.direction = WireDirection::Vertical;
                    self.end = Location::new(self.start.x, new_end.y);
                } else {
                    self.end = new_end;
                }
            }
            WireDirection::Horizontal => {
                // Keep y coordinate from start
                self.end = Location::new(new_end.x, self.start.y);

                // Check if we should switch to vertical
                if new_end.x == self.start.x {
                    if new_end.y == self.start.y {
                        self.direction = WireDirection::None;
                    } else {
                        self.direction = WireDirection::Vertical;
                        self.end = Location::new(self.start.x, new_end.y);
                    }
                }
            }
            WireDirection::Vertical => {
                // Keep x coordinate from start
                self.end = Location::new(self.start.x, new_end.y);

                // Check if we should switch to horizontal
                if new_end.y == self.start.y {
                    if new_end.x == self.start.x {
                        self.direction = WireDirection::None;
                    } else {
                        self.direction = WireDirection::Horizontal;
                        self.end = Location::new(new_end.x, self.start.y);
                    }
                }
            }
        }

        old_end != self.end
    }

    /// Get the length of this wire segment
    fn length(&self) -> i32 {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        dx + dy
    }

    /// Check if this is a valid wire (has non-zero length)
    fn is_valid(&self) -> bool {
        self.start != self.end
    }

    /// Check if this wire is horizontal
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    /// Check if this wire is vertical
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

/// Tool for creating wire connections between components
///
/// This tool allows users to create electrical connections by:
/// - Clicking to start a wire
/// - Moving the mouse to position the wire
/// - Clicking again to complete the wire
/// - Supporting both horizontal and vertical wire segments
pub struct WiringTool {
    /// Current state of the tool
    state: WiringState,

    /// Wire segment currently being placed
    current_wire: Option<WireSegment>,

    /// Whether the mouse is currently in the canvas
    in_canvas: bool,

    /// Whether the user has dragged since starting the wire
    has_dragged: bool,

    /// Current mouse location for feedback
    current_location: Location,
}

impl WiringTool {
    /// Unique identifier for the WiringTool
    pub const ID: &'static str = "Wiring Tool";

    /// Minimum wire length for repair operations
    const MIN_REPAIR_LENGTH: i32 = 10;

    /// Create a new WiringTool
    pub fn new() -> Self {
        Self {
            state: WiringState::Idle,
            current_wire: None,
            in_canvas: false,
            has_dragged: false,
            current_location: Location::new(0, 0),
        }
    }

    /// Find connection points at the given location
    fn find_connection_points(&self, canvas: &dyn Canvas, location: Location) -> Vec<Location> {
        let mut points = Vec::new();

        if let Some(project) = canvas.get_project() {
            if let Some(circuit) = project.get_current_circuit() {
                // Find component pins at this location
                for component in circuit.get_all_components() {
                    // TODO: Check component pins and add connection points
                }

                // Find existing wire endpoints
                // TODO: Check existing wires for connection points
            }
        }

        points
    }

    /// Check if a location is valid for starting or ending a wire
    fn is_valid_connection_point(&self, canvas: &dyn Canvas, location: Location) -> bool {
        // TODO: Implement proper connection point validation
        // - Check for component pins
        // - Check for existing wire endpoints
        // - Check for grid alignment if required
        true
    }

    /// Start placing a new wire
    fn start_wire_placement(&mut self, canvas: &dyn Canvas, location: Location) -> ToolResult<()> {
        if !self.is_valid_connection_point(canvas, location) {
            return Ok(()); // Cannot start wire here
        }

        self.state = WiringState::Placing;
        self.current_wire = Some(WireSegment::new(location));
        self.has_dragged = false;

        canvas.repaint();
        Ok(())
    }

    /// Update wire placement during mouse movement
    fn update_wire_placement(&mut self, canvas: &dyn Canvas, location: Location) -> ToolResult<()> {
        if let Some(ref mut wire) = self.current_wire {
            let changed = wire.update_end(location);
            if changed {
                canvas.repaint();
            }
        }
        Ok(())
    }

    /// Complete wire placement
    fn complete_wire_placement(&mut self, canvas: &dyn Canvas) -> ToolResult<()> {
        if let Some(wire) = self.current_wire.take() {
            if wire.is_valid() && self.is_valid_connection_point(canvas, wire.end) {
                // TODO: Create and execute wire creation action
                // This would involve:
                // 1. Creating a Wire component
                // 2. Adding it to the circuit
                // 3. Creating an action for undo/redo

                if let Some(project) = canvas.get_project() {
                    // TODO: project.do_action(Box::new(AddWireAction::new(wire)));
                }
            }
        }

        self.state = WiringState::Idle;
        self.current_wire = None;
        self.has_dragged = false;

        canvas.repaint();
        Ok(())
    }

    /// Cancel current wire placement
    fn cancel_wire_placement(&mut self, canvas: &dyn Canvas) {
        self.state = WiringState::Idle;
        self.current_wire = None;
        self.has_dragged = false;
        canvas.repaint();
    }

    /// Check for wire repair opportunities
    fn check_wire_repair(&self, canvas: &dyn Canvas, location: Location) -> Option<WireRepairInfo> {
        // TODO: Implement wire repair detection
        // This would look for wires that could be shortened or rerouted
        // to better connect to components
        None
    }

    /// Snap location to grid if enabled
    fn snap_to_grid(&self, location: Location) -> Location {
        // TODO: Implement grid snapping based on preferences
        // For now, return location as-is
        location
    }
}

impl Tool for WiringTool {
    fn clone_tool(&self) -> Box<dyn Tool> {
        Box::new(WiringTool::new())
    }

    fn get_cursor(&self) -> CursorType {
        CursorType::Crosshair
    }

    fn get_description(&self) -> String {
        "Create wire connections between components".to_string()
    }

    fn get_display_name(&self) -> String {
        "Wiring Tool".to_string()
    }

    fn get_name(&self) -> String {
        Self::ID.to_string()
    }

    fn draw(&self, canvas: &dyn Canvas, context: &ComponentDrawContext) {
        if let Some(ref wire) = self.current_wire {
            if wire.is_valid() {
                // TODO: Draw the wire being placed
                // This would typically draw a line from start to end
                // with appropriate styling (color, thickness, etc.)
            }
        }

        // TODO: Draw connection point indicators
        // Show where valid connection points are available
    }

    fn mouse_pressed(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        match event.button {
            MouseButton::Left => {
                let location = self.snap_to_grid(event.location);

                match self.state {
                    WiringState::Idle => {
                        // Start new wire
                        if let Err(e) = self.start_wire_placement(canvas, location) {
                            eprintln!("Error starting wire placement: {}", e);
                        }
                    }
                    WiringState::Placing => {
                        // Complete current wire
                        if let Err(e) = self.complete_wire_placement(canvas) {
                            eprintln!("Error completing wire placement: {}", e);
                        }
                    }
                    WiringState::Repairing => {
                        // TODO: Handle wire repair completion
                    }
                }
            }
            MouseButton::Right => {
                // Cancel current operation
                if self.state != WiringState::Idle {
                    self.cancel_wire_placement(canvas);
                }
            }
            _ => {
                // Ignore other buttons
            }
        }
    }

    fn mouse_moved(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        let location = self.snap_to_grid(event.location);
        self.current_location = location;

        match self.state {
            WiringState::Placing => {
                if let Err(e) = self.update_wire_placement(canvas, location) {
                    eprintln!("Error updating wire placement: {}", e);
                }
            }
            _ => {
                // Just update cursor feedback
                canvas.repaint();
            }
        }
    }

    fn mouse_dragged(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        self.has_dragged = true;
        self.mouse_moved(canvas, event);
    }

    fn mouse_entered(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        self.in_canvas = true;
        self.current_location = event.location;
    }

    fn mouse_exited(&mut self, canvas: &dyn Canvas, _event: &MouseEvent) {
        self.in_canvas = false;
        // Cancel wire placement when leaving canvas
        if self.state == WiringState::Placing {
            self.cancel_wire_placement(canvas);
        }
    }

    fn key_pressed(&mut self, canvas: &dyn Canvas, event: &KeyEvent) {
        // TODO: Handle keyboard shortcuts
        // - Escape to cancel current wire
        // - Shift to constrain direction
        // - Tab to cycle through connection points
    }

    fn deselect(&mut self, canvas: &dyn Canvas) {
        // Cancel any in-progress wire placement
        if self.state != WiringState::Idle {
            self.cancel_wire_placement(canvas);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for WiringTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a wire repair opportunity
#[derive(Debug, Clone)]
struct WireRepairInfo {
    /// The wire that could be repaired
    wire_id: ComponentId,
    /// Original end location
    original_end: Location,
    /// Suggested new end location
    suggested_end: Location,
    /// Reason for the repair
    reason: String,
}

/// Action for adding a wire to the circuit
pub struct AddWireAction {
    wire_segment: WireSegment,
    circuit_id: Option<String>,
}

impl AddWireAction {
    pub fn new(wire_segment: WireSegment) -> Self {
        Self {
            wire_segment,
            circuit_id: None,
        }
    }
}

// TODO: Implement Action trait for AddWireAction

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_segment_creation() {
        let start = Location::new(10, 20);
        let segment = WireSegment::new(start);

        assert_eq!(segment.start, start);
        assert_eq!(segment.end, start);
        assert_eq!(segment.direction, WireDirection::None);
        assert!(!segment.is_valid());
        assert_eq!(segment.length(), 0);
    }

    #[test]
    fn test_wire_segment_direction_detection() {
        let start = Location::new(10, 20);
        let mut segment = WireSegment::new(start);

        // First horizontal movement
        segment.update_end(Location::new(30, 20));
        assert_eq!(segment.direction, WireDirection::Horizontal);
        assert!(segment.is_horizontal());
        assert!(!segment.is_vertical());
        assert!(segment.is_valid());
        assert_eq!(segment.length(), 20);

        // Switch to vertical
        segment.update_end(Location::new(10, 40));
        assert_eq!(segment.direction, WireDirection::Vertical);
        assert!(!segment.is_horizontal());
        assert!(segment.is_vertical());
        assert_eq!(segment.length(), 20);
    }

    #[test]
    fn test_wiring_tool_creation() {
        let tool = WiringTool::new();

        assert_eq!(tool.get_name(), WiringTool::ID);
        assert_eq!(tool.get_display_name(), "Wiring Tool");
        assert_eq!(tool.state, WiringState::Idle);
        assert_eq!(tool.get_cursor(), CursorType::Crosshair);
        assert!(tool.current_wire.is_none());
    }

    #[test]
    fn test_snap_to_grid() {
        let tool = WiringTool::new();
        let location = Location::new(15, 27);

        // Currently just returns the same location
        let snapped = tool.snap_to_grid(location);
        assert_eq!(snapped, location);
    }
}
