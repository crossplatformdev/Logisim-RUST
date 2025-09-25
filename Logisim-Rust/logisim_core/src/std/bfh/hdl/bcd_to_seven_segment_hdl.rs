/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL Generator for BCD to Seven Segment Display
//!
//! This module provides HDL (VHDL/Verilog) code generation for the BCD to
//! Seven Segment Display decoder component to support FPGA synthesis and deployment.

use crate::data::AttributeSet;
use crate::hdl::{HdlGenerator, HdlLanguage, HdlPort, HdlPortType, HdlWire};
use super::super::BcdToSevenSegmentDisplay;
use std::collections::HashMap;

/// HDL Generator for BCD to Seven Segment Display
///
/// Generates VHDL or Verilog code for the BCD to Seven Segment Display decoder.
/// The generated code implements a lookup table for converting BCD digits to
/// 7-segment display patterns.
#[derive(Debug, Clone)]
pub struct BcdToSevenSegmentHdlGenerator;

impl BcdToSevenSegmentHdlGenerator {
    /// Creates a new HDL generator instance
    pub fn new() -> Self {
        Self
    }

    /// Generate VHDL code for the BCD to 7-segment decoder
    fn generate_vhdl(&self, _attrs: &AttributeSet) -> String {
        let mut vhdl = String::new();
        
        // Entity declaration
        vhdl.push_str("entity BCD_to_7_Segment_decoder is\n");
        vhdl.push_str("  port (\n");
        vhdl.push_str("    bcdIn : in std_logic_vector(3 downto 0);\n");
        vhdl.push_str("    segmentA : out std_logic;\n");
        vhdl.push_str("    segmentB : out std_logic;\n");
        vhdl.push_str("    segmentC : out std_logic;\n");
        vhdl.push_str("    segmentD : out std_logic;\n");
        vhdl.push_str("    segmentE : out std_logic;\n");
        vhdl.push_str("    segmentF : out std_logic;\n");
        vhdl.push_str("    segmentG : out std_logic\n");
        vhdl.push_str("  );\n");
        vhdl.push_str("end BCD_to_7_Segment_decoder;\n\n");

        // Architecture declaration
        vhdl.push_str("architecture behavioral of BCD_to_7_Segment_decoder is\n");
        vhdl.push_str("  signal s_outputValue : std_logic_vector(6 downto 0);\n");
        vhdl.push_str("begin\n\n");
        
        // Output assignments
        vhdl.push_str("  segmentA <= s_outputValue(0);\n");
        vhdl.push_str("  segmentB <= s_outputValue(1);\n");
        vhdl.push_str("  segmentC <= s_outputValue(2);\n");
        vhdl.push_str("  segmentD <= s_outputValue(3);\n");
        vhdl.push_str("  segmentE <= s_outputValue(4);\n");
        vhdl.push_str("  segmentF <= s_outputValue(5);\n");
        vhdl.push_str("  segmentG <= s_outputValue(6);\n\n");
        
        // Decoder process
        vhdl.push_str("  makeSegs : process (bcdIn) is\n");
        vhdl.push_str("  begin\n");
        vhdl.push_str("    case bcdIn is\n");
        vhdl.push_str("      when \"0000\" => s_outputValue <= \"0111111\"; -- 0\n");
        vhdl.push_str("      when \"0001\" => s_outputValue <= \"0000110\"; -- 1\n");
        vhdl.push_str("      when \"0010\" => s_outputValue <= \"1011011\"; -- 2\n");
        vhdl.push_str("      when \"0011\" => s_outputValue <= \"1001111\"; -- 3\n");
        vhdl.push_str("      when \"0100\" => s_outputValue <= \"1100110\"; -- 4\n");
        vhdl.push_str("      when \"0101\" => s_outputValue <= \"1101101\"; -- 5\n");
        vhdl.push_str("      when \"0110\" => s_outputValue <= \"1111101\"; -- 6\n");
        vhdl.push_str("      when \"0111\" => s_outputValue <= \"0000111\"; -- 7\n");
        vhdl.push_str("      when \"1000\" => s_outputValue <= \"1111111\"; -- 8\n");
        vhdl.push_str("      when \"1001\" => s_outputValue <= \"1101111\"; -- 9\n");
        vhdl.push_str("      when others => s_outputValue <= \"0000000\"; -- blank\n");
        vhdl.push_str("    end case;\n");
        vhdl.push_str("  end process makeSegs;\n\n");
        
        vhdl.push_str("end behavioral;\n");

        vhdl
    }

