/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL Generator for Binary to BCD Converter
//!
//! This module provides HDL (VHDL/Verilog) code generation for the Binary to BCD
//! converter component to support FPGA synthesis and deployment.

use crate::data::{AttributeSet, BitWidth};
use crate::hdl::{HdlGenerator, HdlLanguage, HdlPort, HdlPortType, HdlWire};
use crate::instance::Port;
use super::super::BinToBcd;
use std::collections::HashMap;

/// HDL Generator for Binary to BCD Converter
///
/// Generates VHDL or Verilog code for the Binary to BCD converter component.
/// The generated code implements the conversion algorithm using hierarchical
/// stages for efficient synthesis.
#[derive(Debug, Clone)]
pub struct BinToBcdHdlGenerator;

impl BinToBcdHdlGenerator {
    /// Parameter name for number of input bits
    const NR_OF_BITS_STR: &'static str = "nrOfBits";
    const NR_OF_BITS_ID: i32 = -1;

    /// Creates a new HDL generator instance
    pub fn new() -> Self {
        Self
    }

    /// Calculate number of BCD output ports based on bit width
    fn calculate_bcd_ports(bit_width: u32) -> usize {
        let max_value = (1u32 << bit_width) - 1;
        let max_decimal = max_value as f64;
        (max_decimal.log10().floor() as usize) + 1
    }

    /// Calculate number of internal signal bits needed
    fn calculate_signal_bits(num_ports: usize) -> usize {
        match num_ports {
            2 => 7,
            3 => 11,
            _ => 16,
        }
    }

    /// Calculate number of internal signals needed
    fn calculate_num_signals(num_ports: usize) -> usize {
        match num_ports {
            2 => 4,
            3 => 7,
            _ => 11,
        }
    }

    /// Generate VHDL code for the binary to BCD converter
    fn generate_vhdl(&self, attrs: &AttributeSet) -> String {
        let bit_width = attrs.get_value(&BinToBcd::ATTR_BIN_BITS)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(bit_width.width() as u32);
        let signal_bits = Self::calculate_signal_bits(num_ports);
        let num_signals = Self::calculate_num_signals(num_ports);

        let mut vhdl = String::new();
        
        // Entity declaration
        vhdl.push_str(&format!(
            "entity Bin2BCD_{}_bcd_ports is\n", num_ports
        ));
        vhdl.push_str("  port (\n");
        vhdl.push_str(&format!("    binValue : in std_logic_vector({} downto 0);\n", 
                               bit_width.width() - 1));
        
        for i in 1..=num_ports {
            let power = num_ports - i;
            let decimal_value = 10_i32.pow(power as u32);
            vhdl.push_str(&format!("    bcd{} : out std_logic_vector(3 downto 0)", decimal_value));
            if i < num_ports {
                vhdl.push_str(";\n");
            } else {
                vhdl.push_str("\n");
            }
        }
        
        vhdl.push_str("  );\n");
        vhdl.push_str(&format!("end Bin2BCD_{}_bcd_ports;\n\n", num_ports));

        // Architecture declaration
        vhdl.push_str(&format!(
            "architecture behavioral of Bin2BCD_{}_bcd_ports is\n", num_ports
        ));
        
        // Signal declarations
        for i in 0..num_signals {
            vhdl.push_str(&format!(
                "  signal s_level{} : std_logic_vector({} downto 0);\n", 
                i, signal_bits - 1
            ));
        }
        
        vhdl.push_str("begin\n\n");
        
        // Implementation (simplified double-dabble algorithm)
        vhdl.push_str("  -- Binary to BCD conversion using double-dabble algorithm\n");
        vhdl.push_str("  process(binValue)\n");
        vhdl.push_str("    variable temp : unsigned(31 downto 0);\n");
        for i in 1..=num_ports {
            let power = num_ports - i;
            let decimal_value = 10_i32.pow(power as u32);
            vhdl.push_str(&format!("    variable bcd{}_var : unsigned(3 downto 0);\n", decimal_value));
        }
        vhdl.push_str("  begin\n");
        vhdl.push_str("    temp := resize(unsigned(binValue), 32);\n");
        
        // Generate BCD digits
        for i in 1..=num_ports {
            let power = num_ports - i;
            let decimal_value = 10_i32.pow(power as u32);
            if i == 1 {
                vhdl.push_str(&format!("    bcd{}_var := temp mod 10;\n", decimal_value));
                vhdl.push_str("    temp := temp / 10;\n");
            } else {
                vhdl.push_str(&format!("    bcd{}_var := temp mod 10;\n", decimal_value));
                if i < num_ports {
                    vhdl.push_str("    temp := temp / 10;\n");
                }
            }
        }
        
        // Output assignments
        for i in 1..=num_ports {
            let power = num_ports - i;
            let decimal_value = 10_i32.pow(power as u32);
            vhdl.push_str(&format!("    bcd{} <= std_logic_vector(bcd{}_var);\n", 
                                  decimal_value, decimal_value));
        }
        
        vhdl.push_str("  end process;\n\n");
        vhdl.push_str("end behavioral;\n");

        vhdl
    }

