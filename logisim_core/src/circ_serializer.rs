//! .circ file format serializer for Logisim-Evolution circuit files
//!
//! This module provides serialization functionality to convert Rust data structures
//! back into .circ XML files that can be read by Logisim-Evolution.

use crate::circ_parser::*;
use quick_xml::events::{Event, BytesEnd, BytesStart};
use quick_xml::writer::Writer;
use std::io::Write;
use thiserror::Error;

/// Errors that can occur during .circ file serialization
#[derive(Error, Debug)]
pub enum CircSerializeError {
    #[error("XML writing error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

/// Serializer for .circ files
pub struct CircSerializer;

impl CircSerializer {
    /// Serialize a CircuitProject to a writer
    pub fn serialize<W: Write>(
        project: &CircuitProject,
        writer: W
    ) -> Result<(), CircSerializeError> {
        let mut xml_writer = Writer::new_with_indent(writer, b' ', 2);
        
        // Write XML declaration  
        xml_writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new(
            "1.0", Some("UTF-8"), Some("no")
        )))?;
        
        Self::write_project(&mut xml_writer, project)?;
        
        Ok(())
    }
    
    /// Write the project element
    fn write_project<W: Write>(
        writer: &mut Writer<W>,
        project: &CircuitProject
    ) -> Result<(), CircSerializeError> {
        let mut project_elem = BytesStart::new("project");
        project_elem.push_attribute(("source", project.source.as_str()));
        project_elem.push_attribute(("version", project.version.as_str()));
        
        writer.write_event(Event::Start(project_elem))?;
        
        // Write libraries
        for library in &project.libraries {
            Self::write_library(writer, library)?;
        }
        
        // Write main circuit reference
        if !project.main_circuit.is_empty() {
            let mut main_elem = BytesStart::new("main");
            main_elem.push_attribute(("name", project.main_circuit.as_str()));
            writer.write_event(Event::Empty(main_elem))?;
        }
        
        // Write options
        if !project.options.is_empty() {
            Self::write_options(writer, &project.options)?;
        }
        
        // Write mappings
        if !project.mappings.is_empty() {
            Self::write_mappings(writer, &project.mappings)?;
        }
        
        // Write toolbar
        if !project.toolbar.is_empty() {
            Self::write_toolbar(writer, &project.toolbar)?;
        }
        
        // Write circuits
        for circuit in &project.circuits {
            Self::write_circuit(writer, circuit)?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("project")))?;
        
        Ok(())
    }
    
    /// Write a library element
    fn write_library<W: Write>(
        writer: &mut Writer<W>,
        library: &Library
    ) -> Result<(), CircSerializeError> {
        let mut lib_elem = BytesStart::new("lib");
        lib_elem.push_attribute(("name", library.name.as_str()));
        if !library.description.is_empty() {
            lib_elem.push_attribute(("desc", library.description.as_str()));
        }
        
        if library.tools.is_empty() {
            writer.write_event(Event::Empty(lib_elem))?;
        } else {
            writer.write_event(Event::Start(lib_elem))?;
            
            for tool in &library.tools {
                Self::write_library_tool(writer, tool)?;
            }
            
            writer.write_event(Event::End(BytesEnd::new("lib")))?;
        }
        
        Ok(())
    }
    
    /// Write a library tool element
    fn write_library_tool<W: Write>(
        writer: &mut Writer<W>,
        tool: &LibraryTool
    ) -> Result<(), CircSerializeError> {
        let mut tool_elem = BytesStart::new("tool");
        tool_elem.push_attribute(("name", tool.name.as_str()));
        
        if tool.attributes.is_empty() {
            writer.write_event(Event::Empty(tool_elem))?;
        } else {
            writer.write_event(Event::Start(tool_elem))?;
            
            for (name, value) in &tool.attributes {
                Self::write_attribute(writer, name, value)?;
            }
            
            writer.write_event(Event::End(BytesEnd::new("tool")))?;
        }
        
        Ok(())
    }
    
    /// Write options element
    fn write_options<W: Write>(
        writer: &mut Writer<W>,
        options: &indexmap::IndexMap<String, String>
    ) -> Result<(), CircSerializeError> {
        writer.write_event(Event::Start(BytesStart::new("options")))?;
        
        for (name, value) in options {
            Self::write_attribute(writer, name, value)?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("options")))?;
        
        Ok(())
    }
    
    /// Write mappings element
    fn write_mappings<W: Write>(
        writer: &mut Writer<W>,
        mappings: &[ToolMapping]
    ) -> Result<(), CircSerializeError> {
        writer.write_event(Event::Start(BytesStart::new("mappings")))?;
        
        for mapping in mappings {
            let mut tool_elem = BytesStart::new("tool");
            tool_elem.push_attribute(("lib", mapping.lib.as_str()));
            tool_elem.push_attribute(("map", mapping.map.as_str()));
            tool_elem.push_attribute(("name", mapping.name.as_str()));
            writer.write_event(Event::Empty(tool_elem))?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("mappings")))?;
        
        Ok(())
    }
    
    /// Write toolbar element
    fn write_toolbar<W: Write>(
        writer: &mut Writer<W>,
        toolbar: &[ToolbarItem]
    ) -> Result<(), CircSerializeError> {
        writer.write_event(Event::Start(BytesStart::new("toolbar")))?;
        
        for item in toolbar {
            match item.item_type {
                ToolbarItemType::Separator => {
                    writer.write_event(Event::Empty(BytesStart::new("sep")))?;
                }
                ToolbarItemType::Tool => {
                    let mut tool_elem = BytesStart::new("tool");
                    if let Some(lib) = &item.lib {
                        tool_elem.push_attribute(("lib", lib.as_str()));
                    }
                    if let Some(name) = &item.name {
                        tool_elem.push_attribute(("name", name.as_str()));
                    }
                    
                    if item.attributes.is_empty() {
                        writer.write_event(Event::Empty(tool_elem))?;
                    } else {
                        writer.write_event(Event::Start(tool_elem))?;
                        
                        for (attr_name, attr_value) in &item.attributes {
                            Self::write_attribute(writer, attr_name, attr_value)?;
                        }
                        
                        writer.write_event(Event::End(BytesEnd::new("tool")))?;
                    }
                }
            }
        }
        
        writer.write_event(Event::End(BytesEnd::new("toolbar")))?;
        
        Ok(())
    }
    
    /// Write a circuit element
    fn write_circuit<W: Write>(
        writer: &mut Writer<W>,
        circuit: &Circuit
    ) -> Result<(), CircSerializeError> {
        let mut circuit_elem = BytesStart::new("circuit");
        circuit_elem.push_attribute(("name", circuit.name.as_str()));
        
        writer.write_event(Event::Start(circuit_elem))?;
        
        // Write circuit attributes
        for (name, value) in &circuit.attributes {
            Self::write_attribute(writer, name, value)?;
        }
        
        // Write appearance if present
        if let Some(appearance) = &circuit.appearance {
            Self::write_appearance(writer, appearance)?;
        }
        
        // Write wires
        for wire in &circuit.wires {
            Self::write_wire(writer, wire)?;
        }
        
        // Write components
        for component in &circuit.components {
            Self::write_component(writer, component)?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("circuit")))?;
        
        Ok(())
    }
    
    /// Write a wire element
    fn write_wire<W: Write>(
        writer: &mut Writer<W>,
        wire: &Wire
    ) -> Result<(), CircSerializeError> {
        let mut wire_elem = BytesStart::new("wire");
        wire_elem.push_attribute(("from", Self::format_coordinates(wire.from).as_str()));
        wire_elem.push_attribute(("to", Self::format_coordinates(wire.to).as_str()));
        
        writer.write_event(Event::Empty(wire_elem))?;
        
        Ok(())
    }
    
    /// Write a component element
    fn write_component<W: Write>(
        writer: &mut Writer<W>,
        component: &Component
    ) -> Result<(), CircSerializeError> {
        let mut comp_elem = BytesStart::new("comp");
        comp_elem.push_attribute(("lib", component.lib.as_str()));
        comp_elem.push_attribute(("loc", Self::format_coordinates(component.location).as_str()));
        comp_elem.push_attribute(("name", component.name.as_str()));
        
        if component.attributes.is_empty() {
            writer.write_event(Event::Empty(comp_elem))?;
        } else {
            writer.write_event(Event::Start(comp_elem))?;
            
            for (name, value) in &component.attributes {
                Self::write_attribute(writer, name, value)?;
            }
            
            writer.write_event(Event::End(BytesEnd::new("comp")))?;
        }
        
        Ok(())
    }
    
    /// Write appearance element
    fn write_appearance<W: Write>(
        writer: &mut Writer<W>,
        appearance: &Appearance
    ) -> Result<(), CircSerializeError> {
        writer.write_event(Event::Start(BytesStart::new("appear")))?;
        
        for element in &appearance.elements {
            let mut elem = BytesStart::new(&element.element_type);
            
            for (name, value) in &element.attributes {
                elem.push_attribute((name.as_str(), value.as_str()));
            }
            
            writer.write_event(Event::Empty(elem))?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("appear")))?;
        
        Ok(())
    }
    
    /// Write an attribute element
    fn write_attribute<W: Write>(
        writer: &mut Writer<W>,
        name: &str,
        value: &str
    ) -> Result<(), CircSerializeError> {
        let mut attr_elem = BytesStart::new("a");
        attr_elem.push_attribute(("name", name));
        attr_elem.push_attribute(("val", value));
        
        writer.write_event(Event::Empty(attr_elem))?;
        
        Ok(())
    }
    
    /// Format coordinates as "(x,y)" string
    fn format_coordinates(coords: (i32, i32)) -> String {
        format!("({},{})", coords.0, coords.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    
    #[test]
    fn test_format_coordinates() {
        assert_eq!(CircSerializer::format_coordinates((160, 130)), "(160,130)");
        assert_eq!(CircSerializer::format_coordinates((-10, 20)), "(-10,20)");
    }
    
    #[test]
    fn test_serialize_simple_project() {
        let mut project = CircuitProject {
            source: "3.8.0".to_string(),
            version: "1.0".to_string(),
            libraries: vec![
                Library {
                    name: "0".to_string(),
                    description: "#Wiring".to_string(),
                    tools: vec![],
                }
            ],
            main_circuit: "main".to_string(),
            options: {
                let mut opts = IndexMap::new();
                opts.insert("gateUndefined".to_string(), "ignore".to_string());
                opts
            },
            mappings: vec![],
            toolbar: vec![],
            circuits: vec![
                Circuit {
                    name: "main".to_string(),
                    attributes: IndexMap::new(),
                    wires: vec![
                        Wire {
                            from: (160, 130),
                            to: (220, 130),
                        }
                    ],
                    components: vec![
                        Component {
                            lib: "0".to_string(),
                            location: (160, 130),
                            name: "Pin".to_string(),
                            attributes: {
                                let mut attrs = IndexMap::new();
                                attrs.insert("label".to_string(), "A".to_string());
                                attrs
                            },
                        }
                    ],
                    appearance: None,
                }
            ],
        };
        
        let mut output = Vec::new();
        CircSerializer::serialize(&project, &mut output).unwrap();
        
        let xml = String::from_utf8(output).unwrap();
        
        // Basic checks that it contains expected elements
        assert!(xml.contains("<project source=\"3.8.0\" version=\"1.0\">"));
        assert!(xml.contains("<lib name=\"0\" desc=\"#Wiring\"/>"));
        assert!(xml.contains("<main name=\"main\"/>"));
        assert!(xml.contains("<circuit name=\"main\">"));
        assert!(xml.contains("<wire from=\"(160,130)\" to=\"(220,130)\"/>"));
        assert!(xml.contains("<comp lib=\"0\" loc=\"(160,130)\" name=\"Pin\">"));
        assert!(xml.contains("<a name=\"label\" val=\"A\"/>"));
    }
    
    #[test]
    fn test_serialize_toolbar_with_separator() {
        let project = CircuitProject {
            source: "3.8.0".to_string(),
            version: "1.0".to_string(),
            libraries: vec![],
            main_circuit: "main".to_string(),
            options: IndexMap::new(),
            mappings: vec![],
            toolbar: vec![
                ToolbarItem {
                    item_type: ToolbarItemType::Tool,
                    lib: Some("6".to_string()),
                    name: Some("Poke Tool".to_string()),
                    attributes: IndexMap::new(),
                },
                ToolbarItem {
                    item_type: ToolbarItemType::Separator,
                    lib: None,
                    name: None,
                    attributes: IndexMap::new(),
                },
            ],
            circuits: vec![],
        };
        
        let mut output = Vec::new();
        CircSerializer::serialize(&project, &mut output).unwrap();
        
        let xml = String::from_utf8(output).unwrap();
        
        assert!(xml.contains("<tool lib=\"6\" name=\"Poke Tool\"/>"));
        assert!(xml.contains("<sep/>"));
    }
}