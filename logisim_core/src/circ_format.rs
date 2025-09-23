//! Logisim-Evolution .circ file format support
//!
//! This module provides parsing and serialization support for the Logisim-Evolution
//! .circ XML file format. It handles loading circuit descriptions, components,
//! wires, and ROM content to and from the Rust simulation structures.
//!
//! ## .circ Format Overview
//!
//! The .circ format is an XML-based circuit description format used by Logisim-Evolution.
//! Key elements include:
//! - `<project>`: Root element with version information
//! - `<lib>`: Library declarations with component tool configurations
//! - `<main>`: Main circuit reference
//! - `<options>`, `<mappings>`, `<toolbar>`: UI configuration
//! - `<circuit>`: Circuit definitions with components and wires
//! - `<comp>`: Component instances with attributes and locations
//! - `<wire>`: Wire connections between components
//! - ROM data is embedded in component attributes using addr/data format
//!
//! ## Mapping to Rust Types
//!
//! - `<project>` -> `CircuitFile`
//! - `<circuit>` -> `Circuit` + netlist structure
//! - `<comp>` -> `Component` implementations
//! - `<wire>` -> Netlist connections
//! - ROM `contents` attribute -> Memory content structures

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::component::{Component, ComponentId};
use crate::netlist::NodeId;
use crate::signal::BusWidth;
use crate::simulation::Simulation;

/// Errors that can occur during .circ file processing
#[derive(Error, Debug)]
pub enum CircFormatError {
    #[error("XML parsing error: {0}")]
    XmlError(#[from] roxmltree::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid circuit format: {0}")]
    InvalidFormat(String),
    
    #[error("Unsupported component: {0}")]
    UnsupportedComponent(String),
    
    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),
    
    #[error("Invalid attribute value: {0}")]
    InvalidAttributeValue(String),
    
    #[error("Component connection error: {0}")]
    ConnectionError(String),
    
    #[error("ROM parsing error: {0}")]
    RomParsingError(String),
}

/// Result type for .circ format operations
pub type CircResult<T> = Result<T, CircFormatError>;

/// Represents a complete .circ file with all its circuits and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitFile {
    /// Logisim version that created this file
    pub source_version: String,
    /// File format version
    pub version: String,
    /// Library configurations
    pub libraries: Vec<LibraryConfig>,
    /// Main circuit name
    pub main_circuit: Option<String>,
    /// All circuits in the file
    pub circuits: HashMap<String, CircuitDefinition>,
    /// VHDL content if any
    pub vhdl_contents: Vec<VhdlContent>,
    /// Project options
    pub options: ProjectOptions,
}

/// Library configuration for component tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryConfig {
    pub name: String,
    pub description: String,
    pub tools: Vec<ToolConfig>,
}

/// Tool configuration within a library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub name: String,
    pub attributes: HashMap<String, String>,
}

/// Circuit definition with components and connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitDefinition {
    pub name: String,
    pub components: Vec<ComponentInstance>,
    pub wires: Vec<WireConnection>,
    pub appearance: Option<CircuitAppearance>,
    pub attributes: HashMap<String, String>,
}

/// A component instance in a circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInstance {
    pub library: Option<String>,
    pub name: String,
    pub location: (i32, i32),
    pub attributes: HashMap<String, String>,
    pub facing: Option<String>,
}

/// Wire connection between points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireConnection {
    pub from: (i32, i32),
    pub to: (i32, i32),
}

/// Circuit appearance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitAppearance {
    pub custom: bool,
    pub elements: Vec<AppearanceElement>,
}

/// Appearance element (SVG-like)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceElement {
    pub element_type: String,
    pub attributes: HashMap<String, String>,
}

/// VHDL content block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VhdlContent {
    pub name: String,
    pub content: String,
}

/// Project options and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOptions {
    pub canvas: CanvasOptions,
    pub simulation: SimulationOptions,
    pub toolbar: ToolbarOptions,
}

/// Canvas display options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasOptions {
    pub printer_view: bool,
    pub gate_undefined: String,
    pub simulation_icons: bool,
}

/// Simulation engine options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOptions {
    pub sim_limit: i32,
    pub sim_rand: i32,
}

/// Toolbar configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolbarOptions {
    pub zoom_enabled: bool,
    pub show_zoom: bool,
}