    /// Generate Verilog code for the binary to BCD converter
    fn generate_verilog(&self, attrs: &AttributeSet) -> String {
        let bit_width = attrs.get_value(&BinToBcd::ATTR_BIN_BITS)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(bit_width.width() as u32);

        let mut verilog = String::new();
        
        // Module declaration
        verilog.push_str(&format!(
            "module Bin2BCD_{}_bcd_ports (\n", num_ports
        ));
        verilog.push_str(&format!("  input [{}:0] binValue,\n", bit_width.width() - 1));
        
        for i in 1..=num_ports {
            let power = num_ports - i;
            let decimal_value = 10_i32.pow(power as u32);
            verilog.push_str(&format!("  output reg [3:0] bcd{}", decimal_value));
            if i < num_ports {
                verilog.push_str(",\n");
            } else {
                verilog.push_str("\n");
            }
        }
        
        verilog.push_str(");\n\n");
        
        // Always block for conversion
        verilog.push_str("  always @(*) begin\n");
        verilog.push_str("    reg [31:0] temp;\n");
        verilog.push_str("    temp = binValue;\n");
        
        // Generate BCD digits
        for i in 1..=num_ports {
            let power = num_ports - i;
            let decimal_value = 10_i32.pow(power as u32);
            verilog.push_str(&format!("    bcd{} = temp % 10;\n", decimal_value));
            if i < num_ports {
                verilog.push_str("    temp = temp / 10;\n");
            }
        }
        
        verilog.push_str("  end\n\n");
        verilog.push_str("endmodule\n");

        verilog
    }
}

impl Default for BinToBcdHdlGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl HdlGenerator for BinToBcdHdlGenerator {
    fn get_module_name(&self, attrs: &AttributeSet) -> String {
        let bit_width = attrs.get_value(&BinToBcd::ATTR_BIN_BITS)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(bit_width.width() as u32);
        format!("Bin2BCD_{}_bcd_ports", num_ports)
    }

    fn generate_hdl(&self, language: HdlLanguage, attrs: &AttributeSet) -> String {
        match language {
            HdlLanguage::VHDL => self.generate_vhdl(attrs),
            HdlLanguage::Verilog => self.generate_verilog(attrs),
        }
    }

    fn get_ports(&self, attrs: &AttributeSet) -> Vec<HdlPort> {
        let bit_width = attrs.get_value(&BinToBcd::ATTR_BIN_BITS)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(bit_width.width() as u32);
        
        let mut ports = Vec::new();
        
        // Input port
        ports.push(HdlPort::new(
            "binValue".to_string(),
            HdlPortType::Input,
            bit_width.width() as i32,
        ));
        
        // BCD output ports
        for i in 1..=num_ports {
            let power = num_ports - i;
            let decimal_value = 10_i32.pow(power as u32);
            ports.push(HdlPort::new(
                format!("bcd{}", decimal_value),
                HdlPortType::Output,
                4,
            ));
        }
        
        ports
    }

