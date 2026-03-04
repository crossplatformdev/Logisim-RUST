//! Attribute panel: shows the properties of the currently selected component,
//! matching the upstream Logisim-Evolution attribute table (AttrTable).

use crate::state::{AppState, BASE_GRID_PX};
use egui::Ui;
use logisim_core::component::{ComponentKind, Facing};
use logisim_core::history::UndoAction;
use logisim_core::value::BitWidth;

/// Renders the attribute table for the currently selected component(s).
///
/// Matches the upstream Logisim-Evolution `AttrTable` panel:
/// - One component selected → full attribute grid with editable label
/// - Multiple selected → count summary
/// - Nothing selected → neutral message
pub fn show_attr_panel(ui: &mut Ui, state: &mut AppState) {
    ui.separator();
    ui.heading("Selection");
    ui.separator();

    if state.selected.is_empty() {
        ui.label("(nothing selected)");
        return;
    }

    let circuit_name = state.active_circuit.clone();
    let circuit = match state.project.circuits.get(&circuit_name) {
        Some(c) => c,
        None => {
            ui.label("Circuit not found.");
            return;
        }
    };

    if state.selected.len() > 1 {
        ui.label(format!("{} components selected.", state.selected.len()));
        return;
    }

    let comp_id = state.selected[0];
    let comp = match circuit.components.get(&comp_id) {
        Some(c) => c,
        None => {
            ui.label("Component not found.");
            return;
        }
    };

    let comp_x = comp.x;
    let comp_y = comp.y;
    let comp_facing = comp.facing;
    let comp_label = comp.label.clone();
    let comp_kind = comp.kind.clone();

    // pending_kind: collected inside the grid closure, applied after the grid closes.
    let mut pending_kind: Option<ComponentKind> = None;

    // Attribute grid: two columns (name, value), matching upstream row layout.
    egui::Grid::new("attr_grid")
        .num_columns(2)
        .striped(true)
        .spacing([8.0, 2.0])
        .min_col_width(60.0)
        .show(ui, |ui| {
            // ── Common attributes ────────────────────────────────────────────
            attr_row(ui, "Type", component_type_name(&comp_kind));
            attr_row(ui, "X", &(comp_x * BASE_GRID_PX as i32).to_string());
            attr_row(ui, "Y", &(comp_y * BASE_GRID_PX as i32).to_string());

            // Editable label field — matches upstream Logisim-Evolution behaviour.
            // Use egui's temporary per-id memory so intermediate typed text is not
            // overwritten each frame when the widget has focus.
            ui.label(egui::RichText::new("Label").weak());
            let edit_id = egui::Id::new(("label_edit", comp_id));
            let mut label_buf: String = ui
                .data_mut(|d| d.get_temp::<String>(edit_id))
                .unwrap_or_else(|| comp_label.clone());
            let label_resp = ui.text_edit_singleline(&mut label_buf);
            ui.data_mut(|d| d.insert_temp(edit_id, label_buf.clone()));
            if label_resp.lost_focus() && label_buf != comp_label {
                // Commit the label change with undo support.
                let action = UndoAction::ChangeLabel {
                    circuit_name: circuit_name.clone(),
                    id: comp_id,
                    old_label: comp_label.clone(),
                    new_label: label_buf.clone(),
                };
                action.apply(&mut state.project);
                state.history.push(action);
                state.modified = true;
                state.sync_simulator();
                // Clear temp buffer so next open shows the committed value.
                ui.data_mut(|d| d.remove::<String>(edit_id));
            }
            ui.end_row();

            // ── Editable Facing ──────────────────────────────────────────────
            ui.label(egui::RichText::new("Facing").weak());
            let all_facings = [Facing::East, Facing::West, Facing::North, Facing::South];
            let mut current_facing = comp_facing;
            egui::ComboBox::from_id_salt(("facing_combo", comp_id))
                .selected_text(facing_name(current_facing))
                .show_ui(ui, |ui| {
                    for &f in &all_facings {
                        ui.selectable_value(&mut current_facing, f, facing_name(f));
                    }
                });
            if current_facing != comp_facing {
                let action = UndoAction::ChangeFacing {
                    circuit_name: circuit_name.clone(),
                    id: comp_id,
                    old_facing: comp_facing,
                    new_facing: current_facing,
                };
                action.apply(&mut state.project);
                state.history.push(action);
                state.modified = true;
                state.sync_simulator();
            }
            ui.end_row();

            // ── Kind-specific attributes ─────────────────────────────────────
            // Collect a pending kind change; applied after the grid closes.
            pending_kind = kind_attrs_editable(ui, &comp_kind);

            // ── Extra XML attributes ──────────────────────────────────────────
            // Re-borrow after potential mutation above.
            // Skip keys that are already shown as first-class editable fields
            // above (label, facing) to avoid stale duplicate rows.
            if let Some(c) = state.project.circuits.get(&circuit_name) {
                if let Some(comp) = c.components.get(&comp_id) {
                    let mut sorted_attrs: Vec<_> = comp
                        .attributes
                        .iter()
                        .filter(|(k, _)| k.as_str() != "label" && k.as_str() != "facing")
                        .collect();
                    sorted_attrs.sort_by_key(|(k, _)| k.as_str());
                    for (k, v) in sorted_attrs {
                        attr_row(ui, k, v);
                    }
                }
            }
        });

    // Apply any kind change that was triggered inside the grid.
    if let Some(new_kind) = pending_kind {
        let action = UndoAction::ChangeKind {
            circuit_name: circuit_name.clone(),
            id: comp_id,
            old_kind: comp_kind,
            new_kind,
        };
        action.apply(&mut state.project);
        state.history.push(action);
        state.modified = true;
        state.sync_simulator();
    }
}

