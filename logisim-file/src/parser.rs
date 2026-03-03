//! Parser for Logisim-Evolution `.circ` files.
//!
//! The parser reads the XML format used by Logisim-Evolution and constructs
//! a [`Project`] containing all circuits, components, and wires.

use crate::error::{FileError, Result};
use logisim_core::{
    circuit::Circuit,
    component::{BitFinderType, Component, ComponentId, ComponentKind, Facing, PullDirection},
    project::Project,
    value::BitWidth,
};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::collections::HashMap;
use std::io::BufRead;

// ── Public entry point ────────────────────────────────────────────────────────

/// Parse a Logisim-Evolution `.circ` file from a reader.
pub fn parse_circ<R: BufRead>(reader: R) -> Result<Project> {
    let mut xml = Reader::from_reader(reader);

    let mut project = Project::new("untitled");
    let mut lib_map: HashMap<String, String> = HashMap::new(); // name → desc
    let mut buf = Vec::new();

    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"project" => {
                        // nothing needed
                    }
                    b"lib" => {
                        let (lib_name, lib_desc) = parse_lib(e)?;
                        lib_map.insert(lib_name, lib_desc);
                    }
                    b"options" => {
                        let opts = parse_options(&mut xml)?;
                        project.options = opts;
                    }
                    b"circuit" => {
                        let circuit = parse_circuit(e, &mut xml, &lib_map)?;
                        project.add_circuit(circuit);
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                match e.name().as_ref() {
                    b"lib" => {
                        let (lib_name, lib_desc) = parse_lib(e)?;
                        lib_map.insert(lib_name, lib_desc);
                    }
                    b"main" => {
                        // <main name="circuitname"/> — the top-level circuit
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"name" {
                                let v = attr.unescape_value()?;
                                project.main_circuit = Some(v.into_owned());
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(FileError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    // Set the main circuit to the first one found.
    if project.main_circuit.is_none() {
        project.main_circuit = project.circuit_order.first().cloned();
    }

    Ok(project)
}

// ── Library element ───────────────────────────────────────────────────────────

fn parse_lib(e: &BytesStart) -> Result<(String, String)> {
    let mut name = String::new();
    let mut desc = String::new();
    for attr in e.attributes() {
        let attr = attr?;
        match attr.key.as_ref() {
            b"name" => name = attr_value(&attr)?,
            b"desc" => desc = attr_value(&attr)?,
            _ => {}
        }
    }
    Ok((name, desc))
}

// ── Options element ───────────────────────────────────────────────────────────

fn parse_options<R: BufRead>(xml: &mut Reader<R>) -> Result<HashMap<String, String>> {
    let mut opts = HashMap::new();
    let mut buf = Vec::new();
    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"a" => {
                let (k, v) = parse_attr_element(e)?;
                opts.insert(k, v);
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"options" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(FileError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(opts)
}

// ── Circuit element ───────────────────────────────────────────────────────────

fn parse_circuit<R: BufRead>(
    start: &BytesStart,
    xml: &mut Reader<R>,
    lib_map: &HashMap<String, String>,
) -> Result<Circuit> {
    let name = get_attr(start, b"name")?.unwrap_or_else(|| "unnamed".to_string());
    let mut circuit = Circuit::new(&name);

    let mut buf = Vec::new();

    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"comp" => {
                    let id = circuit.alloc_id();
                    let comp = parse_component(e, xml, lib_map, id.0)?;
                    circuit.components.insert(id, comp);
                }
                b"appear" => {
                    // Capture raw appearance XML for round-trip fidelity.
                    circuit.appearance_xml = Some(capture_element(xml, "appear")?);
                }
                b"a" => {
                    // circuit-level attribute (ignored as inner text element)
                }
                _ => {}
            },
            Ok(Event::Empty(ref e)) => match e.name().as_ref() {
                b"comp" => {
                    let id = circuit.alloc_id();
                    let comp = parse_component_empty(e, lib_map, id.0)?;
                    circuit.components.insert(id, comp);
                }
                b"wire" => {
                    let wire = parse_wire(e)?;
                    if let Some(w) = wire {
                        circuit.wires.push(w);
                    }
                }
                b"a" => {
                    let (k, v) = parse_attr_element(e)?;
                    circuit.attributes.insert(k, v);
                }
                _ => {}
            },
            Ok(Event::End(ref e)) if e.name().as_ref() == b"circuit" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(FileError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(circuit)
}

/// Consume all events until the matching end tag for `element_name`, returning
/// the captured inner XML as a string (not including the outer open/close tags).
///
/// `depth` starts at 1 because the outer opening tag (`<element_name>`) was already
/// consumed by the caller before invoking this function. The loop decrements depth on
/// every end tag and stops when depth reaches 0 at the matching end tag.
fn capture_element<R: BufRead>(xml: &mut Reader<R>, element_name: &str) -> Result<String> {
    let end_tag = element_name.as_bytes().to_vec();
    let mut buf = Vec::new();
    let mut captured = String::new();
    // depth=1 because the opening <element_name> tag was already consumed by the caller.
    let mut depth = 1usize;

    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                captured.push('<');
                captured.push_str(&tag);
                for attr in e.attributes() {
                    let attr = attr.map_err(FileError::Attr)?;
                    let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
                    let val = attr
                        .unescape_value()
                        .map(|v| v.into_owned())
                        .map_err(FileError::Xml)?;
                    captured.push(' ');
                    captured.push_str(&key);
                    captured.push_str("=\"");
                    captured.push_str(&xml_escape_attr(&val));
                    captured.push('"');
                }
                captured.push('>');
                depth += 1;
            }
            Ok(Event::End(ref e)) => {
                depth -= 1;
                if depth == 0 && e.name().as_ref() == end_tag.as_slice() {
                    break;
                }
                let tag = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                captured.push_str("</");
                captured.push_str(&tag);
                captured.push('>');
            }
            Ok(Event::Empty(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                captured.push('<');
                captured.push_str(&tag);
                for attr in e.attributes() {
                    let attr = attr.map_err(FileError::Attr)?;
                    let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
                    let val = attr
                        .unescape_value()
                        .map(|v| v.into_owned())
                        .map_err(FileError::Xml)?;
                    captured.push(' ');
                    captured.push_str(&key);
                    captured.push_str("=\"");
                    captured.push_str(&xml_escape_attr(&val));
                    captured.push('"');
                }
                captured.push_str("/>");
            }
            Ok(Event::Text(ref e)) => {
                let text = e
                    .unescape()
                    .map(|v| v.into_owned())
                    .map_err(FileError::Xml)?;
                // Re-escape text content so the round-trip stays valid XML.
                captured.push_str(&xml_escape_text(&text));
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(FileError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(captured)
}

/// Escape a string for use in an XML attribute value (double-quoted).
fn xml_escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Escape a string for use as XML text content.
fn xml_escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ── Component parsing ─────────────────────────────────────────────────────────

fn parse_component<R: BufRead>(
    start: &BytesStart,
    xml: &mut Reader<R>,
    lib_map: &HashMap<String, String>,
    id: u64,
) -> Result<Component> {
    let mut attrs: HashMap<String, String> = HashMap::new();
    // Read inner <a> elements.
    let mut buf = Vec::new();
    loop {
        match xml.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"a" => {
                let (k, v) = parse_attr_element(e)?;
                attrs.insert(k, v);
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"a" => {
                // skip
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"comp" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(FileError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }
    build_component(start, lib_map, id, attrs)
}

fn parse_component_empty(
    start: &BytesStart,
    lib_map: &HashMap<String, String>,
    id: u64,
) -> Result<Component> {
    build_component(start, lib_map, id, HashMap::new())
}

fn build_component(
    start: &BytesStart,
    lib_map: &HashMap<String, String>,
    id: u64,
    attrs: HashMap<String, String>,
) -> Result<Component> {
    let lib_num = get_attr(start, b"lib")?.unwrap_or_default();
    let name =
        get_attr(start, b"name")?.ok_or_else(|| FileError::MissingAttribute("name".to_string()))?;
    let loc =
        get_attr(start, b"loc")?.ok_or_else(|| FileError::MissingAttribute("loc".to_string()))?;

    let (x, y) = parse_loc(&loc)?;

    // Resolve library: prefer the desc from the <lib> header; fall back to the
    // raw numeric/string lib attribute so files with missing <lib> headers still
    // parse deterministically instead of yielding UnknownLibrary("").
    let lib_desc = lib_map
        .get(&lib_num)
        .map(|s| s.trim_start_matches('#').to_lowercase())
        .unwrap_or_else(|| lib_num.trim_start_matches('#').to_lowercase());

    let kind = build_kind(&lib_desc, &name, &attrs)?;
    let mut comp = Component::new(ComponentId(id), kind, x, y);

    // Apply common attributes — but skip `label` for Tunnel since it is already
    // embedded in the kind's own field to avoid duplication on write.
    let is_tunnel = matches!(
        comp.kind,
        logisim_core::component::ComponentKind::Tunnel { .. }
    );
    if !is_tunnel {
        if let Some(label) = attrs.get("label") {
            comp.label = label.clone();
        }
    }
    if let Some(facing) = attrs.get("facing") {
        comp.facing = parse_facing(facing);
    }
    comp.attributes = attrs;

    Ok(comp)
}

fn build_kind(lib: &str, name: &str, attrs: &HashMap<String, String>) -> Result<ComponentKind> {
    let get_width = |key: &str| -> BitWidth {
        attrs
            .get(key)
            .and_then(|v| v.parse::<u32>().ok())
            .map(|w| BitWidth::new(w.clamp(1, 64)))
            .unwrap_or(BitWidth::ONE)
    };
    let get_u8 = |key: &str, default: u8| -> u8 {
        attrs
            .get(key)
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(default)
    };
    let get_u64 = |key: &str| -> u64 { attrs.get(key).and_then(|v| parse_integer(v)).unwrap_or(0) };

    match lib {
        "wiring" | "0" => match name {
            "Pin" => {
                let output = attrs.get("output").map(|v| v == "true").unwrap_or(false);
                Ok(ComponentKind::Pin {
                    is_output: output,
                    width: get_width("width"),
                })
            }
            "Clock" => Ok(ComponentKind::Clock),
            "Constant" => Ok(ComponentKind::Constant {
                width: get_width("width"),
                value: get_u64("value"),
            }),
            "Power" => Ok(ComponentKind::Power),
            "Ground" => Ok(ComponentKind::Ground),
            "Splitter" => Ok(ComponentKind::Splitter {
                combined_width: get_width("incoming"),
                fan_out: get_u8("fanout", 2),
            }),
            "Tunnel" => Ok(ComponentKind::Tunnel {
                label: attrs.get("label").cloned().unwrap_or_default(),
                width: get_width("width"),
            }),
            "Probe" => Ok(ComponentKind::Probe {
                width: get_width("width"),
            }),
            "Pull Resistor" => {
                let dir = match attrs.get("pull").map(|s| s.as_str()).unwrap_or("up") {
                    "down" => PullDirection::Down,
                    _ => PullDirection::Up,
                };
                Ok(ComponentKind::PullResistor {
                    direction: dir,
                    width: get_width("width"),
                })
            }
            "Tristate Buffer" => Ok(ComponentKind::TristateBuffer {
                width: get_width("width"),
            }),
            "Transistor" => Ok(ComponentKind::Transistor {
                width: get_width("width"),
                p_type: false,
            }),
            "Transistor P" => Ok(ComponentKind::Transistor {
                width: get_width("width"),
                p_type: true,
            }),
            "Transmission Gate" => Ok(ComponentKind::TransmissionGate {
                width: get_width("width"),
            }),
            "Bit Extender" => Ok(ComponentKind::BitExtender {
                input_width: get_width("in_width"),
                output_width: get_width("out_width"),
            }),
            _ => Err(FileError::UnknownComponent {
                lib: lib.to_string(),
                name: name.to_string(),
            }),
        },

        "gates" | "1" => match name {
            "AND Gate" => Ok(ComponentKind::AndGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
                negate_inputs: vec![],
                negate_output: false,
            }),
            "OR Gate" => Ok(ComponentKind::OrGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
                negate_inputs: vec![],
                negate_output: false,
            }),
            "NAND Gate" => Ok(ComponentKind::NandGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
            }),
            "NOR Gate" => Ok(ComponentKind::NorGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
            }),
            "XOR Gate" => Ok(ComponentKind::XorGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
            }),
            "XNOR Gate" => Ok(ComponentKind::XnorGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
            }),
            "NOT Gate" => Ok(ComponentKind::NotGate {
                width: get_width("width"),
            }),
            "Buffer" => Ok(ComponentKind::Buffer {
                width: get_width("width"),
            }),
            "Controlled Buffer" => Ok(ComponentKind::ControlledBuffer {
                width: get_width("width"),
            }),
            "Odd Parity" => Ok(ComponentKind::OddParityGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
            }),
            "Even Parity" => Ok(ComponentKind::EvenParityGate {
                inputs: get_u8("inputs", 2),
                width: get_width("width"),
            }),
            _ => Err(FileError::UnknownComponent {
                lib: lib.to_string(),
                name: name.to_string(),
            }),
        },

        "plexers" | "2" => match name {
            "Multiplexer" => Ok(ComponentKind::Multiplexer {
                select_bits: get_u8("select", 1),
                data_width: get_width("width"),
            }),
            "Demultiplexer" => Ok(ComponentKind::Demultiplexer {
                select_bits: get_u8("select", 1),
                data_width: get_width("width"),
            }),
            "Decoder" => Ok(ComponentKind::Decoder {
                select_bits: get_u8("select", 2),
            }),
            "Priority Encoder" => Ok(ComponentKind::PriorityEncoder {
                select_bits: get_u8("select", 2),
            }),
            "Bit Selector" => Ok(ComponentKind::BitSelector {
                group_bits: get_u8("group", 1),
                data_width: get_width("width"),
            }),
            _ => Err(FileError::UnknownComponent {
                lib: lib.to_string(),
                name: name.to_string(),
            }),
        },

        "arithmetic" | "3" => match name {
            "Adder" => Ok(ComponentKind::Adder {
                width: get_width("width"),
            }),
            "Subtractor" => Ok(ComponentKind::Subtractor {
                width: get_width("width"),
            }),
            "Multiplier" => Ok(ComponentKind::Multiplier {
                width: get_width("width"),
            }),
            "Divider" => Ok(ComponentKind::Divider {
                width: get_width("width"),
            }),
            "Negator" => Ok(ComponentKind::Negator {
                width: get_width("width"),
            }),
            "Comparator" => Ok(ComponentKind::Comparator {
                width: get_width("width"),
            }),
            "Shift Register" => Ok(ComponentKind::ShiftRegister {
                stages: get_u8("stages", 8),
                width: get_width("width"),
            }),
            "Bit Adder" => Ok(ComponentKind::BitAdder {
                width: get_width("width"),
            }),
            "Bit Finder" => Ok(ComponentKind::BitFinder {
                width: get_width("width"),
                find_type: match attrs.get("type").map(|s| s.as_str()).unwrap_or("high") {
                    "low" => BitFinderType::Low,
                    _ => BitFinderType::High,
                },
            }),
            _ => Err(FileError::UnknownComponent {
                lib: lib.to_string(),
                name: name.to_string(),
            }),
        },

        "memory" | "4" => match name {
            "D Flip-Flop" => Ok(ComponentKind::DFlipFlop {
                width: get_width("width"),
            }),
            "T Flip-Flop" => Ok(ComponentKind::TFlipFlop {
                width: get_width("width"),
            }),
            "JK Flip-Flop" => Ok(ComponentKind::JKFlipFlop {
                width: get_width("width"),
            }),
            "SR Flip-Flop" | "RS Flip-Flop" => Ok(ComponentKind::SRFlipFlop {
                width: get_width("width"),
            }),
            "Register" => Ok(ComponentKind::Register {
                width: get_width("width"),
            }),
            "RAM" => Ok(ComponentKind::Ram {
                addr_bits: get_u8("addrWidth", 8),
                data_bits: get_width("dataWidth"),
                sync: attrs.get("trigger").map(|v| v == "rising").unwrap_or(false),
            }),
            "ROM" => Ok(ComponentKind::Rom {
                addr_bits: get_u8("addrWidth", 8),
                data_bits: get_width("dataWidth"),
                contents: parse_rom_contents(
                    attrs.get("contents").map(|s| s.as_str()).unwrap_or(""),
                ),
            }),
            "Counter" => Ok(ComponentKind::Counter {
                width: get_width("width"),
            }),
            "Shift Register" => Ok(ComponentKind::ShiftRegisterMemory {
                stages: get_u8("length", 8),
                width: get_width("width"),
                parallel_load: attrs.get("load").map(|v| v == "true").unwrap_or(false),
            }),
            _ => Err(FileError::UnknownComponent {
                lib: lib.to_string(),
                name: name.to_string(),
            }),
        },

        "io" | "5" => match name {
            "LED" => Ok(ComponentKind::Led),
            "RGB LED" => Ok(ComponentKind::RgbLed),
            "7-Segment Display" => Ok(ComponentKind::SevenSegDisplay),
            "Hex Digit Display" => Ok(ComponentKind::HexDisplay),
            "Dot Matrix Display" => Ok(ComponentKind::DotMatrix {
                rows: get_u8("rows", 5),
                cols: get_u8("cols", 5),
            }),
            "Button" => Ok(ComponentKind::Button),
            "DIP Switch" => Ok(ComponentKind::DipSwitch {
                switches: get_u8("switches", 8),
            }),
            "Keyboard" => Ok(ComponentKind::Keyboard),
            "TTY" => Ok(ComponentKind::Tty {
                rows: get_u8("rows", 8),
                cols: get_u8("cols", 32),
            }),
            _ => Err(FileError::UnknownComponent {
                lib: lib.to_string(),
                name: name.to_string(),
            }),
        },

        // TTL 74xx library (name "ttl"/"#ttl" or numeric fallback "6")
        "ttl" | "#ttl" | "6" => match name {
            "7400" => Ok(ComponentKind::Ttl7400),
            "7402" => Ok(ComponentKind::Ttl7402),
            "7404" => Ok(ComponentKind::Ttl7404),
            "7408" => Ok(ComponentKind::Ttl7408),
            "7432" => Ok(ComponentKind::Ttl7432),
            "7486" => Ok(ComponentKind::Ttl7486),
            _ => Err(FileError::UnknownComponent {
                lib: lib.to_string(),
                name: name.to_string(),
            }),
        },

        // User-defined subcircuits (explicitly tagged with the user lib)
        "user" | "7" => Ok(ComponentKind::Subcircuit {
            circuit_name: name.to_string(),
        }),

        // Unknown library — return an error rather than silently mis-typing
        unknown => Err(FileError::UnknownLibrary(unknown.to_string())),
    }
}

// ── Wire parsing ──────────────────────────────────────────────────────────────

fn parse_wire(e: &BytesStart) -> Result<Option<logisim_core::circuit::Wire>> {
    let from = get_attr(e, b"from")?;
    let to = get_attr(e, b"to")?;
    match (from, to) {
        (Some(f), Some(t)) => {
            let (x1, y1) = parse_loc(&f)?;
            let (x2, y2) = parse_loc(&t)?;
            Ok(Some(logisim_core::circuit::Wire::new(x1, y1, x2, y2)))
        }
        _ => Ok(None),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parse a Logisim location string like `(160,130)` into grid units `(x, y)`.
/// Logisim `.circ` files store coordinates in pixel units (multiples of 10);
/// the GUI uses grid units where 1 unit = `BASE_GRID_PX` screen pixels.
fn parse_loc(s: &str) -> Result<(i32, i32)> {
    let s = s.trim().trim_start_matches('(').trim_end_matches(')');
    let mut parts = s.splitn(2, ',');
    let x = parts
        .next()
        .ok_or_else(|| FileError::InvalidCoord(s.to_string()))?
        .trim()
        .parse::<i32>()
        .map_err(|_| FileError::InvalidCoord(s.to_string()))?;
    let y = parts
        .next()
        .ok_or_else(|| FileError::InvalidCoord(s.to_string()))?
        .trim()
        .parse::<i32>()
        .map_err(|_| FileError::InvalidCoord(s.to_string()))?;
    // Convert pixel units → grid units (Logisim grid spacing is 10 px).
    Ok((x / 10, y / 10))
}

/// Parse a Logisim facing string.
fn parse_facing(s: &str) -> Facing {
    match s.to_lowercase().as_str() {
        "west" => Facing::West,
        "north" => Facing::North,
        "south" => Facing::South,
        _ => Facing::East,
    }
}

/// Parse the ROM contents string (space/newline separated hex values).
fn parse_rom_contents(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .filter_map(|tok| u64::from_str_radix(tok, 16).ok())
        .collect()
}

/// Parse a Logisim integer (may be hex like `0x1A` or decimal).
fn parse_integer(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u64>().ok()
    }
}

/// Get a UTF-8 string value from an XML attribute, unescaping XML entities
/// (e.g. `&amp;` → `&`, `&quot;` → `"`) so round-trips are lossless.
fn attr_value(attr: &quick_xml::events::attributes::Attribute) -> Result<String> {
    let unescaped = attr.unescape_value()?;
    Ok(unescaped.into_owned())
}

/// Get a named attribute from an XML element.
fn get_attr(e: &BytesStart, key: &[u8]) -> Result<Option<String>> {
    for attr in e.attributes() {
        let attr = attr?;
        if attr.key.as_ref() == key {
            return Ok(Some(attr_value(&attr)?));
        }
    }
    Ok(None)
}

/// Parse an `<a name="..." val="..."/>` element.
fn parse_attr_element(e: &BytesStart) -> Result<(String, String)> {
    let name =
        get_attr(e, b"name")?.ok_or_else(|| FileError::MissingAttribute("name".to_string()))?;
    let val = get_attr(e, b"val")?.ok_or_else(|| FileError::MissingAttribute("val".to_string()))?;
    Ok((name, val))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_CIRC: &str = r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<project version="1.0">
  <lib desc="#Wiring" name="0"/>
  <lib desc="#Gates" name="1"/>
  <lib desc="#Plexers" name="2"/>
  <lib desc="#Arithmetic" name="3"/>
  <lib desc="#Memory" name="4"/>
  <lib desc="#I/O" name="5"/>
  <options>
    <a name="gateUndefined" val="isolated"/>
  </options>
  <mappings/>
  <toolbar/>
  <circuit name="main">
    <a name="circuit" val="main"/>
    <comp lib="0" loc="(30,140)" name="Pin">
      <a name="facing" val="east"/>
    </comp>
    <comp lib="0" loc="(30,160)" name="Pin">
      <a name="facing" val="east"/>
    </comp>
    <comp lib="1" loc="(160,140)" name="AND Gate">
      <a name="inputs" val="2"/>
    </comp>
    <comp lib="0" loc="(290,140)" name="Pin">
      <a name="output" val="true"/>
      <a name="facing" val="west"/>
    </comp>
    <wire from="(30,140)" to="(160,140)"/>
    <wire from="(30,160)" to="(160,141)"/>
    <wire from="(160,142)" to="(290,140)"/>
  </circuit>
</project>"##;

    #[test]
    fn test_parse_simple_circ() {
        let project = parse_circ(SIMPLE_CIRC.as_bytes()).unwrap();
        assert!(project.circuits.contains_key("main"));
        let circuit = &project.circuits["main"];
        assert_eq!(circuit.components.len(), 4);
        assert_eq!(circuit.wires.len(), 3);
    }

    #[test]
    fn test_parse_options() {
        let project = parse_circ(SIMPLE_CIRC.as_bytes()).unwrap();
        assert_eq!(
            project.options.get("gateUndefined"),
            Some(&"isolated".to_string())
        );
    }

    #[test]
    fn test_parse_pins() {
        let project = parse_circ(SIMPLE_CIRC.as_bytes()).unwrap();
        let circuit = &project.circuits["main"];
        let inputs = circuit.input_pins();
        let outputs = circuit.output_pins();
        assert_eq!(inputs.len(), 2);
        assert_eq!(outputs.len(), 1);
    }

    #[test]
    fn test_parse_and_gate() {
        let project = parse_circ(SIMPLE_CIRC.as_bytes()).unwrap();
        let circuit = &project.circuits["main"];
        let gate = circuit
            .components
            .values()
            .find(|c| matches!(c.kind, ComponentKind::AndGate { .. }));
        assert!(gate.is_some());
    }

    #[test]
    fn test_parse_loc() {
        // parse_loc converts from pixel units (as stored in .circ) to grid units (÷10).
        assert_eq!(parse_loc("(160,130)").unwrap(), (16, 13));
        assert_eq!(parse_loc("(0,0)").unwrap(), (0, 0));
        assert_eq!(parse_loc("(-10,200)").unwrap(), (-1, 20));
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_integer("42"), Some(42));
        assert_eq!(parse_integer("0xFF"), Some(255));
        assert_eq!(parse_integer("0x1A"), Some(26));
    }

    #[test]
    fn test_parse_rom_contents() {
        let contents = parse_rom_contents("0A FF 1B 00");
        assert_eq!(contents, vec![0x0A, 0xFF, 0x1B, 0x00]);
    }

    const MULTI_CIRCUIT: &str = r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<project version="1.0">
  <lib desc="#Wiring" name="0"/>
  <lib desc="#Gates" name="1"/>
  <circuit name="main">
    <comp lib="0" loc="(30,140)" name="Pin"/>
  </circuit>
  <circuit name="sub">
    <comp lib="1" loc="(50,50)" name="NOT Gate"/>
  </circuit>
</project>"##;

    #[test]
    fn test_parse_multi_circuit() {
        let project = parse_circ(MULTI_CIRCUIT.as_bytes()).unwrap();
        assert_eq!(project.circuits.len(), 2);
        assert!(project.circuits.contains_key("main"));
        assert!(project.circuits.contains_key("sub"));
        assert_eq!(project.main_circuit_name(), Some("main"));
    }

    const MEMORY_CIRC: &str = r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<project version="1.0">
  <lib desc="#Memory" name="4"/>
  <circuit name="mem">
    <comp lib="4" loc="(100,100)" name="D Flip-Flop">
      <a name="width" val="1"/>
    </comp>
    <comp lib="4" loc="(200,100)" name="Register">
      <a name="width" val="8"/>
    </comp>
    <comp lib="4" loc="(300,100)" name="Counter">
      <a name="width" val="4"/>
    </comp>
  </circuit>
</project>"##;

    #[test]
    fn test_parse_memory_components() {
        let project = parse_circ(MEMORY_CIRC.as_bytes()).unwrap();
        let circuit = &project.circuits["mem"];
        assert!(circuit
            .components
            .values()
            .any(|c| matches!(c.kind, ComponentKind::DFlipFlop { .. })));
        assert!(circuit
            .components
            .values()
            .any(|c| matches!(c.kind, ComponentKind::Register { .. })));
        assert!(circuit
            .components
            .values()
            .any(|c| matches!(c.kind, ComponentKind::Counter { .. })));
    }
}
