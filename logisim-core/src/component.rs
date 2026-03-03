//! Component model: kinds, ports, attributes, and placement.
//!
//! Every circuit element is a [`Component`] with a [`ComponentKind`] that
//! defines its logical behaviour, a set of [`Port`]s, and an arbitrary map of
//! string attributes (matching the Logisim-Evolution XML attribute format).

use crate::value::BitWidth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// ── Identifiers ───────────────────────────────────────────────────────────────

/// A unique handle for a component within a circuit.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ComponentId(pub u64);

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}", self.0)
    }
}

// ── Port direction / type ─────────────────────────────────────────────────────

/// Direction of a component port.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum PortDirection {
    Input,
    Output,
    /// Bidirectional (tristate I/O).
    BiDi,
}

// ── Port ──────────────────────────────────────────────────────────────────────

/// A single port on a component.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Port {
    /// Port name (unique within a component).
    pub name: String,
    /// Input or output.
    pub direction: PortDirection,
    /// Number of bits.
    pub width: BitWidth,
    /// Grid-relative offset from the component's origin.
    pub offset: (i32, i32),
}

impl Port {
    pub fn input(name: impl Into<String>, width: BitWidth, offset: (i32, i32)) -> Self {
        Port {
            name: name.into(),
            direction: PortDirection::Input,
            width,
            offset,
        }
    }

    pub fn output(name: impl Into<String>, width: BitWidth, offset: (i32, i32)) -> Self {
        Port {
            name: name.into(),
            direction: PortDirection::Output,
            width,
            offset,
        }
    }
}

// ── ComponentKind ─────────────────────────────────────────────────────────────

/// Every distinct logical element type in Logisim-Evolution.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ComponentKind {
    // ── Wiring ────────────────────────────────────────────────────────────────
    /// Input pin (drives a signal into the circuit).
    Pin {
        is_output: bool,
        width: BitWidth,
    },
    /// Clock signal generator.
    Clock,
    /// Constant value driver.
    Constant {
        width: BitWidth,
        value: u64,
    },
    /// Power (logic 1) driver.
    Power,
    /// Ground (logic 0) driver.
    Ground,
    /// Bit splitter: merges or splits multi-bit buses.
    Splitter {
        combined_width: BitWidth,
        fan_out: u8,
    },
    /// Tunnel (named wire connection across distance).
    Tunnel {
        label: String,
        width: BitWidth,
    },
    /// Probe (passive observation point).
    Probe {
        width: BitWidth,
    },
    /// Pull resistor (pull-up or pull-down).
    PullResistor {
        direction: PullDirection,
        width: BitWidth,
    },
    /// Tristate buffer.
    TristateBuffer {
        width: BitWidth,
    },
    /// Transistor (n-type or p-type).  A gate signal enables/disables data flow.
    Transistor {
        width: BitWidth,
        /// If true, gate=0 enables the transistor (p-type); otherwise gate=1 enables (n-type).
        p_type: bool,
    },
    /// Transmission gate: bidirectional switch controlled by complementary gate signals.
    TransmissionGate {
        width: BitWidth,
    },
    /// Bit extender: zero-extends an input bus to a wider output width.
    BitExtender {
        input_width: BitWidth,
        output_width: BitWidth,
    },

    // ── Basic Gates ───────────────────────────────────────────────────────────
    AndGate {
        inputs: u8,
        width: BitWidth,
        negate_inputs: Vec<bool>,
        negate_output: bool,
    },
    OrGate {
        inputs: u8,
        width: BitWidth,
        negate_inputs: Vec<bool>,
        negate_output: bool,
    },
    NandGate {
        inputs: u8,
        width: BitWidth,
    },
    NorGate {
        inputs: u8,
        width: BitWidth,
    },
    XorGate {
        inputs: u8,
        width: BitWidth,
    },
    XnorGate {
        inputs: u8,
        width: BitWidth,
    },
    NotGate {
        width: BitWidth,
    },
    Buffer {
        width: BitWidth,
    },
    ControlledBuffer {
        width: BitWidth,
    },
    /// Odd-parity gate: output is 1 when an odd number of input bits are 1.
    OddParityGate {
        inputs: u8,
        width: BitWidth,
    },
    /// Even-parity gate: output is 1 when an even number of input bits are 1.
    EvenParityGate {
        inputs: u8,
        width: BitWidth,
    },

    // ── Plexers ───────────────────────────────────────────────────────────────
    Multiplexer {
        select_bits: u8,
        data_width: BitWidth,
    },
    Demultiplexer {
        select_bits: u8,
        data_width: BitWidth,
    },
    Decoder {
        select_bits: u8,
    },
    PriorityEncoder {
        select_bits: u8,
    },
    BitSelector {
        group_bits: u8,
        data_width: BitWidth,
    },

    // ── Arithmetic ────────────────────────────────────────────────────────────
    Adder {
        width: BitWidth,
    },
    Subtractor {
        width: BitWidth,
    },
    Multiplier {
        width: BitWidth,
    },
    Divider {
        width: BitWidth,
    },
    Negator {
        width: BitWidth,
    },
    Comparator {
        width: BitWidth,
    },
    ShiftRegister {
        stages: u8,
        width: BitWidth,
    },
    BitAdder {
        width: BitWidth,
    },
    BitFinder {
        width: BitWidth,
        find_type: BitFinderType,
    },

    // ── Memory ────────────────────────────────────────────────────────────────
    DFlipFlop {
        width: BitWidth,
    },
    TFlipFlop {
        width: BitWidth,
    },
    JKFlipFlop {
        width: BitWidth,
    },
    SRFlipFlop {
        width: BitWidth,
    },
    Register {
        width: BitWidth,
    },
    Ram {
        addr_bits: u8,
        data_bits: BitWidth,
        sync: bool,
    },
    Rom {
        addr_bits: u8,
        data_bits: BitWidth,
        contents: Vec<u64>,
    },
    Counter {
        width: BitWidth,
    },
    ShiftRegisterMemory {
        stages: u8,
        width: BitWidth,
        parallel_load: bool,
    },

    // ── I/O ───────────────────────────────────────────────────────────────────
    Led,
    RgbLed,
    SevenSegDisplay,
    HexDisplay,
    DotMatrix {
        rows: u8,
        cols: u8,
    },
    Button,
    DipSwitch {
        switches: u8,
    },
    Keyboard,
    Tty {
        rows: u8,
        cols: u8,
    },

    // ── Subcircuit ────────────────────────────────────────────────────────────
    Subcircuit {
        circuit_name: String,
    },

    // ── TTL 74xx library ─────────────────────────────────────────────────────
    /// 7400 — Quad 2-Input NAND gate package.
    Ttl7400,
    /// 7402 — Quad 2-Input NOR gate package.
    Ttl7402,
    /// 7404 — Hex Inverter package.
    Ttl7404,
    /// 7408 — Quad 2-Input AND gate package.
    Ttl7408,
    /// 7432 — Quad 2-Input OR gate package.
    Ttl7432,
    /// 7486 — Quad 2-Input XOR gate package.
    Ttl7486,
}

