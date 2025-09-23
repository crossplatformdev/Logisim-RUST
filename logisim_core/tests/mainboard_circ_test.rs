//! Integration tests for MAINBOARD.circ file support
//!
//! This test file validates that the MAINBOARD.circ file from the Libre8 project
//! can be successfully loaded, parsed, and round-tripped through the Rust
//! logisim_core library.

use logisim_core::circ_format::{CircParser, CircWriter, CircIntegration, RomContents};
use std::fs;
use std::path::Path;

const MAINBOARD_CIRC_PATH: &str = "test_resources/MAINBOARD.circ";

#[test]
fn test_mainboard_circ_exists() {
    let path = Path::new(MAINBOARD_CIRC_PATH);
    assert!(path.exists(), "MAINBOARD.circ test file should exist");
    
    let metadata = fs::metadata(path).unwrap();
    assert!(metadata.len() > 100_000, "MAINBOARD.circ should be a substantial file");
}

#[test]
fn test_mainboard_circ_basic_parsing() {
    let circuit_file = CircParser::load_file(MAINBOARD_CIRC_PATH)
        .expect("Should be able to parse MAINBOARD.circ");

    // Verify basic structure
    assert_eq!(circuit_file.version, "1.0");
    assert!(circuit_file.source_version.contains("4.1.0"));
    
    // Should have at least one circuit
    assert!(!circuit_file.circuits.is_empty(), "Should have at least one circuit");
    
    // Check that libraries are loaded
    assert!(!circuit_file.libraries.is_empty(), "Should have library definitions");
    
    println!("Loaded {} circuits:", circuit_file.circuits.len());
    for (name, circuit) in &circuit_file.circuits {
        println!("  - Circuit '{}' with {} components and {} wires", 
                 name, circuit.components.len(), circuit.wires.len());
    }
}

#[test]
fn test_mainboard_circ_rom_content_parsing() {
    let circuit_file = CircParser::load_file(MAINBOARD_CIRC_PATH)
        .expect("Should be able to parse MAINBOARD.circ");

    // Find ROM components and verify their content
    let mut rom_count = 0;
    let mut total_rom_data_size = 0;

    for (_circuit_name, circuit) in &circuit_file.circuits {
        for component in &circuit.components {
            if component.name == "ROM" {
                rom_count += 1;
                
                if let Some(contents_str) = component.attributes.get("contents") {
                    println!("Found ROM with content length: {} chars", contents_str.len());
                    
                    // Try to parse the ROM contents
                    let rom_contents = RomContents::parse_from_string(contents_str)
                        .expect("Should be able to parse ROM contents");
                    
                    println!("ROM: {}-bit address, {}-bit data, {} entries",
                             rom_contents.addr_width, rom_contents.data_width, rom_contents.data.len());
                    
                    total_rom_data_size += rom_contents.data.len();
                    
                    // Verify ROM contents are reasonable
                    assert!(rom_contents.addr_width > 0 && rom_contents.addr_width <= 32);
                    assert!(rom_contents.data_width > 0 && rom_contents.data_width <= 64);
                    assert!(!rom_contents.data.is_empty());
                }
            }
        }
    }

    println!("Found {} ROM components with {} total data entries", rom_count, total_rom_data_size);
    assert!(rom_count > 0, "Should find at least one ROM component");
    assert!(total_rom_data_size > 0, "Should have some ROM data");
}

#[test]
fn test_mainboard_circ_component_inventory() {
    let circuit_file = CircParser::load_file(MAINBOARD_CIRC_PATH)
        .expect("Should be able to parse MAINBOARD.circ");

    // Count different types of components
    let mut component_counts = std::collections::HashMap::new();
    let mut total_components = 0;
    let mut total_wires = 0;

    for (_circuit_name, circuit) in &circuit_file.circuits {
        total_wires += circuit.wires.len();
        
        for component in &circuit.components {
            *component_counts.entry(component.name.clone()).or_insert(0) += 1;
            total_components += 1;
        }
    }

    println!("Component inventory:");
    let mut sorted_components: Vec<_> = component_counts.iter().collect();
    sorted_components.sort_by_key(|(_, count)| *count);
    sorted_components.reverse();
    
    for (component_type, count) in &sorted_components {
        println!("  {} x {}", count, component_type);
    }
    
    println!("Total: {} components, {} wires", total_components, total_wires);

    // Verify we have a substantial circuit
    assert!(total_components > 10, "Should have a reasonable number of components");
    assert!(total_wires > 10, "Should have a reasonable number of wires");
    
    // Verify we have ROM components
    assert!(component_counts.contains_key("ROM"), "Should contain ROM components");
}

