//! Circuit canvas: handles rendering and interaction for the circuit editor.

use crate::state::{AppState, Tool, BASE_GRID_PX};
use egui::{Color32, Painter, Pos2, Rect, Sense, Stroke, Vec2};
use logisim_core::{
    circuit::Wire,
    component::{Component, ComponentId, ComponentKind},
    history::UndoAction,
    value::{Bus, Value},
};

// ── Upstream-matching colour constants ────────────────────────────────────────
/// Wire colour when the net carries logic 0.
const WIRE_COLOR_0: Color32 = Color32::from_rgb(0, 0, 192);
/// Wire colour when the net carries logic 1.
const WIRE_COLOR_1: Color32 = Color32::from_rgb(0, 160, 0);
/// Wire colour when the net value is unknown (X / uninitialised).
const WIRE_COLOR_X: Color32 = Color32::from_rgb(160, 0, 0);
/// Wire colour when the net is high-Z.
const WIRE_COLOR_Z: Color32 = Color32::from_rgb(150, 150, 150);
/// Wire colour when there is a multi-driver conflict (Error).
const WIRE_COLOR_ERR: Color32 = Color32::from_rgb(220, 0, 0);

/// Upstream gate body colour (off-white, like Logisim's default JPanel background).
const GATE_FILL: Color32 = Color32::from_rgb(255, 255, 255);
/// Upstream gate border / wire colour (Logisim uses near-black dark blue).
const GATE_BORDER: Color32 = Color32::from_rgb(0, 0, 0);
/// Upstream selected-component highlight fill.
const GATE_FILL_SEL: Color32 = Color32::from_rgb(200, 220, 255);
/// Upstream selected-component border colour.
const GATE_BORDER_SEL: Color32 = Color32::from_rgb(0, 100, 200);
/// Wire colour for a multi-bit bus that is fully driven (all bits 0 or 1).
const WIRE_COLOR_MULTI: Color32 = Color32::from_rgb(0, 100, 0);

/// Hit-test tolerance in grid units for component selection and dragging.
const HIT_TOLERANCE: i32 = 2;

/// The circuit editing canvas widget.
pub struct CircuitCanvas {
    /// Component currently being dragged (id, grid-pos at drag start).
    dragging: Option<(ComponentId, i32, i32)>,
    /// Rubber-band selection rectangle (start screen pos, current screen pos).
    /// Only active when the user drags in Select tool without hitting a component.
    rubber_band: Option<(egui::Pos2, egui::Pos2)>,
}

