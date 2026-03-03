//! Circuit canvas: handles rendering and interaction for the circuit editor.

use crate::state::{AppState, Tool};
use egui::{Color32, Painter, Pos2, Rect, Sense, Stroke, Vec2};
use logisim_core::{
    circuit::Wire,
    component::{Component, ComponentId, ComponentKind},
    history::UndoAction,
    value::{Bus, Value},
};

const GRID: f32 = 10.0;
/// Hit-test tolerance in grid units for component selection and dragging.
const HIT_TOLERANCE: i32 = 2;

/// The circuit editing canvas widget.
pub struct CircuitCanvas {
    /// Component currently being dragged (id, grid-pos at drag start).
    dragging: Option<(ComponentId, i32, i32)>,
}

impl CircuitCanvas {
    pub fn new() -> Self {
        CircuitCanvas { dragging: None }
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
            for wire in &circuit.wires {
                draw_wire(&painter, wire, origin, state, Color32::from_rgb(0, 0, 192));
            }
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

        // ── Component drag-to-move (Select tool, primary button) ──────────
        if state.tool == crate::state::Tool::Select {
            if response.drag_started_by(egui::PointerButton::Primary) {
                if let Some(pos) = response.interact_pointer_pos() {
                    let (gx, gy) = state.screen_to_grid(pos, origin);
                    let active = state.active_circuit.clone();
                    let hit = state
                        .project
                        .circuits
                        .get(&active)
                        .and_then(|c| {
                            c.components.iter().find(|(_, comp)| {
                                (comp.x - gx).abs() <= HIT_TOLERANCE
                                    && (comp.y - gy).abs() <= HIT_TOLERANCE
                            })
                        })
                        .map(|(id, comp)| (*id, comp.x, comp.y));
                    if let Some((id, ox, oy)) = hit {
                        self.dragging = Some((id, ox, oy));
                        state.selected = vec![id];
                    }
                }
            }

            if response.dragged_by(egui::PointerButton::Primary) {
                if let Some((id, ox, oy)) = self.dragging {
                    if let Some(cursor) = response.hover_pos() {
                        let (gx, gy) = state.screen_to_grid(cursor, origin);
                        let active = state.active_circuit.clone();
                        if let Some(circuit) = state.project.circuits.get_mut(&active) {
                            if let Some(comp) = circuit.components.get_mut(&id) {
                                comp.x = gx;
                                comp.y = gy;
                            }
                        }
                        // Keep old_x/old_y from drag start for history.
                        self.dragging = Some((id, ox, oy));
                    }
                }
            }

            if response.drag_stopped_by(egui::PointerButton::Primary) {
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
            }
        } else {
            // Cancel any drag if we switched tools.
            self.dragging = None;
        }

        // ── Handle pointer click events ───────────────────────────────────
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.handle_click(pos, origin, state);
            }
        }

        // ── Ghost component for placement tool ────────────────────────────
        if let Tool::Place(ref kind) = state.tool.clone() {
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

    fn handle_click(&mut self, pos: Pos2, origin: Pos2, state: &mut AppState) {
        let (gx, gy) = state.screen_to_grid(pos, origin);
        let active = state.active_circuit.clone();

        match &state.tool.clone() {
            Tool::Place(kind) => {
                if let Some(circuit) = state.project.circuits.get_mut(&active) {
                    let id = circuit.add_component(kind.clone(), gx, gy);
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
                        let w = if let ComponentKind::Pin { width, .. } = comp.kind {
                            width
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
    let grid_px = GRID * zoom;
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

fn draw_component(
    painter: &Painter,
    comp: &Component,
    origin: Pos2,
    state: &AppState,
    selected: bool,
) {
    let pos = state.grid_to_screen(comp.x, comp.y, origin);
    let g = state.grid_px();

    let body_color = if selected {
        Color32::from_rgb(180, 230, 255)
    } else {
        Color32::from_rgb(240, 240, 200)
    };
    let border = if selected {
        Color32::from_rgb(0, 120, 200)
    } else {
        Color32::from_rgb(80, 80, 80)
    };

    match &comp.kind {
        ComponentKind::Pin {
            is_output,
            width: _,
        } => {
            let r = g * 0.8;
            let fill = if *is_output {
                Color32::from_rgb(200, 220, 255)
            } else {
                Color32::from_rgb(220, 255, 220)
            };
            painter.circle_filled(pos, r, fill);
            painter.circle_stroke(pos, r, Stroke::new(1.5, border));
            if !comp.label.is_empty() {
                painter.text(
                    Pos2::new(pos.x, pos.y - r - 4.0),
                    egui::Align2::CENTER_BOTTOM,
                    &comp.label,
                    egui::FontId::proportional(10.0 * state.zoom),
                    Color32::BLACK,
                );
            }
        }

        ComponentKind::AndGate { inputs, .. } | ComponentKind::NandGate { inputs, .. } => {
            let n = *inputs as f32;
            let w = g * 3.0;
            let h = g * n;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.3, body_color, Stroke::new(1.5, border));
            let negate = matches!(comp.kind, ComponentKind::NandGate { .. });
            if negate {
                let out_pos = Pos2::new(pos.x + w + g * 0.3, pos.y + h / 2.0);
                painter.circle_filled(out_pos, g * 0.2, border);
            }
            draw_gate_label(painter, pos, w, h, "& ", state);
        }

        ComponentKind::OrGate { inputs, .. } | ComponentKind::NorGate { inputs, .. } => {
            let n = *inputs as f32;
            let w = g * 3.0;
            let h = g * n;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.8, body_color, Stroke::new(1.5, border));
            let negate = matches!(comp.kind, ComponentKind::NorGate { .. });
            if negate {
                let out_pos = Pos2::new(pos.x + w + g * 0.3, pos.y + h / 2.0);
                painter.circle_filled(out_pos, g * 0.2, border);
            }
            draw_gate_label(painter, pos, w, h, "≥1", state);
        }

        ComponentKind::XorGate { inputs, .. } | ComponentKind::XnorGate { inputs, .. } => {
            let n = *inputs as f32;
            let w = g * 3.0;
            let h = g * n;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.8, body_color, Stroke::new(1.5, border));
            let negate = matches!(comp.kind, ComponentKind::XnorGate { .. });
            if negate {
                let out_pos = Pos2::new(pos.x + w + g * 0.3, pos.y + h / 2.0);
                painter.circle_filled(out_pos, g * 0.2, border);
            }
            draw_gate_label(painter, pos, w, h, "=1", state);
        }

        ComponentKind::NotGate { .. } => {
            let w = g * 2.0;
            let h = g * 1.5;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, 0.0, body_color, Stroke::new(1.5, border));
            let out_pos = Pos2::new(pos.x + w + g * 0.3, pos.y + h / 2.0);
            painter.circle_filled(out_pos, g * 0.25, border);
            draw_gate_label(painter, pos, w, h, "1", state);
        }

        ComponentKind::Buffer { .. } => {
            let w = g * 2.0;
            let h = g * 1.5;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, 0.0, body_color, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, w, h, "1", state);
        }

        ComponentKind::Clock => {
            let r = g * 0.8;
            painter.circle_filled(pos, r, Color32::from_rgb(255, 220, 180));
            painter.circle_stroke(pos, r, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, r * 2.0, r * 2.0, "CLK", state);
        }

        ComponentKind::Constant { value, width: _ } => {
            let w = g * 3.0;
            let h = g * 1.5;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(
                rect,
                g * 0.2,
                Color32::from_rgb(255, 255, 200),
                Stroke::new(1.5, border),
            );
            let label = format!("0x{:X}", value);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::monospace(9.0 * state.zoom),
                Color32::BLACK,
            );
        }

        ComponentKind::Multiplexer { select_bits, .. } => {
            let w = g * 3.0;
            let h = g * (1 << select_bits) as f32;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, w, h, "MUX", state);
        }

        ComponentKind::Demultiplexer { select_bits, .. } => {
            let w = g * 3.0;
            let h = g * (1 << select_bits) as f32;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, w, h, "DEMUX", state);
        }

        ComponentKind::Adder { .. } => {
            let w = g * 3.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, w, h, "+", state);
        }

        ComponentKind::Subtractor { .. } => {
            let w = g * 3.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, w, h, "−", state);
        }

        ComponentKind::Multiplier { .. } => {
            let w = g * 3.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, w, h, "×", state);
        }

        ComponentKind::Comparator { .. } => {
            let w = g * 3.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
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
            let w = g * 5.0;
            let h = g * 4.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
            draw_gate_label(painter, pos, w, h, "RAM", state);
        }
        ComponentKind::Rom { .. } => {
            let w = g * 5.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(
                rect,
                g * 0.2,
                Color32::from_rgb(220, 220, 255),
                Stroke::new(1.5, border),
            );
            draw_gate_label(painter, pos, w, h, "ROM", state);
        }

        ComponentKind::Led => {
            let r = g * 0.7;
            painter.circle_filled(pos, r, Color32::from_rgb(255, 80, 80));
            painter.circle_stroke(pos, r, Stroke::new(1.5, Color32::from_rgb(180, 0, 0)));
        }

        ComponentKind::SevenSegDisplay => {
            let w = g * 3.0;
            let h = g * 5.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, Color32::BLACK, Stroke::new(1.5, border));
            draw_seven_seg(painter, rect, &[false; 8]);
        }

        ComponentKind::HexDisplay => {
            let w = g * 3.0;
            let h = g * 5.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(rect, g * 0.2, Color32::BLACK, Stroke::new(1.5, border));
        }

        ComponentKind::Button => {
            let w = g * 2.0;
            let h = g * 2.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(
                rect,
                g * 0.4,
                Color32::from_rgb(200, 200, 200),
                Stroke::new(2.0, border),
            );
            draw_gate_label(painter, pos, w, h, "BTN", state);
        }

        ComponentKind::Subcircuit { circuit_name } => {
            let w = g * 4.0;
            let h = g * 3.0;
            let rect = Rect::from_min_size(pos, Vec2::new(w, h));
            painter.rect(
                rect,
                g * 0.2,
                Color32::from_rgb(230, 200, 230),
                Stroke::new(1.5, border),
            );
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
            painter.rect(rect, g * 0.2, body_color, Stroke::new(1.5, border));
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
    let w = g * 4.0;
    let h = g * 4.0;
    let rect = Rect::from_min_size(pos, Vec2::new(w, h));
    painter.rect(rect, g * 0.2, fill, Stroke::new(1.5, border));
    draw_gate_label(painter, pos, w, h, label, state);
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
