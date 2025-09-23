//! .circ file format parser for Logisim-Evolution circuit files
//!
//! This module provides parsing functionality for .circ XML files used by Logisim-Evolution.
//! The parser converts XML circuit definitions into Rust data structures that can be used
//! by the simulation kernel.

use indexmap::IndexMap;
use quick_xml::events::{Event, BytesStart};
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use thiserror::Error;

/// Errors that can occur during .circ file parsing
#[derive(Error, Debug)]
pub enum CircParseError {
    #[error("XML parsing error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("Invalid XML structure: {0}")]
    InvalidStructure(String),
    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),
    #[error("Invalid attribute value: {0}")]
    InvalidAttributeValue(String),
    #[error("Unsupported element: {0}")]
    UnsupportedElement(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Attribute error: {0}")]
    AttributeError(#[from] quick_xml::events::attributes::AttrError),
}

/// Represents a complete .circ project file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CircuitProject {
    /// Project source version (e.g., "3.8.0")
    pub source: String,
    /// Project format version (e.g., "1.0")
    pub version: String,
    /// Library definitions
    pub libraries: Vec<Library>,
    /// Main circuit name
    pub main_circuit: String,
    /// Project options
    pub options: IndexMap<String, String>,
    /// Tool mappings
    pub mappings: Vec<ToolMapping>,
    /// Toolbar configuration
    pub toolbar: Vec<ToolbarItem>,
    /// Circuit definitions
    pub circuits: Vec<Circuit>,
}

/// Represents a library definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Library {
    /// Library name/identifier
    pub name: String,
    /// Library description
    pub description: String,
    /// Tools defined in this library
    pub tools: Vec<LibraryTool>,
}

/// Represents a tool defined in a library
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibraryTool {
    /// Tool name
    pub name: String,
    /// Tool attributes
    pub attributes: IndexMap<String, String>,
}

/// Represents a tool mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolMapping {
    /// Library reference
    pub lib: String,
    /// Mouse/keyboard mapping
    pub map: String,
    /// Tool name
    pub name: String,
}

/// Represents a toolbar item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolbarItem {
    /// Item type (tool or separator)
    pub item_type: ToolbarItemType,
    /// Library reference (for tools)
    pub lib: Option<String>,
    /// Tool name (for tools)
    pub name: Option<String>,
    /// Tool attributes (for tools)
    pub attributes: IndexMap<String, String>,
}

/// Types of toolbar items  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolbarItemType {
    Tool,
    Separator,
}

/// Represents a circuit definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Circuit {
    /// Circuit name
    pub name: String,
    /// Circuit attributes
    pub attributes: IndexMap<String, String>,
    /// Wires in the circuit
    pub wires: Vec<Wire>,
    /// Components in the circuit
    pub components: Vec<Component>,
    /// Appearance definition (optional)
    pub appearance: Option<Appearance>,
}

/// Represents a wire connection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Wire {
    /// Starting point coordinates (x,y)
    pub from: (i32, i32),
    /// Ending point coordinates (x,y)
    pub to: (i32, i32),
}

/// Represents a component instance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Component {
    /// Library reference
    pub lib: String,
    /// Component location (x,y)
    pub location: (i32, i32),
    /// Component name/type
    pub name: String,
    /// Component attributes
    pub attributes: IndexMap<String, String>,
}

/// Represents visual appearance definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Appearance {
    /// SVG elements for custom appearance
    pub elements: Vec<AppearanceElement>,
}

/// Represents an appearance element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppearanceElement {
    /// Element type (rect, circle, text, etc.)
    pub element_type: String,
    /// Element attributes
    pub attributes: IndexMap<String, String>,
}

/// Parser for .circ files
pub struct CircParser;