impl CircuitCanvas {
    pub fn new() -> Self {
        CircuitCanvas {
            dragging: None,
            rubber_band: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

        let origin = response.rect.min;

        // ── Pan with middle mouse button or right-drag ─────────────────────
        if response.dragged_by(egui::PointerButton::Middle)
            || (response.dragged_by(egui::PointerButton::Secondary)
                && ui.input(|i| i.modifiers.alt))
        {
            state.pan += response.drag_delta();
        }

        // ── Zoom with scroll wheel ─────────────────────────────────────────
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 && response.hovered() {
            let factor = if scroll > 0.0 { 1.1f32 } else { 1.0 / 1.1 };
            state.zoom = (state.zoom * factor).clamp(0.25, 8.0);
        }

        // ── Draw grid ─────────────────────────────────────────────────────
        if state.show_grid {
            draw_grid(&painter, response.rect, state.pan, state.zoom);
        }

        // ── Draw wires and components ─────────────────────────────────────
        let active = state.active_circuit.clone();
        if let Some(circuit) = state.project.circuits.get(&active) {
            // Precompute net map so we can colour wires by their live logic value.
            let net_map = circuit.compute_nets();
            let sim_state = state.simulator.state(&active);
            for wire in &circuit.wires {
                let net = net_map
                    .get(&(wire.from.x, wire.from.y))
                    .copied()
                    .unwrap_or((wire.from.x, wire.from.y));
                let color = if let Some(s) = &sim_state {
                    bus_to_wire_color(&s.net_value(net, 1))
                } else {
                    WIRE_COLOR_Z
                };
                draw_wire(&painter, wire, origin, state, color);
            }
            // Draw T-junction dots at points where 3+ wire segments meet.
            draw_junctions(&painter, &circuit.wires, origin, state);
            let comp_ids: Vec<_> = circuit.components.keys().copied().collect();
            for id in &comp_ids {
                if let Some(comp) = circuit.get_component(*id) {
                    let selected = state.selected.contains(id);
                    draw_component(&painter, comp, origin, state, selected);
                }
            }
        }

        // ── Wire-in-progress preview ───────────────────────────────────────
        if let (Tool::Wire, Some((sx, sy))) = (&state.tool, state.wire_start) {
            if let Some(cursor) = response.hover_pos() {
                let (ex, ey) = state.screen_to_grid(cursor, origin);
                let p1 = state.grid_to_screen(sx, sy, origin);
                let p2 = state.grid_to_screen(ex, sy, origin); // horizontal leg
                let p3 = state.grid_to_screen(ex, ey, origin); // then vertical
                painter.line_segment([p1, p2], Stroke::new(2.0, Color32::from_rgb(0, 0, 192)));
                painter.line_segment([p2, p3], Stroke::new(2.0, Color32::from_rgb(0, 0, 192)));
            }
        }

        // ── Component drag-to-move / rubber-band (Select tool, primary button) ──
        if state.tool == crate::state::Tool::Select {
            if response.drag_started_by(egui::PointerButton::Primary) {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.on_drag_start(pos, origin, state);
                }
            }

            if response.dragged_by(egui::PointerButton::Primary) {
                if let Some(cursor) = response.hover_pos() {
                    self.on_drag_move(cursor, origin, state);
                }
            }

            if response.drag_stopped_by(egui::PointerButton::Primary) {
                self.on_drag_end(state);
            }

            // Draw rubber-band rectangle.
            if let Some((start, end)) = self.rubber_band {
                let rect = Rect::from_two_pos(start, end);
                painter.rect_stroke(
                    rect,
                    0.0,
                    Stroke::new(1.0, egui::Color32::from_rgb(0, 120, 255)),
                );
                painter.rect_filled(
                    rect,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(0, 120, 255, 30),
                );
            }
        } else {
            // Cancel any drag if we switched tools.
            self.dragging = None;
            self.rubber_band = None;
        }

        // ── Handle pointer click events ───────────────────────────────────
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.on_click(pos, origin, state);
            }
        }

        // ── Ghost component for placement tool ────────────────────────────
        if let Tool::Place(ref kind) = state.tool {
            if let Some(cursor) = response.hover_pos() {
                let (gx, gy) = state.screen_to_grid(cursor, origin);
                let ghost = logisim_core::component::Component::new(
                    logisim_core::component::ComponentId(0),
                    kind.clone(),
                    gx,
                    gy,
                );
                draw_component_ghost(&painter, &ghost, origin, state);
            }
        }
    }

    // ── Extracted interaction methods (also called by the GUI harness) ────

    /// Initiate a component drag from `pos` (screen coords) with the given
    /// `origin` (top-left corner of the canvas rect).
    pub(crate) fn on_drag_start(&mut self, pos: Pos2, origin: Pos2, state: &mut AppState) {
        if state.tool != crate::state::Tool::Select {
            return;
        }
        let (gx, gy) = state.screen_to_grid(pos, origin);
        let active = state.active_circuit.clone();
        let hit = state
            .project
            .circuits
            .get(&active)
            .and_then(|c| {
                c.components.iter().find(|(_, comp)| {
                    (comp.x - gx).abs() <= HIT_TOLERANCE && (comp.y - gy).abs() <= HIT_TOLERANCE
                })
            })
            .map(|(id, comp)| (*id, comp.x, comp.y));
        if let Some((id, ox, oy)) = hit {
            self.dragging = Some((id, ox, oy));
            // Clicking any component always makes it the sole selection (standard UI behaviour).
            state.selected = vec![id];
        } else {
            // No component hit — start rubber-band selection.
            self.rubber_band = Some((pos, pos));
        }
    }

    /// Update a drag in progress: move the component to the new cursor `pos`.
    pub(crate) fn on_drag_move(&mut self, pos: Pos2, origin: Pos2, state: &mut AppState) {
        if let Some((id, ox, oy)) = self.dragging {
            let (gx, gy) = state.screen_to_grid(pos, origin);
            let active = state.active_circuit.clone();
            if let Some(circuit) = state.project.circuits.get_mut(&active) {
                if let Some(comp) = circuit.components.get_mut(&id) {
                    comp.x = gx;
                    comp.y = gy;
                }
            }
            self.dragging = Some((id, ox, oy));
        } else if let Some((start, _)) = self.rubber_band {
            // Extend rubber-band.
            self.rubber_band = Some((start, pos));
            // Live-select all components inside the rubber-band rect.
            let rect = Rect::from_two_pos(start, pos);
            // Collect matching IDs first (releases the circuits borrow) then assign.
            let selected: Vec<ComponentId> = state
                .project
                .circuits
                .get(&state.active_circuit)
                .map(|c| {
                    c.components
                        .iter()
                        .filter(|(_, comp)| {
                            let sp = state.grid_to_screen(comp.x, comp.y, origin);
                            rect.contains(sp)
                        })
                        .map(|(id, _)| *id)
                        .collect()
                })
                .unwrap_or_default();
            state.selected = selected;
        }
    }

    /// Finish the drag, commit a `MoveComponent` undo action if position changed.
    pub(crate) fn on_drag_end(&mut self, state: &mut AppState) {
        if let Some((id, old_x, old_y)) = self.dragging.take() {
            let active = state.active_circuit.clone();
            if let Some(circuit) = state.project.circuits.get(&active) {
                if let Some(comp) = circuit.components.get(&id) {
                    let new_x = comp.x;
                    let new_y = comp.y;
                    if new_x != old_x || new_y != old_y {
                        state.history.push(UndoAction::MoveComponent {
                            circuit_name: active,
                            id,
                            old_x,
                            old_y,
                            new_x,
                            new_y,
                        });
                        state.modified = true;
                        state.sync_simulator();
                    }
                }
            }
        }
        // Clear rubber-band (selection already finalised during on_drag_move).
        self.rubber_band = None;
    }

    /// Handle a click at screen position `pos` relative to canvas `origin`.
    pub(crate) fn on_click(&mut self, pos: Pos2, origin: Pos2, state: &mut AppState) {
        self.handle_click(pos, origin, state);
    }

    fn handle_click(&mut self, pos: Pos2, origin: Pos2, state: &mut AppState) {
        let (gx, gy) = state.screen_to_grid(pos, origin);
        let active = state.active_circuit.clone();

        match &state.tool {
            Tool::Place(kind) => {
                let kind = kind.clone();
                if let Some(circuit) = state.project.circuits.get_mut(&active) {
                    let id = circuit.add_component(kind, gx, gy);
                    let comp = circuit.components[&id].clone();
                    state.history.push(UndoAction::AddComponent {
                        circuit_name: active,
                        id,
                        component: comp,
                    });
                    state.modified = true;
                    state.sync_simulator();
                }
            }

            Tool::Wire => {
                match state.wire_start {
                    None => {
                        state.wire_start = Some((gx, gy));
                    }
                    Some((sx, sy)) => {
                        // Draw L-shaped wire: horizontal then vertical.
                        let mut wire_actions = Vec::new();
                        if let Some(circuit) = state.project.circuits.get_mut(&active) {
                            if sx != gx {
                                circuit.add_wire(sx, sy, gx, sy);
                                wire_actions.push(UndoAction::AddWire {
                                    circuit_name: active.clone(),
                                    wire: Wire::new(sx, sy, gx, sy),
                                });
                            }
                            if sy != gy {
                                circuit.add_wire(gx, sy, gx, gy);
                                wire_actions.push(UndoAction::AddWire {
                                    circuit_name: active.clone(),
                                    wire: Wire::new(gx, sy, gx, gy),
                                });
                            }
                        }
                        if !wire_actions.is_empty() {
                            let action = if wire_actions.len() == 1 {
                                wire_actions.remove(0)
                            } else {
                                UndoAction::Batch(wire_actions)
                            };
                            state.history.push(action);
                            state.modified = true;
                            state.sync_simulator();
                        }
                        state.wire_start = None;
                    }
                }
            }

            Tool::Select => {
                // Hit test components.
                let hit = state
                    .project
                    .circuits
                    .get(&active)
                    .and_then(|c| {
                        c.components.iter().find(|(_, comp)| {
                            let cp = state.grid_to_screen(comp.x, comp.y, origin);
                            let half = state.grid_px() * 2.0;
                            (cp.x - pos.x).abs() < half && (cp.y - pos.y).abs() < half
                        })
                    })
                    .map(|(id, _)| *id);

                if let Some(id) = hit {
                    state.selected = vec![id];
                } else {
                    state.selected.clear();
                }
            }

            Tool::Poke => {
                // Toggle input pins.
                let hit = state
                    .project
                    .circuits
                    .get(&active)
                    .and_then(|c| {
                        c.components.iter().find(|(_, comp)| {
                            matches!(
                                comp.kind,
                                ComponentKind::Pin {
                                    is_output: false,
                                    ..
                                }
                            ) && {
                                let cp = state.grid_to_screen(comp.x, comp.y, origin);
                                let half = state.grid_px() * 2.0;
                                (cp.x - pos.x).abs() < half && (cp.y - pos.y).abs() < half
                            }
                        })
                    })
                    .map(|(id, comp)| {
                        let w = if let ComponentKind::Pin { width, .. } = &comp.kind {
                            *width
                        } else {
                            logisim_core::value::BitWidth::ONE
                        };
                        (*id, w)
                    });

                if let Some((id, width)) = hit {
                    let cur = state
                        .simulator
                        .read_pin(&active, id)
                        .unwrap_or_else(|| Bus::from_u64(0, width.get() as usize));
                    let next = if cur.get(0) == Value::True {
                        Bus::from_u64(0, width.get() as usize)
                    } else {
                        Bus::from_u64(1, width.get() as usize)
                    };
                    state.simulator.set_pin_value(&active, id, next);
                    let _ = state.simulator.propagate(&active);
                }
            }

            _ => {}
        }
    }
}