/// ROM memory contents parser
#[derive(Debug, Clone)]
pub struct RomContents {
    pub addr_width: u32,
    pub data_width: u32,
    pub data: Vec<u64>,
}

impl RomContents {
    /// Parse ROM contents from the Logisim format string
    /// Format: "addr/data: <addr_width> <data_width>\n<hex_data>"
    /// Also handles run-length encoding like "4*a2000" meaning repeat "a2000" 4 times
    pub fn parse_from_string(contents: &str) -> CircResult<Self> {
        let lines: Vec<&str> = contents.trim().lines().collect();
        if lines.is_empty() {
            return Err(CircFormatError::RomParsingError(
                "Empty ROM contents".to_string(),
            ));
        }

        // Parse header line: "addr/data: 20 35"
        let header = lines[0];
        let header_parts: Vec<&str> = header.split_whitespace().collect();
        if header_parts.len() != 3 || header_parts[0] != "addr/data:" {
            return Err(CircFormatError::RomParsingError(format!(
                "Invalid ROM header: {}",
                header
            )));
        }

        let addr_width: u32 = header_parts[1]
            .parse()
            .map_err(|_| CircFormatError::RomParsingError("Invalid address width".to_string()))?;
        let data_width: u32 = header_parts[2]
            .parse()
            .map_err(|_| CircFormatError::RomParsingError("Invalid data width".to_string()))?;

        // Parse data lines with support for run-length encoding
        let mut data = Vec::new();
        for line in &lines[1..] {
            let hex_values: Vec<&str> = line.split_whitespace().collect();
            for hex_val in hex_values {
                if !hex_val.is_empty() {
                    // Check for run-length encoding pattern like "4*a2000"
                    if hex_val.contains('*') {
                        let parts: Vec<&str> = hex_val.split('*').collect();
                        if parts.len() == 2 {
                            let repeat_count: usize = parts[0].parse().map_err(|_| {
                                CircFormatError::RomParsingError(format!(
                                    "Invalid repeat count in: {}",
                                    hex_val
                                ))
                            })?;
                            let value = u64::from_str_radix(parts[1], 16).map_err(|_| {
                                CircFormatError::RomParsingError(format!(
                                    "Invalid hex value in run-length: {}",
                                    hex_val
                                ))
                            })?;
                            
                            // Add the value repeat_count times
                            for _ in 0..repeat_count {
                                data.push(value);
                            }
                        } else {
                            return Err(CircFormatError::RomParsingError(format!(
                                "Invalid run-length format: {}",
                                hex_val
                            )));
                        }
                    } else {
                        // Regular hex value
                        let value = u64::from_str_radix(hex_val, 16).map_err(|_| {
                            CircFormatError::RomParsingError(format!("Invalid hex value: {}", hex_val))
                        })?;
                        data.push(value);
                    }
                }
            }
        }

        Ok(RomContents {
            addr_width,
            data_width,
            data,
        })
    }

    /// Serialize ROM contents to Logisim format string
    pub fn to_string(&self) -> String {
        let mut result = format!("addr/data: {} {}\n", self.addr_width, self.data_width);
        
        // Write data in lines of 8 values each (typical Logisim format)
        const VALUES_PER_LINE: usize = 8;
        for chunk in self.data.chunks(VALUES_PER_LINE) {
            let hex_values: Vec<String> = chunk.iter().map(|&val| format!("{:x}", val)).collect();
            result.push_str(&hex_values.join(" "));
            result.push('\n');
        }
        
        result
    }
}

/// Main parser for .circ files
pub struct CircParser;