/// Pull-up or pull-down direction for a pull resistor.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PullDirection {
    Up,
    Down,
}

/// What kind of bit finder (first 0 / first 1).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum BitFinderType {
    High,
    Low,
}

impl ComponentKind {
    /// Return the library name used in Logisim-Evolution XML (`lib` attribute).
    pub fn library_name(&self) -> &'static str {
        match self {
            ComponentKind::Pin { .. }
            | ComponentKind::Clock
            | ComponentKind::Constant { .. }
            | ComponentKind::Power
            | ComponentKind::Ground
            | ComponentKind::Splitter { .. }
            | ComponentKind::Tunnel { .. }
            | ComponentKind::Probe { .. }
            | ComponentKind::PullResistor { .. }
            | ComponentKind::TristateBuffer { .. }
            | ComponentKind::Transistor { .. }
            | ComponentKind::TransmissionGate { .. }
            | ComponentKind::BitExtender { .. } => "wiring",

            ComponentKind::AndGate { .. }
            | ComponentKind::OrGate { .. }
            | ComponentKind::NandGate { .. }
            | ComponentKind::NorGate { .. }
            | ComponentKind::XorGate { .. }
            | ComponentKind::XnorGate { .. }
            | ComponentKind::NotGate { .. }
            | ComponentKind::Buffer { .. }
            | ComponentKind::ControlledBuffer { .. }
            | ComponentKind::OddParityGate { .. }
            | ComponentKind::EvenParityGate { .. } => "gates",

            ComponentKind::Multiplexer { .. }
            | ComponentKind::Demultiplexer { .. }
            | ComponentKind::Decoder { .. }
            | ComponentKind::PriorityEncoder { .. }
            | ComponentKind::BitSelector { .. } => "plexers",

            ComponentKind::Adder { .. }
            | ComponentKind::Subtractor { .. }
            | ComponentKind::Multiplier { .. }
            | ComponentKind::Divider { .. }
            | ComponentKind::Negator { .. }
            | ComponentKind::Comparator { .. }
            | ComponentKind::ShiftRegister { .. }
            | ComponentKind::BitAdder { .. }
            | ComponentKind::BitFinder { .. } => "arithmetic",

            ComponentKind::DFlipFlop { .. }
            | ComponentKind::TFlipFlop { .. }
            | ComponentKind::JKFlipFlop { .. }
            | ComponentKind::SRFlipFlop { .. }
            | ComponentKind::Register { .. }
            | ComponentKind::Ram { .. }
            | ComponentKind::Rom { .. }
            | ComponentKind::Counter { .. }
            | ComponentKind::ShiftRegisterMemory { .. } => "memory",

            ComponentKind::Led
            | ComponentKind::RgbLed
            | ComponentKind::SevenSegDisplay
            | ComponentKind::HexDisplay
            | ComponentKind::DotMatrix { .. }
            | ComponentKind::Button
            | ComponentKind::DipSwitch { .. }
            | ComponentKind::Keyboard
            | ComponentKind::Tty { .. } => "io",

            ComponentKind::Subcircuit { .. } => "user",

            ComponentKind::Ttl7400
            | ComponentKind::Ttl7402
            | ComponentKind::Ttl7404
            | ComponentKind::Ttl7408
            | ComponentKind::Ttl7432
            | ComponentKind::Ttl7486 => "ttl",
        }
    }

    /// Human-readable component name (matches Logisim-Evolution XML `name` attribute).
    pub fn component_name(&self) -> String {
        match self {
            ComponentKind::Pin { .. } => "Pin".to_string(),
            ComponentKind::Clock => "Clock".to_string(),
            ComponentKind::Constant { .. } => "Constant".to_string(),
            ComponentKind::Power => "Power".to_string(),
            ComponentKind::Ground => "Ground".to_string(),
            ComponentKind::Splitter { .. } => "Splitter".to_string(),
            ComponentKind::Tunnel { .. } => "Tunnel".to_string(),
            ComponentKind::Probe { .. } => "Probe".to_string(),
            ComponentKind::PullResistor { .. } => "Pull Resistor".to_string(),
            ComponentKind::TristateBuffer { .. } => "Tristate Buffer".to_string(),
            ComponentKind::Transistor { .. } => "Transistor".to_string(),
            ComponentKind::TransmissionGate { .. } => "Transmission Gate".to_string(),
            ComponentKind::BitExtender { .. } => "Bit Extender".to_string(),
            ComponentKind::AndGate { .. } => "AND Gate".to_string(),
            ComponentKind::OrGate { .. } => "OR Gate".to_string(),
            ComponentKind::NandGate { .. } => "NAND Gate".to_string(),
            ComponentKind::NorGate { .. } => "NOR Gate".to_string(),
            ComponentKind::XorGate { .. } => "XOR Gate".to_string(),
            ComponentKind::XnorGate { .. } => "XNOR Gate".to_string(),
            ComponentKind::NotGate { .. } => "NOT Gate".to_string(),
            ComponentKind::Buffer { .. } => "Buffer".to_string(),
            ComponentKind::ControlledBuffer { .. } => "Controlled Buffer".to_string(),
            ComponentKind::OddParityGate { .. } => "Odd Parity".to_string(),
            ComponentKind::EvenParityGate { .. } => "Even Parity".to_string(),
            ComponentKind::Multiplexer { .. } => "Multiplexer".to_string(),
            ComponentKind::Demultiplexer { .. } => "Demultiplexer".to_string(),
            ComponentKind::Decoder { .. } => "Decoder".to_string(),
            ComponentKind::PriorityEncoder { .. } => "Priority Encoder".to_string(),
            ComponentKind::BitSelector { .. } => "Bit Selector".to_string(),
            ComponentKind::Adder { .. } => "Adder".to_string(),
            ComponentKind::Subtractor { .. } => "Subtractor".to_string(),
            ComponentKind::Multiplier { .. } => "Multiplier".to_string(),
            ComponentKind::Divider { .. } => "Divider".to_string(),
            ComponentKind::Negator { .. } => "Negator".to_string(),
            ComponentKind::Comparator { .. } => "Comparator".to_string(),
            ComponentKind::ShiftRegister { .. } => "Shift Register".to_string(),
            ComponentKind::BitAdder { .. } => "Bit Adder".to_string(),
            ComponentKind::BitFinder { .. } => "Bit Finder".to_string(),
            ComponentKind::DFlipFlop { .. } => "D Flip-Flop".to_string(),
            ComponentKind::TFlipFlop { .. } => "T Flip-Flop".to_string(),
            ComponentKind::JKFlipFlop { .. } => "JK Flip-Flop".to_string(),
            ComponentKind::SRFlipFlop { .. } => "SR Flip-Flop".to_string(),
            ComponentKind::Register { .. } => "Register".to_string(),
            ComponentKind::Ram { .. } => "RAM".to_string(),
            ComponentKind::Rom { .. } => "ROM".to_string(),
            ComponentKind::Counter { .. } => "Counter".to_string(),
            ComponentKind::ShiftRegisterMemory { .. } => "Shift Register".to_string(),
            ComponentKind::Led => "LED".to_string(),
            ComponentKind::RgbLed => "RGB LED".to_string(),
            ComponentKind::SevenSegDisplay => "7-Segment Display".to_string(),
            ComponentKind::HexDisplay => "Hex Digit Display".to_string(),
            ComponentKind::DotMatrix { .. } => "Dot Matrix Display".to_string(),
            ComponentKind::Button => "Button".to_string(),
            ComponentKind::DipSwitch { .. } => "DIP Switch".to_string(),
            ComponentKind::Keyboard => "Keyboard".to_string(),
            ComponentKind::Tty { .. } => "TTY".to_string(),
            ComponentKind::Subcircuit { circuit_name } => circuit_name.clone(),
            ComponentKind::Ttl7400 => "7400".to_string(),
            ComponentKind::Ttl7402 => "7402".to_string(),
            ComponentKind::Ttl7404 => "7404".to_string(),
            ComponentKind::Ttl7408 => "7408".to_string(),
            ComponentKind::Ttl7432 => "7432".to_string(),
            ComponentKind::Ttl7486 => "7486".to_string(),
        }
    }

    /// Build the list of ports for this component.
    pub fn ports(&self) -> Vec<Port> {
        match self {
            ComponentKind::Pin { is_output, width } => {
                if *is_output {
                    vec![Port::input("in", *width, (0, 0))]
                } else {
                    vec![Port::output("out", *width, (0, 0))]
                }
            }
            ComponentKind::Clock => vec![Port::output("out", BitWidth::ONE, (0, 0))],
            ComponentKind::Constant { width, .. } => {
                vec![Port::output("out", *width, (0, 0))]
            }
            ComponentKind::Power => vec![Port::output("out", BitWidth::ONE, (0, 0))],
            ComponentKind::Ground => vec![Port::output("out", BitWidth::ONE, (0, 0))],

            ComponentKind::Splitter {
                combined_width,
                fan_out,
            } => {
                let mut ports = vec![Port::input("combined", *combined_width, (0, 0))];
                let total_bits = combined_width.get();
                let fan_out_u32 = *fan_out as u32;
                let group_width = if fan_out_u32 != 0 && total_bits % fan_out_u32 == 0 {
                    BitWidth::new(total_bits / fan_out_u32)
                } else {
                    BitWidth::ONE
                };
                for i in 0..*fan_out {
                    ports.push(Port::output(
                        format!("bit{}", i),
                        group_width,
                        (0, i as i32 + 1),
                    ));
                }
                ports
            }

            ComponentKind::Tunnel { width, .. } => {
                vec![
                    Port::input("in", *width, (0, 0)),
                    Port::output("out", *width, (0, 0)),
                ]
            }

            ComponentKind::Probe { width } => {
                vec![Port::input("in", *width, (0, 0))]
            }

            ComponentKind::PullResistor { width, .. } => {
                vec![Port::output("out", *width, (0, 0))]
            }

            ComponentKind::TristateBuffer { width } => {
                vec![
                    Port::input("in", *width, (0, 0)),
                    Port::input("enable", BitWidth::ONE, (1, 0)),
                    Port::output("out", *width, (2, 0)),
                ]
            }

            ComponentKind::Transistor { width, .. } => {
                vec![
                    Port::input("gate", BitWidth::ONE, (0, 0)),
                    Port::input("source", *width, (0, 1)),
                    Port::output("drain", *width, (0, 2)),
                ]
            }

            ComponentKind::TransmissionGate { width } => {
                vec![
                    Port::input("gate", BitWidth::ONE, (0, 0)),
                    Port::input("gate_n", BitWidth::ONE, (0, 1)),
                    Port::input("source", *width, (0, 2)),
                    Port::output("drain", *width, (0, 3)),
                ]
            }

            ComponentKind::AndGate { inputs, width, .. }
            | ComponentKind::OrGate { inputs, width, .. } => {
                let mut ports: Vec<Port> = (0..*inputs)
                    .map(|i| Port::input(format!("in{}", i), *width, (0, i as i32)))
                    .collect();
                ports.push(Port::output("out", *width, (0, *inputs as i32)));
                ports
            }

            ComponentKind::NandGate { inputs, width }
            | ComponentKind::NorGate { inputs, width }
            | ComponentKind::XorGate { inputs, width }
            | ComponentKind::XnorGate { inputs, width } => {
                let mut ports: Vec<Port> = (0..*inputs)
                    .map(|i| Port::input(format!("in{}", i), *width, (0, i as i32)))
                    .collect();
                ports.push(Port::output("out", *width, (0, *inputs as i32)));
                ports
            }

            ComponentKind::NotGate { width } | ComponentKind::Buffer { width } => {
                vec![
                    Port::input("in", *width, (0, 0)),
                    Port::output("out", *width, (0, 1)),
                ]
            }

            ComponentKind::ControlledBuffer { width } => {
                vec![
                    Port::input("in", *width, (0, 0)),
                    Port::input("enable", BitWidth::ONE, (1, 0)),
                    Port::output("out", *width, (2, 0)),
                ]
            }

            ComponentKind::OddParityGate { inputs, width }
            | ComponentKind::EvenParityGate { inputs, width } => {
                let mut ports: Vec<Port> = (0..*inputs)
                    .map(|i| Port::input(format!("in{}", i), *width, (0, i as i32)))
                    .collect();
                ports.push(Port::output("out", BitWidth::ONE, (0, *inputs as i32)));
                ports
            }

            ComponentKind::BitExtender {
                input_width,
                output_width,
            } => {
                vec![
                    Port::input("in", *input_width, (0, 0)),
                    Port::output("out", *output_width, (0, 1)),
                ]
            }

            ComponentKind::Multiplexer {
                select_bits,
                data_width,
            } => {
                let n = 1u8 << select_bits;
                let mut ports: Vec<Port> = (0..n)
                    .map(|i| Port::input(format!("in{}", i), *data_width, (0, i as i32)))
                    .collect();
                ports.push(Port::input(
                    "sel",
                    BitWidth::new(*select_bits as u32),
                    (0, n as i32),
                ));
                ports.push(Port::output("out", *data_width, (0, n as i32 + 1)));
                ports
            }

            ComponentKind::Demultiplexer {
                select_bits,
                data_width,
            } => {
                let n = 1u8 << select_bits;
                let mut ports = vec![
                    Port::input("in", *data_width, (0, 0)),
                    Port::input("sel", BitWidth::new(*select_bits as u32), (0, 1)),
                ];
                for i in 0..n {
                    ports.push(Port::output(
                        format!("out{}", i),
                        *data_width,
                        (0, i as i32 + 2),
                    ));
                }
                ports
            }

            ComponentKind::Decoder { select_bits } => {
                let n = 1u8 << select_bits;
                let mut ports = vec![Port::input(
                    "sel",
                    BitWidth::new(*select_bits as u32),
                    (0, 0),
                )];
                for i in 0..n {
                    ports.push(Port::output(
                        format!("out{}", i),
                        BitWidth::ONE,
                        (0, i as i32 + 1),
                    ));
                }
                ports
            }

            ComponentKind::PriorityEncoder { select_bits } => {
                let n = 1u8 << select_bits;
                let mut ports: Vec<Port> = (0..n)
                    .map(|i| Port::input(format!("in{}", i), BitWidth::ONE, (0, i as i32)))
                    .collect();
                ports.push(Port::output(
                    "out",
                    BitWidth::new(*select_bits as u32),
                    (0, n as i32),
                ));
                ports.push(Port::output("en_out", BitWidth::ONE, (0, n as i32 + 1)));
                ports
            }

            ComponentKind::BitSelector {
                group_bits,
                data_width,
            } => {
                vec![
                    Port::input("in", *data_width, (0, 0)),
                    Port::input("sel", BitWidth::new(*group_bits as u32), (0, 1)),
                    Port::output("out", BitWidth::ONE, (0, 2)),
                ]
            }

            ComponentKind::Adder { width } => {
                vec![
                    Port::input("a", *width, (0, 0)),
                    Port::input("b", *width, (0, 1)),
                    Port::input("c_in", BitWidth::ONE, (0, 2)),
                    Port::output("sum", *width, (0, 3)),
                    Port::output("c_out", BitWidth::ONE, (0, 4)),
                ]
            }

            ComponentKind::Subtractor { width } => {
                vec![
                    Port::input("a", *width, (0, 0)),
                    Port::input("b", *width, (0, 1)),
                    Port::input("b_in", BitWidth::ONE, (0, 2)),
                    Port::output("out", *width, (0, 3)),
                    Port::output("b_out", BitWidth::ONE, (0, 4)),
                ]
            }

            ComponentKind::Multiplier { width } => {
                vec![
                    Port::input("a", *width, (0, 0)),
                    Port::input("b", *width, (0, 1)),
                    Port::input("c_in", *width, (0, 2)),
                    Port::output("out", *width, (0, 3)),
                    Port::output("upper", *width, (0, 4)),
                ]
            }

            ComponentKind::Divider { width } => {
                vec![
                    Port::input("a", *width, (0, 0)),
                    Port::input("b", *width, (0, 1)),
                    Port::input("upper", *width, (0, 2)),
                    Port::output("result", *width, (0, 3)),
                    Port::output("rem", *width, (0, 4)),
                ]
            }

            ComponentKind::Negator { width } => {
                vec![
                    Port::input("in", *width, (0, 0)),
                    Port::output("out", *width, (0, 1)),
                ]
            }

            ComponentKind::Comparator { width } => {
                vec![
                    Port::input("a", *width, (0, 0)),
                    Port::input("b", *width, (0, 1)),
                    Port::output("gt", BitWidth::ONE, (0, 2)),
                    Port::output("eq", BitWidth::ONE, (0, 3)),
                    Port::output("lt", BitWidth::ONE, (0, 4)),
                ]
            }

            ComponentKind::ShiftRegister { stages, width } => {
                let mut ports = vec![
                    Port::input("in", *width, (0, 0)),
                    Port::input("shift", BitWidth::ONE, (0, 1)),
                    Port::input("clk", BitWidth::ONE, (0, 2)),
                    Port::input("reset", BitWidth::ONE, (0, 3)),
                ];
                for i in 0..*stages {
                    ports.push(Port::output(format!("out{}", i), *width, (0, i as i32 + 4)));
                }
                ports
            }

            ComponentKind::BitAdder { width } => {
                vec![
                    Port::input("in", *width, (0, 0)),
                    Port::output("out", BitWidth::new((*width).get().max(1)), (0, 1)),
                ]
            }

            ComponentKind::BitFinder { width, .. } => {
                vec![
                    Port::input("in", *width, (0, 0)),
                    Port::output("out", BitWidth::new((*width).get().max(1)), (0, 1)),
                    Port::output("found", BitWidth::ONE, (0, 2)),
                ]
            }

            ComponentKind::DFlipFlop { width } => {
                vec![
                    Port::input("d", *width, (0, 0)),
                    Port::input("clk", BitWidth::ONE, (0, 1)),
                    Port::input("en", BitWidth::ONE, (0, 2)),
                    Port::input("reset", BitWidth::ONE, (0, 3)),
                    Port::input("preset", BitWidth::ONE, (0, 4)),
                    Port::output("q", *width, (0, 5)),
                    Port::output("q_n", *width, (0, 6)),
                ]
            }

            ComponentKind::TFlipFlop { width } => {
                vec![
                    Port::input("t", *width, (0, 0)),
                    Port::input("clk", BitWidth::ONE, (0, 1)),
                    Port::input("en", BitWidth::ONE, (0, 2)),
                    Port::input("reset", BitWidth::ONE, (0, 3)),
                    Port::input("preset", BitWidth::ONE, (0, 4)),
                    Port::output("q", *width, (0, 5)),
                    Port::output("q_n", *width, (0, 6)),
                ]
            }

            ComponentKind::JKFlipFlop { width } => {
                vec![
                    Port::input("j", *width, (0, 0)),
                    Port::input("k", *width, (0, 1)),
                    Port::input("clk", BitWidth::ONE, (0, 2)),
                    Port::input("en", BitWidth::ONE, (0, 3)),
                    Port::input("reset", BitWidth::ONE, (0, 4)),
                    Port::input("preset", BitWidth::ONE, (0, 5)),
                    Port::output("q", *width, (0, 6)),
                    Port::output("q_n", *width, (0, 7)),
                ]
            }

            ComponentKind::SRFlipFlop { width } => {
                vec![
                    Port::input("s", *width, (0, 0)),
                    Port::input("r", *width, (0, 1)),
                    Port::input("clk", BitWidth::ONE, (0, 2)),
                    Port::input("en", BitWidth::ONE, (0, 3)),
                    Port::input("reset", BitWidth::ONE, (0, 4)),
                    Port::input("preset", BitWidth::ONE, (0, 5)),
                    Port::output("q", *width, (0, 6)),
                    Port::output("q_n", *width, (0, 7)),
                ]
            }

            ComponentKind::Register { width } => {
                vec![
                    Port::input("d", *width, (0, 0)),
                    Port::input("clk", BitWidth::ONE, (0, 1)),
                    Port::input("en", BitWidth::ONE, (0, 2)),
                    Port::input("reset", BitWidth::ONE, (0, 3)),
                    Port::input("load", *width, (0, 4)),
                    Port::output("q", *width, (0, 5)),
                ]
            }

            ComponentKind::Ram {
                addr_bits,
                data_bits,
                sync,
            } => {
                let mut ports = vec![
                    Port::input("addr", BitWidth::new(*addr_bits as u32), (0, 0)),
                    Port::input("data_in", *data_bits, (0, 1)),
                    Port::input("we", BitWidth::ONE, (0, 2)),
                    Port::input("clk", BitWidth::ONE, (0, 3)),
                    Port::output("data_out", *data_bits, (0, 4)),
                ];
                if *sync {
                    ports.push(Port::input("oe", BitWidth::ONE, (0, 5)));
                }
                ports
            }

            ComponentKind::Rom {
                addr_bits,
                data_bits,
                ..
            } => {
                vec![
                    Port::input("addr", BitWidth::new(*addr_bits as u32), (0, 0)),
                    Port::output("data", *data_bits, (0, 1)),
                ]
            }

            ComponentKind::Counter { width } => {
                vec![
                    Port::input("clk", BitWidth::ONE, (0, 0)),
                    Port::input("en", BitWidth::ONE, (0, 1)),
                    Port::input("load", *width, (0, 2)),
                    Port::input("ld_en", BitWidth::ONE, (0, 3)),
                    Port::input("reset", BitWidth::ONE, (0, 4)),
                    Port::output("count", *width, (0, 5)),
                    Port::output("terminal", BitWidth::ONE, (0, 6)),
                ]
            }

            ComponentKind::ShiftRegisterMemory {
                stages,
                width,
                parallel_load,
            } => {
                let mut ports = vec![
                    Port::input("in", *width, (0, 0)),
                    Port::input("shift", BitWidth::ONE, (0, 1)),
                    Port::input("clk", BitWidth::ONE, (0, 2)),
                    Port::input("reset", BitWidth::ONE, (0, 3)),
                ];
                if *parallel_load {
                    for i in 0..*stages {
                        ports.push(Port::input(format!("load{}", i), *width, (0, i as i32 + 4)));
                    }
                }
                ports.push(Port::output("out", *width, (0, *stages as i32 + 4)));
                ports
            }

            ComponentKind::Led => {
                vec![Port::input("in", BitWidth::ONE, (0, 0))]
            }

            ComponentKind::RgbLed => {
                vec![
                    Port::input("r", BitWidth::ONE, (0, 0)),
                    Port::input("g", BitWidth::ONE, (0, 1)),
                    Port::input("b", BitWidth::ONE, (0, 2)),
                ]
            }

            ComponentKind::SevenSegDisplay => {
                // a-g segments plus decimal point
                let segs = ["a", "b", "c", "d", "e", "f", "g", "dp"];
                segs.iter()
                    .enumerate()
                    .map(|(i, &s)| Port::input(s, BitWidth::ONE, (0, i as i32)))
                    .collect()
            }

            ComponentKind::HexDisplay => {
                vec![Port::input("in", BitWidth::FOUR, (0, 0))]
            }

            ComponentKind::DotMatrix { rows, cols } => {
                let mut ports = Vec::new();
                for r in 0..*rows {
                    ports.push(Port::input(
                        format!("row{}", r),
                        BitWidth::ONE,
                        (0, r as i32),
                    ));
                }
                for c in 0..*cols {
                    ports.push(Port::input(
                        format!("col{}", c),
                        BitWidth::ONE,
                        (1, c as i32),
                    ));
                }
                ports
            }

            ComponentKind::Button => {
                vec![Port::output("out", BitWidth::ONE, (0, 0))]
            }

            ComponentKind::DipSwitch { switches } => (0..*switches)
                .map(|i| Port::output(format!("out{}", i), BitWidth::ONE, (0, i as i32)))
                .collect(),

            ComponentKind::Keyboard => {
                vec![
                    Port::input("clk", BitWidth::ONE, (0, 0)),
                    Port::input("clear", BitWidth::ONE, (0, 1)),
                    Port::output("data", BitWidth::EIGHT, (0, 2)),
                    Port::output("available", BitWidth::ONE, (0, 3)),
                ]
            }

            ComponentKind::Tty { .. } => {
                vec![
                    Port::input("data", BitWidth::EIGHT, (0, 0)),
                    Port::input("clk", BitWidth::ONE, (0, 1)),
                    Port::input("we", BitWidth::ONE, (0, 2)),
                    Port::input("clear", BitWidth::ONE, (0, 3)),
                ]
            }

            // Subcircuit ports are defined dynamically based on the referenced circuit
            ComponentKind::Subcircuit { .. } => vec![],

            // ── TTL 74xx ──────────────────────────────────────────────────────
            // Each TTL package exposes all individual gate I/O pins.
            // Layout: 4 × 2-input gates (A1, B1→Y1 ... A4, B4→Y4)
            ComponentKind::Ttl7400
            | ComponentKind::Ttl7408
            | ComponentKind::Ttl7432
            | ComponentKind::Ttl7486 => {
                let mut ports = Vec::new();
                for i in 1..=4u8 {
                    let y_offset = (i as i32 - 1) * 30;
                    ports.push(Port::input(format!("A{i}"), BitWidth::ONE, (0, y_offset)));
                    ports.push(Port::input(format!("B{i}"), BitWidth::ONE, (10, y_offset)));
                    ports.push(Port::output(format!("Y{i}"), BitWidth::ONE, (30, y_offset)));
                }
                ports
            }

            // 7402: quad 2-input NOR; inputs/outputs same layout
            ComponentKind::Ttl7402 => {
                let mut ports = Vec::new();
                for i in 1..=4u8 {
                    let y_offset = (i as i32 - 1) * 30;
                    ports.push(Port::input(format!("A{i}"), BitWidth::ONE, (0, y_offset)));
                    ports.push(Port::input(format!("B{i}"), BitWidth::ONE, (10, y_offset)));
                    ports.push(Port::output(format!("Y{i}"), BitWidth::ONE, (30, y_offset)));
                }
                ports
            }

            // 7404: hex inverter (6 × NOT), single input per gate
            ComponentKind::Ttl7404 => {
                let mut ports = Vec::new();
                for i in 1..=6u8 {
                    let y_offset = (i as i32 - 1) * 20;
                    ports.push(Port::input(format!("A{i}"), BitWidth::ONE, (0, y_offset)));
                    ports.push(Port::output(format!("Y{i}"), BitWidth::ONE, (20, y_offset)));
                }
                ports
            }
        }
    }
}

