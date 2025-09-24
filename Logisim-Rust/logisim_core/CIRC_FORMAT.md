# Logisim-Evolution .circ Format Support

This document describes the implementation of Logisim-Evolution .circ file format support in the Rust logisim_core library, including the mapping between XML structures and Rust types.

## Overview

The .circ format is an XML-based circuit description format used by Logisim-Evolution. This implementation provides:

- **Complete parsing** of .circ files into Rust data structures
- **Round-trip serialization** back to .circ format
- **ROM content support** with run-length encoding
- **Integration with simulation engine** (where supported components exist)

## Format Structure

### XML Schema Hierarchy

```xml
<project source="version" version="1.0">
  <!-- Library definitions -->
  <lib desc="#Wiring" name="0">
    <tool name="Pin">
      <a name="width" val="8"/>
    </tool>
  </lib>
  
  <!-- Main circuit reference -->
  <main name="main"/>
  
  <!-- Project options -->
  <options>
    <a name="gateUndefined" val="ignore"/>
  </options>
  
  <!-- Circuit definitions -->
  <circuit name="main">
    <!-- Circuit attributes -->
    <a name="circuit" val="main"/>
    
    <!-- Component instances -->
    <comp lib="4" loc="(1030,190)" name="ROM">
      <a name="addrWidth" val="20"/>
      <a name="contents">addr/data: 20 35
2500 4*a2000 10004400</a>
    </comp>
    
    <!-- Wire connections -->
    <wire from="(80,90)" to="(100,90)"/>
  </circuit>
  
  <!-- VHDL content (optional) -->
  <vhdl name="example">
    -- VHDL code here
  </vhdl>
</project>
```

## Rust Type Mapping

### Core Structures

| XML Element | Rust Type | Description |
|-------------|-----------|-------------|
| `<project>` | `CircuitFile` | Root container for entire .circ file |
| `<lib>` | `LibraryConfig` | Library definition with tools |
| `<tool>` | `ToolConfig` | Tool configuration within library |
| `<circuit>` | `CircuitDefinition` | Circuit definition with components |
| `<comp>` | `ComponentInstance` | Component instance with location |
| `<wire>` | `WireConnection` | Wire connection between points |
| `<vhdl>` | `VhdlContent` | VHDL code block |

### Detailed Type Definitions

```rust
/// Complete .circ file representation
pub struct CircuitFile {
    pub source_version: String,           // Logisim version
    pub version: String,                  // File format version
    pub libraries: Vec<LibraryConfig>,    // Library configurations
    pub main_circuit: Option<String>,     // Main circuit name
    pub circuits: HashMap<String, CircuitDefinition>, // All circuits
    pub vhdl_contents: Vec<VhdlContent>,  // VHDL blocks
    pub options: ProjectOptions,          // Project settings
}

/// Circuit definition with components and wires
pub struct CircuitDefinition {
    pub name: String,                     // Circuit name
    pub components: Vec<ComponentInstance>, // Component instances
    pub wires: Vec<WireConnection>,       // Wire connections
    pub appearance: Option<CircuitAppearance>, // Custom appearance
    pub attributes: HashMap<String, String>, // Circuit attributes
}

/// Component instance in circuit
pub struct ComponentInstance {
    pub library: Option<String>,         // Library reference ("0", "1", etc.)
    pub name: String,                    // Component type ("AND Gate", "ROM")
    pub location: (i32, i32),           // Position (x, y)
    pub attributes: HashMap<String, String>, // Component attributes
    pub facing: Option<String>,          // Orientation
}
```

## ROM Content Format

### Storage Format

ROM contents are stored as text within component attributes using a special format:

```
addr/data: <address_width> <data_width>
<hex_value> <hex_value> <run_length_value> ...
```

### Run-Length Encoding

The format supports run-length encoding for repeated values:
- `4*a2000` means repeat the value `a2000` four times
- Regular hex values are stored as-is

### Rust Implementation

```rust
pub struct RomContents {
    pub addr_width: u32,    // Address bus width (e.g., 20)
    pub data_width: u32,    // Data bus width (e.g., 35)
    pub data: Vec<u64>,     // Actual data values
}

impl RomContents {
    /// Parse from Logisim format string with run-length encoding support
    pub fn parse_from_string(contents: &str) -> CircResult<Self>
    
    /// Serialize back to Logisim format
    pub fn to_string(&self) -> String
}
```

