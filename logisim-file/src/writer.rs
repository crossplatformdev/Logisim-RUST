//! Writer for Logisim-Evolution `.circ` files.
//!
//! Serialises a [`Project`] into the XML format used by Logisim-Evolution,
//! producing output that can be re-read by both this implementation and the
//! original Java application.

use crate::error::Result;
use logisim_core::{
    circuit::{Circuit, Wire},
    component::{ComponentKind, Facing, PullDirection, BitFinderType},
    project::Project,
};
use std::io::Write;

// ── Public entry point ────────────────────────────────────────────────────────

/// Write a project to a writer in `.circ` XML format.
pub fn write_circ<W: Write>(project: &Project, writer: &mut W) -> Result<()> {
    writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
    writeln!(writer, r#"<project version="1.0">"#)?;

    // Standard library declarations (matching Logisim-Evolution order).
    writeln!(writer, "  <lib desc=\"#Wiring\" name=\"0\"/>")?;
    writeln!(writer, "  <lib desc=\"#Gates\" name=\"1\"/>")?;
    writeln!(writer, "  <lib desc=\"#Plexers\" name=\"2\"/>")?;
    writeln!(writer, "  <lib desc=\"#Arithmetic\" name=\"3\"/>")?;
    writeln!(writer, "  <lib desc=\"#Memory\" name=\"4\"/>")?;
    writeln!(writer, "  <lib desc=\"#I/O\" name=\"5\"/>")?;

    // Options
    writeln!(writer, "  <options>")?;
    for (k, v) in &project.options {
        writeln!(writer, r#"    <a name="{}" val="{}"/>"#, escape(k), escape(v))?;
    }
    writeln!(writer, "  </options>")?;
    writeln!(writer, "  <mappings/>")?;
    writeln!(writer, "  <toolbar/>")?;

    // Circuits (in defined order)
    for circuit in project.ordered_circuits() {
        write_circuit(circuit, writer)?;
    }

    writeln!(writer, "</project>")?;
    Ok(())
}

// ── Circuit ───────────────────────────────────────────────────────────────────

fn write_circuit<W: Write>(circuit: &Circuit, writer: &mut W) -> Result<()> {
    writeln!(writer, r#"  <circuit name="{}">"#, escape(&circuit.name))?;
    writeln!(writer, r#"    <a name="circuit" val="{}"/>"#, escape(&circuit.name))?;

    // Circuit attributes
    for (k, v) in &circuit.attributes {
        if k != "circuit" {
            writeln!(writer, r#"    <a name="{}" val="{}"/>"#, escape(k), escape(v))?;
        }
    }

    // Components
    let mut comp_ids: Vec<_> = circuit.components.keys().copied().collect();
    comp_ids.sort();
    for id in comp_ids {
        let comp = &circuit.components[&id];
        write_component(comp, writer)?;
    }

    // Wires
    for wire in &circuit.wires {
        write_wire(wire, writer)?;
    }

    writeln!(writer, "  </circuit>")?;
    Ok(())
}

// ── Component ─────────────────────────────────────────────────────────────────

fn write_component<W: Write>(
    comp: &logisim_core::component::Component,
    writer: &mut W,
) -> Result<()> {
    let lib = lib_number(&comp.kind);
    let name = comp.kind.component_name();
    let loc = format!("({},{})", comp.x, comp.y);

    writeln!(
        writer,
        r#"    <comp lib="{}" loc="{}" name="{}">"#,
        lib,
        loc,
        escape(&name)
    )?;

    // Facing
    if comp.facing != Facing::East {
        writeln!(
            writer,
            r#"      <a name="facing" val="{}"/>"#,
            comp.facing
        )?;
    }

    // Label — skip for Tunnel since write_kind_attrs emits it as a kind-specific attribute.
    if !comp.label.is_empty() && !matches!(comp.kind, ComponentKind::Tunnel { .. }) {
        writeln!(
            writer,
            r#"      <a name="label" val="{}"/>"#,
            escape(&comp.label)
        )?;
    }

    // Kind-specific attributes
    write_kind_attrs(&comp.kind, writer)?;

    writeln!(writer, "    </comp>")?;
    Ok(())
}

fn lib_number(kind: &ComponentKind) -> &'static str {
    match kind.library_name() {
        "wiring" => "0",
        "gates" => "1",
        "plexers" => "2",
        "arithmetic" => "3",
        "memory" => "4",
        "io" => "5",
        _ => "0",
    }
}

fn write_kind_attrs<W: Write>(kind: &ComponentKind, writer: &mut W) -> Result<()> {
    match kind {
        ComponentKind::Pin { is_output, width } => {
            if *is_output {
                writeln!(writer, r#"      <a name="output" val="true"/>"#)?;
            }
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }
        ComponentKind::Constant { width, value } => {
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
            writeln!(writer, r#"      <a name="value" val="0x{:X}"/>"#, value)?;
        }
        ComponentKind::Splitter { combined_width, fan_out } => {
            writeln!(writer, r#"      <a name="incoming" val="{}"/>"#, combined_width.get())?;
            writeln!(writer, r#"      <a name="fanout" val="{}"/>"#, fan_out)?;
        }
        ComponentKind::Tunnel { label, width } => {
            writeln!(writer, r#"      <a name="label" val="{}"/>"#, escape(label))?;
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }
        ComponentKind::Probe { width } => {
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }
        ComponentKind::PullResistor { direction, width } => {
            let pull = match direction {
                PullDirection::Up => "up",
                PullDirection::Down => "down",
            };
            writeln!(writer, r#"      <a name="pull" val="{}"/>"#, pull)?;
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }

        ComponentKind::AndGate { inputs, width, .. }
        | ComponentKind::OrGate { inputs, width, .. } => {
            if *inputs != 2 {
                writeln!(writer, r#"      <a name="inputs" val="{}"/>"#, inputs)?;
            }
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }
        ComponentKind::NandGate { inputs, width }
        | ComponentKind::NorGate { inputs, width }
        | ComponentKind::XorGate { inputs, width }
        | ComponentKind::XnorGate { inputs, width } => {
            if *inputs != 2 {
                writeln!(writer, r#"      <a name="inputs" val="{}"/>"#, inputs)?;
            }
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }
        ComponentKind::NotGate { width }
        | ComponentKind::Buffer { width }
        | ComponentKind::ControlledBuffer { width }
        | ComponentKind::TristateBuffer { width } => {
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }

        ComponentKind::Multiplexer { select_bits, data_width }
        | ComponentKind::Demultiplexer { select_bits, data_width } => {
            writeln!(writer, r#"      <a name="select" val="{}"/>"#, select_bits)?;
            if data_width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, data_width.get())?;
            }
        }
        ComponentKind::Decoder { select_bits }
        | ComponentKind::PriorityEncoder { select_bits } => {
            writeln!(writer, r#"      <a name="select" val="{}"/>"#, select_bits)?;
        }
        ComponentKind::BitSelector { group_bits, data_width } => {
            writeln!(writer, r#"      <a name="group" val="{}"/>"#, group_bits)?;
            if data_width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, data_width.get())?;
            }
        }

        ComponentKind::Adder { width }
        | ComponentKind::Subtractor { width }
        | ComponentKind::Multiplier { width }
        | ComponentKind::Divider { width }
        | ComponentKind::Negator { width }
        | ComponentKind::Comparator { width }
        | ComponentKind::BitAdder { width } => {
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }
        ComponentKind::BitFinder { width, find_type } => {
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
            let ft = match find_type {
                BitFinderType::High => "high",
                BitFinderType::Low => "low",
            };
            writeln!(writer, r#"      <a name="type" val="{}"/>"#, ft)?;
        }
        ComponentKind::ShiftRegister { stages, width } => {
            writeln!(writer, r#"      <a name="stages" val="{}"/>"#, stages)?;
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }

        ComponentKind::DFlipFlop { width }
        | ComponentKind::TFlipFlop { width }
        | ComponentKind::JKFlipFlop { width }
        | ComponentKind::SRFlipFlop { width }
        | ComponentKind::Register { width }
        | ComponentKind::Counter { width } => {
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
        }

        ComponentKind::Ram { addr_bits, data_bits, sync } => {
            writeln!(writer, r#"      <a name="addrWidth" val="{}"/>"#, addr_bits)?;
            writeln!(writer, r#"      <a name="dataWidth" val="{}"/>"#, data_bits.get())?;
            if *sync {
                writeln!(writer, r#"      <a name="trigger" val="rising"/>"#)?;
            }
        }

        ComponentKind::Rom { addr_bits, data_bits, contents } => {
            writeln!(writer, r#"      <a name="addrWidth" val="{}"/>"#, addr_bits)?;
            writeln!(writer, r#"      <a name="dataWidth" val="{}"/>"#, data_bits.get())?;
            if !contents.is_empty() {
                let hex: Vec<String> = contents.iter().map(|v| format!("{:02X}", v)).collect();
                writeln!(writer, r#"      <a name="contents" val="{}"/>"#, hex.join(" "))?;
            }
        }

        ComponentKind::ShiftRegisterMemory { stages, width, parallel_load } => {
            writeln!(writer, r#"      <a name="length" val="{}"/>"#, stages)?;
            if width.get() != 1 {
                writeln!(writer, r#"      <a name="width" val="{}"/>"#, width.get())?;
            }
            if *parallel_load {
                writeln!(writer, r#"      <a name="load" val="true"/>"#)?;
            }
        }

        ComponentKind::DotMatrix { rows, cols } => {
            writeln!(writer, r#"      <a name="rows" val="{}"/>"#, rows)?;
            writeln!(writer, r#"      <a name="cols" val="{}"/>"#, cols)?;
        }

        ComponentKind::DipSwitch { switches } => {
            writeln!(writer, r#"      <a name="switches" val="{}"/>"#, switches)?;
        }

        ComponentKind::Tty { rows, cols } => {
            writeln!(writer, r#"      <a name="rows" val="{}"/>"#, rows)?;
            writeln!(writer, r#"      <a name="cols" val="{}"/>"#, cols)?;
        }

        // Components with no extra attributes
        ComponentKind::Clock
        | ComponentKind::Power
        | ComponentKind::Ground
        | ComponentKind::Led
        | ComponentKind::RgbLed
        | ComponentKind::SevenSegDisplay
        | ComponentKind::HexDisplay
        | ComponentKind::Button
        | ComponentKind::Keyboard
        | ComponentKind::Subcircuit { .. } => {}
    }
    Ok(())
}

// ── Wire ──────────────────────────────────────────────────────────────────────

fn write_wire<W: Write>(wire: &Wire, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"    <wire from="({},{})" to="({},{})"/>"#,
        wire.from.x, wire.from.y, wire.to.x, wire.to.y
    )?;
    Ok(())
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Escape XML special characters.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_circ;
    use logisim_core::{
        circuit::Circuit,
        component::ComponentKind,
        project::Project,
        value::BitWidth,
    };

    fn make_test_project() -> Project {
        let mut project = Project::new("test");
        let mut circuit = Circuit::new("main");
        circuit.add_component(
            ComponentKind::Pin { is_output: false, width: BitWidth::ONE },
            30,
            140,
        );
        circuit.add_component(
            ComponentKind::AndGate {
                inputs: 2,
                width: BitWidth::ONE,
                negate_inputs: vec![],
                negate_output: false,
            },
            160,
            140,
        );
        circuit.add_component(
            ComponentKind::Pin { is_output: true, width: BitWidth::ONE },
            290,
            140,
        );
        circuit.add_wire(30, 140, 160, 140);
        circuit.add_wire(160, 142, 290, 140);
        project.add_circuit(circuit);
        project
    }

    #[test]
    fn test_write_roundtrip() {
        let project = make_test_project();
        let mut buf = Vec::new();
        write_circ(&project, &mut buf).unwrap();
        let xml = String::from_utf8(buf).unwrap();

        // Re-parse
        let project2 = parse_circ(xml.as_bytes()).unwrap();
        assert!(project2.circuits.contains_key("main"));
        let circuit = &project2.circuits["main"];
        assert_eq!(circuit.components.len(), 3);
        assert_eq!(circuit.wires.len(), 2);
    }

    #[test]
    fn test_write_contains_lib_decls() {
        let project = make_test_project();
        let mut buf = Vec::new();
        write_circ(&project, &mut buf).unwrap();
        let xml = String::from_utf8(buf).unwrap();
        assert!(xml.contains("desc=\"#Wiring\""));
        assert!(xml.contains("desc=\"#Gates\""));
    }

    #[test]
    fn test_write_circuit_element() {
        let project = make_test_project();
        let mut buf = Vec::new();
        write_circ(&project, &mut buf).unwrap();
        let xml = String::from_utf8(buf).unwrap();
        assert!(xml.contains(r#"<circuit name="main">"#));
        assert!(xml.contains(r#"name="AND Gate""#));
    }

    #[test]
    fn test_write_wire() {
        let project = make_test_project();
        let mut buf = Vec::new();
        write_circ(&project, &mut buf).unwrap();
        let xml = String::from_utf8(buf).unwrap();
        assert!(xml.contains(r#"<wire from="(30,140)" to="(160,140)"/>"#));
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape("a&b"), "a&amp;b");
        assert_eq!(escape(r#"a"b"#), "a&quot;b");
        assert_eq!(escape("a<b>c"), "a&lt;b&gt;c");
    }
}
