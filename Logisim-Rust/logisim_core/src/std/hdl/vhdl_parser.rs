//! VHDL Parser
//!
//! VHDL parsing functionality ported from Java VhdlParser.
//! Provides lightweight VHDL parsing using regular expressions to extract
//! critical information such as entity ports and architecture details.

use crate::hdl::model::PortDescription;
use regex::Regex;
use thiserror::Error;

/// VHDL parsing errors
#[derive(Error, Debug)]
pub enum VhdlParseError {
    #[error("Invalid VHDL syntax: {0}")]
    InvalidSyntax(String),
    #[error("Missing entity declaration")]
    MissingEntity,
    #[error("Missing architecture declaration")]
    MissingArchitecture,
    #[error("Port parsing error: {0}")]
    PortError(String),
    #[error("Regex compilation error: {0}")]
    RegexError(#[from] regex::Error),
}

/// VHDL parser result type
pub type VhdlResult<T> = Result<T, VhdlParseError>;

/// VHDL Parser
/// 
/// Lightly parses VHDL using regexes to extract critical information.
/// Equivalent to Java VhdlParser class.
pub struct VhdlParser {
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    source: String,
    name: Option<String>,
    libraries: Option<String>,
    architecture: Option<String>,
}

impl VhdlParser {
    // Pattern constants equivalent to Java
    const LINE_PATTERN: &'static str = r":\s*(\w+)\s+std_logic";
    const VECTOR_PATTERN: &'static str = r":\s*(\w+)\s+std_logic_vector\s*\(\s*(\d+)\s+downto\s+(\d+)\s*\)";
    
    /// Create a new VHDL parser with the given source code
    pub fn new(source: String) -> VhdlResult<Self> {
        let mut parser = Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            source,
            name: None,
            libraries: None,
            architecture: None,
        };
        
        parser.parse()?;
        Ok(parser)
    }

    /// Get the architecture code
    pub fn get_architecture(&self) -> Option<&str> {
        self.architecture.as_deref()
    }

    /// Get the entity name
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get the libraries section
    pub fn get_libraries(&self) -> Option<&str> {
        self.libraries.as_deref()
    }

    /// Get input ports
    pub fn get_inputs(&self) -> &[PortDescription] {
        &self.inputs
    }

    /// Get output ports
    pub fn get_outputs(&self) -> &[PortDescription] {
        &self.outputs
    }

    /// Parse the VHDL source code
    fn parse(&mut self) -> VhdlResult<()> {
        self.extract_entity_name()?;
        self.extract_libraries()?;
        self.extract_architecture()?;
        self.extract_ports()?;
        Ok(())
    }

    /// Extract entity name from VHDL source
    fn extract_entity_name(&mut self) -> VhdlResult<()> {
        let entity_regex = Regex::new(r"(?i)entity\s+(\w+)\s+is")?;
        
        if let Some(captures) = entity_regex.captures(&self.source) {
            if let Some(name_match) = captures.get(1) {
                self.name = Some(name_match.as_str().to_string());
            }
        }
        
        Ok(())
    }

    /// Extract libraries section
    fn extract_libraries(&mut self) -> VhdlResult<()> {
        // Find everything before the entity declaration
        let entity_regex = Regex::new(r"(?i)entity\s+\w+\s+is")?;
        
        if let Some(entity_match) = entity_regex.find(&self.source) {
            let libraries_section = &self.source[..entity_match.start()].trim();
            if !libraries_section.is_empty() {
                self.libraries = Some(libraries_section.to_string());
            }
        }
        
        Ok(())
    }

    /// Extract architecture section
    fn extract_architecture(&mut self) -> VhdlResult<()> {
        let arch_regex = Regex::new(r"(?i)architecture\s+(\w+)\s+of\s+(\w+)\s+is(.*?)(?:end\s+(?:architecture\s+)?\w*\s*;|\z)")?;
        
        if let Some(captures) = arch_regex.captures(&self.source) {
            if let Some(arch_body) = captures.get(3) {
                self.architecture = Some(arch_body.as_str().trim().to_string());
            }
        }
        
        Ok(())
    }

    /// Extract port declarations
    fn extract_ports(&mut self) -> VhdlResult<()> {
        // Find the port section
        let port_regex = Regex::new(r"(?i)port\s*\((.*?)\)\s*;")?;
        
        if let Some(captures) = port_regex.captures(self.source.clone().as_str()) {
            if let Some(port_section) = captures.get(1) {
                let port_section_str = port_section.as_str().to_string();
                self.parse_port_section(&port_section_str)?;
            }
        }
        
        Ok(())
    }