impl CircParser {
    /// Load a .circ file from a path
    pub fn load_file<P: AsRef<Path>>(path: P) -> CircResult<CircuitFile> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Self::parse_string(&contents)
    }

    /// Parse a .circ file from a string
    pub fn parse_string(xml_content: &str) -> CircResult<CircuitFile> {
        let doc = roxmltree::Document::parse(xml_content)?;
        let root = doc.root_element();

        if root.tag_name().name() != "project" {
            return Err(CircFormatError::InvalidFormat(
                "Root element must be 'project'".to_string(),
            ));
        }

        let source_version = root
            .attribute("source")
            .unwrap_or("unknown")
            .to_string();
        let version = root
            .attribute("version")
            .unwrap_or("1.0")
            .to_string();

        // Parse libraries
        let mut libraries = Vec::new();
        for lib_node in root.children().filter(|n| n.tag_name().name() == "lib") {
            libraries.push(Self::parse_library(lib_node)?);
        }

        // Parse main circuit
        let main_circuit = root
            .children()
            .find(|n| n.tag_name().name() == "main")
            .and_then(|n| n.attribute("name"))
            .map(String::from);

        // Parse circuits
        let mut circuits = HashMap::new();
        for circuit_node in root.children().filter(|n| n.tag_name().name() == "circuit") {
            let circuit = Self::parse_circuit(circuit_node)?;
            circuits.insert(circuit.name.clone(), circuit);
        }

        // Parse VHDL contents
        let mut vhdl_contents = Vec::new();
        for vhdl_node in root.children().filter(|n| n.tag_name().name() == "vhdl") {
            vhdl_contents.push(Self::parse_vhdl(vhdl_node)?);
        }

        // Parse options (simplified for now)
        let options = Self::parse_options(&root)?;

        Ok(CircuitFile {
            source_version,
            version,
            libraries,
            main_circuit,
            circuits,
            vhdl_contents,
            options,
        })
    }

    fn parse_library(lib_node: roxmltree::Node) -> CircResult<LibraryConfig> {
        let name = lib_node
            .attribute("name")
            .unwrap_or("unknown")
            .to_string();
        let description = lib_node
            .attribute("desc")
            .unwrap_or("")
            .to_string();

        let mut tools = Vec::new();
        for tool_node in lib_node.children().filter(|n| n.tag_name().name() == "tool") {
            tools.push(Self::parse_tool(tool_node)?);
        }

        Ok(LibraryConfig {
            name,
            description,
            tools,
        })
    }

    fn parse_tool(tool_node: roxmltree::Node) -> CircResult<ToolConfig> {
        let name = tool_node
            .attribute("name")
            .unwrap_or("unknown")
            .to_string();

        let mut attributes = HashMap::new();
        for attr_node in tool_node.children().filter(|n| n.tag_name().name() == "a") {
            if let (Some(attr_name), Some(attr_value)) = (
                attr_node.attribute("name"),
                attr_node.attribute("val")
            ) {
                attributes.insert(attr_name.to_string(), attr_value.to_string());
            }
        }

        Ok(ToolConfig { name, attributes })
    }

    fn parse_circuit(circuit_node: roxmltree::Node) -> CircResult<CircuitDefinition> {
        let name = circuit_node
            .attribute("name")
            .ok_or_else(|| CircFormatError::MissingAttribute("circuit name".to_string()))?
            .to_string();

        // Parse circuit attributes
        let mut attributes = HashMap::new();
        for attr_node in circuit_node.children().filter(|n| n.tag_name().name() == "a") {
            if let (Some(attr_name), Some(attr_value)) = (
                attr_node.attribute("name"),
                attr_node.attribute("val")
            ) {
                attributes.insert(attr_name.to_string(), attr_value.to_string());
            }
        }

        // Parse components
        let mut components = Vec::new();
        for comp_node in circuit_node.children().filter(|n| n.tag_name().name() == "comp") {
            components.push(Self::parse_component(comp_node)?);
        }

        // Parse wires
        let mut wires = Vec::new();
        for wire_node in circuit_node.children().filter(|n| n.tag_name().name() == "wire") {
            wires.push(Self::parse_wire(wire_node)?);
        }

        // Parse appearance (simplified)
        let appearance = circuit_node
            .children()
            .find(|n| n.tag_name().name() == "appear")
            .map(|_| CircuitAppearance {
                custom: true,
                elements: Vec::new(), // TODO: Parse appearance elements
            });

        Ok(CircuitDefinition {
            name,
            components,
            wires,
            appearance,
            attributes,
        })
    }

    fn parse_component(comp_node: roxmltree::Node) -> CircResult<ComponentInstance> {
        let library = comp_node.attribute("lib").map(String::from);
        let name = comp_node
            .attribute("name")
            .ok_or_else(|| CircFormatError::MissingAttribute("component name".to_string()))?
            .to_string();

        // Parse location
        let loc_str = comp_node
            .attribute("loc")
            .ok_or_else(|| CircFormatError::MissingAttribute("component location".to_string()))?;
        let location = Self::parse_location(loc_str)?;

        // Parse component attributes
        let mut attributes = HashMap::new();
        for attr_node in comp_node.children().filter(|n| n.tag_name().name() == "a") {
            if let (Some(attr_name), Some(attr_value)) = (
                attr_node.attribute("name"),
                attr_node.attribute("val")
            ) {
                attributes.insert(attr_name.to_string(), attr_value.to_string());
            }
        }

        // Special handling for ROM contents
        if name == "ROM" {
            // Look for contents in text content of the node or in child nodes
            for child in comp_node.children() {
                if child.tag_name().name() == "a" && child.attribute("name") == Some("contents") {
                    if let Some(contents_text) = child.text() {
                        attributes.insert("contents".to_string(), contents_text.to_string());
                    }
                }
            }
        }

        let facing = comp_node.attribute("facing").map(String::from);

        Ok(ComponentInstance {
            library,
            name,
            location,
            attributes,
            facing,
        })
    }

    fn parse_wire(wire_node: roxmltree::Node) -> CircResult<WireConnection> {
        let from_str = wire_node
            .attribute("from")
            .ok_or_else(|| CircFormatError::MissingAttribute("wire from".to_string()))?;
        let to_str = wire_node
            .attribute("to")
            .ok_or_else(|| CircFormatError::MissingAttribute("wire to".to_string()))?;

        let from = Self::parse_location(from_str)?;
        let to = Self::parse_location(to_str)?;

        Ok(WireConnection { from, to })
    }

    fn parse_location(loc_str: &str) -> CircResult<(i32, i32)> {
        // Format: "(x,y)"
        let trimmed = loc_str.trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() != 2 {
            return Err(CircFormatError::InvalidAttributeValue(format!(
                "Invalid location format: {}",
                loc_str
            )));
        }

        let x: i32 = parts[0].trim().parse().map_err(|_| {
            CircFormatError::InvalidAttributeValue(format!("Invalid x coordinate: {}", parts[0]))
        })?;
        let y: i32 = parts[1].trim().parse().map_err(|_| {
            CircFormatError::InvalidAttributeValue(format!("Invalid y coordinate: {}", parts[1]))
        })?;

        Ok((x, y))
    }

    fn parse_vhdl(vhdl_node: roxmltree::Node) -> CircResult<VhdlContent> {
        let name = vhdl_node
            .attribute("name")
            .unwrap_or("unnamed")
            .to_string();
        let content = vhdl_node.text().unwrap_or("").to_string();

        Ok(VhdlContent { name, content })
    }

    fn parse_options(_root: &roxmltree::Node) -> CircResult<ProjectOptions> {
        // Simplified options parsing - return defaults for now
        Ok(ProjectOptions {
            canvas: CanvasOptions {
                printer_view: false,
                gate_undefined: "ignore".to_string(),
                simulation_icons: true,
            },
            simulation: SimulationOptions {
                sim_limit: 1000,
                sim_rand: 0,
            },
            toolbar: ToolbarOptions {
                zoom_enabled: true,
                show_zoom: true,
            },
        })
    }
}

