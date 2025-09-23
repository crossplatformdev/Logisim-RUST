//! Example usage of .circ format support in logisim_core
//!
//! This example demonstrates how to:
//! 1. Load a .circ file 
//! 2. Analyze its contents
//! 3. Extract ROM data
//! 4. Round-trip serialize back to .circ format
//! 
//! Run with: cargo run --example circ_example

use logisim_core::circ_format::{CircParser, CircWriter, CircIntegration, RomContents};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Logisim .circ Format Example ===\n");

    // Create a sample .circ content for demonstration
    let sample_circ = create_sample_circ();
    
    // 1. Parse the .circ content
    println!("1. Parsing .circ content...");
    let circuit_file = CircParser::parse_string(&sample_circ)?;
    
    println!("   Source: {}", circuit_file.source_version);
    println!("   Version: {}", circuit_file.version);
    println!("   Libraries: {}", circuit_file.libraries.len());
    println!("   Circuits: {}", circuit_file.circuits.len());
    
    // 2. Analyze circuit contents
    println!("\n2. Analyzing circuit contents...");
    for (circuit_name, circuit) in &circuit_file.circuits {
        println!("   Circuit '{}': {} components, {} wires", 
                 circuit_name, circuit.components.len(), circuit.wires.len());
        
        // Count component types
        let mut component_counts = HashMap::new();
        for component in &circuit.components {
            *component_counts.entry(component.name.clone()).or_insert(0) += 1;
        }
        
        for (comp_type, count) in component_counts {
            println!("     - {} x {}", count, comp_type);
        }
    }
    
    // 3. Extract and analyze ROM contents
    println!("\n3. Analyzing ROM contents...");
    let mut found_rom = false;
    for (_circuit_name, circuit) in &circuit_file.circuits {
        for component in &circuit.components {
            if component.name == "ROM" {
                found_rom = true;
                println!("   Found ROM at location ({}, {})", 
                         component.location.0, component.location.1);
                
                // Extract ROM attributes
                for (attr_name, attr_value) in &component.attributes {
                    println!("     {}: {}", attr_name, 
                             if attr_value.len() > 50 {
                                 format!("{}... ({} chars)", &attr_value[..50], attr_value.len())
                             } else {
                                 attr_value.clone()
                             });
                }
                
                // Parse ROM contents if available
                if let Some(contents_str) = component.attributes.get("contents") {
                    match RomContents::parse_from_string(contents_str) {
                        Ok(rom_data) => {
                            println!("     ROM Data: {}-bit address, {}-bit data, {} entries",
                                     rom_data.addr_width, rom_data.data_width, rom_data.data.len());
                            
                            // Show first few entries
                            println!("     First entries: {:?}", 
                                     &rom_data.data[..std::cmp::min(8, rom_data.data.len())]);
                        }
                        Err(e) => {
                            println!("     ROM parsing error: {}", e);
                        }
                    }
                }
            }
        }
    }
    
    if !found_rom {
        println!("   No ROM components found");
    }
    
    // 4. Round-trip serialization test
    println!("\n4. Testing round-trip serialization...");
    let serialized = CircWriter::serialize_to_string(&circuit_file)?;
    println!("   Serialized XML length: {} characters", serialized.len());
    
    // Parse the serialized version to verify it's valid
    let reparsed = CircParser::parse_string(&serialized)?;
    println!("   Reparsed successfully: {} circuits preserved", reparsed.circuits.len());
    
    // Compare key properties
    let circuits_match = reparsed.circuits.len() == circuit_file.circuits.len();
    let libraries_match = reparsed.libraries.len() == circuit_file.libraries.len();
    
    println!("   Circuits match: {}", circuits_match);
    println!("   Libraries match: {}", libraries_match);
    
    // 5. Attempt to load into simulation
    println!("\n5. Testing simulation integration...");
    match CircIntegration::circuit_file_to_simulation(&circuit_file) {
        Ok(simulation) => {
            println!("   Successfully created simulation!");
            println!("   Simulation stats: {:?}", simulation.stats());
        }
        Err(e) => {
            println!("   Simulation loading failed (expected): {}", e);
            println!("   This is normal - not all components are implemented yet");
        }
    }
    
    println!("\n=== Example completed successfully! ===");
    Ok(())
}

