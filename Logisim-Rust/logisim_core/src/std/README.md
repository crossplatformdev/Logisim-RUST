# Standard Component Libraries

This module contains the Rust port of Logisim's standard component libraries, originally from Java package `com.cburch.logisim.std`.

## Organization

- **`base/`** - Basic utilities and text components (BaseLibrary)
- **`gates/`** - Logic gates and related components (GatesLibrary)

## Usage

```rust
use logisim_core::std::{
    gates::{AndGate, OrGate, NotGate, GatesLibrary},
    base::{Text, BaseLibrary}
};

// Create gates directly
let and_gate = AndGate::new(ComponentId(1));
let or_gate = OrGate::new(ComponentId(2));

// Or use the factory pattern
let nand_gate = GatesLibrary::create_nand_gate(ComponentId(3));
let text = BaseLibrary::create_text(ComponentId(4));

// Create by name
let xor_gate = GatesLibrary::create_gate_by_name("XOR Gate", ComponentId(5));
```

## Migration Status

**Completed**: 7/28 gates + 3/3 base components
- Core logic gates: AND, OR, NOT, NAND, NOR, XOR, XNOR
- Text annotation system with styling
- Factory pattern libraries

**In Progress**: 5/28 gates have placeholder implementations
- Buffer, Controlled Buffer, Parity gates, PLA

**TODO**: 16/28 specialized components and supporting infrastructure

See `STD_GATES_MIGRATION.md` in the repository root for detailed status.

## Key Features

- **Type Safety**: Leverages Rust's type system for compile-time safety
- **Performance**: Optimized signal propagation with short-circuit logic  
- **Compatibility**: Maintains behavioral equivalence with Java implementation
- **Extensibility**: Factory pattern allows easy addition of new components
- **Testing**: Comprehensive test coverage for all implemented components

## Architecture Notes

Each component implements the `Component` and `Propagator` traits, providing:
- Pin management and signal routing
- State updates and propagation logic
- Reset and initialization capabilities
- Serialization support for circuit persistence

The factory libraries (`GatesLibrary`, `BaseLibrary`) provide centralized component creation and management, supporting both direct instantiation and creation by name for dynamic loading scenarios.