    /// Generate Verilog code for the BCD to 7-segment decoder
    fn generate_verilog(&self, _attrs: &AttributeSet) -> String {
        let mut verilog = String::new();
        
        // Module declaration
        verilog.push_str("module BCD_to_7_Segment_decoder (\n");
        verilog.push_str("  input [3:0] bcdIn,\n");
        verilog.push_str("  output reg segmentA,\n");
        verilog.push_str("  output reg segmentB,\n");
        verilog.push_str("  output reg segmentC,\n");
        verilog.push_str("  output reg segmentD,\n");
        verilog.push_str("  output reg segmentE,\n");
        verilog.push_str("  output reg segmentF,\n");
        verilog.push_str("  output reg segmentG\n");
        verilog.push_str(");\n\n");
        
        // Internal signal
        verilog.push_str("  reg [6:0] s_outputValue;\n\n");
        
        // Output assignments
        verilog.push_str("  assign segmentA = s_outputValue[0];\n");
        verilog.push_str("  assign segmentB = s_outputValue[1];\n");
        verilog.push_str("  assign segmentC = s_outputValue[2];\n");
        verilog.push_str("  assign segmentD = s_outputValue[3];\n");
        verilog.push_str("  assign segmentE = s_outputValue[4];\n");
        verilog.push_str("  assign segmentF = s_outputValue[5];\n");
        verilog.push_str("  assign segmentG = s_outputValue[6];\n\n");
        
        // Decoder always block
        verilog.push_str("  always @(*) begin\n");
        verilog.push_str("    case (bcdIn)\n");
        verilog.push_str("      4'b0000: s_outputValue = 7'b0111111; // 0\n");
        verilog.push_str("      4'b0001: s_outputValue = 7'b0000110; // 1\n");
        verilog.push_str("      4'b0010: s_outputValue = 7'b1011011; // 2\n");
        verilog.push_str("      4'b0011: s_outputValue = 7'b1001111; // 3\n");
        verilog.push_str("      4'b0100: s_outputValue = 7'b1100110; // 4\n");
        verilog.push_str("      4'b0101: s_outputValue = 7'b1101101; // 5\n");
        verilog.push_str("      4'b0110: s_outputValue = 7'b1111101; // 6\n");
        verilog.push_str("      4'b0111: s_outputValue = 7'b0000111; // 7\n");
        verilog.push_str("      4'b1000: s_outputValue = 7'b1111111; // 8\n");
        verilog.push_str("      4'b1001: s_outputValue = 7'b1101111; // 9\n");
        verilog.push_str("      default: s_outputValue = 7'b0000000; // blank\n");
        verilog.push_str("    endcase\n");
        verilog.push_str("  end\n\n");
        
        verilog.push_str("endmodule\n");

        verilog
    }
}

impl Default for BcdToSevenSegmentHdlGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl HdlGenerator for BcdToSevenSegmentHdlGenerator {
    fn get_module_name(&self, _attrs: &AttributeSet) -> String {
        "BCD_to_7_Segment_decoder".to_string()
    }

    fn generate_hdl(&self, language: HdlLanguage, attrs: &AttributeSet) -> String {
        match language {
            HdlLanguage::VHDL => self.generate_vhdl(attrs),
            HdlLanguage::Verilog => self.generate_verilog(attrs),
        }
    }

    fn get_ports(&self, _attrs: &AttributeSet) -> Vec<HdlPort> {
        vec![
            HdlPort::new("bcdIn".to_string(), HdlPortType::Input, 4),
            HdlPort::new("segmentA".to_string(), HdlPortType::Output, 1),
            HdlPort::new("segmentB".to_string(), HdlPortType::Output, 1),
            HdlPort::new("segmentC".to_string(), HdlPortType::Output, 1),
            HdlPort::new("segmentD".to_string(), HdlPortType::Output, 1),
            HdlPort::new("segmentE".to_string(), HdlPortType::Output, 1),
            HdlPort::new("segmentF".to_string(), HdlPortType::Output, 1),
            HdlPort::new("segmentG".to_string(), HdlPortType::Output, 1),
        ]
    }