// ── Drawing helpers ───────────────────────────────────────────────────────────

fn draw_grid(painter: &Painter, rect: Rect, pan: Vec2, zoom: f32) {
    let grid_px = BASE_GRID_PX * zoom;
    let color = Color32::from_gray(220);
    let stroke = Stroke::new(0.5, color);

    // Vertical lines
    let x_start = (rect.min.x - pan.x).rem_euclid(grid_px);
    let mut x = rect.min.x + x_start - grid_px;
    while x < rect.max.x {
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
        x += grid_px;
    }

    // Horizontal lines
    let y_start = (rect.min.y - pan.y).rem_euclid(grid_px);
    let mut y = rect.min.y + y_start - grid_px;
    while y < rect.max.y {
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
        y += grid_px;
    }
}

fn draw_wire(painter: &Painter, wire: &Wire, origin: Pos2, state: &AppState, color: Color32) {
    let p1 = state.grid_to_screen(wire.from.x, wire.from.y, origin);
    let p2 = state.grid_to_screen(wire.to.x, wire.to.y, origin);
    painter.line_segment([p1, p2], Stroke::new(2.0 * state.zoom, color));
}

/// Draw junction (filled circle) at every point where 3 or more wire endpoints meet,
/// matching upstream Logisim-Evolution's T/X junction dot convention.
fn draw_junctions(painter: &Painter, wires: &[Wire], origin: Pos2, state: &AppState) {
    use std::collections::HashMap;
    let mut endpoint_count: HashMap<(i32, i32), usize> = HashMap::new();
    for w in wires {
        *endpoint_count.entry((w.from.x, w.from.y)).or_insert(0) += 1;
        *endpoint_count.entry((w.to.x, w.to.y)).or_insert(0) += 1;
    }
    let radius = 3.5 * state.zoom;
    for ((gx, gy), count) in &endpoint_count {
        if *count >= 3 {
            let sp = state.grid_to_screen(*gx, *gy, origin);
            painter.circle_filled(sp, radius, GATE_BORDER);
        }
    }
}