/// Circuit file serializer/writer
pub struct CircWriter;

impl CircWriter {
    /// Save a circuit file to a path
    pub fn save_file<P: AsRef<Path>>(circuit_file: &CircuitFile, path: P) -> CircResult<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let xml_content = Self::serialize_to_string(circuit_file)?;
        writer.write_all(xml_content.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    /// Serialize a circuit file to XML string
    pub fn serialize_to_string(circuit_file: &CircuitFile) -> CircResult<String> {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n");
        xml.push_str(&format!(
            "<project source=\"{}\" version=\"{}\">\n",
            circuit_file.source_version, circuit_file.version
        ));

        xml.push_str("  This file is intended to be loaded by Logisim-evolution.\n\n");

        // Write libraries
        for lib in &circuit_file.libraries {
            Self::write_library(&mut xml, lib);
        }

        // Write main circuit reference
        if let Some(main) = &circuit_file.main_circuit {
            xml.push_str(&format!("  <main name=\"{}\"/>\n", main));
        }

        // Write options (simplified)
        Self::write_options(&mut xml, &circuit_file.options);

        // Write circuits
        for circuit in circuit_file.circuits.values() {
            Self::write_circuit(&mut xml, circuit);
        }

        // Write VHDL contents
        for vhdl in &circuit_file.vhdl_contents {
            Self::write_vhdl(&mut xml, vhdl);
        }

        xml.push_str("</project>\n");
        Ok(xml)
    }

    fn write_library(xml: &mut String, lib: &LibraryConfig) {
        xml.push_str(&format!(
            "  <lib desc=\"{}\" name=\"{}\">\n",
            lib.description, lib.name
        ));

        for tool in &lib.tools {
            xml.push_str(&format!("    <tool name=\"{}\">\n", tool.name));
            for (attr_name, attr_value) in &tool.attributes {
                xml.push_str(&format!(
                    "      <a name=\"{}\" val=\"{}\"/>\n",
                    attr_name, attr_value
                ));
            }
            xml.push_str("    </tool>\n");
        }

        xml.push_str("  </lib>\n");
    }

    fn write_options(xml: &mut String, _options: &ProjectOptions) {
        // Simplified options writing - just add placeholder
        xml.push_str("  <options>\n");
        xml.push_str("    <a name=\"gateUndefined\" val=\"ignore\"/>\n");
        xml.push_str("    <a name=\"simlimit\" val=\"1000\"/>\n");
        xml.push_str("    <a name=\"simrand\" val=\"0\"/>\n");
        xml.push_str("  </options>\n");
    }

    fn write_circuit(xml: &mut String, circuit: &CircuitDefinition) {
        xml.push_str(&format!("  <circuit name=\"{}\">\n", circuit.name));

        // Write circuit attributes
        for (attr_name, attr_value) in &circuit.attributes {
            xml.push_str(&format!(
                "    <a name=\"{}\" val=\"{}\"/>\n",
                attr_name, attr_value
            ));
        }

        // Write appearance if custom
        if let Some(appearance) = &circuit.appearance {
            if appearance.custom {
                xml.push_str("    <appear>\n");
                // TODO: Write appearance elements
                xml.push_str("    </appear>\n");
            }
        }

        // Write wires
        for wire in &circuit.wires {
            xml.push_str(&format!(
                "    <wire from=\"({},{})\" to=\"({},{})\"/>\n",
                wire.from.0, wire.from.1, wire.to.0, wire.to.1
            ));
        }

        // Write components
        for comp in &circuit.components {
            Self::write_component(xml, comp);
        }

        xml.push_str("  </circuit>\n");
    }

    fn write_component(xml: &mut String, comp: &ComponentInstance) {
        let lib_attr = comp
            .library
            .as_ref()
            .map(|lib| format!(" lib=\"{}\"", lib))
            .unwrap_or_default();

        xml.push_str(&format!(
            "    <comp{} loc=\"({},{})\" name=\"{}\">\n",
            lib_attr, comp.location.0, comp.location.1, comp.name
        ));

        // Write component attributes
        for (attr_name, attr_value) in &comp.attributes {
            if attr_name == "contents" && comp.name == "ROM" {
                // Special handling for ROM contents - write as text content
                xml.push_str(&format!("      <a name=\"contents\">{}</a>\n", attr_value));
            } else {
                xml.push_str(&format!(
                    "      <a name=\"{}\" val=\"{}\"/>\n",
                    attr_name, attr_value
                ));
            }
        }

        xml.push_str("    </comp>\n");
    }

    fn write_vhdl(xml: &mut String, vhdl: &VhdlContent) {
        xml.push_str(&format!("  <vhdl name=\"{}\">{}</vhdl>\n", vhdl.name, vhdl.content));
    }
}

/// Integration functions to convert between .circ format and simulation structures
pub struct CircIntegration;

impl CircIntegration {
    /// Load a .circ file into a Simulation
    pub fn load_into_simulation<P: AsRef<Path>>(path: P) -> CircResult<Simulation> {
        let circuit_file = CircParser::load_file(path)?;
        Self::circuit_file_to_simulation(&circuit_file)
    }