// ── Component ─────────────────────────────────────────────────────────────────

/// A placed component instance within a circuit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Component {
    /// Unique identifier within the circuit.
    pub id: ComponentId,
    /// The type of this component.
    pub kind: ComponentKind,
    /// Grid position (in Logisim grid units, 10px each).
    pub x: i32,
    pub y: i32,
    /// Human-readable label (optional).
    pub label: String,
    /// Rotation in degrees (0, 90, 180, 270).
    pub facing: Facing,
    /// Extra attributes from the XML.
    pub attributes: HashMap<String, String>,
}

/// Component orientation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum Facing {
    /// Facing east (default / 0°).
    #[default]
    East,
    /// Facing west (180°).
    West,
    /// Facing north (270°).
    North,
    /// Facing south (90°).
    South,
}

impl Facing {
    pub fn from_degrees(deg: i32) -> Self {
        match deg.rem_euclid(360) {
            90 => Facing::South,
            180 => Facing::West,
            270 => Facing::North,
            _ => Facing::East,
        }
    }

    pub fn to_degrees(self) -> i32 {
        match self {
            Facing::East => 0,
            Facing::South => 90,
            Facing::West => 180,
            Facing::North => 270,
        }
    }
}

impl fmt::Display for Facing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Facing::East => write!(f, "east"),
            Facing::West => write!(f, "west"),
            Facing::North => write!(f, "north"),
            Facing::South => write!(f, "south"),
        }
    }
}

