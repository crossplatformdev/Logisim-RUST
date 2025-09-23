//! Round-trip tests for .circ file parsing and serialization
//!
//! These tests verify that we can parse a .circ file and serialize it back
//! while preserving the essential content and structure.

use logisim_core::{CircParser, CircSerializer};
use std::io::Cursor;

#[test]
fn test_simple_and_gate_roundtrip() {
    let original_xml = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n",
        "<project source=\"3.8.0\" version=\"1.0\">\n",
        "  <lib desc=\"#Wiring\" name=\"0\"/>\n",
        "  <lib desc=\"#Gates\" name=\"1\"/>\n",
        "  <main name=\"main\"/>\n",
        "  <options>\n",
        "    <a name=\"gateUndefined\" val=\"ignore\"/>\n",
        "    <a name=\"simlimit\" val=\"1000\"/>\n",
        "  </options>\n",
        "  <circuit name=\"main\">\n",
        "    <a name=\"circuit\" val=\"main\"/>\n",
        "    <wire from=\"(160,130)\" to=\"(220,130)\"/>\n",
        "    <wire from=\"(160,170)\" to=\"(220,170)\"/>\n",
        "    <wire from=\"(270,150)\" to=\"(320,150)\"/>\n",
        "    <comp lib=\"0\" loc=\"(160,130)\" name=\"Pin\">\n",
        "      <a name=\"tristate\" val=\"false\"/>\n",
        "      <a name=\"label\" val=\"A\"/>\n",
        "    </comp>\n",
        "    <comp lib=\"0\" loc=\"(160,170)\" name=\"Pin\">\n",
        "      <a name=\"tristate\" val=\"false\"/>\n",
        "      <a name=\"label\" val=\"B\"/>\n",
        "    </comp>\n",
        "    <comp lib=\"1\" loc=\"(270,150)\" name=\"AND Gate\">\n",
        "      <a name=\"size\" val=\"50\"/>\n",
        "      <a name=\"inputs\" val=\"2\"/>\n",
        "    </comp>\n",
        "    <comp lib=\"0\" loc=\"(320,150)\" name=\"Pin\">\n",
        "      <a name=\"facing\" val=\"west\"/>\n",
        "      <a name=\"output\" val=\"true\"/>\n",
        "      <a name=\"label\" val=\"Y\"/>\n",
        "    </comp>\n",
        "  </circuit>\n",
        "</project>"
    );
    
    // Parse the original XML
    let cursor = Cursor::new(original_xml);
    let project = CircParser::parse(cursor).expect("Failed to parse original XML");
    
    // Verify the parsed structure
    assert_eq!(project.source, "3.8.0");
    assert_eq!(project.version, "1.0");
    assert_eq!(project.main_circuit, "main");
    assert_eq!(project.libraries.len(), 2);
    assert_eq!(project.circuits.len(), 1);
    
    let circuit = &project.circuits[0];
    assert_eq!(circuit.name, "main");
    assert_eq!(circuit.wires.len(), 3);
    assert_eq!(circuit.components.len(), 4);
    
    // Serialize back to XML
    let mut serialized = Vec::new();
    CircSerializer::serialize(&project, &mut serialized).expect("Failed to serialize");
    let serialized_xml = String::from_utf8(serialized).expect("Invalid UTF-8");
    
    // Parse the serialized XML to verify it's valid
    let cursor2 = Cursor::new(&serialized_xml);
    let project2 = CircParser::parse(cursor2).expect("Failed to parse serialized XML");
    
    // Verify the round-trip preserves structure
    assert_eq!(project2.source, project.source);
    assert_eq!(project2.version, project.version);
    assert_eq!(project2.main_circuit, project.main_circuit);
    assert_eq!(project2.libraries.len(), project.libraries.len());
    assert_eq!(project2.circuits.len(), project.circuits.len());
    
    // Verify circuit details
    let circuit2 = &project2.circuits[0];
    assert_eq!(circuit2.name, circuit.name);
    assert_eq!(circuit2.wires.len(), circuit.wires.len());
    assert_eq!(circuit2.components.len(), circuit.components.len());
    
    // Verify wire preservation
    for (original_wire, roundtrip_wire) in circuit.wires.iter().zip(circuit2.wires.iter()) {
        assert_eq!(original_wire.from, roundtrip_wire.from);
        assert_eq!(original_wire.to, roundtrip_wire.to);
    }
    
    // Verify component preservation
    for (original_comp, roundtrip_comp) in circuit.components.iter().zip(circuit2.components.iter()) {
        assert_eq!(original_comp.lib, roundtrip_comp.lib);
        assert_eq!(original_comp.name, roundtrip_comp.name);
        assert_eq!(original_comp.location, roundtrip_comp.location);
        assert_eq!(original_comp.attributes.len(), roundtrip_comp.attributes.len());
    }
}

