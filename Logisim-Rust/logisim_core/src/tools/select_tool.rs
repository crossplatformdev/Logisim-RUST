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

//! SelectTool for selecting and manipulating components
//!
//! The SelectTool is the primary interaction tool that allows users to:
//! - Select individual components by clicking
//! - Select multiple components with rectangle selection
//! - Move selected components by dragging
//! - Access component properties and context menus

use crate::{
    component::ComponentId,
    data::{Bounds, Location},
    tools::tool::{
        Canvas, ComponentDrawContext, CursorType, KeyEvent, KeyModifiers, MouseButton, MouseEvent,
        Tool,
    },
};
use std::collections::HashSet;

/// State of the SelectTool during interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectToolState {
    /// No active interaction
    Idle,
    /// Moving selected components
    Moving,
    /// Rectangle selection in progress
    RectSelect,
}

/// Selection mode for how selection changes are applied
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionMode {
    /// Replace current selection
    Replace,
    /// Add to current selection (Shift+click)
    Add,
    /// Toggle component in selection (Ctrl+click)
    Toggle,
}

/// Tool for selecting and manipulating circuit components
///
/// This is the primary interaction tool that handles:
/// - Component selection (single and multiple)
/// - Moving components
/// - Rectangle selection
/// - Accessing component properties
pub struct SelectTool {
    /// Current state of the tool
    state: SelectToolState,

    /// Starting location for drag operations
    start_location: Option<Location>,

    /// Current drag offset
    current_dx: i32,
    current_dy: i32,

    /// Whether to draw connection feedback
    draw_connections: bool,

    /// Components that were selected when the current operation started
    initial_selection: HashSet<ComponentId>,

    /// Rectangle selection bounds (start and current)
    selection_rect: Option<(Location, Location)>,
}

impl SelectTool {
    /// Unique identifier for the SelectTool
    pub const ID: &'static str = "Select Tool";

    /// Create a new SelectTool
    pub fn new() -> Self {
        Self {
            state: SelectToolState::Idle,
            start_location: None,
            current_dx: 0,
            current_dy: 0,
            draw_connections: false,
            initial_selection: HashSet::new(),
            selection_rect: None,
        }
    }

    /// Determine selection mode based on key modifiers
    fn get_selection_mode(&self, modifiers: &KeyModifiers) -> SelectionMode {
        if modifiers.shift {
            SelectionMode::Add
        } else if modifiers.ctrl {
            SelectionMode::Toggle
        } else {
            SelectionMode::Replace
        }
    }

    /// Find component at the given location
    fn find_component_at(&self, canvas: &dyn Canvas, location: Location) -> Option<ComponentId> {
        if let Some(project) = canvas.get_project() {
            if let Some(circuit) = project.get_current_circuit() {
                // Find the topmost component at this location
                for component in circuit.get_all_components() {
                    // TODO: Check if location is within component bounds
                    // This would require proper bounds checking
                }
            }
        }
        None
    }

    /// Get all components within the selection rectangle
    fn find_components_in_rect(
        &self,
        canvas: &dyn Canvas,
        start: Location,
        end: Location,
    ) -> Vec<ComponentId> {
        let mut components = Vec::new();

        if let Some(project) = canvas.get_project() {
            if let Some(circuit) = project.get_current_circuit() {
                let selection_bounds = Bounds::from_locations(start, end);

                for component in circuit.get_all_components() {
                    // TODO: Check if component intersects with selection rectangle
                    // This would require proper bounds checking
                }
            }
        }

        components
    }

    /// Apply selection changes based on mode
    fn apply_selection(
        &self,
        _canvas: &dyn Canvas,
        _components: Vec<ComponentId>,
        _mode: SelectionMode,
    ) {
        // TODO: Implement proper selection application
        // This would require access to the circuit's selection mechanism
    }

