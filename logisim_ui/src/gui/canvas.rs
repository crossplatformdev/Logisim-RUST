//! Canvas implementation for schematic editing - equivalent to the Java Canvas class

use eframe::egui::{self, Ui, Painter, Rect, Pos2, Vec2, Color32, Stroke, Response};
use logisim_core::{Simulation, ComponentId, NetId};
use super::common::{snap_to_grid, GRID_SIZE, default_stroke, selected_stroke, HALO_COLOR};
use std::collections::HashSet;

/// Main canvas for schematic drawing and editing
pub struct Canvas {
    /// Current simulation being displayed
    simulation: Option<Simulation>,
    
    /// Selected components
    selected_components: HashSet<ComponentId>,
    
    /// Selected nets/wires
    selected_nets: HashSet<NetId>,
    
    /// Canvas offset for panning
    offset: Vec2,
    
    /// Mouse position tracking
    last_mouse_pos: Option<Pos2>,
    
    /// Drag state
    dragging: bool,
    
    /// Current tool mode
    tool_mode: ToolMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolMode {
    /// Edit/select tool
    Edit,
    /// Add component tool
    AddComponent,
    /// Wire tool
    Wire,
    /// Probe tool for simulation
    Probe,
}

impl Canvas {
    /// Create a new canvas
    pub fn new() -> Self {
        Self {
            simulation: None,
            selected_components: HashSet::new(),
            selected_nets: HashSet::new(),
            offset: Vec2::ZERO,
            last_mouse_pos: None,
            dragging: false,
            tool_mode: ToolMode::Edit,
        }
    }
    
    /// Set the current simulation
    pub fn set_simulation(&mut self, simulation: &Simulation) {
        // Note: We'll need to implement proper state management here
        // For now, we just track that we have a simulation
        self.simulation = None; // TODO: Properly handle simulation reference
        self.selected_components.clear();
        self.selected_nets.clear();
    }
    
    /// Show the canvas UI
    pub fn show(&mut self, ui: &mut Ui, zoom: f32, show_grid: bool) -> Response {
        let available_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
        
        // Handle input
        self.handle_input(&response, zoom);
        
        // Draw the canvas
        let painter = ui.painter_at(available_rect);
        self.paint(&painter, available_rect, zoom, show_grid);
        
        response
    }
    
    /// Handle mouse and keyboard input
    fn handle_input(&mut self, response: &Response, zoom: f32) {
        let mouse_pos = response.hover_pos();
        
        // Handle panning
        if response.dragged_by(egui::PointerButton::Middle) {
            if let Some(delta) = response.drag_delta() {
                self.offset += delta / zoom;
            }
        }
        
        // Handle selection and interaction
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let canvas_pos = self.screen_to_canvas(pos, zoom);
                self.handle_click(canvas_pos);
            }
        }
        