    /// Convert a CircuitFile to a Simulation
    pub fn circuit_file_to_simulation(circuit_file: &CircuitFile) -> CircResult<Simulation> {
        let mut sim = Simulation::new();

        // Find the main circuit
        let main_circuit_name = circuit_file
            .main_circuit
            .as_ref()
            .or_else(|| circuit_file.circuits.keys().next())
            .ok_or_else(|| CircFormatError::InvalidFormat("No circuits found".to_string()))?;

        let main_circuit = circuit_file
            .circuits
            .get(main_circuit_name)
            .ok_or_else(|| {
                CircFormatError::InvalidFormat(format!(
                    "Main circuit '{}' not found",
                    main_circuit_name
                ))
            })?;

        // Build the simulation from the main circuit
        Self::build_circuit_in_simulation(&mut sim, main_circuit, &circuit_file.circuits)?;

        Ok(sim)
    }

    fn build_circuit_in_simulation(
        sim: &mut Simulation,
        circuit: &CircuitDefinition,
        _all_circuits: &HashMap<String, CircuitDefinition>,
    ) -> CircResult<()> {
        use crate::component::{AndGate, ClockedLatch};

        // Create a mapping from locations to node IDs for wire connections
        let mut location_to_node: HashMap<(i32, i32), NodeId> = HashMap::new();

        // First pass: Create components and identify connection points
        let mut component_ids = Vec::new();
        for comp_instance in &circuit.components {
            let component_id = ComponentId((component_ids.len() as u32 + 1).into());

            // Create appropriate component based on name
            let component: Box<dyn Component> = match comp_instance.name.as_str() {
                "AND Gate" => Box::new(AndGate::new(component_id)),
                "Clocked Latch" => Box::new(ClockedLatch::new(component_id)),
                "ROM" => {
                    // Create a ROM component (simplified for now)
                    // TODO: Implement proper ROM component with contents
                    Box::new(AndGate::new(component_id)) // Placeholder
                }
                _ => {
                    return Err(CircFormatError::UnsupportedComponent(
                        comp_instance.name.clone(),
                    ));
                }
            };

            let added_id = sim.add_component(component);
            component_ids.push(added_id);

            // Create nodes for component pins (simplified approach)
            let comp_location = comp_instance.location;
            
            // Create nodes for standard pin locations relative to component
            // This is a simplified mapping - real Logisim has complex pin layouts
            let pin_offsets = match comp_instance.name.as_str() {
                "AND Gate" => vec![(-30, 0), (-30, 10), (30, 0)], // A, B, Y
                _ => vec![(0, 0)], // Default single node
            };

            for (i, (dx, dy)) in pin_offsets.into_iter().enumerate() {
                let pin_location = (comp_location.0 + dx, comp_location.1 + dy);
                if !location_to_node.contains_key(&pin_location) {
                    let node = sim.netlist_mut().create_named_node(
                        BusWidth(1),
                        format!("node_{}_{}", pin_location.0, pin_location.1),
                    );
                    location_to_node.insert(pin_location, node);
                }
            }
        }

        // Second pass: Create nodes for wire endpoints
        for wire in &circuit.wires {
            for &location in &[wire.from, wire.to] {
                if !location_to_node.contains_key(&location) {
                    let node = sim.netlist_mut().create_named_node(
                        BusWidth(1),
                        format!("wire_{}_{}", location.0, location.1),
                    );
                    location_to_node.insert(location, node);
                }
            }
        }

        // Third pass: Connect components to nodes (simplified)
        // This is a placeholder - real connection logic would be much more complex
        for (i, comp_instance) in circuit.components.iter().enumerate() {
            let component_id = component_ids[i];
            let comp_location = comp_instance.location;

            // Connect to nearby nodes (simplified logic)
            if let Some(&node) = location_to_node.get(&comp_location) {
                // This is oversimplified - real pin mapping would be component-specific
                let _ = sim.netlist_mut().connect(component_id, "A".to_string(), node);
            }
        }

        Ok(())
    }

