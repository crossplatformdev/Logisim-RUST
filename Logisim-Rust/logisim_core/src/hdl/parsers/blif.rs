//! BLIF Parser
//!
//! BLIF (Berkeley Logic Interchange Format) parsing functionality
//! ported from Java BlifParser. Provides parsing for BLIF circuit descriptions.

use crate::hdl::model::PortDescription;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// BLIF parsing errors
#[derive(Error, Debug)]
pub enum BlifParseError {
    #[error("Invalid BLIF syntax: {0}")]
    InvalidSyntax(String),
    #[error("Missing model declaration")]
    MissingModel,
    #[error("Missing inputs declaration")]
    MissingInputs,
    #[error("Missing outputs declaration")]
    MissingOutputs,
    #[error("Parse error on line {line}: {message}")]
    LineError { line: usize, message: String },
    #[error("Unknown directive: {0}")]
    UnknownDirective(String),
}

/// BLIF parser result type
pub type BlifResult<T> = Result<T, BlifParseError>;

/// BLIF logic gate representation
#[derive(Debug, Clone, PartialEq)]
pub enum BlifGate {
    /// Names directive (inputs and outputs)
    Names {
        inputs: Vec<String>,
        output: String,
        truth_table: Vec<String>,
    },
    /// Latch directive
    Latch {
        input: String,
        output: String,
        clock: Option<String>,
        initial_value: Option<String>,
    },
    /// Subcircuit directive
    Subcircuit {
        model_name: String,
        connections: HashMap<String, String>,
    },
}

/// BLIF Parser
///
/// Parses BLIF (Berkeley Logic Interchange Format) files.
/// Equivalent to Java BlifParser class.
pub struct BlifParser {
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    source: String,
    model_name: Option<String>,
    gates: Vec<BlifGate>,
    input_names: HashSet<String>,
    output_names: HashSet<String>,
}

impl BlifParser {
    /// Create a new BLIF parser with the given source code
    pub fn new(source: String) -> BlifResult<Self> {
        let mut parser = Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            source,
            model_name: None,
            gates: Vec::new(),
            input_names: HashSet::new(),
            output_names: HashSet::new(),
        };