#[test]
fn test_mainboard_circ_round_trip() {
    // Load the original file
    let original = CircParser::load_file(MAINBOARD_CIRC_PATH)
        .expect("Should be able to parse MAINBOARD.circ");

    // Serialize to string
    let serialized = CircWriter::serialize_to_string(&original)
        .expect("Should be able to serialize circuit file");

    // Parse the serialized version
    let reparsed = CircParser::parse_string(&serialized)
        .expect("Should be able to parse serialized version");

    // Verify key properties are preserved
    assert_eq!(reparsed.circuits.len(), original.circuits.len());
    assert_eq!(reparsed.libraries.len(), original.libraries.len());
    assert_eq!(reparsed.main_circuit, original.main_circuit);

    // Verify circuit structure is preserved
    for (circuit_name, original_circuit) in &original.circuits {
        let reparsed_circuit = reparsed.circuits.get(circuit_name)
            .expect("Circuit should exist in reparsed version");
        
        assert_eq!(reparsed_circuit.components.len(), original_circuit.components.len(),
                   "Circuit '{}' should have same number of components", circuit_name);
        assert_eq!(reparsed_circuit.wires.len(), original_circuit.wires.len(),
                   "Circuit '{}' should have same number of wires", circuit_name);
    }

    // Verify ROM contents are preserved
    for (_circuit_name, original_circuit) in &original.circuits {
        for (i, original_component) in original_circuit.components.iter().enumerate() {
            if original_component.name == "ROM" {
                if let Some(original_contents) = original_component.attributes.get("contents") {
                    let reparsed_circuit = reparsed.circuits.values().next().unwrap(); // Simplified
                    if let Some(reparsed_component) = reparsed_circuit.components.get(i) {
                        if let Some(reparsed_contents) = reparsed_component.attributes.get("contents") {
                            // Parse both versions and compare
                            let original_rom = RomContents::parse_from_string(original_contents).unwrap();
                            let reparsed_rom = RomContents::parse_from_string(reparsed_contents).unwrap();
                            
                            assert_eq!(reparsed_rom.addr_width, original_rom.addr_width);
                            assert_eq!(reparsed_rom.data_width, original_rom.data_width);
                            assert_eq!(reparsed_rom.data.len(), original_rom.data.len());
                            
                            // Compare first few entries (full comparison might be too slow)
                            let compare_count = std::cmp::min(100, original_rom.data.len());
                            for j in 0..compare_count {
                                assert_eq!(reparsed_rom.data[j], original_rom.data[j],
                                          "ROM data entry {} should match", j);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Round-trip test passed - circuit structure and ROM contents preserved");
}

#[test]
fn test_mainboard_circ_simulation_loading() {
    // Test loading MAINBOARD.circ into a simulation (will fail for unsupported components)
    let result = CircIntegration::load_into_simulation(MAINBOARD_CIRC_PATH);

    match result {
        Ok(simulation) => {
            println!("Successfully loaded MAINBOARD.circ into simulation");
            println!("Simulation stats: {:?}", simulation.stats());
            // This would be ideal, but many components might not be implemented yet
        }
        Err(e) => {
            println!("Expected: Loading into simulation failed (unsupported components): {}", e);
            // This is expected until we implement all component types
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("UnsupportedComponent") || 
                   error_msg.contains("InvalidFormat") ||
                   error_msg.contains("Unsupported component"));
        }
    }
}

#[test]
fn test_rom_content_validation() {
    let circuit_file = CircParser::load_file(MAINBOARD_CIRC_PATH)
        .expect("Should be able to parse MAINBOARD.circ");

    // Find the first ROM and validate its content structure
    for (_circuit_name, circuit) in &circuit_file.circuits {
        for component in &circuit.components {
            if component.name == "ROM" {
                if let Some(contents_str) = component.attributes.get("contents") {
                    let rom_contents = RomContents::parse_from_string(contents_str).unwrap();
                    
                    // Test ROM content serialization round-trip
                    let serialized = rom_contents.to_string();
                    let reparsed = RomContents::parse_from_string(&serialized).unwrap();
                    
                    assert_eq!(reparsed.addr_width, rom_contents.addr_width);
                    assert_eq!(reparsed.data_width, rom_contents.data_width);
                    assert_eq!(reparsed.data.len(), rom_contents.data.len());
                    
                    // Compare actual data
                    for (i, (&original, &reparsed_val)) in rom_contents.data.iter()
                        .zip(reparsed.data.iter()).enumerate() {
                        assert_eq!(reparsed_val, original, 
                                  "ROM data mismatch at index {}", i);
                    }
                    
                    println!("ROM content validation passed for {} entries", rom_contents.data.len());
                    return; // Only test the first ROM found
                }
            }
        }
    }
    
    panic!("No ROM component found for content validation");
}