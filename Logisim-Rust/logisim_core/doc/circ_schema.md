# .circ File Format Schema and Rust Mapping

This document describes the Logisim-Evolution .circ file format and how it maps to Rust data structures in the `logisim_core` crate.

## Overview

The .circ format is an XML-based format used by Logisim-Evolution to store digital circuit designs. It contains circuit definitions, component placements, wire connections, library references, and project configuration.

For complete usage examples and API documentation, see the generated docs with `cargo doc`.

## Key Features Supported

- Complete XML parsing and serialization 
- Round-trip preservation of circuit data
- Support for all major .circ elements: circuits, wires, components, libraries
- Comprehensive error handling
- Integration-ready data structures for simulation kernel

## Usage

```rust
use logisim_core::{CircParser, CircSerializer};
use std::fs::File;
use std::io::BufReader;

// Parse a .circ file
let file = File::open("circuit.circ")?;  
let reader = BufReader::new(file);
let project = CircParser::parse(reader)?;

// Serialize back to .circ format
let output = File::create("output.circ")?;
CircSerializer::serialize(&project, output)?;
```

See the inline documentation and tests for detailed API information.