impl Component {
    /// Create a new component with default attributes.
    pub fn new(id: ComponentId, kind: ComponentKind, x: i32, y: i32) -> Self {
        Component {
            id,
            kind,
            x,
            y,
            label: String::new(),
            facing: Facing::East,
            attributes: HashMap::new(),
        }
    }

    /// Return the absolute grid position of a named port.
    pub fn port_position(&self, port_name: &str) -> Option<(i32, i32)> {
        let ports = self.kind.ports();
        ports.iter().find(|p| p.name == port_name).map(|p| {
            let (dx, dy) = rotate_offset(p.offset, self.facing);
            (self.x + dx, self.y + dy)
        })
    }

    /// Get all port absolute positions.
    pub fn all_port_positions(&self) -> Vec<(String, (i32, i32))> {
        self.kind
            .ports()
            .into_iter()
            .map(|p| {
                let (dx, dy) = rotate_offset(p.offset, self.facing);
                (p.name, (self.x + dx, self.y + dy))
            })
            .collect()
    }
}

/// Rotate a port offset according to component facing.
fn rotate_offset((dx, dy): (i32, i32), facing: Facing) -> (i32, i32) {
    match facing {
        Facing::East => (dx, dy),
        Facing::South => (-dy, dx),
        Facing::West => (-dx, -dy),
        Facing::North => (dy, -dx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate_ports() {
        let kind = ComponentKind::AndGate {
            inputs: 2,
            width: BitWidth::ONE,
            negate_inputs: vec![false, false],
            negate_output: false,
        };
        let ports = kind.ports();
        assert_eq!(ports.len(), 3);
        assert_eq!(ports[0].name, "in0");
        assert_eq!(ports[0].direction, PortDirection::Input);
        assert_eq!(ports[2].name, "out");
        assert_eq!(ports[2].direction, PortDirection::Output);
    }

    #[test]
    fn test_mux_ports() {
        let kind = ComponentKind::Multiplexer {
            select_bits: 2,
            data_width: BitWidth::ONE,
        };
        let ports = kind.ports();
        // 4 data inputs + 1 select + 1 output = 6
        assert_eq!(ports.len(), 6);
    }

    #[test]
    fn test_dff_ports() {
        let kind = ComponentKind::DFlipFlop {
            width: BitWidth::ONE,
        };
        let ports = kind.ports();
        // d, clk, en, reset, preset, q, q_n = 7
        assert_eq!(ports.len(), 7);
        assert_eq!(ports[5].name, "q");
        assert_eq!(ports[6].name, "q_n");
    }

    #[test]
    fn test_component_port_position() {
        let kind = ComponentKind::Buffer {
            width: BitWidth::ONE,
        };
        let comp = Component::new(ComponentId(1), kind, 10, 20);
        let pos = comp.port_position("in").unwrap();
        assert_eq!(pos, (10, 20));
        let pos_out = comp.port_position("out").unwrap();
        assert_eq!(pos_out, (10, 21));
    }

    #[test]
    fn test_facing_rotation() {
        assert_eq!(Facing::from_degrees(0), Facing::East);
        assert_eq!(Facing::from_degrees(90), Facing::South);
        assert_eq!(Facing::from_degrees(180), Facing::West);
        assert_eq!(Facing::from_degrees(270), Facing::North);
        assert_eq!(Facing::from_degrees(360), Facing::East);
        assert_eq!(Facing::from_degrees(-90), Facing::North);
    }

    #[test]
    fn test_library_names() {
        assert_eq!(
            ComponentKind::AndGate {
                inputs: 2,
                width: BitWidth::ONE,
                negate_inputs: vec![],
                negate_output: false
            }
            .library_name(),
            "gates"
        );
        assert_eq!(
            ComponentKind::Ram {
                addr_bits: 8,
                data_bits: BitWidth::EIGHT,
                sync: false
            }
            .library_name(),
            "memory"
        );
        assert_eq!(ComponentKind::Led.library_name(), "io");
    }
}