    fn get_wires(&self, _attrs: &AttributeSet) -> Vec<HdlWire> {
        vec![
            HdlWire::new("s_outputValue".to_string(), 7),
        ]
    }

    fn get_parameters(&self) -> HashMap<String, String> {
        HashMap::new() // No parameters needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        assert!(format!("{:?}", generator).contains("BcdToSevenSegmentHdlGenerator"));
    }

    #[test]
    fn test_module_name() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        let attrs = AttributeSet::new();
        let module_name = generator.get_module_name(&attrs);
        assert_eq!(module_name, "BCD_to_7_Segment_decoder");
    }

    #[test]
    fn test_vhdl_generation() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        let attrs = AttributeSet::new();
        
        let vhdl = generator.generate_vhdl(&attrs);
        assert!(vhdl.contains("entity BCD_to_7_Segment_decoder"));
        assert!(vhdl.contains("bcdIn : in std_logic_vector(3 downto 0)"));
        assert!(vhdl.contains("segmentA : out std_logic"));
        assert!(vhdl.contains("when \"0000\" => s_outputValue <= \"0111111\""));
        assert!(vhdl.contains("when \"1001\" => s_outputValue <= \"1101111\""));
        assert!(vhdl.contains("when others => s_outputValue <= \"0000000\""));
    }

    #[test]
    fn test_verilog_generation() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        let attrs = AttributeSet::new();
        
        let verilog = generator.generate_verilog(&attrs);
        assert!(verilog.contains("module BCD_to_7_Segment_decoder"));
        assert!(verilog.contains("input [3:0] bcdIn"));
        assert!(verilog.contains("output reg segmentA"));
        assert!(verilog.contains("4'b0000: s_outputValue = 7'b0111111"));
        assert!(verilog.contains("4'b1001: s_outputValue = 7'b1101111"));
        assert!(verilog.contains("default: s_outputValue = 7'b0000000"));
    }

    #[test]
    fn test_ports_generation() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        let attrs = AttributeSet::new();
        let ports = generator.get_ports(&attrs);
        
        assert_eq!(ports.len(), 8); // 1 input + 7 outputs
        assert_eq!(ports[0].name, "bcdIn");
        assert_eq!(ports[0].port_type, HdlPortType::Input);
        assert_eq!(ports[0].width, 4);
        
        let segment_names = ["segmentA", "segmentB", "segmentC", "segmentD", 
                           "segmentE", "segmentF", "segmentG"];
        for (i, name) in segment_names.iter().enumerate() {
            assert_eq!(ports[i + 1].name, *name);
            assert_eq!(ports[i + 1].port_type, HdlPortType::Output);
            assert_eq!(ports[i + 1].width, 1);
        }
    }

    #[test]
    fn test_wires_generation() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        let attrs = AttributeSet::new();
        let wires = generator.get_wires(&attrs);
        
        assert_eq!(wires.len(), 1);
        assert_eq!(wires[0].name, "s_outputValue");
        assert_eq!(wires[0].width, 7);
    }

    #[test]
    fn test_parameters() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        let params = generator.get_parameters();
        assert!(params.is_empty()); // No parameters needed
    }

    #[test]
    fn test_segment_patterns_in_hdl() {
        let generator = BcdToSevenSegmentHdlGenerator::new();
        let attrs = AttributeSet::new();
        let vhdl = generator.generate_vhdl(&attrs);
        
        // Verify specific patterns match our component logic
        assert!(vhdl.contains("\"0000\" => s_outputValue <= \"0111111\"")); // 0 -> 0b0111111
        assert!(vhdl.contains("\"0001\" => s_outputValue <= \"0000110\"")); // 1 -> 0b0000110
        assert!(vhdl.contains("\"0010\" => s_outputValue <= \"1011011\"")); // 2 -> 0b1011011
        assert!(vhdl.contains("\"1001\" => s_outputValue <= \"1101111\"")); // 9 -> 0b1101111
    }
}