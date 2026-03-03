//! Left-side component palette panel.

use crate::state::{AppState, Tool};
use egui::Ui;
use logisim_core::{component::ComponentKind, value::BitWidth};

/// The component palette widget.
pub struct ComponentPanel;

impl ComponentPanel {
    pub fn new() -> Self {
        ComponentPanel
    }

    pub fn show(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.heading("Components");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── Wiring ─────────────────────────────────────────────────────
            ui.collapsing("Wiring", |ui| {
                comp_button(
                    ui,
                    state,
                    "Input Pin",
                    ComponentKind::Pin {
                        is_output: false,
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Output Pin",
                    ComponentKind::Pin {
                        is_output: true,
                        width: BitWidth::ONE,
                    },
                );
                comp_button(ui, state, "Clock", ComponentKind::Clock);
                comp_button(
                    ui,
                    state,
                    "Constant",
                    ComponentKind::Constant {
                        width: BitWidth::FOUR,
                        value: 0,
                    },
                );
                comp_button(ui, state, "Power", ComponentKind::Power);
                comp_button(ui, state, "Ground", ComponentKind::Ground);
                comp_button(
                    ui,
                    state,
                    "Splitter",
                    ComponentKind::Splitter {
                        combined_width: BitWidth::FOUR,
                        fan_out: 4,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Tunnel",
                    ComponentKind::Tunnel {
                        label: "net".to_string(),
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Probe",
                    ComponentKind::Probe {
                        width: BitWidth::ONE,
                    },
                );
            });

            // ── Gates ──────────────────────────────────────────────────────
            ui.collapsing("Gates", |ui| {
                comp_button(
                    ui,
                    state,
                    "AND Gate",
                    ComponentKind::AndGate {
                        inputs: 2,
                        width: BitWidth::ONE,
                        negate_inputs: vec![false, false],
                        negate_output: false,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "OR Gate",
                    ComponentKind::OrGate {
                        inputs: 2,
                        width: BitWidth::ONE,
                        negate_inputs: vec![false, false],
                        negate_output: false,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "NAND Gate",
                    ComponentKind::NandGate {
                        inputs: 2,
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "NOR Gate",
                    ComponentKind::NorGate {
                        inputs: 2,
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "XOR Gate",
                    ComponentKind::XorGate {
                        inputs: 2,
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "XNOR Gate",
                    ComponentKind::XnorGate {
                        inputs: 2,
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "NOT Gate",
                    ComponentKind::NotGate {
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Buffer",
                    ComponentKind::Buffer {
                        width: BitWidth::ONE,
                    },
                );
            });

            // ── Plexers ────────────────────────────────────────────────────
            ui.collapsing("Plexers", |ui| {
                comp_button(
                    ui,
                    state,
                    "Multiplexer",
                    ComponentKind::Multiplexer {
                        select_bits: 1,
                        data_width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Demultiplexer",
                    ComponentKind::Demultiplexer {
                        select_bits: 1,
                        data_width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Decoder",
                    ComponentKind::Decoder { select_bits: 2 },
                );
                comp_button(
                    ui,
                    state,
                    "Priority Encoder",
                    ComponentKind::PriorityEncoder { select_bits: 2 },
                );
            });

            // ── Arithmetic ─────────────────────────────────────────────────
            ui.collapsing("Arithmetic", |ui| {
                comp_button(
                    ui,
                    state,
                    "Adder",
                    ComponentKind::Adder {
                        width: BitWidth::FOUR,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Subtractor",
                    ComponentKind::Subtractor {
                        width: BitWidth::FOUR,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Multiplier",
                    ComponentKind::Multiplier {
                        width: BitWidth::FOUR,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Divider",
                    ComponentKind::Divider {
                        width: BitWidth::FOUR,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Negator",
                    ComponentKind::Negator {
                        width: BitWidth::FOUR,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Comparator",
                    ComponentKind::Comparator {
                        width: BitWidth::FOUR,
                    },
                );
            });

            // ── Memory ─────────────────────────────────────────────────────
            ui.collapsing("Memory", |ui| {
                comp_button(
                    ui,
                    state,
                    "D Flip-Flop",
                    ComponentKind::DFlipFlop {
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "T Flip-Flop",
                    ComponentKind::TFlipFlop {
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "JK Flip-Flop",
                    ComponentKind::JKFlipFlop {
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "SR Flip-Flop",
                    ComponentKind::SRFlipFlop {
                        width: BitWidth::ONE,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Register",
                    ComponentKind::Register {
                        width: BitWidth::EIGHT,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "RAM",
                    ComponentKind::Ram {
                        addr_bits: 8,
                        data_bits: BitWidth::EIGHT,
                        sync: false,
                    },
                );
                comp_button(
                    ui,
                    state,
                    "ROM",
                    ComponentKind::Rom {
                        addr_bits: 8,
                        data_bits: BitWidth::EIGHT,
                        contents: vec![],
                    },
                );
                comp_button(
                    ui,
                    state,
                    "Counter",
                    ComponentKind::Counter {
                        width: BitWidth::FOUR,
                    },
                );
            });

            // ── I/O ────────────────────────────────────────────────────────
            ui.collapsing("I/O", |ui| {
                comp_button(ui, state, "LED", ComponentKind::Led);
                comp_button(ui, state, "RGB LED", ComponentKind::RgbLed);
                comp_button(ui, state, "7-Segment", ComponentKind::SevenSegDisplay);
                comp_button(ui, state, "Hex Display", ComponentKind::HexDisplay);
                comp_button(ui, state, "Button", ComponentKind::Button);
                comp_button(
                    ui,
                    state,
                    "DIP Switch",
                    ComponentKind::DipSwitch { switches: 8 },
                );
            });
        });
    }
}

fn comp_button(ui: &mut Ui, state: &mut AppState, label: &str, kind: ComponentKind) {
    let active = matches!(&state.tool, Tool::Place(k) if k == &kind);
    if ui.selectable_label(active, label).clicked() {
        state.tool = Tool::Place(kind);
    }
}