impl CircParser {
    /// Parse a .circ file from a reader
    pub fn parse<R: BufRead>(reader: R) -> Result<CircuitProject, CircParseError> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);
        
        let mut buf = Vec::new();
        let mut project = None;
        
        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"project" => {
                            project = Some(Self::parse_project(&mut xml_reader, e)?);
                        }
                        _ => {
                            return Err(CircParseError::InvalidStructure(
                                "Expected project root element".to_string()
                            ));
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        project.ok_or_else(|| {
            CircParseError::InvalidStructure("No project element found".to_string())
        })
    }
    
    /// Parse the project element
    fn parse_project<R: BufRead>(
        reader: &mut Reader<R>, 
        start: &BytesStart
    ) -> Result<CircuitProject, CircParseError> {
        let mut source = String::new();
        let mut version = String::new();
        
        // Parse project attributes
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"source" => source = String::from_utf8_lossy(&attr.value).to_string(),
                b"version" => version = String::from_utf8_lossy(&attr.value).to_string(),
                _ => {} // Ignore unknown attributes
            }
        }
        
        let mut libraries = Vec::new();
        let mut main_circuit = String::new();
        let mut options = IndexMap::new();
        let mut mappings = Vec::new();
        let mut toolbar = Vec::new();
        let mut circuits = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"lib" => {
                            libraries.push(Self::parse_library(reader, e)?);
                        }
                        b"options" => {
                            options = Self::parse_options(reader)?;
                        }
                        b"mappings" => {
                            mappings = Self::parse_mappings(reader)?;
                        }
                        b"toolbar" => {
                            toolbar = Self::parse_toolbar(reader)?;
                        }
                        b"circuit" => {
                            circuits.push(Self::parse_circuit(reader, e)?);
                        }
                        _ => {
                            // Skip unknown elements
                            Self::skip_element(reader)?;
                        }
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"lib" => {
                            let mut name = String::new();
                            let mut description = String::new();
                            
                            for attr in e.attributes() {
                                let attr = attr?;
                                match attr.key.as_ref() {
                                    b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                                    b"desc" => description = String::from_utf8_lossy(&attr.value).to_string(),
                                    _ => {}
                                }
                            }
                            
                            libraries.push(Library {
                                name,
                                description,
                                tools: vec![],
                            });
                        }
                        b"main" => {
                            for attr in e.attributes() {
                                let attr = attr?;
                                if attr.key.as_ref() == b"name" {
                                    main_circuit = String::from_utf8_lossy(&attr.value).to_string();
                                    break;
                                }
                            }
                        }
                        _ => {
                            // Ignore unknown empty elements
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"project" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in project".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(CircuitProject {
            source,
            version,
            libraries,
            main_circuit,
            options,
            mappings,
            toolbar,
            circuits,
        })
    }
    
    /// Parse a library element
    fn parse_library<R: BufRead>(
        reader: &mut Reader<R>,
        start: &BytesStart
    ) -> Result<Library, CircParseError> {
        let mut name = String::new();
        let mut description = String::new();
        
        // Parse library attributes
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                b"desc" => description = String::from_utf8_lossy(&attr.value).to_string(),
                _ => {}
            }
        }
        
        let mut tools = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"tool" => {
                            tools.push(Self::parse_library_tool(reader, e)?);
                        }
                        _ => {
                            Self::skip_element(reader)?;
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"lib" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in library".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(Library {
            name,
            description,
            tools,
        })
    }
    
    /// Parse a library tool element
    fn parse_library_tool<R: BufRead>(
        reader: &mut Reader<R>,
        start: &BytesStart
    ) -> Result<LibraryTool, CircParseError> {
        let mut name = String::new();
        
        // Parse tool attributes
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                _ => {}
            }
        }
        
        let mut attributes = IndexMap::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"a" => {
                            let (attr_name, attr_value) = Self::parse_attribute(e)?;
                            attributes.insert(attr_name, attr_value);
                        }
                        _ => {
                            Self::skip_element(reader)?;
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"tool" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in tool".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(LibraryTool { name, attributes })
    }
    

    /// Parse options section
    fn parse_options<R: BufRead>(reader: &mut Reader<R>) -> Result<IndexMap<String, String>, CircParseError> {
        let mut options = IndexMap::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"a" => {
                            let (name, value) = Self::parse_attribute(e)?;
                            options.insert(name, value);
                        }
                        _ => {
                            // Ignore unknown empty elements
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"options" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in options".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(options)
    }
    
    /// Parse mappings section
    fn parse_mappings<R: BufRead>(reader: &mut Reader<R>) -> Result<Vec<ToolMapping>, CircParseError> {
        let mut mappings = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"tool" => {
                            mappings.push(Self::parse_tool_mapping(e)?);
                        }
                        _ => {
                            // Ignore unknown empty elements
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"mappings" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in mappings".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(mappings)
    }
    
    /// Parse a tool mapping
    fn parse_tool_mapping(start: &BytesStart) -> Result<ToolMapping, CircParseError> {
        let mut lib = String::new();
        let mut map = String::new();
        let mut name = String::new();
        
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"lib" => lib = String::from_utf8_lossy(&attr.value).to_string(),
                b"map" => map = String::from_utf8_lossy(&attr.value).to_string(),
                b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                _ => {}
            }
        }
        
        Ok(ToolMapping { lib, map, name })
    }
    
    /// Parse toolbar section
    fn parse_toolbar<R: BufRead>(reader: &mut Reader<R>) -> Result<Vec<ToolbarItem>, CircParseError> {
        let mut toolbar = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"tool" => {
                            toolbar.push(Self::parse_toolbar_tool(reader, e)?);
                        }
                        _ => {
                            Self::skip_element(reader)?;
                        }
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"tool" => {
                            let mut lib = None;
                            let mut name = None;
                            
                            for attr in e.attributes() {
                                let attr = attr?;
                                match attr.key.as_ref() {
                                    b"lib" => lib = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    b"name" => name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    _ => {}
                                }
                            }
                            
                            toolbar.push(ToolbarItem {
                                item_type: ToolbarItemType::Tool,
                                lib,
                                name,
                                attributes: IndexMap::new(),
                            });
                        }
                        b"sep" => {
                            toolbar.push(ToolbarItem {
                                item_type: ToolbarItemType::Separator,
                                lib: None,
                                name: None,
                                attributes: IndexMap::new(),
                            });
                        }
                        _ => {
                            // Ignore unknown empty elements
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"toolbar" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in toolbar".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(toolbar)
    }
    
    /// Parse a toolbar tool
    fn parse_toolbar_tool<R: BufRead>(
        reader: &mut Reader<R>,
        start: &BytesStart
    ) -> Result<ToolbarItem, CircParseError> {
        let mut lib = None;
        let mut name = None;
        
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"lib" => lib = Some(String::from_utf8_lossy(&attr.value).to_string()),
                b"name" => name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                _ => {}
            }
        }
        
        let mut attributes = IndexMap::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"a" => {
                            let (attr_name, attr_value) = Self::parse_attribute(e)?;
                            attributes.insert(attr_name, attr_value);
                        }
                        _ => {
                            Self::skip_element(reader)?;
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"tool" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in toolbar tool".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(ToolbarItem {
            item_type: ToolbarItemType::Tool,
            lib,
            name,
            attributes,
        })
    }
    
    /// Parse a circuit element
    fn parse_circuit<R: BufRead>(
        reader: &mut Reader<R>,
        start: &BytesStart
    ) -> Result<Circuit, CircParseError> {
        let mut name = String::new();
        
        for attr in start.attributes() {
            let attr = attr?;
            if attr.key.as_ref() == b"name" {
                name = String::from_utf8_lossy(&attr.value).to_string();
                break;
            }
        }
        
        let mut attributes = IndexMap::new();
        let mut wires = Vec::new();
        let mut components = Vec::new();
        let mut appearance = None;
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"comp" => {
                            components.push(Self::parse_component(reader, e)?);
                        }
                        b"appear" => {
                            appearance = Some(Self::parse_appearance(reader)?);
                        }
                        _ => {
                            Self::skip_element(reader)?;
                        }
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"a" => {
                            let (attr_name, attr_value) = Self::parse_attribute(e)?;
                            attributes.insert(attr_name, attr_value);
                        }
                        b"wire" => {
                            wires.push(Self::parse_wire(e)?);
                        }
                        b"comp" => {
                            // Parse components without attributes (self-closing)
                            let mut lib = String::new();
                            let mut location = None;
                            let mut name = String::new();
                            
                            for attr in e.attributes() {
                                let attr = attr?;
                                match attr.key.as_ref() {
                                    b"lib" => lib = String::from_utf8_lossy(&attr.value).to_string(),
                                    b"loc" => {
                                        location = Some(Self::parse_coordinates(&String::from_utf8_lossy(&attr.value))?);
                                    }
                                    b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                                    _ => {}
                                }
                            }
                            
                            let location = location.ok_or_else(|| CircParseError::MissingAttribute("loc".to_string()))?;
                            
                            components.push(Component {
                                lib,
                                location,
                                name,
                                attributes: IndexMap::new(),
                            });
                        }
                        _ => {
                            // Ignore unknown empty elements
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"circuit" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in circuit".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(Circuit {
            name,
            attributes,
            wires,
            components,
            appearance,
        })
    }
    
    /// Parse a wire element
    fn parse_wire(start: &BytesStart) -> Result<Wire, CircParseError> {
        let mut from = None;
        let mut to = None;
        
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"from" => {
                    from = Some(Self::parse_coordinates(&String::from_utf8_lossy(&attr.value))?);
                }
                b"to" => {
                    to = Some(Self::parse_coordinates(&String::from_utf8_lossy(&attr.value))?);
                }
                _ => {}
            }
        }
        
        let from = from.ok_or_else(|| CircParseError::MissingAttribute("from".to_string()))?;
        let to = to.ok_or_else(|| CircParseError::MissingAttribute("to".to_string()))?;
        
        Ok(Wire { from, to })
    }
    
    /// Parse a component element
    fn parse_component<R: BufRead>(
        reader: &mut Reader<R>,
        start: &BytesStart
    ) -> Result<Component, CircParseError> {
        let mut lib = String::new();
        let mut location = None;
        let mut name = String::new();
        
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"lib" => lib = String::from_utf8_lossy(&attr.value).to_string(),
                b"loc" => {
                    location = Some(Self::parse_coordinates(&String::from_utf8_lossy(&attr.value))?);
                }
                b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                _ => {}
            }
        }
        
        let location = location.ok_or_else(|| CircParseError::MissingAttribute("loc".to_string()))?;
        
        let mut attributes = IndexMap::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"a" => {
                            let (attr_name, attr_value) = Self::parse_attribute(e)?;
                            attributes.insert(attr_name, attr_value);
                        }
                        _ => {
                            // Ignore unknown empty elements
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"comp" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in component".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(Component {
            lib,
            location,
            name,
            attributes,
        })
    }
    
    /// Parse appearance section (simplified - full SVG parsing would be more complex)
    fn parse_appearance<R: BufRead>(reader: &mut Reader<R>) -> Result<Appearance, CircParseError> {
        let mut elements = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let element_type = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let mut attributes = IndexMap::new();
                    
                    for attr in e.attributes() {
                        let attr = attr?;
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        attributes.insert(key, value);
                    }
                    
                    elements.push(AppearanceElement {
                        element_type,
                        attributes,
                    });
                    
                    Self::skip_element(reader)?;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"appear" => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file in appearance".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(Appearance { elements })
    }
    
    /// Parse an attribute element
    fn parse_attribute(start: &BytesStart) -> Result<(String, String), CircParseError> {
        let mut name = None;
        let mut value = None;
        
        for attr in start.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"name" => name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                b"val" => value = Some(String::from_utf8_lossy(&attr.value).to_string()),
                _ => {}
            }
        }
        
        let name = name.ok_or_else(|| CircParseError::MissingAttribute("name".to_string()))?;
        let value = value.ok_or_else(|| CircParseError::MissingAttribute("val".to_string()))?;
        
        Ok((name, value))
    }
    
    /// Parse coordinate string like "(160,130)" into (x, y) tuple
    fn parse_coordinates(coord_str: &str) -> Result<(i32, i32), CircParseError> {
        let coord_str = coord_str.trim();
        if !coord_str.starts_with('(') || !coord_str.ends_with(')') {
            return Err(CircParseError::InvalidAttributeValue(
                format!("Invalid coordinate format: {}", coord_str)
            ));
        }
        
        let inner = &coord_str[1..coord_str.len()-1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 2 {
            return Err(CircParseError::InvalidAttributeValue(
                format!("Invalid coordinate format: {}", coord_str)
            ));
        }
        
        let x = parts[0].trim().parse::<i32>()
            .map_err(|_| CircParseError::InvalidAttributeValue(
                format!("Invalid x coordinate: {}", parts[0])
            ))?;
        let y = parts[1].trim().parse::<i32>()
            .map_err(|_| CircParseError::InvalidAttributeValue(
                format!("Invalid y coordinate: {}", parts[1])
            ))?;
        
        Ok((x, y))
    }
    
    /// Skip over an element and all its children
    fn skip_element<R: BufRead>(reader: &mut Reader<R>) -> Result<(), CircParseError> {
        let mut depth = 1;
        let mut buf = Vec::new();
        
        while depth > 0 {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(_)) => depth += 1,
                Ok(Event::End(_)) => depth -= 1,
                Ok(Event::Eof) => {
                    return Err(CircParseError::InvalidStructure(
                        "Unexpected end of file while skipping element".to_string()
                    ));
                }
                Err(e) => return Err(CircParseError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_parse_coordinates() {
        assert_eq!(CircParser::parse_coordinates("(160,130)").unwrap(), (160, 130));
        assert_eq!(CircParser::parse_coordinates("(-10,20)").unwrap(), (-10, 20));
        assert!(CircParser::parse_coordinates("160,130").is_err());
        assert!(CircParser::parse_coordinates("(160)").is_err());
        assert!(CircParser::parse_coordinates("(abc,def)").is_err());
    }
    
    #[test]
    fn test_parse_simple_project() {
        let xml = concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n",
            "<project source=\"3.8.0\" version=\"1.0\">\n",
            "  <lib desc=\"#Wiring\" name=\"0\"/>\n",
            "  <main name=\"main\"/>\n",
            "  <options>\n",
            "    <a name=\"gateUndefined\" val=\"ignore\"/>\n",
            "  </options>\n",
            "  <circuit name=\"main\">\n",
            "    <wire from=\"(160,130)\" to=\"(220,130)\"/>\n",
            "    <comp lib=\"0\" loc=\"(160,130)\" name=\"Pin\">\n",
            "      <a name=\"label\" val=\"A\"/>\n",
            "    </comp>\n",
            "  </circuit>\n",
            "</project>"
        );
        
        let cursor = Cursor::new(xml);
        let project = CircParser::parse(cursor).unwrap();
        
        assert_eq!(project.source, "3.8.0");
        assert_eq!(project.version, "1.0");
        assert_eq!(project.main_circuit, "main");
        assert_eq!(project.libraries.len(), 1);
        assert_eq!(project.circuits.len(), 1);
        
        let circuit = &project.circuits[0];
        assert_eq!(circuit.name, "main");
        assert_eq!(circuit.wires.len(), 1);
        assert_eq!(circuit.components.len(), 1);
        
        let wire = &circuit.wires[0];
        assert_eq!(wire.from, (160, 130));
        assert_eq!(wire.to, (220, 130));
        
        let component = &circuit.components[0];
        assert_eq!(component.lib, "0");
        assert_eq!(component.name, "Pin");
        assert_eq!(component.location, (160, 130));
        assert_eq!(component.attributes.get("label").unwrap(), "A");
    }
}