fn create_sample_circ() -> String {
    // Create a sample .circ file content with ROM data using regular strings to avoid prefix issues
    let xml_content = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n\
<project source=\"logisim-rust-example\" version=\"1.0\">\n\
  This file demonstrates .circ format support.\n\
\n\
  <lib desc=\"#Wiring\" name=\"0\">\n\
    <tool name=\"Pin\">\n\
      <a name=\"width\" val=\"8\"/>\n\
    </tool>\n\
  </lib>\n\
  \n\
  <lib desc=\"#Gates\" name=\"1\">\n\
    <tool name=\"AND Gate\">\n\
      <a name=\"size\" val=\"30\"/>\n\
    </tool>\n\
  </lib>\n\
  \n\
  <lib desc=\"#Memory\" name=\"4\">\n\
    <tool name=\"ROM\">\n\
      <a name=\"appearance\" val=\"logisim_evolution\"/>\n\
    </tool>\n\
  </lib>\n\
  \n\
  <main name=\"example_circuit\"/>\n\
  \n\
  <options>\n\
    <a name=\"gateUndefined\" val=\"ignore\"/>\n\
    <a name=\"simlimit\" val=\"1000\"/>\n\
  </options>\n\
  \n\
  <circuit name=\"example_circuit\">\n\
    <a name=\"circuit\" val=\"example_circuit\"/>\n\
    \n\
    <!-- Input pins -->\n\
    <comp lib=\"0\" loc=\"(60,160)\" name=\"Pin\">\n\
      <a name=\"width\" val=\"8\"/>\n\
    </comp>\n\
    \n\
    <!-- AND gate -->\n\
    <comp lib=\"1\" loc=\"(140,160)\" name=\"AND Gate\">\n\
      <a name=\"size\" val=\"30\"/>\n\
    </comp>\n\
    \n\
    <!-- ROM with sample data -->\n\
    <comp lib=\"4\" loc=\"(220,160)\" name=\"ROM\">\n\
      <a name=\"addrWidth\" val=\"8\"/>\n\
      <a name=\"dataWidth\" val=\"16\"/>\n\
      <a name=\"contents\">addr/data: 8 16\n\
1234 5678 abcd 4*ef00 9999 aaaa bbbb cccc\n\
dddd eeee ffff 0123 4567 8901 2345 6789</a>\n\
    </comp>\n\
    \n\
    <!-- Output pin -->\n\
    <comp lib=\"0\" loc=\"(300,160)\" name=\"Pin\">\n\
      <a name=\"facing\" val=\"west\"/>\n\
      <a name=\"output\" val=\"true\"/>\n\
      <a name=\"width\" val=\"16\"/>\n\
    </comp>\n\
    \n\
    <!-- Wires connecting components -->\n\
    <wire from=\"(60,160)\" to=\"(100,160)\"/>\n\
    <wire from=\"(120,150)\" to=\"(120,140)\"/>\n\
    <wire from=\"(120,170)\" to=\"(120,180)\"/>\n\
    <wire from=\"(140,160)\" to=\"(180,160)\"/>\n\
    <wire from=\"(220,160)\" to=\"(300,160)\"/>\n\
  </circuit>\n\
</project>";
    
    xml_content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_circ_creation() {
        let sample = create_sample_circ();
        assert!(sample.contains("example_circuit"));
        assert!(sample.contains("ROM"));
        assert!(sample.contains("4*ef00")); // Run-length encoding test
    }

    #[test]
    fn test_sample_circ_parsing() {
        let sample = create_sample_circ();
        let circuit_file = CircParser::parse_string(&sample).unwrap();
        
        assert_eq!(circuit_file.circuits.len(), 1);
        assert!(circuit_file.circuits.contains_key("example_circuit"));
        
        let circuit = &circuit_file.circuits["example_circuit"];
        assert!(circuit.components.len() > 0);
        assert!(circuit.wires.len() > 0);
        
        // Should have ROM component
        let has_rom = circuit.components.iter().any(|c| c.name == "ROM");
        assert!(has_rom);
    }

    #[test]
    fn test_rom_data_extraction() {
        let sample = create_sample_circ();
        let circuit_file = CircParser::parse_string(&sample).unwrap();
        
        let circuit = &circuit_file.circuits["example_circuit"];
        let rom_component = circuit.components.iter().find(|c| c.name == "ROM").unwrap();
        
        let contents = rom_component.attributes.get("contents").unwrap();
        let rom_data = RomContents::parse_from_string(contents).unwrap();
        
        assert_eq!(rom_data.addr_width, 8);
        assert_eq!(rom_data.data_width, 16);
        assert!(rom_data.data.len() > 0);
        
        // Check that run-length encoding worked
        assert_eq!(rom_data.data[0], 0x1234);
        assert_eq!(rom_data.data[1], 0x5678);
        assert_eq!(rom_data.data[2], 0xabcd);
        // The next 4 values should all be 0xef00
        for i in 3..7 {
            assert_eq!(rom_data.data[i], 0xef00);
        }
    }
}