## Component Support

### Currently Supported Components

| Component | Library | Rust Implementation | Status |
|-----------|---------|-------------------|---------|
| AND Gate | 1 | `AndGate` | ✅ Full |
| Clocked Latch | 4 | `ClockedLatch` | ✅ Full |
| ROM | 4 | Planned | 🔄 Parsing only |

### Component Attributes

Common attributes parsed from `<a>` elements:

| Attribute | Description | Example |
|-----------|-------------|---------|
| `addrWidth` | Address width for memory | `val="20"` |
| `dataWidth` | Data width for memory | `val="8"` |
| `contents` | ROM/RAM content data | Text content |
| `facing` | Component orientation | `val="west"` |
| `size` | Visual size | `val="30"` |

## Usage Examples

### Loading a .circ File

```rust
use logisim_core::circ_format::{CircParser, CircIntegration};

// Parse .circ file
let circuit_file = CircParser::load_file("design.circ")?;

// Load into simulation (for supported components)
let simulation = CircIntegration::load_into_simulation("design.circ")?;
```

### Accessing ROM Contents

```rust
// Find ROM components and extract their data
for (circuit_name, circuit) in &circuit_file.circuits {
    for component in &circuit.components {
        if component.name == "ROM" {
            if let Some(contents_str) = component.attributes.get("contents") {
                let rom_data = RomContents::parse_from_string(contents_str)?;
                println!("ROM: {}x{} with {} entries", 
                         rom_data.addr_width, rom_data.data_width, rom_data.data.len());
            }
        }
    }
}
```

### Round-trip Serialization

```rust
use logisim_core::circ_format::{CircParser, CircWriter};

// Load, modify, and save
let mut circuit_file = CircParser::load_file("input.circ")?;
// ... modify circuit_file ...
CircWriter::save_file(&circuit_file, "output.circ")?;
```

## Testing

### Test Coverage

The implementation includes comprehensive tests:

- **Unit tests** for ROM parsing, location parsing, basic XML parsing
- **Integration tests** for MAINBOARD.circ (132k lines, complex ROM data)
- **Round-trip tests** ensuring data preservation
- **Component inventory** validation

### MAINBOARD.circ Test Results

```
Component inventory:
  533 x comp
  89 x Tunnel
  67 x Splitter
  52 x Constant
  29 x Pin
  22 x Probe
  17 x Text
  10 x AND Gate
  8 x OR Gate
  4 x ROM
  ...
Total: 1000+ components, 2000+ wires
```

### Running Tests

```bash
# Run all tests
cargo test

# Run only .circ format tests
cargo test circ_format

# Run MAINBOARD.circ integration tests
cargo test mainboard_circ_test
```

## Limitations and Extensions

### Current Limitations

1. **Component Support**: Only AND Gate and Clocked Latch fully implemented for simulation
2. **Appearance Parsing**: Custom circuit appearances not fully parsed
3. **FPGA Mappings**: Board mapping information parsed but not used
4. **Complex Attributes**: Some component-specific attributes may need special handling

### Planned Extensions

1. **More Components**: Implement OR, NOT, XOR gates, multiplexers, etc.
2. **Memory Components**: Full RAM and ROM simulation support
3. **Sub-circuits**: Support for hierarchical circuit definitions
4. **Wire Routing**: Advanced wire routing and connection analysis
5. **Timing Analysis**: Support for propagation delays and timing constraints

### Adding New Components

To add support for a new component:

1. Implement the `Component` trait:
```rust
pub struct XorGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl Component for XorGate {
    fn update(&mut self, time: Timestamp) -> ComponentUpdateResult { ... }
    // ... other trait methods
}
```

2. Add to the component factory in `CircIntegration::build_circuit_in_simulation`:
```rust
match comp_instance.name.as_str() {
    "XOR Gate" => Box::new(XorGate::new(component_id)),
    // ... existing cases
}
```

## Error Handling

The implementation uses a comprehensive error system:

```rust
#[derive(Error, Debug)]
pub enum CircFormatError {
    #[error("XML parsing error: {0}")]
    XmlError(#[from] roxmltree::Error),
    
    #[error("Unsupported component: {0}")]
    UnsupportedComponent(String),
    
    #[error("ROM parsing error: {0}")]
    RomParsingError(String),
    
    // ... other error types
}
```

This ensures proper error propagation and debugging information for all parsing and serialization operations.