    /// Handle single click selection
    fn handle_single_click(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        let mode = self.get_selection_mode(&event.modifiers);

        if let Some(component_id) = self.find_component_at(canvas, event.location) {
            // Select the clicked component
            self.apply_selection(canvas, vec![component_id], mode);
        } else {
            // Clicked on empty space
            match mode {
                SelectionMode::Replace => {
                    // Clear selection
                    self.apply_selection(canvas, vec![], SelectionMode::Replace);
                }
                _ => {
                    // Don't change selection for Add/Toggle on empty space
                }
            }
        }
    }

    /// Start a move operation
    fn start_move(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        self.state = SelectToolState::Moving;
        self.start_location = Some(event.location);
        self.current_dx = 0;
        self.current_dy = 0;

        // Store initial selection for potential undo
        // TODO: Get current selection from canvas
        self.initial_selection.clear();
    }

    /// Start a rectangle selection
    fn start_rect_select(&mut self, event: &MouseEvent) {
        self.state = SelectToolState::RectSelect;
        self.start_location = Some(event.location);
        self.selection_rect = Some((event.location, event.location));
    }

    /// Update move operation
    fn update_move(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        if let Some(start) = self.start_location {
            self.current_dx = event.location.x - start.x;
            self.current_dy = event.location.y - start.y;

            // TODO: Update component positions and repaint
            canvas.repaint();
        }
    }

    /// Update rectangle selection
    fn update_rect_select(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        if let Some(start) = self.start_location {
            self.selection_rect = Some((start, event.location));
            canvas.repaint();
        }
    }

    /// Complete move operation
    fn complete_move(&mut self, canvas: &dyn Canvas) {
        if self.current_dx != 0 || self.current_dy != 0 {
            // TODO: Create and execute move action for undo/redo
        }

        self.state = SelectToolState::Idle;
        self.start_location = None;
        self.current_dx = 0;
        self.current_dy = 0;
    }

    /// Complete rectangle selection
    fn complete_rect_select(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        if let Some((start, _)) = self.selection_rect {
            let mode = self.get_selection_mode(&event.modifiers);
            let components = self.find_components_in_rect(canvas, start, event.location);
            self.apply_selection(canvas, components, mode);
        }

        self.state = SelectToolState::Idle;
        self.start_location = None;
        self.selection_rect = None;
    }
}

impl Tool for SelectTool {
    fn clone_tool(&self) -> Box<dyn Tool> {
        Box::new(SelectTool::new())
    }

    fn get_cursor(&self) -> CursorType {
        match self.state {
            SelectToolState::Idle => CursorType::Default,
            SelectToolState::Moving => CursorType::Move,
            SelectToolState::RectSelect => CursorType::Crosshair,
        }
    }

    fn get_description(&self) -> String {
        "Select and move circuit components".to_string()
    }

    fn get_display_name(&self) -> String {
        "Select Tool".to_string()
    }

    fn get_name(&self) -> String {
        Self::ID.to_string()
    }

    fn draw(&self, canvas: &dyn Canvas, context: &ComponentDrawContext) {
        match self.state {
            SelectToolState::Moving => {
                // TODO: Draw selection with move feedback
                // This would show the selected components at their new positions
            }
            SelectToolState::RectSelect => {
                // TODO: Draw selection rectangle
                if let Some((start, end)) = self.selection_rect {
                    // Draw rectangle from start to end
                }
            }
            SelectToolState::Idle => {
                // TODO: Draw selection handles for selected components
            }
        }
    }

    fn mouse_pressed(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        match event.button {
            MouseButton::Left => {
                let component_at_location = self.find_component_at(canvas, event.location);

                match component_at_location {
                    Some(_component_id) => {
                        // TODO: Check if component is already selected
                        // If selected, start move operation
                        // If not selected, select it and potentially start move
                        self.handle_single_click(canvas, event);
                        // For now, assume we start moving
                        self.start_move(canvas, event);
                    }
                    None => {
                        // Clicked on empty space - start rectangle selection
                        self.start_rect_select(event);
                    }
                }
            }
            MouseButton::Right => {
                // TODO: Show context menu
            }
            _ => {
                // Ignore other buttons
            }
        }
    }