        parser.parse()?;
        Ok(parser)
    }

    /// Get the model name
    pub fn get_name(&self) -> Option<&str> {
        self.model_name.as_deref()
    }

    /// Get input ports
    pub fn get_inputs(&self) -> &[PortDescription] {
        &self.inputs
    }

    /// Get output ports
    pub fn get_outputs(&self) -> &[PortDescription] {
        &self.outputs
    }

    /// Get parsed gates
    pub fn get_gates(&self) -> &[BlifGate] {
        &self.gates
    }

    /// Parse the BLIF source code
    fn parse(&mut self) -> BlifResult<()> {
        let lines: Vec<String> = self.source.lines().map(|s| s.to_string()).collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                i += 1;
                continue;
            }

            // Handle line continuations
            let mut full_line = line.to_string();
            let mut j = i + 1;
            while j < lines.len() && full_line.ends_with('\\') {
                full_line.pop(); // Remove backslash
                full_line.push(' ');
                full_line.push_str(lines[j].trim());
                j += 1;
            }
            i = j;

            self.parse_line(&full_line, i)?;
        }

        self.build_port_descriptions()?;
        Ok(())
    }

    /// Parse a single logical line
    fn parse_line(&mut self, line: &str, line_num: usize) -> BlifResult<()> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            return Ok(());
        }

        match tokens[0] {
            ".model" => self.parse_model(&tokens, line_num)?,
            ".inputs" => self.parse_inputs(&tokens, line_num)?,
            ".outputs" => self.parse_outputs(&tokens, line_num)?,
            ".names" => self.parse_names(&tokens, line_num)?,
            ".latch" => self.parse_latch(&tokens, line_num)?,
            ".subckt" => self.parse_subcircuit(&tokens, line_num)?,
            ".end" => {} // End of model, nothing to do
            directive if directive.starts_with('.') => {
                return Err(BlifParseError::UnknownDirective(directive.to_string()));
            }
            _ => {
                // This might be a truth table entry for a .names directive
                // We'll handle this when we parse .names
            }
        }

        Ok(())
    }

    /// Parse .model directive
    fn parse_model(&mut self, tokens: &[&str], line_num: usize) -> BlifResult<()> {
        if tokens.len() < 2 {
            return Err(BlifParseError::LineError {
                line: line_num,
                message: "Missing model name".to_string(),
            });
        }

        self.model_name = Some(tokens[1].to_string());
        Ok(())
    }

    /// Parse .inputs directive
    fn parse_inputs(&mut self, tokens: &[&str], _line_num: usize) -> BlifResult<()> {
        for &input_name in &tokens[1..] {
            self.input_names.insert(input_name.to_string());
        }
        Ok(())
    }

    /// Parse .outputs directive
    fn parse_outputs(&mut self, tokens: &[&str], _line_num: usize) -> BlifResult<()> {
        for &output_name in &tokens[1..] {
            self.output_names.insert(output_name.to_string());
        }
        Ok(())
    }

    /// Parse .names directive
    fn parse_names(&mut self, tokens: &[&str], line_num: usize) -> BlifResult<()> {
        if tokens.len() < 2 {
            return Err(BlifParseError::LineError {
                line: line_num,
                message: "Missing signal names in .names directive".to_string(),
            });
        }

        let inputs: Vec<String> = tokens[1..tokens.len() - 1]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let output = tokens[tokens.len() - 1].to_string();

        // For now, we'll create the gate without the truth table
        // In a full implementation, we'd need to parse the following lines for the truth table
        let gate = BlifGate::Names {
            inputs,
            output,
            truth_table: Vec::new(), // TODO: Parse truth table from following lines
        };

        self.gates.push(gate);
        Ok(())
    }

    /// Parse .latch directive
    fn parse_latch(&mut self, tokens: &[&str], line_num: usize) -> BlifResult<()> {
        if tokens.len() < 3 {
            return Err(BlifParseError::LineError {
                line: line_num,
                message: "Missing input/output names in .latch directive".to_string(),
            });
        }

        let input = tokens[1].to_string();
        let output = tokens[2].to_string();
        let clock = if tokens.len() > 3 {
            Some(tokens[3].to_string())
        } else {
            None
        };
        let initial_value = if tokens.len() > 4 {
            Some(tokens[4].to_string())
        } else {
            None
        };

        let gate = BlifGate::Latch {
            input,
            output,
            clock,
            initial_value,
        };

        self.gates.push(gate);
        Ok(())
    }

    /// Parse .subckt directive
    fn parse_subcircuit(&mut self, tokens: &[&str], line_num: usize) -> BlifResult<()> {
        if tokens.len() < 2 {
            return Err(BlifParseError::LineError {
                line: line_num,
                message: "Missing model name in .subckt directive".to_string(),
            });
        }

        let model_name = tokens[1].to_string();
        let mut connections = HashMap::new();

        // Parse formal=actual connections
        for &token in &tokens[2..] {
            if let Some(eq_pos) = token.find('=') {
                let formal = token[..eq_pos].to_string();
                let actual = token[eq_pos + 1..].to_string();
                connections.insert(formal, actual);
            }
        }

        let gate = BlifGate::Subcircuit {
            model_name,
            connections,
        };

        self.gates.push(gate);
        Ok(())
    }

    /// Build port descriptions from parsed input/output names
    fn build_port_descriptions(&mut self) -> BlifResult<()> {
        for input_name in &self.input_names {
            self.inputs.push(PortDescription::new(
                input_name.clone(),
                "logic".to_string(),
                1, // BLIF signals are typically single-bit
            ));
        }

        for output_name in &self.output_names {
            self.outputs.push(PortDescription::new(
                output_name.clone(),
                "logic".to_string(),
                1, // BLIF signals are typically single-bit
            ));
        }

        Ok(())
    }
}

/// BLIF content component
///
/// Manages BLIF content similar to VhdlContentComponent.
/// Equivalent to Java BlifContentComponent.
#[derive(Debug, Clone)]
pub struct BlifContentComponent {
    content: String,
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    name: String,
    gates: Vec<BlifGate>,
}