/// Renders kind-specific attribute rows, returning a new `ComponentKind` if any
/// editable field was changed by the user (via DragValue). Returns `None` when
/// nothing changed.
///
/// Editable fields match upstream Logisim-Evolution:
/// - **Data Bits** (1–64) — all components that carry a bit-width
/// - **Inputs** (2–32) — multi-input gates
/// - **Select Bits** (1–10) — plexers
/// - **Fan Out** (2–32) — Splitter
fn kind_attrs_editable(ui: &mut Ui, kind: &ComponentKind) -> Option<ComponentKind> {
    match kind {
        ComponentKind::Pin { is_output, width } => {
            attr_row(ui, "I/O", if *is_output { "Output" } else { "Input" });
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::Pin {
                is_output: *is_output,
                width: new_w,
            })
        }
        ComponentKind::Constant { width, value } => {
            let new_w = edit_width(ui, "Data Bits", *width);
            attr_row(ui, "Value", &format!("0x{:X}", value));
            new_w.map(|w| ComponentKind::Constant {
                width: w,
                value: *value,
            })
        }
        ComponentKind::Probe { width } => {
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::Probe { width: new_w })
        }
        ComponentKind::Tunnel { label, width } => {
            attr_row(ui, "Label", label);
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::Tunnel {
                label: label.clone(),
                width: new_w,
            })
        }
        ComponentKind::Splitter {
            combined_width,
            fan_out,
        } => {
            let new_w = edit_width(ui, "Bit Width", *combined_width);
            let mut new_fo = *fan_out as u32;
            let fo_changed = drag_row(ui, "Fan Out", &mut new_fo, 2, 32);
            if new_w.is_some() || fo_changed {
                Some(ComponentKind::Splitter {
                    combined_width: new_w.unwrap_or(*combined_width),
                    fan_out: new_fo as u8,
                })
            } else {
                None
            }
        }
        ComponentKind::PullResistor { direction, width } => {
            attr_row(ui, "Pull", &format!("{:?}", direction));
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::PullResistor {
                direction: *direction,
                width: new_w,
            })
        }
        ComponentKind::TristateBuffer { width } => {
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::TristateBuffer { width: new_w })
        }
        ComponentKind::ControlledBuffer { width } => {
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::ControlledBuffer { width: new_w })
        }
        ComponentKind::Buffer { width } => {
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::Buffer { width: new_w })
        }
        ComponentKind::NotGate { width } => {
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::NotGate { width: new_w })
        }
        ComponentKind::AndGate {
            inputs,
            width,
            negate_inputs,
            negate_output,
        } => {
            let changed = edit_inputs_width(ui, *inputs, *width);
            attr_row(ui, "Negate Out", if *negate_output { "Yes" } else { "No" });
            let neg_str: String = negate_inputs
                .iter()
                .map(|&n| if n { '1' } else { '0' })
                .collect();
            attr_row(ui, "Negate In", &neg_str);
            changed.map(|(new_inputs, new_w)| {
                let new_negs = resize_negates(negate_inputs, new_inputs as usize);
                ComponentKind::AndGate {
                    inputs: new_inputs,
                    width: new_w,
                    negate_inputs: new_negs,
                    negate_output: *negate_output,
                }
            })
        }
        ComponentKind::OrGate {
            inputs,
            width,
            negate_inputs,
            negate_output,
        } => {
            let changed = edit_inputs_width(ui, *inputs, *width);
            attr_row(ui, "Negate Out", if *negate_output { "Yes" } else { "No" });
            let neg_str: String = negate_inputs
                .iter()
                .map(|&n| if n { '1' } else { '0' })
                .collect();
            attr_row(ui, "Negate In", &neg_str);
            changed.map(|(new_inputs, new_w)| {
                let new_negs = resize_negates(negate_inputs, new_inputs as usize);
                ComponentKind::OrGate {
                    inputs: new_inputs,
                    width: new_w,
                    negate_inputs: new_negs,
                    negate_output: *negate_output,
                }
            })
        }
        ComponentKind::NandGate { inputs, width } => {
            let (new_inputs, new_w) = edit_inputs_width(ui, *inputs, *width)?;
            Some(ComponentKind::NandGate {
                inputs: new_inputs,
                width: new_w,
            })
        }
        ComponentKind::NorGate { inputs, width } => {
            let (new_inputs, new_w) = edit_inputs_width(ui, *inputs, *width)?;
            Some(ComponentKind::NorGate {
                inputs: new_inputs,
                width: new_w,
            })
        }
        ComponentKind::XorGate { inputs, width } => {
            let (new_inputs, new_w) = edit_inputs_width(ui, *inputs, *width)?;
            Some(ComponentKind::XorGate {
                inputs: new_inputs,
                width: new_w,
            })
        }
        ComponentKind::XnorGate { inputs, width } => {
            let (new_inputs, new_w) = edit_inputs_width(ui, *inputs, *width)?;
            Some(ComponentKind::XnorGate {
                inputs: new_inputs,
                width: new_w,
            })
        }
        ComponentKind::OddParityGate { inputs, width } => {
            let (new_inputs, new_w) = edit_inputs_width(ui, *inputs, *width)?;
            Some(ComponentKind::OddParityGate {
                inputs: new_inputs,
                width: new_w,
            })
        }
        ComponentKind::EvenParityGate { inputs, width } => {
            let (new_inputs, new_w) = edit_inputs_width(ui, *inputs, *width)?;
            Some(ComponentKind::EvenParityGate {
                inputs: new_inputs,
                width: new_w,
            })
        }
        ComponentKind::Multiplexer {
            select_bits,
            data_width,
        } => {
            let (new_sel, new_w) = edit_select_bits_width(ui, *select_bits, *data_width)?;
            Some(ComponentKind::Multiplexer {
                select_bits: new_sel,
                data_width: new_w,
            })
        }
        ComponentKind::Demultiplexer {
            select_bits,
            data_width,
        } => {
            let (new_sel, new_w) = edit_select_bits_width(ui, *select_bits, *data_width)?;
            Some(ComponentKind::Demultiplexer {
                select_bits: new_sel,
                data_width: new_w,
            })
        }
        ComponentKind::Decoder { select_bits } => {
            let mut new_s = *select_bits as u32;
            if drag_row(ui, "Select Bits", &mut new_s, 1, 10) {
                Some(ComponentKind::Decoder {
                    select_bits: new_s as u8,
                })
            } else {
                None
            }
        }
        ComponentKind::PriorityEncoder { select_bits } => {
            let mut new_s = *select_bits as u32;
            if drag_row(ui, "Select Bits", &mut new_s, 1, 10) {
                Some(ComponentKind::PriorityEncoder {
                    select_bits: new_s as u8,
                })
            } else {
                None
            }
        }
        ComponentKind::BitSelector {
            group_bits,
            data_width,
        } => {
            let mut new_g = *group_bits as u32;
            let g_changed = drag_row(ui, "Group Bits", &mut new_g, 1, 10);
            let new_w = edit_width(ui, "Data Bits", *data_width);
            let new_w_val = new_w.unwrap_or(*data_width);
            if g_changed || new_w.is_some() {
                Some(ComponentKind::BitSelector {
                    group_bits: new_g as u8,
                    data_width: new_w_val,
                })
            } else {
                None
            }
        }
        ComponentKind::BitExtender {
            input_width,
            output_width,
        } => {
            let new_in = edit_width(ui, "In Bits", *input_width);
            let new_out = edit_width(ui, "Out Bits", *output_width);
            if new_in.is_some() || new_out.is_some() {
                Some(ComponentKind::BitExtender {
                    input_width: new_in.unwrap_or(*input_width),
                    output_width: new_out.unwrap_or(*output_width),
                })
            } else {
                None
            }
        }
        ComponentKind::Adder { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Adder { width: w })
        }
        ComponentKind::Subtractor { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Subtractor { width: w })
        }
        ComponentKind::Multiplier { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Multiplier { width: w })
        }
        ComponentKind::Divider { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Divider { width: w })
        }
        ComponentKind::Negator { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Negator { width: w })
        }
        ComponentKind::Comparator { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Comparator { width: w })
        }
        ComponentKind::BitAdder { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::BitAdder { width: w })
        }
        ComponentKind::DFlipFlop { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::DFlipFlop { width: w })
        }
        ComponentKind::TFlipFlop { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::TFlipFlop { width: w })
        }
        ComponentKind::JKFlipFlop { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::JKFlipFlop { width: w })
        }
        ComponentKind::SRFlipFlop { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::SRFlipFlop { width: w })
        }
        ComponentKind::Register { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Register { width: w })
        }
        ComponentKind::Counter { width } => {
            edit_width(ui, "Data Bits", *width).map(|w| ComponentKind::Counter { width: w })
        }
        ComponentKind::ShiftRegister { stages, width } => {
            let mut new_s = *stages as u32;
            let s_changed = drag_row(ui, "Stages", &mut new_s, 1, 64);
            let new_w = edit_width(ui, "Data Bits", *width);
            if s_changed || new_w.is_some() {
                Some(ComponentKind::ShiftRegister {
                    stages: new_s as u8,
                    width: new_w.unwrap_or(*width),
                })
            } else {
                None
            }
        }
        ComponentKind::BitFinder { width, find_type } => {
            let new_w = edit_width(ui, "Data Bits", *width);
            attr_row(ui, "Find", &format!("{:?}", find_type));
            new_w.map(|w| ComponentKind::BitFinder {
                width: w,
                find_type: *find_type,
            })
        }
        ComponentKind::Ram {
            addr_bits,
            data_bits,
            sync,
        } => {
            let mut new_a = *addr_bits as u32;
            let a_changed = drag_row(ui, "Address Bits", &mut new_a, 1, 24);
            let new_d = edit_width(ui, "Data Bits", *data_bits);
            attr_row(ui, "Synchronous", if *sync { "Yes" } else { "No" });
            if a_changed || new_d.is_some() {
                Some(ComponentKind::Ram {
                    addr_bits: new_a as u8,
                    data_bits: new_d.unwrap_or(*data_bits),
                    sync: *sync,
                })
            } else {
                None
            }
        }
        ComponentKind::Rom {
            addr_bits,
            data_bits,
            contents,
        } => {
            let mut new_a = *addr_bits as u32;
            let a_changed = drag_row(ui, "Address Bits", &mut new_a, 1, 24);
            let new_d = edit_width(ui, "Data Bits", *data_bits);
            if a_changed || new_d.is_some() {
                Some(ComponentKind::Rom {
                    addr_bits: new_a as u8,
                    data_bits: new_d.unwrap_or(*data_bits),
                    contents: contents.clone(),
                })
            } else {
                None
            }
        }
        ComponentKind::ShiftRegisterMemory {
            stages,
            width,
            parallel_load,
        } => {
            let mut new_s = *stages as u32;
            let s_changed = drag_row(ui, "Stages", &mut new_s, 1, 64);
            let new_w = edit_width(ui, "Data Bits", *width);
            attr_row(
                ui,
                "Parallel Load",
                if *parallel_load { "Yes" } else { "No" },
            );
            if s_changed || new_w.is_some() {
                Some(ComponentKind::ShiftRegisterMemory {
                    stages: new_s as u8,
                    width: new_w.unwrap_or(*width),
                    parallel_load: *parallel_load,
                })
            } else {
                None
            }
        }
        ComponentKind::Transistor { width, p_type } => {
            let new_w = edit_width(ui, "Data Bits", *width);
            attr_row(ui, "Type", if *p_type { "P-type" } else { "N-type" });
            new_w.map(|w| ComponentKind::Transistor {
                width: w,
                p_type: *p_type,
            })
        }
        ComponentKind::TransmissionGate { width } => {
            let new_w = edit_width(ui, "Data Bits", *width)?;
            Some(ComponentKind::TransmissionGate { width: new_w })
        }
        ComponentKind::DipSwitch { switches } => {
            let mut new_s = *switches as u32;
            if drag_row(ui, "Switches", &mut new_s, 1, 32) {
                Some(ComponentKind::DipSwitch {
                    switches: new_s as u8,
                })
            } else {
                None
            }
        }
        ComponentKind::DotMatrix { rows, cols } => {
            attr_row(ui, "Rows", &rows.to_string());
            attr_row(ui, "Columns", &cols.to_string());
            None
        }
        ComponentKind::Tty { rows, cols } => {
            attr_row(ui, "Rows", &rows.to_string());
            attr_row(ui, "Columns", &cols.to_string());
            None
        }
        ComponentKind::Subcircuit { circuit_name } => {
            attr_row(ui, "Circuit", circuit_name);
            None
        }
        // No extra attrs for these kinds:
        ComponentKind::Power
        | ComponentKind::Ground
        | ComponentKind::Clock
        | ComponentKind::Led
        | ComponentKind::RgbLed
        | ComponentKind::SevenSegDisplay
        | ComponentKind::HexDisplay
        | ComponentKind::Button
        | ComponentKind::Keyboard
        | ComponentKind::Ttl7400
        | ComponentKind::Ttl7402
        | ComponentKind::Ttl7404
        | ComponentKind::Ttl7408
        | ComponentKind::Ttl7432
        | ComponentKind::Ttl7486 => None,
    }
}