    /// Parse the port section to extract individual ports
    fn parse_port_section(&mut self, port_section: &str) -> VhdlResult<()> {
        // Split by semicolons to get individual port declarations
        for port_decl in port_section.split(';') {
            let port_decl = port_decl.trim();
            if port_decl.is_empty() {
                continue;
            }
            
            self.parse_single_port(port_decl)?;
        }
        
        Ok(())
    }

    /// Parse a single port declaration
    fn parse_single_port(&mut self, port_decl: &str) -> VhdlResult<()> {
        // Check for direction (in, out, inout)
        let in_regex = Regex::new(r"(?i)(\w+)\s*:\s*in\s+(.+)")?;
        let out_regex = Regex::new(r"(?i)(\w+)\s*:\s*out\s+(.+)")?;
        let inout_regex = Regex::new(r"(?i)(\w+)\s*:\s*inout\s+(.+)")?;
        
        if let Some(captures) = in_regex.captures(port_decl) {
            let port_name = captures.get(1).unwrap().as_str().to_string();
            let port_type = captures.get(2).unwrap().as_str().trim().to_string();
            let width = self.extract_width(&port_type)?;
            
            self.inputs.push(PortDescription::new(port_name, port_type, width));
        } else if let Some(captures) = out_regex.captures(port_decl) {
            let port_name = captures.get(1).unwrap().as_str().to_string();
            let port_type = captures.get(2).unwrap().as_str().trim().to_string();
            let width = self.extract_width(&port_type)?;
            
            self.outputs.push(PortDescription::new(port_name, port_type, width));
        } else if let Some(captures) = inout_regex.captures(port_decl) {
            let port_name = captures.get(1).unwrap().as_str().to_string();
            let port_type = captures.get(2).unwrap().as_str().trim().to_string();
            let width = self.extract_width(&port_type)?;
            
            // Inout ports are treated as both input and output
            self.inputs.push(PortDescription::new(port_name.clone(), port_type.clone(), width));
            self.outputs.push(PortDescription::new(port_name, port_type, width));
        }
        
        Ok(())
    }

    /// Extract bit width from port type
    fn extract_width(&self, port_type: &str) -> VhdlResult<i32> {
        if port_type.to_lowercase().contains("std_logic_vector") {
            let vector_regex = Regex::new(Self::VECTOR_PATTERN)?;
            if let Some(captures) = vector_regex.captures(port_type) {
                if let (Some(high), Some(low)) = (captures.get(2), captures.get(3)) {
                    let high_val: i32 = high.as_str().parse()
                        .map_err(|_| VhdlParseError::PortError("Invalid high index".to_string()))?;
                    let low_val: i32 = low.as_str().parse()
                        .map_err(|_| VhdlParseError::PortError("Invalid low index".to_string()))?;
                    return Ok(high_val - low_val + 1);
                }
            }
            // Default vector width if parsing fails
            Ok(8)
        } else if port_type.to_lowercase().contains("std_logic") {
            Ok(1)
        } else {
            // Unknown type, assume single bit
            Ok(1)
        }
    }

    /// Get line end index (utility function equivalent to Java version)
    fn get_line_end_index(&self, input: &str, from: usize) -> usize {
        if let Some(newline_pos) = input[from..].find('\n') {
            from + newline_pos
        } else {
            input.len()
        }
    }
}

/// VHDL content component
/// 
/// Connects the VHDL interface parser with other code.
/// The parsed VHDL interface is used for the ports of a VHDL entity component.
/// Equivalent to Java VhdlContentComponent.
#[derive(Debug, Clone)]
pub struct VhdlContentComponent {
    content: String,
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    name: String,
    libraries: String,
    architecture: String,
}