fn draw_component(
    painter: &Painter,
    comp: &Component,
    origin: Pos2,
    state: &AppState,
    selected: bool,
) {
    let pos = state.grid_to_screen(comp.x, comp.y, origin);
    let g = state.grid_px();

    let body_color = if selected { GATE_FILL_SEL } else { GATE_FILL };
    let border = if selected {
        GATE_BORDER_SEL
    } else {
        GATE_BORDER
    };
    let stroke = Stroke::new(1.5 * state.zoom, border);

    match &comp.kind {
        ComponentKind::Pin {
            is_output,
            width: _,
        } => {
            // Logisim: input pins are squares, output pins are circles.
            let s = g * 1.0;
            if *is_output {
                painter.circle_filled(
                    Pos2::new(pos.x + s / 2.0, pos.y + s / 2.0),
                    s / 2.0,
                    Color32::from_rgb(200, 220, 255),
                );
                painter.circle_stroke(Pos2::new(pos.x + s / 2.0, pos.y + s / 2.0), s / 2.0, stroke);
            } else {
                let r = Rect::from_min_size(pos, Vec2::splat(s));
                painter.rect(r, 0.0, Color32::from_rgb(220, 255, 220), stroke);
            }
            if !comp.label.is_empty() {
                painter.text(
                    Pos2::new(pos.x + s / 2.0, pos.y - 3.0 * state.zoom),
                    egui::Align2::CENTER_BOTTOM,
                    &comp.label,
                    egui::FontId::proportional(10.0 * state.zoom),
                    Color32::BLACK,
                );
            }
        }

        ComponentKind::AndGate { inputs, .. } | ComponentKind::NandGate { inputs, .. } => {
            let n = (*inputs as usize).max(2);
            let negate = matches!(comp.kind, ComponentKind::NandGate { .. });
            draw_and_gate(painter, pos, g, n, body_color, stroke, negate);
        }

        ComponentKind::OrGate { inputs, .. } | ComponentKind::NorGate { inputs, .. } => {
            let n = (*inputs as usize).max(2);
            let negate = matches!(comp.kind, ComponentKind::NorGate { .. });
            draw_or_gate(painter, pos, g, n, body_color, stroke, false, negate);
        }

        ComponentKind::XorGate { inputs, .. } | ComponentKind::XnorGate { inputs, .. } => {
            let n = (*inputs as usize).max(2);
            let negate = matches!(comp.kind, ComponentKind::XnorGate { .. });
            draw_or_gate(painter, pos, g, n, body_color, stroke, true, negate);
        }

        ComponentKind::NotGate { .. } => {
            draw_not_gate(painter, pos, g, body_color, stroke);
        }

        ComponentKind::Buffer { .. } => {
            draw_buffer_gate(painter, pos, g, body_color, stroke);
        }

        ComponentKind::Clock => {
            // Logisim: clock is a rectangle with a clock-edge symbol inside.
            let s = g * 1.0;
            let r = Rect::from_min_size(pos, Vec2::splat(s));
            painter.rect(r, 0.0, Color32::from_rgb(255, 245, 210), stroke);
            // Clock edge: rising edge zigzag in the lower-left
            let mid_y = pos.y + s * 0.5;
            let edge_pts = [
                Pos2::new(pos.x + s * 0.15, pos.y + s * 0.75),
                Pos2::new(pos.x + s * 0.15, mid_y),
                Pos2::new(pos.x + s * 0.5, mid_y),
                Pos2::new(pos.x + s * 0.5, pos.y + s * 0.75),
            ];
            for i in 0..edge_pts.len() - 1 {
                painter.line_segment([edge_pts[i], edge_pts[i + 1]], stroke);
            }
            if !comp.label.is_empty() {
                painter.text(
                    Pos2::new(pos.x + s / 2.0, pos.y - 3.0 * state.zoom),
                    egui::Align2::CENTER_BOTTOM,
                    &comp.label,
                    egui::FontId::proportional(10.0 * state.zoom),
                    Color32::BLACK,
                );
            }
        }

        ComponentKind::Constant { value, width: _ } => {
            let w = g * 2.0;
            let h = g * 1.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, 0.0, Color32::from_rgb(255, 255, 200), stroke);
            let label = format!("0x{:X}", value);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::monospace(9.0 * state.zoom),
                Color32::BLACK,
            );
        }

        ComponentKind::Power => {
            // VCC symbol: upward arrow / positive power rail
            let h = g * 1.5;
            let mid_x = pos.x + g * 0.5;
            painter.line_segment(
                [
                    Pos2::new(mid_x, pos.y + h),
                    Pos2::new(mid_x, pos.y + g * 0.3),
                ],
                stroke,
            );
            painter.line_segment(
                [Pos2::new(pos.x, pos.y), Pos2::new(pos.x + g, pos.y)],
                Stroke::new(3.0 * state.zoom, border),
            );
        }

        ComponentKind::Ground => {
            // GND symbol: downward lines
            let mid_x = pos.x + g * 0.5;
            painter.line_segment(
                [Pos2::new(mid_x, pos.y), Pos2::new(mid_x, pos.y + g * 0.5)],
                stroke,
            );
            let offsets = [0.0f32, 0.2, 0.4];
            for (i, &off) in offsets.iter().enumerate() {
                let y = pos.y + g * (0.5 + off);
                let half = g * (0.5 - i as f32 * 0.15);
                painter.line_segment(
                    [Pos2::new(mid_x - half, y), Pos2::new(mid_x + half, y)],
                    stroke,
                );
            }
        }

        ComponentKind::Multiplexer { select_bits, .. } => {
            let n_in = 1usize << select_bits;
            let w = g * 4.0;
            let h = g * n_in.max(2) as f32;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            draw_gate_label(painter, pos, w, h, "MUX", state);
        }

        ComponentKind::Demultiplexer { select_bits, .. } => {
            let n_out = 1usize << select_bits;
            let w = g * 4.0;
            let h = g * n_out.max(2) as f32;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            draw_gate_label(painter, pos, w, h, "DEMUX", state);
        }

        ComponentKind::Adder { .. } => {
            let w = g * 4.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            draw_gate_label(painter, pos, w, h, "+", state);
        }

        ComponentKind::Subtractor { .. } => {
            let w = g * 4.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            draw_gate_label(painter, pos, w, h, "−", state);
        }

        ComponentKind::Multiplier { .. } => {
            let w = g * 4.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            draw_gate_label(painter, pos, w, h, "×", state);
        }

        ComponentKind::Comparator { .. } => {
            let w = g * 4.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            draw_gate_label(painter, pos, w, h, "CMP", state);
        }

        ComponentKind::DFlipFlop { .. } => {
            draw_ff_box(painter, pos, g, body_color, border, "D FF", state);
        }
        ComponentKind::TFlipFlop { .. } => {
            draw_ff_box(painter, pos, g, body_color, border, "T FF", state);
        }
        ComponentKind::JKFlipFlop { .. } => {
            draw_ff_box(painter, pos, g, body_color, border, "JK FF", state);
        }
        ComponentKind::SRFlipFlop { .. } => {
            draw_ff_box(painter, pos, g, body_color, border, "SR FF", state);
        }
        ComponentKind::Register { .. } => {
            draw_ff_box(painter, pos, g, body_color, border, "REG", state);
        }
        ComponentKind::Counter { .. } => {
            draw_ff_box(painter, pos, g, body_color, border, "CTR", state);
        }
        ComponentKind::Ram { .. } => {
            let w = g * 6.0;
            let h = g * 5.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            draw_gate_label(painter, pos, w, h, "RAM", state);
        }
        ComponentKind::Rom { .. } => {
            let w = g * 6.0;
            let h = g * 4.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, Color32::from_rgb(220, 220, 255), stroke);
            draw_gate_label(painter, pos, w, h, "ROM", state);
        }

        ComponentKind::Led => {
            let r = g * 0.6;
            let center = Pos2::new(pos.x + r, pos.y + r);
            painter.circle_filled(center, r, Color32::from_rgb(255, 80, 80));
            painter.circle_stroke(center, r, stroke);
        }

        ComponentKind::SevenSegDisplay => {
            let w = g * 3.0;
            let h = g * 5.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, Color32::BLACK, stroke);
            draw_seven_seg(painter, rect, &[false; 8]);
        }

        ComponentKind::HexDisplay => {
            let w = g * 3.0;
            let h = g * 5.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, Color32::BLACK, stroke);
        }

        ComponentKind::Button => {
            let w = g * 2.0;
            let h = g * 2.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.4, Color32::from_rgb(210, 210, 210), stroke);
            draw_gate_label(painter, pos, w, h, "BTN", state);
        }

        ComponentKind::Splitter {
            combined_width,
            fan_out,
        } => {
            // Splitter: a fork symbol
            let w = g * 2.0;
            let h = g * *fan_out as f32;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, 0.0, body_color, stroke);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{}", combined_width.get()),
                egui::FontId::proportional(8.0 * state.zoom),
                Color32::BLACK,
            );
        }

        ComponentKind::Tunnel { label, .. } => {
            // Tunnel: pentagon shape pointing right
            let w = g * 2.5;
            let h = g * 1.0;
            let pts = vec![
                Pos2::new(pos.x, pos.y),
                Pos2::new(pos.x + w * 0.75, pos.y),
                Pos2::new(pos.x + w, pos.y + h / 2.0),
                Pos2::new(pos.x + w * 0.75, pos.y + h),
                Pos2::new(pos.x, pos.y + h),
            ];
            painter.add(egui::Shape::convex_polygon(pts, body_color, stroke));
            painter.text(
                Pos2::new(pos.x + w * 0.4, pos.y + h / 2.0),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(8.0 * state.zoom),
                Color32::BLACK,
            );
        }

        ComponentKind::Probe { .. } => {
            // Probe: circle with cross-hairs
            let r = g * 0.7;
            let center = Pos2::new(pos.x + r, pos.y + r);
            painter.circle_filled(center, r, Color32::from_rgb(255, 240, 200));
            painter.circle_stroke(center, r, stroke);
            painter.line_segment(
                [
                    Pos2::new(center.x - r * 0.6, center.y),
                    Pos2::new(center.x + r * 0.6, center.y),
                ],
                stroke,
            );
        }

        ComponentKind::Subcircuit { circuit_name } => {
            let w = g * 5.0;
            let h = g * 4.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, Color32::from_rgb(235, 215, 235), stroke);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                circuit_name,
                egui::FontId::proportional(9.0 * state.zoom),
                Color32::BLACK,
            );
        }

        _ => {
            // Generic fallback box.
            let w = g * 3.0;
            let h = g * 2.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, stroke);
            let label = comp.kind.component_name();
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(8.0 * state.zoom),
                Color32::BLACK,
            );
        }
    }
}