impl BlifContentComponent {
    /// Create a new BlifContentComponent
    pub fn create() -> Self {
        Self {
            content: Self::load_template(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            name: "circuit".to_string(),
            gates: Vec::new(),
        }
    }

    /// Load the BLIF template
    fn load_template() -> String {
        r#".model circuit
.inputs 
.outputs 
# Add your logic here
.end"#
            .to_string()
    }

    /// Update content from BLIF source
    pub fn set_content(&mut self, blif_source: String) -> BlifResult<()> {
        let parser = BlifParser::new(blif_source.clone())?;

        self.content = blif_source;
        self.inputs = parser.get_inputs().to_vec();
        self.outputs = parser.get_outputs().to_vec();
        self.gates = parser.get_gates().to_vec();

        if let Some(name) = parser.get_name() {
            self.name = name.to_string();
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

    /// Get gates
    pub fn get_gates(&self) -> &[BlifGate] {
        &self.gates
    }

    /// Compare with another BLIF model
    pub fn compare(&self, other: &BlifContentComponent) -> bool {
        self.content == other.content
            && self.name == other.name
            && self.inputs == other.inputs
            && self.outputs == other.outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_BLIF: &str = r#"
# Simple BLIF example
.model and_gate
.inputs a b
.outputs y

.names a b y
11 1

.end
"#;

    const COMPLEX_BLIF: &str = r#"
.model counter
.inputs clk reset enable
.outputs q0 q1 q2

.latch n_q0 q0 clk 0
.latch n_q1 q1 clk 0
.latch n_q2 q2 clk 0

.names reset enable q0 q1 q2 n_q0
0 1 0 0 0 1
0 1 1 0 0 0
0 1 0 1 0 1
0 1 1 1 0 0
0 1 0 0 1 1
0 1 1 0 1 0
0 1 0 1 1 1
0 1 1 1 1 0

.end
"#;

    #[test]
    fn test_blif_parser_creation() {
        let parser = BlifParser::new(SAMPLE_BLIF.to_string());
        assert!(parser.is_ok());
    }

    #[test]
    fn test_model_name_extraction() {
        let parser = BlifParser::new(SAMPLE_BLIF.to_string()).unwrap();
        assert_eq!(parser.get_name(), Some("and_gate"));
    }

    #[test]
    fn test_input_output_extraction() {
        let parser = BlifParser::new(SAMPLE_BLIF.to_string()).unwrap();

        let inputs = parser.get_inputs();
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0].get_name(), "a");
        assert_eq!(inputs[1].get_name(), "b");

        let outputs = parser.get_outputs();
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].get_name(), "y");
    }

    #[test]
    fn test_gates_extraction() {
        let parser = BlifParser::new(SAMPLE_BLIF.to_string()).unwrap();
        let gates = parser.get_gates();
        assert_eq!(gates.len(), 1);

        if let BlifGate::Names { inputs, output, .. } = &gates[0] {
            assert_eq!(inputs.len(), 2);
            assert_eq!(inputs[0], "a");
            assert_eq!(inputs[1], "b");
            assert_eq!(output, "y");
        } else {
            panic!("Expected Names gate");
        }
    }

    #[test]
    fn test_complex_blif_parsing() {
        let parser = BlifParser::new(COMPLEX_BLIF.to_string()).unwrap();

        assert_eq!(parser.get_name(), Some("counter"));
        assert_eq!(parser.get_inputs().len(), 3);
        assert_eq!(parser.get_outputs().len(), 3);

        let gates = parser.get_gates();
        assert!(gates.len() > 1);

        // Check for latch gates
        let latch_count = gates
            .iter()
            .filter(|g| matches!(g, BlifGate::Latch { .. }))
            .count();
        assert!(latch_count > 0);
    }

    #[test]
    fn test_blif_content_component() {
        let mut component = BlifContentComponent::create();
        assert!(component.set_content(SAMPLE_BLIF.to_string()).is_ok());

        assert_eq!(component.get_name(), "and_gate");
        assert_eq!(component.get_inputs().len(), 2);
        assert_eq!(component.get_outputs().len(), 1);
    }

    #[test]
    fn test_error_handling() {
        let invalid_blif = ".unknown_directive test";
        let result = BlifParser::new(invalid_blif.to_string());
        assert!(result.is_err());

        if let Err(BlifParseError::UnknownDirective(directive)) = result {
            assert_eq!(directive, ".unknown_directive");
        } else {
            panic!("Expected UnknownDirective error");
        }
    }
}