    fn get_wires(&self, attrs: &AttributeSet) -> Vec<HdlWire> {
        let bit_width = attrs.get_value(&BinToBcd::ATTR_BIN_BITS)
            .unwrap_or(BitWidth::create(9));
        let num_ports = Self::calculate_bcd_ports(bit_width.width() as u32);
        let signal_bits = Self::calculate_signal_bits(num_ports);
        let num_signals = Self::calculate_num_signals(num_ports);
        
        let mut wires = Vec::new();
        
        // Internal level signals for conversion stages
        for i in 0..num_signals {
            wires.push(HdlWire::new(
                format!("s_level{}", i),
                signal_bits as i32,
            ));
        }
        
        wires
    }

    fn get_parameters(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert(Self::NR_OF_BITS_STR.to_string(), "9".to_string());
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let generator = BinToBcdHdlGenerator::new();
        assert!(format!("{:?}", generator).contains("BinToBcdHdlGenerator"));
    }

    #[test]
    fn test_bcd_ports_calculation() {
        assert_eq!(BinToBcdHdlGenerator::calculate_bcd_ports(4), 2);  // max 15
        assert_eq!(BinToBcdHdlGenerator::calculate_bcd_ports(8), 3);  // max 255
        assert_eq!(BinToBcdHdlGenerator::calculate_bcd_ports(10), 4); // max 1023
    }

    #[test]
    fn test_signal_calculations() {
        assert_eq!(BinToBcdHdlGenerator::calculate_signal_bits(2), 7);
        assert_eq!(BinToBcdHdlGenerator::calculate_signal_bits(3), 11);
        assert_eq!(BinToBcdHdlGenerator::calculate_signal_bits(4), 16);

        assert_eq!(BinToBcdHdlGenerator::calculate_num_signals(2), 4);
        assert_eq!(BinToBcdHdlGenerator::calculate_num_signals(3), 7);
        assert_eq!(BinToBcdHdlGenerator::calculate_num_signals(4), 11);
    }

    #[test]
    fn test_module_name_generation() {
        let generator = BinToBcdHdlGenerator::new();
        let mut attrs = AttributeSet::new();
        attrs.set_value(BinToBcd::ATTR_BIN_BITS, BitWidth::create(8));
        
        let module_name = generator.get_module_name(&attrs);
        assert_eq!(module_name, "Bin2BCD_3_bcd_ports");
    }

    #[test]
    fn test_vhdl_generation() {
        let generator = BinToBcdHdlGenerator::new();
        let mut attrs = AttributeSet::new();
        attrs.set_value(BinToBcd::ATTR_BIN_BITS, BitWidth::create(8));
        
        let vhdl = generator.generate_vhdl(&attrs);
        assert!(vhdl.contains("entity Bin2BCD_3_bcd_ports"));
        assert!(vhdl.contains("binValue : in std_logic_vector(7 downto 0)"));
        assert!(vhdl.contains("bcd100 : out std_logic_vector(3 downto 0)"));
        assert!(vhdl.contains("bcd10 : out std_logic_vector(3 downto 0)"));
        assert!(vhdl.contains("bcd1 : out std_logic_vector(3 downto 0)"));
    }

    #[test]
    fn test_verilog_generation() {
        let generator = BinToBcdHdlGenerator::new();
        let mut attrs = AttributeSet::new();
        attrs.set_value(BinToBcd::ATTR_BIN_BITS, BitWidth::create(8));
        
        let verilog = generator.generate_verilog(&attrs);
        assert!(verilog.contains("module Bin2BCD_3_bcd_ports"));
        assert!(verilog.contains("input [7:0] binValue"));
        assert!(verilog.contains("output reg [3:0] bcd100"));
        assert!(verilog.contains("output reg [3:0] bcd10"));
        assert!(verilog.contains("output reg [3:0] bcd1"));
    }
}