// ── ANSI gate shape helpers ───────────────────────────────────────────────────

/// Draw a proper ANSI AND gate (or NAND if `negate` is true).
/// `pos` = top-left corner of bounding box; gate is right-facing.
fn draw_and_gate(
    painter: &Painter,
    pos: Pos2,
    g: f32,
    n: usize,
    fill: Color32,
    stroke: Stroke,
    negate: bool,
) {
    let h = g * n as f32;
    let flat = g * 2.0; // flat (left) portion width
    let r = h / 2.0; // arc radius = half gate height
    let cy = pos.y + h / 2.0;

    // Build gate body clockwise: left side → top flat → right arc → bottom flat
    let mut pts = Vec::with_capacity(32);
    pts.push(Pos2::new(pos.x, pos.y)); // top-left
    pts.push(Pos2::new(pos.x + flat, pos.y)); // top-right (start of arc)
                                              // Right semicircular arc: angle from +π/2 (top) to −π/2 (bottom)
    let nseg = 20;
    for i in 0..=nseg {
        let a = std::f32::consts::FRAC_PI_2 * (1.0 - 2.0 * i as f32 / nseg as f32);
        pts.push(Pos2::new(pos.x + flat + r * a.cos(), cy - r * a.sin()));
    }
    pts.push(Pos2::new(pos.x, pos.y + h)); // bottom-left

    painter.add(egui::Shape::Path(egui::epaint::PathShape {
        points: pts,
        closed: true,
        fill,
        stroke: stroke.into(),
    }));

    // Input stub lines on left edge
    let in_step = h / n as f32;
    for i in 0..n {
        let iy = pos.y + in_step * (i as f32 + 0.5);
        painter.line_segment([Pos2::new(pos.x - g, iy), Pos2::new(pos.x, iy)], stroke);
    }

    // Output line (and optional NAND bubble)
    let out_x = pos.x + flat + r;
    if negate {
        let br = g * 0.25;
        painter.circle(Pos2::new(out_x + br, cy), br, fill, stroke);
        painter.line_segment(
            [Pos2::new(out_x + br * 2.0, cy), Pos2::new(out_x + g, cy)],
            stroke,
        );
    } else {
        painter.line_segment([Pos2::new(out_x, cy), Pos2::new(out_x + g, cy)], stroke);
    }
}