impl VhdlContentComponent {
    /// Create a new VhdlContentComponent
    pub fn create() -> Self {
        Self {
            content: Self::load_template(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            name: "entity_name".to_string(),
            libraries: String::new(),
            architecture: String::new(),
        }
    }

    /// Load the VHDL template
    fn load_template() -> String {
        // This would load from a resource file in the Java version
        // For now, provide a basic template
        r#"library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity entity_name is
    Port ( 
        -- Add your ports here
    );
end entity_name;

architecture Behavioral of entity_name is
begin
    -- Add your architecture here
end Behavioral;"#.to_string()
    }

    /// Update content from VHDL source
    pub fn set_content(&mut self, vhdl_source: String) -> VhdlResult<()> {
        let parser = VhdlParser::new(vhdl_source.clone())?;
        
        self.content = vhdl_source;
        self.inputs = parser.get_inputs().to_vec();
        self.outputs = parser.get_outputs().to_vec();
        
        if let Some(name) = parser.get_name() {
            self.name = name.to_string();
        }
        
        if let Some(libraries) = parser.get_libraries() {
            self.libraries = libraries.to_string();
        }
        
        if let Some(architecture) = parser.get_architecture() {
            self.architecture = architecture.to_string();
        }
        
        Ok(())
    }

    /// Get content
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Get inputs
    pub fn get_inputs(&self) -> &[PortDescription] {
        &self.inputs
    }

    /// Get outputs
    pub fn get_outputs(&self) -> &[PortDescription] {
        &self.outputs
    }

    /// Get name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get libraries
    pub fn get_libraries(&self) -> &str {
        &self.libraries
    }

    /// Get architecture
    pub fn get_architecture(&self) -> &str {
        &self.architecture
    }

    /// Compare with another VHDL model
    pub fn compare(&self, other: &VhdlContentComponent) -> bool {
        self.content == other.content &&
        self.name == other.name &&
        self.inputs == other.inputs &&
        self.outputs == other.outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_VHDL: &str = r#"
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity counter is
    Port ( 
        clk : in STD_LOGIC;
        reset : in STD_LOGIC;
        enable : in STD_LOGIC;
        count : out STD_LOGIC_VECTOR (7 downto 0)
    );
end counter;

architecture Behavioral of counter is
    signal count_reg : STD_LOGIC_VECTOR (7 downto 0) := (others => '0');
begin
    process(clk, reset)
    begin
        if reset = '1' then
            count_reg <= (others => '0');
        elsif rising_edge(clk) then
            if enable = '1' then
                count_reg <= count_reg + 1;
            end if;
        end if;
    end process;
    
    count <= count_reg;
end Behavioral;"#;

    #[test]
    fn test_vhdl_parser_creation() {
        let parser = VhdlParser::new(SAMPLE_VHDL.to_string());
        assert!(parser.is_ok());
    }

    #[test]
    fn test_entity_name_extraction() {
        let parser = VhdlParser::new(SAMPLE_VHDL.to_string()).unwrap();
        assert_eq!(parser.get_name(), Some("counter"));
    }

    #[test]
    fn test_libraries_extraction() {
        let parser = VhdlParser::new(SAMPLE_VHDL.to_string()).unwrap();
        let libraries = parser.get_libraries().unwrap();
        assert!(libraries.contains("library IEEE"));
        assert!(libraries.contains("use IEEE.STD_LOGIC_1164.ALL"));
    }

    #[test]
    fn test_port_extraction() {
        let parser = VhdlParser::new(SAMPLE_VHDL.to_string()).unwrap();
        
        let inputs = parser.get_inputs();
        assert_eq!(inputs.len(), 3);
        assert_eq!(inputs[0].get_name(), "clk");
        assert_eq!(inputs[0].get_width_int(), 1);
        assert_eq!(inputs[1].get_name(), "reset");
        assert_eq!(inputs[2].get_name(), "enable");
        
        let outputs = parser.get_outputs();
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].get_name(), "count");
        assert_eq!(outputs[0].get_width_int(), 8);
    }

    #[test]
    fn test_architecture_extraction() {
        let parser = VhdlParser::new(SAMPLE_VHDL.to_string()).unwrap();
        let architecture = parser.get_architecture().unwrap();
        assert!(architecture.contains("count_reg"));
        assert!(architecture.contains("process"));
    }

    #[test]
    fn test_vhdl_content_component() {
        let mut component = VhdlContentComponent::create();
        assert!(component.set_content(SAMPLE_VHDL.to_string()).is_ok());
        
        assert_eq!(component.get_name(), "counter");
        assert_eq!(component.get_inputs().len(), 3);
        assert_eq!(component.get_outputs().len(), 1);
    }

    #[test]
    fn test_vector_width_parsing() {
        let vhdl = r#"
entity test is
    Port ( 
        data8 : in STD_LOGIC_VECTOR (7 downto 0);
        data16 : out STD_LOGIC_VECTOR (15 downto 0);
        data32 : inout STD_LOGIC_VECTOR (31 downto 0)
    );
end test;"#;

        let parser = VhdlParser::new(vhdl.to_string()).unwrap();
        
        let inputs = parser.get_inputs();
        assert_eq!(inputs.len(), 2); // data8 and data32 (inout)
        assert_eq!(inputs[0].get_width_int(), 8);
        
        let outputs = parser.get_outputs();
        assert_eq!(outputs.len(), 2); // data16 and data32 (inout)
        assert_eq!(outputs[0].get_width_int(), 16);
        assert_eq!(outputs[1].get_width_int(), 32);
    }
}