    fn mouse_dragged(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        match self.state {
            SelectToolState::Moving => {
                self.update_move(canvas, event);
            }
            SelectToolState::RectSelect => {
                self.update_rect_select(canvas, event);
            }
            SelectToolState::Idle => {
                // Shouldn't happen, but handle gracefully
            }
        }
    }

    fn mouse_released(&mut self, canvas: &dyn Canvas, event: &MouseEvent) {
        match self.state {
            SelectToolState::Moving => {
                self.complete_move(canvas);
            }
            SelectToolState::RectSelect => {
                self.complete_rect_select(canvas, event);
            }
            SelectToolState::Idle => {
                // Single click without drag
                self.handle_single_click(canvas, event);
            }
        }
    }

    fn key_pressed(&mut self, canvas: &dyn Canvas, event: &KeyEvent) {
        // TODO: Handle keyboard shortcuts
        // - Delete key to delete selected components
        // - Ctrl+A to select all
        // - Escape to clear selection
        // - Arrow keys to nudge selected components
    }

    fn select(&mut self, canvas: &dyn Canvas) {
        // TODO: Set up selection listener
    }

    fn deselect(&mut self, canvas: &dyn Canvas) {
        // Cancel any in-progress operations
        self.state = SelectToolState::Idle;
        self.start_location = None;
        self.selection_rect = None;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for SelectTool {
    fn default() -> Self {
        Self::new()
    }
}

// Helper implementation for Bounds
impl Bounds {
    pub fn from_locations(start: Location, end: Location) -> Self {
        let min_x = start.x.min(end.x);
        let min_y = start.y.min(end.y);
        let max_x = start.x.max(end.x);
        let max_y = start.y.max(end.y);

        Bounds {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_tool_creation() {
        let tool = SelectTool::new();

        assert_eq!(tool.get_name(), SelectTool::ID);
        assert_eq!(tool.get_display_name(), "Select Tool");
        assert_eq!(tool.state, SelectToolState::Idle);
        assert_eq!(tool.get_cursor(), CursorType::Default);
    }

    #[test]
    fn test_selection_mode_determination() {
        let tool = SelectTool::new();

        let no_mods = KeyModifiers::new();
        assert_eq!(tool.get_selection_mode(&no_mods), SelectionMode::Replace);

        let shift_mods = KeyModifiers {
            shift: true,
            ..KeyModifiers::new()
        };
        assert_eq!(tool.get_selection_mode(&shift_mods), SelectionMode::Add);

        let ctrl_mods = KeyModifiers {
            ctrl: true,
            ..KeyModifiers::new()
        };
        assert_eq!(tool.get_selection_mode(&ctrl_mods), SelectionMode::Toggle);
    }

    #[test]
    fn test_state_transitions() {
        let mut tool = SelectTool::new();

        // Initial state
        assert_eq!(tool.state, SelectToolState::Idle);

        // Start rectangle selection
        let event = MouseEvent {
            location: Location::new(10, 20),
            button: MouseButton::Left,
            modifiers: KeyModifiers::new(),
            click_count: 1,
        };

        tool.start_rect_select(&event);
        assert_eq!(tool.state, SelectToolState::RectSelect);
        assert!(tool.selection_rect.is_some());

        // TODO: Test other state transitions with mock canvas
    }

    #[test]
    fn test_bounds_from_locations() {
        let start = Location::new(10, 20);
        let end = Location::new(30, 40);

        let bounds = Bounds::from_locations(start, end);
        assert_eq!(bounds.x, 10);
        assert_eq!(bounds.y, 20);
        assert_eq!(bounds.width, 20);
        assert_eq!(bounds.height, 20);

        // Test with reversed coordinates
        let bounds2 = Bounds::from_locations(end, start);
        assert_eq!(bounds2, bounds);
    }
}