/// Draw a proper ANSI OR/NOR/XOR/XNOR gate.
/// `xor = true` draws the extra left arc for XOR/XNOR.
#[allow(clippy::too_many_arguments)]
fn draw_or_gate(
    painter: &Painter,
    pos: Pos2,
    g: f32,
    n: usize,
    fill: Color32,
    stroke: Stroke,
    xor: bool,
    negate: bool,
) {
    let h = g * n as f32;
    let w = g * 3.0;
    let cy = pos.y + h / 2.0;

    // Back (left) concavity: depth relative to gate width
    let back_d = w * 0.2;
    // Front (right) arc radius
    let fr = h / 2.0;
    // x-position where the front arc starts
    let fx = w - fr;

    let nseg = 16;

    // Build gate outline clockwise from back-top
    let mut pts = Vec::with_capacity(nseg * 2 + 6);

    // Back: quadratic bezier from (pos.x, top) via (pos.x+back_d, cy) to (pos.x, bottom)
    for i in 0..=nseg {
        let t = i as f32 / nseg as f32;
        let mt = 1.0 - t;
        let bx = mt * mt * pos.x + 2.0 * mt * t * (pos.x + back_d) + t * t * pos.x;
        let by = mt * mt * pos.y + 2.0 * mt * t * cy + t * t * (pos.y + h);
        pts.push(Pos2::new(bx, by));
    }

    // Bottom diagonal from back-bottom to front-bottom
    pts.push(Pos2::new(pos.x + fx, pos.y + h));

    // Front arc: from bottom to top via right tip
    for i in 0..=nseg {
        let t = i as f32 / nseg as f32;
        let a = -std::f32::consts::FRAC_PI_2 + t * std::f32::consts::PI;
        pts.push(Pos2::new(pos.x + fx + fr * a.cos(), cy - fr * a.sin()));
    }

    // Top diagonal from front-top back to back-top
    pts.push(Pos2::new(pos.x + fx, pos.y));
    // (the path closes back to pts[0] = back-top)

    painter.add(egui::Shape::Path(egui::epaint::PathShape {
        points: pts,
        closed: true,
        fill,
        stroke: stroke.into(),
    }));

    // XOR extra back arc (parallel curve left of the back, same bezier + offset)
    if xor {
        let off = g * 0.4;
        let mut xor_pts = Vec::with_capacity(nseg + 2);
        for i in 0..=nseg {
            let t = i as f32 / nseg as f32;
            let mt = 1.0 - t;
            let bx = mt * mt * (pos.x - off)
                + 2.0 * mt * t * (pos.x - off + back_d)
                + t * t * (pos.x - off);
            let by = mt * mt * pos.y + 2.0 * mt * t * cy + t * t * (pos.y + h);
            xor_pts.push(Pos2::new(bx, by));
        }
        painter.add(egui::Shape::Path(egui::epaint::PathShape {
            points: xor_pts,
            closed: false,
            fill: Color32::TRANSPARENT,
            stroke: stroke.into(),
        }));
    }

    // Input stub lines
    let in_step = h / n as f32;
    for i in 0..n {
        let iy = pos.y + in_step * (i as f32 + 0.5);
        // Stubs start at the back curve x for this y
        let t = (iy - pos.y) / h;
        let mt = 1.0 - t;
        let back_x = mt * mt * pos.x + 2.0 * mt * t * (pos.x + back_d) + t * t * pos.x;
        let start_x = if xor { pos.x - g * 0.4 - g } else { pos.x - g };
        painter.line_segment([Pos2::new(start_x, iy), Pos2::new(back_x, iy)], stroke);
    }

    // Output
    let out_x = pos.x + fx + fr;
    if negate {
        let br = g * 0.25;
        painter.circle(Pos2::new(out_x + br, cy), br, fill, stroke);
        painter.line_segment(
            [Pos2::new(out_x + br * 2.0, cy), Pos2::new(out_x + g, cy)],
            stroke,
        );
    } else {
        painter.line_segment([Pos2::new(out_x, cy), Pos2::new(out_x + g, cy)], stroke);
    }
}