    /// Extract a Simulation back to a CircuitFile
    pub fn simulation_to_circuit_file(_sim: &Simulation) -> CircResult<CircuitFile> {
        // This would extract the current simulation state back to .circ format
        // Simplified implementation for now
        let mut circuits = HashMap::new();
        
        circuits.insert(
            "main".to_string(),
            CircuitDefinition {
                name: "main".to_string(),
                components: Vec::new(), // TODO: Extract from simulation
                wires: Vec::new(),      // TODO: Extract from netlist
                appearance: None,
                attributes: HashMap::new(),
            },
        );

        Ok(CircuitFile {
            source_version: "logisim-rust-0.1.0".to_string(),
            version: "1.0".to_string(),
            libraries: Vec::new(), // TODO: Add standard libraries
            main_circuit: Some("main".to_string()),
            circuits,
            vhdl_contents: Vec::new(),
            options: ProjectOptions {
                canvas: CanvasOptions {
                    printer_view: false,
                    gate_undefined: "ignore".to_string(),
                    simulation_icons: true,
                },
                simulation: SimulationOptions {
                    sim_limit: 1000,
                    sim_rand: 0,
                },
                toolbar: ToolbarOptions {
                    zoom_enabled: true,
                    show_zoom: true,
                },
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom_contents_parsing() {
        let rom_data = "addr/data: 20 35\n2500 10004400 2500 10004400 2500 10004400 2500 10004400\n2500 10004400 2500 10004400 2500 10004400 2500 10004400";
        
        let rom = RomContents::parse_from_string(rom_data).unwrap();
        assert_eq!(rom.addr_width, 20);
        assert_eq!(rom.data_width, 35);
        assert_eq!(rom.data.len(), 16);
        assert_eq!(rom.data[0], 0x2500);
        assert_eq!(rom.data[1], 0x10004400);
    }

    #[test]
    fn test_rom_contents_run_length_parsing() {
        let rom_data = "addr/data: 16 8\n12 34 4*ab 56";
        
        let rom = RomContents::parse_from_string(rom_data).unwrap();
        assert_eq!(rom.addr_width, 16);
        assert_eq!(rom.data_width, 8);
        assert_eq!(rom.data.len(), 7); // 12, 34, ab, ab, ab, ab, 56
        assert_eq!(rom.data[0], 0x12);
        assert_eq!(rom.data[1], 0x34);
        assert_eq!(rom.data[2], 0xab);
        assert_eq!(rom.data[3], 0xab);
        assert_eq!(rom.data[4], 0xab);
        assert_eq!(rom.data[5], 0xab);
        assert_eq!(rom.data[6], 0x56);
    }

    #[test]
    fn test_rom_contents_serialization() {
        let rom = RomContents {
            addr_width: 16,
            data_width: 8,
            data: vec![0x12, 0x34, 0x56, 0x78],
        };
        
        let serialized = rom.to_string();
        assert!(serialized.contains("addr/data: 16 8"));
        assert!(serialized.contains("12 34 56 78"));
    }

    #[test]
    fn test_location_parsing() {
        assert_eq!(CircParser::parse_location("(100,200)").unwrap(), (100, 200));
        assert_eq!(CircParser::parse_location("(-50, 75)").unwrap(), (-50, 75));
        assert!(CircParser::parse_location("invalid").is_err());
    }

    #[test]
    fn test_simple_circuit_parsing() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n\
<project source=\"test\" version=\"1.0\">\n\
  <lib desc=\"#Wiring\" name=\"0\"/>\n\
  <main name=\"main\"/>\n\
  <circuit name=\"main\">\n\
    <comp lib=\"1\" loc=\"(100,100)\" name=\"AND Gate\">\n\
      <a name=\"size\" val=\"30\"/>\n\
    </comp>\n\
    <wire from=\"(80,90)\" to=\"(80,100)\"/>\n\
  </circuit>\n\
</project>";

        let circuit_file = CircParser::parse_string(xml).unwrap();
        assert_eq!(circuit_file.source_version, "test");
        assert_eq!(circuit_file.main_circuit, Some("main".to_string()));
        assert_eq!(circuit_file.circuits.len(), 1);
        
        let main_circuit = &circuit_file.circuits["main"];
        assert_eq!(main_circuit.components.len(), 1);
        assert_eq!(main_circuit.wires.len(), 1);
        assert_eq!(main_circuit.components[0].name, "AND Gate");
        assert_eq!(main_circuit.components[0].location, (100, 100));
    }

    #[test]
    fn test_circuit_round_trip() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n\
<project source=\"test\" version=\"1.0\">\n\
  <main name=\"test\"/>\n\
  <circuit name=\"test\">\n\
    <comp lib=\"1\" loc=\"(50,50)\" name=\"AND Gate\"/>\n\
  </circuit>\n\
</project>";

        let circuit_file = CircParser::parse_string(xml).unwrap();
        let serialized = CircWriter::serialize_to_string(&circuit_file).unwrap();
        
        // Parse the serialized version to ensure it's valid
        let reparsed = CircParser::parse_string(&serialized).unwrap();
        assert_eq!(reparsed.circuits.len(), circuit_file.circuits.len());
    }
}