        // Update mouse position
        self.last_mouse_pos = mouse_pos;
    }
    
    /// Convert screen coordinates to canvas coordinates
    fn screen_to_canvas(&self, screen_pos: Pos2, zoom: f32) -> Pos2 {
        Pos2::new(
            (screen_pos.x / zoom) - self.offset.x,
            (screen_pos.y / zoom) - self.offset.y,
        )
    }
    
    /// Convert canvas coordinates to screen coordinates
    fn canvas_to_screen(&self, canvas_pos: Pos2, zoom: f32) -> Pos2 {
        Pos2::new(
            (canvas_pos.x + self.offset.x) * zoom,
            (canvas_pos.y + self.offset.y) * zoom,
        )
    }
    
    /// Handle mouse click on canvas
    fn handle_click(&mut self, canvas_pos: Pos2) {
        match self.tool_mode {
            ToolMode::Edit => {
                // Try to select a component at this position
                self.select_at_position(canvas_pos);
            }
            ToolMode::AddComponent => {
                // TODO: Add component at position
            }
            ToolMode::Wire => {
                // TODO: Handle wire placement
            }
            ToolMode::Probe => {
                // TODO: Add probe at position
            }
        }
    }
    
    /// Select component at the given position
    fn select_at_position(&mut self, pos: Pos2) {
        // TODO: Implement component hit testing
        // For now, just clear selection
        self.selected_components.clear();
        self.selected_nets.clear();
    }
    
    /// Paint the canvas
    fn paint(&self, painter: &Painter, rect: Rect, zoom: f32, show_grid: bool) {
        // Clear background
        painter.rect_filled(rect, 0.0, Color32::WHITE);
        
        // Draw grid if enabled
        if show_grid {
            self.draw_grid(painter, rect, zoom);
        }
        
        // Draw circuit components and wires
        if let Some(simulation) = &self.simulation {
            self.draw_circuit(painter, rect, zoom, simulation);
        }
        
        // Draw selection highlights
        self.draw_selection_highlights(painter, rect, zoom);
    }
    
    /// Draw the grid
    fn draw_grid(&self, painter: &Painter, rect: Rect, zoom: f32) {
        let grid_spacing = GRID_SIZE * zoom;
        
        if grid_spacing < 5.0 {
            return; // Grid too small to draw
        }
        
        let stroke = Stroke::new(0.5, Color32::LIGHT_GRAY);
        
        // Calculate grid bounds
        let start_x = (rect.min.x / grid_spacing).floor() * grid_spacing;
        let start_y = (rect.min.y / grid_spacing).floor() * grid_spacing;
        
        // Draw vertical lines
        let mut x = start_x;
        while x <= rect.max.x {
            painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
            x += grid_spacing;
        }
        
        // Draw horizontal lines
        let mut y = start_y;
        while y <= rect.max.y {
            painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
            y += grid_spacing;
        }
    }
    
    /// Draw the circuit components and wires
    fn draw_circuit(&self, painter: &Painter, rect: Rect, zoom: f32, simulation: &Simulation) {
        // TODO: Implement proper circuit rendering
        // For now, draw a simple placeholder
        
        // Draw some example components to test the rendering
        self.draw_example_components(painter, rect, zoom);
    }
    
    /// Draw example components for testing
    fn draw_example_components(&self, painter: &Painter, _rect: Rect, zoom: f32) {
        // Draw an AND gate as an example
        let gate_pos = self.canvas_to_screen(Pos2::new(100.0, 100.0), zoom);
        let gate_size = Vec2::new(40.0 * zoom, 30.0 * zoom);
        
        let gate_rect = Rect::from_min_size(gate_pos, gate_size);
        painter.rect_stroke(gate_rect, 2.0, default_stroke());
        painter.text(
            gate_rect.center(),
            egui::Align2::CENTER_CENTER,
            "&",
            egui::FontId::default(),
            Color32::BLACK,
        );
        
        // Draw some connection points
        let input1 = gate_pos + Vec2::new(0.0, gate_size.y * 0.3);
        let input2 = gate_pos + Vec2::new(0.0, gate_size.y * 0.7);
        let output = gate_pos + Vec2::new(gate_size.x, gate_size.y * 0.5);
        
        painter.circle_filled(input1, 2.0, Color32::BLACK);
        painter.circle_filled(input2, 2.0, Color32::BLACK);
        painter.circle_filled(output, 2.0, Color32::BLACK);
    }
    
    /// Draw selection highlights
    fn draw_selection_highlights(&self, painter: &Painter, _rect: Rect, zoom: f32) {
        // TODO: Draw highlights around selected components
        // For now, just draw a simple example if something is selected
        if !self.selected_components.is_empty() {
            let highlight_pos = self.canvas_to_screen(Pos2::new(90.0, 90.0), zoom);
            let highlight_size = Vec2::new(60.0 * zoom, 50.0 * zoom);
            let highlight_rect = Rect::from_min_size(highlight_pos, highlight_size);
            painter.rect_stroke(highlight_rect, 2.0, selected_stroke());
        }
    }
    
    /// Set the current tool mode
    pub fn set_tool_mode(&mut self, mode: ToolMode) {
        self.tool_mode = mode;
    }
    
    /// Get the current tool mode
    pub fn tool_mode(&self) -> ToolMode {
        self.tool_mode
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}