/// Draw a proper ANSI NOT gate (triangle with bubble at output).
fn draw_not_gate(painter: &Painter, pos: Pos2, g: f32, fill: Color32, stroke: Stroke) {
    let w = g * 2.0;
    let h = g * 2.0;
    let cy = pos.y + h / 2.0;
    let br = g * 0.25;

    // Triangle body
    let pts = vec![
        Pos2::new(pos.x, pos.y),
        Pos2::new(pos.x, pos.y + h),
        Pos2::new(pos.x + w, cy),
    ];
    painter.add(egui::Shape::convex_polygon(pts, fill, stroke));

    // Inversion bubble at tip
    painter.circle(Pos2::new(pos.x + w + br, cy), br, fill, stroke);

    // Input stub
    painter.line_segment([Pos2::new(pos.x - g, cy), Pos2::new(pos.x, cy)], stroke);
    // Output stub
    painter.line_segment(
        [
            Pos2::new(pos.x + w + br * 2.0, cy),
            Pos2::new(pos.x + w + g, cy),
        ],
        stroke,
    );
}

/// Draw a Buffer gate (triangle, no inversion bubble).
fn draw_buffer_gate(painter: &Painter, pos: Pos2, g: f32, fill: Color32, stroke: Stroke) {
    let w = g * 2.0;
    let h = g * 2.0;
    let cy = pos.y + h / 2.0;

    let pts = vec![
        Pos2::new(pos.x, pos.y),
        Pos2::new(pos.x, pos.y + h),
        Pos2::new(pos.x + w, cy),
    ];
    painter.add(egui::Shape::convex_polygon(pts, fill, stroke));

    painter.line_segment([Pos2::new(pos.x - g, cy), Pos2::new(pos.x, cy)], stroke);
    painter.line_segment(
        [Pos2::new(pos.x + w, cy), Pos2::new(pos.x + w + g, cy)],
        stroke,
    );
}