#[test]
fn test_empty_project_roundtrip() {
    let original_xml = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n",
        "<project source=\"3.8.0\" version=\"1.0\">\n",
        "  <main name=\"main\"/>\n",
        "  <circuit name=\"main\">\n",
        "  </circuit>\n",
        "</project>"
    );
    
    // Parse the original XML
    let cursor = Cursor::new(original_xml);
    let project = CircParser::parse(cursor).expect("Failed to parse original XML");
    
    // Verify basic structure
    assert_eq!(project.source, "3.8.0");
    assert_eq!(project.version, "1.0");
    assert_eq!(project.main_circuit, "main");
    assert_eq!(project.circuits.len(), 1);
    
    let circuit = &project.circuits[0];
    assert_eq!(circuit.name, "main");
    assert_eq!(circuit.wires.len(), 0);
    assert_eq!(circuit.components.len(), 0);
    
    // Serialize back to XML
    let mut serialized = Vec::new();
    CircSerializer::serialize(&project, &mut serialized).expect("Failed to serialize");
    let serialized_xml = String::from_utf8(serialized).expect("Invalid UTF-8");
    
    // Parse the serialized XML to verify it's valid
    let cursor2 = Cursor::new(&serialized_xml);
    let project2 = CircParser::parse(cursor2).expect("Failed to parse serialized XML");
    
    // Verify the round-trip preserves structure
    assert_eq!(project2.source, project.source);
    assert_eq!(project2.version, project.version);
    assert_eq!(project2.main_circuit, project.main_circuit);
    assert_eq!(project2.circuits.len(), project.circuits.len());
    
    let circuit2 = &project2.circuits[0];
    assert_eq!(circuit2.name, circuit.name);
    assert_eq!(circuit2.wires.len(), circuit.wires.len());
    assert_eq!(circuit2.components.len(), circuit.components.len());
}

#[test]
fn test_complex_project_with_attributes_roundtrip() {
    let original_xml = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n",
        "<project source=\"3.8.0\" version=\"1.0\">\n",
        "  <lib desc=\"#Wiring\" name=\"0\"/>\n",
        "  <main name=\"main\"/>\n",
        "  <options>\n",
        "    <a name=\"gateUndefined\" val=\"ignore\"/>\n",
        "    <a name=\"simlimit\" val=\"1000\"/>\n",
        "    <a name=\"simrand\" val=\"0\"/>\n",
        "  </options>\n",
        "  <mappings>\n",
        "    <tool lib=\"6\" map=\"Button2\" name=\"Menu Tool\"/>\n",
        "    <tool lib=\"6\" map=\"Button3\" name=\"Menu Tool\"/>\n",
        "  </mappings>\n",
        "  <toolbar>\n",
        "    <tool lib=\"6\" name=\"Poke Tool\"/>\n",
        "    <sep/>\n",
        "    <tool lib=\"0\" name=\"Pin\">\n",
        "      <a name=\"tristate\" val=\"false\"/>\n",
        "    </tool>\n",
        "  </toolbar>\n",
        "  <circuit name=\"main\">\n",
        "    <a name=\"circuit\" val=\"main\"/>\n",
        "    <comp lib=\"0\" loc=\"(160,130)\" name=\"Pin\">\n",
        "      <a name=\"label\" val=\"test\"/>\n",
        "    </comp>\n",
        "  </circuit>\n",
        "</project>"
    );
    
    // Parse the original XML
    let cursor = Cursor::new(original_xml);
    let project = CircParser::parse(cursor).expect("Failed to parse original XML");
    
    // Verify complex structure
    assert_eq!(project.options.len(), 3);
    assert_eq!(project.mappings.len(), 2);
    assert_eq!(project.toolbar.len(), 3);
    
    // Serialize back to XML
    let mut serialized = Vec::new();
    CircSerializer::serialize(&project, &mut serialized).expect("Failed to serialize");
    let serialized_xml = String::from_utf8(serialized).expect("Invalid UTF-8");
    
    // Parse the serialized XML to verify it's valid
    let cursor2 = Cursor::new(&serialized_xml);
    let project2 = CircParser::parse(cursor2).expect("Failed to parse serialized XML");
    
    // Verify round-trip preserves complex structure
    assert_eq!(project2.options.len(), project.options.len());
    assert_eq!(project2.mappings.len(), project.mappings.len());
    assert_eq!(project2.toolbar.len(), project.toolbar.len());
}

