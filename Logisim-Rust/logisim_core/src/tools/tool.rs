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

//! Base tool abstraction for Logisim-RUST
//!
//! This module provides the core tool framework for the digital logic design tool.
//! Tools are the primary means of interaction with the circuit canvas, allowing users
//! to add components, make connections, and edit circuits.

use crate::{
    comp::{Component, ComponentId},
    data::{AttributeSet, Location},
};
use std::collections::HashSet;

/// Logisim version information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogisimVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl LogisimVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Component drawing context for rendering operations
#[derive(Debug, Clone)]
pub struct ComponentDrawContext {
    // Placeholder for drawing context - will be expanded as needed
    pub scale: f64,
    pub print_view: bool,
}

impl ComponentDrawContext {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            print_view: false,
        }
    }
}

/// Mouse cursor type for tools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorType {
    Default,
    Crosshair,
    Hand,
    Move,
    Text,
    Wait,
    NorthResize,
    SouthResize,
    EastResize,
    WestResize,
    NorthEastResize,
    NorthWestResize,
    SouthEastResize,
    SouthWestResize,
}

impl Default for CursorType {
    fn default() -> Self {
        CursorType::Crosshair
    }
}

/// Key event information
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key_code: u32,
    pub char_code: Option<char>,
    pub modifiers: KeyModifiers,
}

/// Mouse event information
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub location: Location,
    pub button: MouseButton,
    pub modifiers: KeyModifiers,
    pub click_count: u32,
}

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    None,
}

/// Key modifier flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyModifiers {
    pub fn new() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }

    pub fn none() -> Self {
        Self::new()
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self::new()
    }
}

/// Canvas abstraction for rendering and interaction
pub trait Canvas {
    /// Get the current project associated with this canvas
    fn get_project(&self) -> Option<&dyn Project>;

    /// Get the current selection
    fn get_selection(&self) -> &dyn Selection;

    /// Get the current zoom level
    fn get_zoom_factor(&self) -> f64;

    /// Request a repaint of the canvas
    fn repaint(&self);

    /// Convert screen coordinates to circuit coordinates
    fn screen_to_circuit(&self, screen_x: i32, screen_y: i32) -> Location;

    /// Convert circuit coordinates to screen coordinates
    fn circuit_to_screen(&self, location: Location) -> (i32, i32);
}

/// Project abstraction
pub trait Project {
    /// Get the current circuit being edited
    fn get_current_circuit(&self) -> Option<&dyn Circuit>;

    /// Perform an action on the project
    fn do_action(&mut self, action: Box<dyn Action>);
}

/// Circuit abstraction
pub trait Circuit {
    /// Get all components in the circuit
    fn get_all_components(&self) -> Vec<&dyn Component>;

    /// Add a component to the circuit
    fn add_component(&mut self, component: Box<dyn Component>);

    /// Remove a component from the circuit
    fn remove_component(&mut self, id: ComponentId);
}

/// Action abstraction for undo/redo
pub trait Action {
    /// Execute the action
    fn execute(&mut self);

    /// Undo the action
    fn undo(&mut self);

    /// Get a description of the action
    fn get_name(&self) -> String;
}

/// Selection abstraction
pub trait Selection {
    /// Get all selected components
    fn get_selected_components(&self) -> Vec<ComponentId>;

    /// Check if a component is selected
    fn is_selected(&self, id: ComponentId) -> bool;

    /// Add a component to the selection
    fn add(&mut self, id: ComponentId);

    /// Remove a component from the selection
    fn remove(&mut self, id: ComponentId);

    /// Clear the selection
    fn clear(&mut self);
}

/// Base trait for all tools in the Logisim-RUST system
///
/// Tools are the primary means of interaction with the circuit canvas.
/// They handle mouse and keyboard events, draw on the canvas, and can
/// modify the circuit through actions.
pub trait Tool: Send + Sync {
    /// Clone this tool instance
    fn clone_tool(&self) -> Box<dyn Tool>;

    /// Called when this tool is deselected from the toolbar
    fn deselect(&mut self, _canvas: &dyn Canvas) {
        // Default implementation does nothing
    }

    /// Draw the tool's visual representation on the canvas
    fn draw(&self, _canvas: &dyn Canvas, _context: &ComponentDrawContext) {
        // Default implementation does nothing
    }

    /// Get the attribute set for this tool
    fn get_attribute_set(&self) -> Option<&AttributeSet> {
        None
    }

    /// Get the attribute set for this tool with canvas context
    fn get_attribute_set_with_canvas(&self, _canvas: &dyn Canvas) -> Option<&AttributeSet> {
        self.get_attribute_set()
    }

    /// Get the cursor that should be displayed when this tool is active
    fn get_cursor(&self) -> CursorType {
        CursorType::default()
    }