fn draw_component_ghost(painter: &Painter, comp: &Component, origin: Pos2, state: &AppState) {
    // Draw with transparency.
    let pos = state.grid_to_screen(comp.x, comp.y, origin);
    let g = state.grid_px();
    let w = g * 3.0;
    let h = g * 2.0;
    let rect = Rect::from_min_size(pos, Vec2::new(w, h));
    painter.rect(
        rect,
        g * 0.2,
        Color32::from_rgba_premultiplied(150, 200, 255, 100),
        Stroke::new(1.5, Color32::from_rgba_premultiplied(0, 80, 180, 150)),
    );
}

fn draw_gate_label(painter: &Painter, pos: Pos2, w: f32, h: f32, label: &str, state: &AppState) {
    let center = Pos2::new(pos.x + w / 2.0, pos.y + h / 2.0);
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(9.0 * state.zoom),
        Color32::from_gray(40),
    );
}

fn draw_ff_box(
    painter: &Painter,
    pos: Pos2,
    g: f32,
    fill: Color32,
    border: Color32,
    label: &str,
    state: &AppState,
) {
    let w = g * 5.0;
    let h = g * 5.0;
    let rect = Rect::from_min_size(pos, Vec2::new(w, h));
    let stroke = Stroke::new(1.5 * state.zoom, border);
    painter.rect(rect, g * 0.2, fill, stroke);
    // Label at top
    painter.text(
        Pos2::new(pos.x + w / 2.0, pos.y + g * 0.5),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(9.0 * state.zoom),
        Color32::from_gray(40),
    );
    // Clock edge indicator (small triangle at bottom-left).
    let clk_pts = [
        Pos2::new(pos.x, pos.y + h - g * 0.4),
        Pos2::new(pos.x + g * 0.5, pos.y + h - g * 0.7),
        Pos2::new(pos.x, pos.y + h - g * 1.0),
    ];
    painter.add(egui::Shape::convex_polygon(
        clk_pts.to_vec(),
        border,
        Stroke::NONE,
    ));
}

/// Draw a 7-segment display with the given active segments (a-g + dp).
fn draw_seven_seg(painter: &Painter, rect: Rect, segs: &[bool; 8]) {
    let w = rect.width();
    let h = rect.height();
    let x = rect.min.x;
    let y = rect.min.y;
    let on = Color32::from_rgb(255, 60, 0);
    let off = Color32::from_rgba_premultiplied(80, 20, 0, 255);
    let t = 2.0;

    // Segment positions (a=top, b=top-right, c=bot-right, d=bot,
    //                    e=bot-left, f=top-left, g=middle)
    let segs_lines = [
        // a: top horizontal
        (
            Pos2::new(x + w * 0.15, y + h * 0.05),
            Pos2::new(x + w * 0.85, y + h * 0.05),
        ),
        // b: top-right vertical
        (
            Pos2::new(x + w * 0.88, y + h * 0.08),
            Pos2::new(x + w * 0.88, y + h * 0.48),
        ),
        // c: bot-right vertical
        (
            Pos2::new(x + w * 0.88, y + h * 0.52),
            Pos2::new(x + w * 0.88, y + h * 0.92),
        ),
        // d: bottom horizontal
        (
            Pos2::new(x + w * 0.15, y + h * 0.95),
            Pos2::new(x + w * 0.85, y + h * 0.95),
        ),
        // e: bot-left vertical
        (
            Pos2::new(x + w * 0.12, y + h * 0.52),
            Pos2::new(x + w * 0.12, y + h * 0.92),
        ),
        // f: top-left vertical
        (
            Pos2::new(x + w * 0.12, y + h * 0.08),
            Pos2::new(x + w * 0.12, y + h * 0.48),
        ),
        // g: middle horizontal
        (
            Pos2::new(x + w * 0.15, y + h * 0.5),
            Pos2::new(x + w * 0.85, y + h * 0.5),
        ),
    ];

    for (i, (p1, p2)) in segs_lines.iter().enumerate() {
        let color = if segs.get(i).copied().unwrap_or(false) {
            on
        } else {
            off
        };
        painter.line_segment([*p1, *p2], Stroke::new(t, color));
    }
}

/// Map a logic bus value to the upstream Logisim wire colour.
fn bus_to_wire_color(bus: &Bus) -> Color32 {
    if bus.has_error() {
        return WIRE_COLOR_ERR;
    }
    if bus.is_high_z() {
        return WIRE_COLOR_Z;
    }
    if bus.width() == 1 {
        match bus.get(0) {
            Value::False => WIRE_COLOR_0,
            Value::True => WIRE_COLOR_1,
            Value::Unknown => WIRE_COLOR_X,
            Value::HighZ => WIRE_COLOR_Z,
            Value::Error => WIRE_COLOR_ERR,
        }
    } else if bus.is_fully_known() {
        // Multi-bit fully driven: dark green
        WIRE_COLOR_MULTI
    } else {
        WIRE_COLOR_Z
    }
}