#[test]
fn test_real_circ_file_roundtrip() {
    // Test with the sample file we created
    let sample_file_content = std::fs::read_to_string("/tmp/logisim_samples/simple_and.circ")
        .expect("Failed to read sample file");
    
    // Parse the sample file
    let cursor = Cursor::new(&sample_file_content);
    let project = CircParser::parse(cursor).expect("Failed to parse sample file");
    
    // Serialize it back
    let mut serialized = Vec::new();
    CircSerializer::serialize(&project, &mut serialized).expect("Failed to serialize");
    let serialized_xml = String::from_utf8(serialized).expect("Invalid UTF-8");
    
    // Parse the serialized version
    let cursor2 = Cursor::new(&serialized_xml);
    let project2 = CircParser::parse(cursor2).expect("Failed to parse serialized XML");
    
    // Verify key structure is preserved
    assert_eq!(project2.source, project.source);
    assert_eq!(project2.version, project.version);
    assert_eq!(project2.main_circuit, project.main_circuit);
    assert_eq!(project2.circuits.len(), project.circuits.len());
}

#[test]
fn test_parse_error_handling() {
    // Test invalid XML
    let invalid_xml = "not xml at all";
    let cursor = Cursor::new(invalid_xml);
    assert!(CircParser::parse(cursor).is_err());
    
    // Test missing required attributes
    let missing_attr_xml = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n",
        "<project>\n", // Missing source and version
        "</project>"
    );
    let cursor = Cursor::new(missing_attr_xml);
    let result = CircParser::parse(cursor);
    // Should still work but with empty source/version
    assert!(result.is_ok());
    
    // Test malformed coordinates
    let bad_coords_xml = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n",
        "<project source=\"3.8.0\" version=\"1.0\">\n",
        "  <circuit name=\"main\">\n",
        "    <wire from=\"invalid\" to=\"(220,130)\"/>\n",
        "  </circuit>\n",
        "</project>"
    );
    let cursor = Cursor::new(bad_coords_xml);
    assert!(CircParser::parse(cursor).is_err());
}

#[test]
fn test_debug_parsing() {
    let xml = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n",
        "<project source=\"3.8.0\" version=\"1.0\">\n",
        "  <lib desc=\"#Wiring\" name=\"0\"/>\n",
        "  <main name=\"main\"/>\n",
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
    
    println!("Source: {}", project.source);
    println!("Version: {}", project.version);
    println!("Main circuit: {}", project.main_circuit);
    println!("Libraries: {}", project.libraries.len());
    println!("Circuits: {}", project.circuits.len());
    
    if let Some(circuit) = project.circuits.first() {
        println!("Circuit name: {}", circuit.name);
        println!("Wires: {}", circuit.wires.len());
        println!("Components: {}", circuit.components.len());
        
        for wire in &circuit.wires {
            println!("Wire: {:?} to {:?}", wire.from, wire.to);
        }
        
        for comp in &circuit.components {
            println!("Component: {} at {:?} from lib {}", comp.name, comp.location, comp.lib);
            for (k, v) in &comp.attributes {
                println!("  {}: {}", k, v);
            }
        }
    }
    
    // The test should pass if parsing works
    assert_eq!(project.source, "3.8.0");
    assert_eq!(project.main_circuit, "main");
    assert_eq!(project.libraries.len(), 1);
    assert_eq!(project.circuits.len(), 1);
    
    let circuit = &project.circuits[0];
    assert_eq!(circuit.wires.len(), 1);
    assert_eq!(circuit.components.len(), 1);
}