    /// Get the default value for an attribute
    fn get_default_attribute_value(&self, _attr: &str, _version: &LogisimVersion) -> Option<String> {
        None
    }

    /// Get a human-readable description of this tool
    fn get_description(&self) -> String;

    /// Get the display name of this tool
    fn get_display_name(&self) -> String;

    /// Get the unique name/identifier of this tool
    fn get_name(&self) -> String;

    /// Get components that should be hidden when this tool is active
    fn get_hidden_components(&self, _canvas: &dyn Canvas) -> Option<HashSet<ComponentId>> {
        None
    }

    /// Check if all attributes have default values
    fn is_all_default_values(&self, _attrs: &AttributeSet, _version: &LogisimVersion) -> bool {
        false
    }

    /// Handle key press events
    fn key_pressed(&mut self, _canvas: &dyn Canvas, _event: &KeyEvent) {
        // Default implementation does nothing
    }

    /// Handle key release events
    fn key_released(&mut self, _canvas: &dyn Canvas, _event: &KeyEvent) {
        // Default implementation does nothing
    }

    /// Handle key typed events
    fn key_typed(&mut self, _canvas: &dyn Canvas, _event: &KeyEvent) {
        // Default implementation does nothing
    }

    /// Handle mouse drag events
    fn mouse_dragged(&mut self, _canvas: &dyn Canvas, _event: &MouseEvent) {
        // Default implementation does nothing
    }

    /// Handle mouse enter events
    fn mouse_entered(&mut self, _canvas: &dyn Canvas, _event: &MouseEvent) {
        // Default implementation does nothing
    }

    /// Handle mouse exit events
    fn mouse_exited(&mut self, _canvas: &dyn Canvas, _event: &MouseEvent) {
        // Default implementation does nothing
    }

    /// Handle mouse move events
    fn mouse_moved(&mut self, _canvas: &dyn Canvas, _event: &MouseEvent) {
        // Default implementation does nothing
    }

    /// Handle mouse press events
    fn mouse_pressed(&mut self, _canvas: &dyn Canvas, _event: &MouseEvent) {
        // Default implementation does nothing
    }

    /// Handle mouse release events
    fn mouse_released(&mut self, _canvas: &dyn Canvas, _event: &MouseEvent) {
        // Default implementation does nothing
    }

    /// Paint the tool's icon at the specified location
    fn paint_icon(&self, _context: &ComponentDrawContext, _x: i32, _y: i32) {
        // Default implementation does nothing
    }

    /// Called when this tool is selected from the toolbar
    fn select(&mut self, _canvas: &dyn Canvas) {
        // Default implementation does nothing
    }

    /// Set the attribute set for this tool
    fn set_attribute_set(&mut self, _attrs: AttributeSet) {
        // Default implementation does nothing
    }

    /// Check if this tool shares the same source as another tool
    fn shares_source(&self, _other: &dyn Tool) -> bool {
        // Default implementation: tools are equal only if they're the same instance
        // We can't use ptr::eq here due to trait object limitations
        false // Conservative default - subclasses should override
    }

    /// Get this tool as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

impl std::fmt::Debug for dyn Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tool({})", self.get_name())
    }
}

impl std::fmt::Display for dyn Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTool {
        name: String,
        description: String,
    }

    impl MockTool {
        fn new(name: &str, description: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
            }
        }
    }

    impl Tool for MockTool {
        fn clone_tool(&self) -> Box<dyn Tool> {
            Box::new(MockTool::new(&self.name, &self.description))
        }

        fn get_description(&self) -> String {
            self.description.clone()
        }

        fn get_display_name(&self) -> String {
            self.name.clone()
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_tool_basic_properties() {
        let tool = MockTool::new("test_tool", "A test tool");

        assert_eq!(tool.get_name(), "test_tool");
        assert_eq!(tool.get_description(), "A test tool");
        assert_eq!(tool.get_display_name(), "test_tool");
        assert_eq!(tool.get_cursor(), CursorType::Crosshair);
    }

    #[test]
    fn test_tool_clone() {
        let tool = MockTool::new("test_tool", "A test tool");
        let cloned = tool.clone_tool();

        assert_eq!(tool.get_name(), cloned.get_name());
        assert_eq!(tool.get_description(), cloned.get_description());
    }

    #[test]
    fn test_key_modifiers() {
        let mut modifiers = KeyModifiers::new();
        assert!(!modifiers.shift);
        assert!(!modifiers.ctrl);
        assert!(!modifiers.alt);
        assert!(!modifiers.meta);

        modifiers.shift = true;
        modifiers.ctrl = true;
        assert!(modifiers.shift);
        assert!(modifiers.ctrl);
        assert!(!modifiers.alt);
        assert!(!modifiers.meta);
    }

    #[test]
    fn test_cursor_type_default() {
        assert_eq!(CursorType::default(), CursorType::Crosshair);
    }
}