/// Renders a "Data Bits" drag-value row. Returns `Some(new_width)` if the value changed.
fn edit_width(ui: &mut Ui, label: &str, current: BitWidth) -> Option<BitWidth> {
    let mut v = current.get();
    if drag_row(ui, label, &mut v, 1, 64) {
        Some(BitWidth::new(v))
    } else {
        None
    }
}

/// Renders "Inputs" and "Data Bits" rows.  Returns `Some((new_inputs, new_width))` if
/// either changed.
fn edit_inputs_width(ui: &mut Ui, inputs: u8, width: BitWidth) -> Option<(u8, BitWidth)> {
    let mut new_i = inputs as u32;
    let i_changed = drag_row(ui, "Inputs", &mut new_i, 2, 32);
    let new_w = edit_width(ui, "Data Bits", width);
    if i_changed || new_w.is_some() {
        Some((new_i as u8, new_w.unwrap_or(width)))
    } else {
        None
    }
}

/// Renders "Select Bits" and "Data Bits" rows. Returns `Some((new_sel, new_width))` if
/// either changed.
fn edit_select_bits_width(ui: &mut Ui, sel: u8, width: BitWidth) -> Option<(u8, BitWidth)> {
    let mut new_s = sel as u32;
    let s_changed = drag_row(ui, "Select Bits", &mut new_s, 1, 10);
    let new_w = edit_width(ui, "Data Bits", width);
    if s_changed || new_w.is_some() {
        Some((new_s as u8, new_w.unwrap_or(width)))
    } else {
        None
    }
}

