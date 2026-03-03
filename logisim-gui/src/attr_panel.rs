//! Attribute panel: shows the properties of the currently selected component,
//! matching the upstream Logisim-Evolution attribute table (AttrTable).

use crate::state::AppState;
use egui::Ui;
use logisim_core::component::{ComponentKind, Facing};
use logisim_core::history::UndoAction;

/// Grid-to-pixel conversion factor (10 px per grid unit at zoom 1×).
const GRID_PX: i32 = 10;

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

    // Attribute grid: two columns (name, value), matching upstream row layout.
    egui::Grid::new("attr_grid")
        .num_columns(2)
        .striped(true)
        .spacing([8.0, 2.0])
        .min_col_width(60.0)
        .show(ui, |ui| {
            // ── Common attributes ────────────────────────────────────────────
            attr_row(ui, "Type", component_type_name(&comp_kind));
            attr_row(ui, "X", &(comp_x * GRID_PX).to_string());
            attr_row(ui, "Y", &(comp_y * GRID_PX).to_string());

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
            kind_attrs(ui, &comp_kind);

            // ── Extra XML attributes ──────────────────────────────────────────
            // Re-borrow after potential mutation above.
            if let Some(c) = state.project.circuits.get(&circuit_name) {
                if let Some(comp) = c.components.get(&comp_id) {
                    let mut sorted_attrs: Vec<_> = comp.attributes.iter().collect();
                    sorted_attrs.sort_by_key(|(k, _)| k.as_str());
                    for (k, v) in sorted_attrs {
                        attr_row(ui, k, v);
                    }
                }
            }
        });
}

fn kind_attrs(ui: &mut Ui, kind: &ComponentKind) {
    match kind {
        ComponentKind::Pin { is_output, width } => {
            attr_row(ui, "I/O", if *is_output { "Output" } else { "Input" });
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::Constant { width, value } => {
            attr_row(ui, "Data Bits", &width.get().to_string());
            attr_row(ui, "Value", &format!("0x{:X}", value));
        }
        ComponentKind::Probe { width } => {
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::Tunnel { label, width } => {
            attr_row(ui, "Label", label);
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::Splitter {
            combined_width,
            fan_out,
        } => {
            attr_row(ui, "Bit Width", &combined_width.get().to_string());
            attr_row(ui, "Fan Out", &fan_out.to_string());
        }
        ComponentKind::PullResistor { direction, width } => {
            attr_row(ui, "Pull", &format!("{:?}", direction));
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::TristateBuffer { width }
        | ComponentKind::ControlledBuffer { width }
        | ComponentKind::Buffer { width }
        | ComponentKind::NotGate { width } => {
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::AndGate {
            inputs,
            width,
            negate_inputs,
            negate_output,
        }
        | ComponentKind::OrGate {
            inputs,
            width,
            negate_inputs,
            negate_output,
        } => {
            attr_row(ui, "Inputs", &inputs.to_string());
            attr_row(ui, "Data Bits", &width.get().to_string());
            attr_row(ui, "Negate Out", if *negate_output { "Yes" } else { "No" });
            let neg_str: String = negate_inputs
                .iter()
                .map(|&n| if n { '1' } else { '0' })
                .collect();
            attr_row(ui, "Negate In", &neg_str);
        }
        ComponentKind::NandGate { inputs, width }
        | ComponentKind::NorGate { inputs, width }
        | ComponentKind::XorGate { inputs, width }
        | ComponentKind::XnorGate { inputs, width }
        | ComponentKind::OddParityGate { inputs, width }
        | ComponentKind::EvenParityGate { inputs, width } => {
            attr_row(ui, "Inputs", &inputs.to_string());
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::Multiplexer {
            select_bits,
            data_width,
        }
        | ComponentKind::Demultiplexer {
            select_bits,
            data_width,
        } => {
            attr_row(ui, "Select Bits", &select_bits.to_string());
            attr_row(ui, "Data Bits", &data_width.get().to_string());
        }
        ComponentKind::Decoder { select_bits } | ComponentKind::PriorityEncoder { select_bits } => {
            attr_row(ui, "Select Bits", &select_bits.to_string());
        }
        ComponentKind::BitSelector {
            group_bits,
            data_width,
        } => {
            attr_row(ui, "Group Bits", &group_bits.to_string());
            attr_row(ui, "Data Bits", &data_width.get().to_string());
        }
        ComponentKind::BitExtender {
            input_width,
            output_width,
        } => {
            attr_row(ui, "In Bits", &input_width.get().to_string());
            attr_row(ui, "Out Bits", &output_width.get().to_string());
        }
        ComponentKind::Adder { width }
        | ComponentKind::Subtractor { width }
        | ComponentKind::Multiplier { width }
        | ComponentKind::Divider { width }
        | ComponentKind::Negator { width }
        | ComponentKind::Comparator { width }
        | ComponentKind::BitAdder { width }
        | ComponentKind::DFlipFlop { width }
        | ComponentKind::TFlipFlop { width }
        | ComponentKind::JKFlipFlop { width }
        | ComponentKind::SRFlipFlop { width }
        | ComponentKind::Register { width }
        | ComponentKind::Counter { width } => {
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::ShiftRegister { stages, width } => {
            attr_row(ui, "Stages", &stages.to_string());
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::BitFinder { width, find_type } => {
            attr_row(ui, "Data Bits", &width.get().to_string());
            attr_row(ui, "Find", &format!("{:?}", find_type));
        }
        ComponentKind::Ram {
            addr_bits,
            data_bits,
            sync,
        } => {
            attr_row(ui, "Address Bits", &addr_bits.to_string());
            attr_row(ui, "Data Bits", &data_bits.get().to_string());
            attr_row(ui, "Synchronous", if *sync { "Yes" } else { "No" });
        }
        ComponentKind::Rom {
            addr_bits,
            data_bits,
            ..
        } => {
            attr_row(ui, "Address Bits", &addr_bits.to_string());
            attr_row(ui, "Data Bits", &data_bits.get().to_string());
        }
        ComponentKind::ShiftRegisterMemory {
            stages,
            width,
            parallel_load,
        } => {
            attr_row(ui, "Stages", &stages.to_string());
            attr_row(ui, "Data Bits", &width.get().to_string());
            attr_row(
                ui,
                "Parallel Load",
                if *parallel_load { "Yes" } else { "No" },
            );
        }
        ComponentKind::Transistor { width, p_type } => {
            attr_row(ui, "Data Bits", &width.get().to_string());
            attr_row(ui, "Type", if *p_type { "P-type" } else { "N-type" });
        }
        ComponentKind::TransmissionGate { width } => {
            attr_row(ui, "Data Bits", &width.get().to_string());
        }
        ComponentKind::DipSwitch { switches } => {
            attr_row(ui, "Switches", &switches.to_string());
        }
        ComponentKind::DotMatrix { rows, cols } => {
            attr_row(ui, "Rows", &rows.to_string());
            attr_row(ui, "Columns", &cols.to_string());
        }
        ComponentKind::Tty { rows, cols } => {
            attr_row(ui, "Rows", &rows.to_string());
            attr_row(ui, "Columns", &cols.to_string());
        }
        ComponentKind::Subcircuit { circuit_name } => {
            attr_row(ui, "Circuit", circuit_name);
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
        | ComponentKind::Ttl7486 => {}
    }
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
