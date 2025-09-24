# Logisim Core

Core simulation kernel for the Logisim-RUST digital logic simulator.

## Features

This crate provides the foundational types and algorithms for simulating digital circuits:

- **Event-driven simulation**: Efficient discrete event simulation engine
- **Signal propagation**: Multi-bit buses and logical value types (High, Low, Unknown, Error)
- **Component modeling**: Trait-based architecture for digital logic components
- **Netlist management**: Connection and wiring infrastructure
- **Built-in components**: AND gates, clocked latches, and more

## Architecture

The simulation kernel is built around several key concepts:

- **Signals**: Represent logical values and multi-bit buses
- **Components**: Digital logic elements (gates, latches, etc.)
- **Events**: Time-ordered simulation events that drive the simulation
- **Netlist**: The network of connected components and signals
- **Simulation**: The main simulation engine that processes events

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
logisim_core = "0.1.0"
```

## Example

```rust
use logisim_core::*;
use logisim_core::component::AndGate;
use logisim_core::signal::{Value, BusWidth};
use logisim_core::simulation::Simulation;

// Create a simulation
let mut sim = Simulation::new();

// Add an AND gate
let gate = Box::new(AndGate::new(ComponentId(1)));
let gate_id = sim.add_component(gate);

// Create nodes for connections
let input_a = sim.netlist_mut().create_named_node(BusWidth(1), "A".to_string());
let input_b = sim.netlist_mut().create_named_node(BusWidth(1), "B".to_string());
let output = sim.netlist_mut().create_named_node(BusWidth(1), "Y".to_string());

// Connect the gate
sim.netlist_mut().connect(gate_id, "A".to_string(), input_a)?;
sim.netlist_mut().connect(gate_id, "B".to_string(), input_b)?;
sim.netlist_mut().connect(gate_id, "Y".to_string(), output)?;

// Set up initial conditions and run simulation
sim.reset();
sim.schedule_signal_change(Timestamp(10), input_a, Signal::new_single(Value::High), ComponentId(0));
sim.schedule_signal_change(Timestamp(10), input_b, Signal::new_single(Value::High), ComponentId(0));
sim.run()?;
```

## Testing

Run the test suite:

```bash
cargo test
```

This includes:
- Unit tests for all core components
- Integration tests for simulation scenarios
- Documentation tests for code examples

## Status

This is the Phase 1 implementation of the Logisim-RUST simulation kernel, providing:

✅ Core types (NodeId, NetId, ComponentId, BusWidth, Signal, Timestamp, Bus)  
✅ Event queue and simulation step logic  
✅ Component traits (Component, Pin, Propagator)  
✅ Basic components (2-input AND gate, clocked latch)  
✅ Comprehensive unit tests  
✅ Documentation and examples  

## License

This project is licensed under the GPL-3.0-or-later license.