/// Renders a single label + DragValue row. Returns `true` if the value changed.
fn drag_row(ui: &mut Ui, name: &str, value: &mut u32, min: u32, max: u32) -> bool {
    ui.label(egui::RichText::new(name).weak());
    let resp = ui.add(egui::DragValue::new(value).range(min..=max).speed(1.0));
    ui.end_row();
    resp.changed()
}

/// Resize a `negate_inputs` vector to the new number of inputs, padding with `false`.
fn resize_negates(existing: &[bool], new_len: usize) -> Vec<bool> {
    let mut v = existing.to_vec();
    v.resize(new_len, false);
    v
}

fn attr_row(ui: &mut Ui, name: &str, value: &str) {
    ui.label(egui::RichText::new(name).weak());
    ui.label(value);
    ui.end_row();
}

fn facing_name(facing: Facing) -> &'static str {
    match facing {
        Facing::East => "East",
        Facing::West => "West",
        Facing::North => "North",
        Facing::South => "South",
    }
}

fn component_type_name(kind: &ComponentKind) -> &'static str {
    match kind {
        ComponentKind::Pin { .. } => "Pin",
        ComponentKind::Clock => "Clock",
        ComponentKind::Constant { .. } => "Constant",
        ComponentKind::Power => "Power",
        ComponentKind::Ground => "Ground",
        ComponentKind::Splitter { .. } => "Splitter",
        ComponentKind::Tunnel { .. } => "Tunnel",
        ComponentKind::Probe { .. } => "Probe",
        ComponentKind::PullResistor { .. } => "Pull Resistor",
        ComponentKind::TristateBuffer { .. } => "Tristate Buffer",
        ComponentKind::ControlledBuffer { .. } => "Controlled Buffer",
        ComponentKind::Transistor { .. } => "Transistor",
        ComponentKind::TransmissionGate { .. } => "Transmission Gate",
        ComponentKind::BitExtender { .. } => "Bit Extender",
        ComponentKind::AndGate { .. } => "AND Gate",
        ComponentKind::OrGate { .. } => "OR Gate",
        ComponentKind::NandGate { .. } => "NAND Gate",
        ComponentKind::NorGate { .. } => "NOR Gate",
        ComponentKind::XorGate { .. } => "XOR Gate",
        ComponentKind::XnorGate { .. } => "XNOR Gate",
        ComponentKind::NotGate { .. } => "NOT Gate",
        ComponentKind::Buffer { .. } => "Buffer",
        ComponentKind::OddParityGate { .. } => "Odd Parity Gate",
        ComponentKind::EvenParityGate { .. } => "Even Parity Gate",
        ComponentKind::Multiplexer { .. } => "Multiplexer",
        ComponentKind::Demultiplexer { .. } => "Demultiplexer",
        ComponentKind::Decoder { .. } => "Decoder",
        ComponentKind::PriorityEncoder { .. } => "Priority Encoder",
        ComponentKind::BitSelector { .. } => "Bit Selector",
        ComponentKind::Adder { .. } => "Adder",
        ComponentKind::Subtractor { .. } => "Subtractor",
        ComponentKind::Multiplier { .. } => "Multiplier",
        ComponentKind::Divider { .. } => "Divider",
        ComponentKind::Negator { .. } => "Negator",
        ComponentKind::Comparator { .. } => "Comparator",
        ComponentKind::ShiftRegister { .. } => "Shift Register",
        ComponentKind::BitAdder { .. } => "Bit Adder",
        ComponentKind::BitFinder { .. } => "Bit Finder",
        ComponentKind::DFlipFlop { .. } => "D Flip-Flop",
        ComponentKind::TFlipFlop { .. } => "T Flip-Flop",
        ComponentKind::JKFlipFlop { .. } => "JK Flip-Flop",
        ComponentKind::SRFlipFlop { .. } => "SR Flip-Flop",
        ComponentKind::Register { .. } => "Register",
        ComponentKind::Counter { .. } => "Counter",
        ComponentKind::Ram { .. } => "RAM",
        ComponentKind::Rom { .. } => "ROM",
        ComponentKind::ShiftRegisterMemory { .. } => "Shift Register (Mem)",
        ComponentKind::Led => "LED",
        ComponentKind::RgbLed => "RGB LED",
        ComponentKind::SevenSegDisplay => "7-Segment Display",
        ComponentKind::HexDisplay => "Hex Digit Display",
        ComponentKind::DotMatrix { .. } => "Dot Matrix",
        ComponentKind::Button => "Button",
        ComponentKind::DipSwitch { .. } => "DIP Switch",
        ComponentKind::Keyboard => "Keyboard",
        ComponentKind::Tty { .. } => "TTY",
        ComponentKind::Subcircuit { .. } => "Subcircuit",
        ComponentKind::Ttl7400 => "74x00 NAND",
        ComponentKind::Ttl7402 => "74x02 NOR",
        ComponentKind::Ttl7404 => "74x04 NOT",
        ComponentKind::Ttl7408 => "74x08 AND",
        ComponentKind::Ttl7432 => "74x32 OR",
        ComponentKind::Ttl7486 => "74x86 XOR",
    }
}
