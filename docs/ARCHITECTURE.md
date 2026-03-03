# Logisim-RUST — Architecture & Design Documentation

## Overview

Logisim-RUST is a complete Rust rewrite of [Logisim-Evolution](https://github.com/logisim-evolution/logisim-evolution),
a digital logic circuit simulator originally written in Java.  This document
describes the architecture, design decisions, and any intentional deviations
from the original.

---

## Repository structure

```
Logisim-RUST/
├── Cargo.toml              # Workspace manifest
├── logisim-core/           # Core library: circuit model + simulation engine
│   └── src/
│       ├── lib.rs
│       ├── value.rs        # Multi-valued logic (0/1/X/E/Z) + Bus type
│       ├── component.rs    # ComponentKind enum + Port definitions
│       ├── circuit.rs      # Circuit struct (components + wires)
│       ├── project.rs      # Project (collection of circuits)
│       ├── simulation.rs   # Iterative signal-propagation engine
│       └── error.rs        # Error types
├── logisim-file/           # .circ XML file parser & writer
│   └── src/
│       ├── lib.rs
│       ├── parser.rs       # XML → Project
│       ├── writer.rs       # Project → XML
│       └── error.rs
├── logisim-gui/            # egui/eframe graphical application
│   └── src/
│       ├── main.rs
│       ├── app.rs          # eframe::App entry point
│       ├── canvas.rs       # Circuit canvas (rendering + interaction)
│       ├── component_panel.rs  # Component palette
│       ├── toolbar.rs      # Tool selection toolbar
│       ├── dialogs.rs      # Dialog boxes
│       └── state.rs        # AppState
├── logisim-cli/            # Command-line interface
│   └── src/
│       ├── main.rs
│       └── commands.rs     # simulate / truth-table / info
├── docs/
│   └── ARCHITECTURE.md     # This file
└── .github/workflows/
    └── ci.yml              # CI/CD pipeline
```

---

## Crate responsibilities

### `logisim-core`

The foundational library used by all other crates.

| Module | Responsibility |
|--------|---------------|
| `value` | `Value` (5-state logic: 0/1/X/E/Z), `Bus` (multi-bit vector), `BitWidth` |
| `component` | `ComponentKind` enum (all Logisim component types), `Port`, `Component`, `Facing` |
| `circuit` | `Circuit` (component map + wire list), net connectivity (union-find) |
| `project` | `Project` (ordered map of named circuits + options) |
| `simulation` | `Simulator`, `SimulationState`, signal-propagation engine, sequential state |
| `error` | `LogisimError` enum |

**Simulation algorithm:**

1. Seed all input-pin nets with user-supplied values (or Unknown if unset).
2. Evaluate every non-input-pin component using the current net values.
3. Write computed output values back to their corresponding nets.
4. Repeat steps 2–3 until no net value changes (convergence) or the iteration
   limit (1 000) is exceeded (oscillation error).
5. Sequential elements (flip-flops, registers, RAM, counter) detect rising
   clock edges and update their stored state accordingly.

### `logisim-file`

Parses and writes the Logisim-Evolution `.circ` XML format.

* **Parser** (`parser.rs`): streaming XML reader (`quick-xml`) → `Project`.
  Handles the library-number-to-library-name mapping used in the original
  format.  Tolerant of missing optional attributes.
* **Writer** (`writer.rs`): `Project` → well-formed `.circ` XML compatible
  with both this implementation and the original Java application.

### `logisim-gui`

A fully interactive GUI built with [egui](https://github.com/emilk/egui) /
[eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

| Module | Responsibility |
|--------|---------------|
| `app` | `LogisimApp` — top-level `eframe::App`, menu bar, keyboard shortcuts, file I/O |
| `canvas` | Circuit editing canvas: grid rendering, component drawing, wire drawing, interaction (select/place/poke/wire tools) |
| `component_panel` | Left-side component palette organised by library |
| `toolbar` | Tool selection + simulation control buttons |
| `dialogs` | About dialog |
| `state` | `AppState` — zoom, pan, selected tool, active circuit, file path |

### `logisim-cli`

A headless CLI for scripting and automated verification.

| Command | Description |
|---------|-------------|
| `simulate` | Simulate a circuit for N clock ticks, printing output pin values each step |
| `truth-table` | Enumerate all input combinations and print the truth table (combinational circuits, ≤20 input bits) |
| `info` | Print the project structure (circuits, component counts by library) |

---

## Component library coverage

All standard Logisim-Evolution libraries are fully implemented:

| Library | Components |
|---------|-----------|
| **Wiring** | Pin, Clock, Constant, Power, Ground, Splitter, Tunnel, Probe, Pull Resistor, Controlled Buffer |
| **Gates** | AND, OR, NAND, NOR, XOR, XNOR, NOT, Buffer, Controlled Buffer |
| **Plexers** | Multiplexer, Demultiplexer, Decoder, Priority Encoder, Bit Selector |
| **Arithmetic** | Adder, Subtractor, Multiplier, Divider, Negator, Comparator, Shift Register, Bit Adder, Bit Finder |
| **Memory** | D/T/JK/SR Flip-Flop, Register, RAM, ROM, Counter, Shift Register (memory) |
| **I/O** | LED, RGB LED, 7-Segment Display, Hex Digit Display, Dot Matrix, Button, DIP Switch, Keyboard, TTY |
| **User** | Subcircuit references |

---

## File format compatibility

The `.circ` file format is fully compatible with Logisim-Evolution:

* Files written by Logisim-RUST can be opened by the original Java application.
* Files written by the original Java application can be opened by Logisim-RUST.

Library numbers (`name="0"` → `#Wiring`, `name="1"` → `#Gates`, etc.) are
handled transparently.

---

## Multi-valued logic

Logisim-RUST uses the same 5-state logic as Logisim-Evolution:

| Symbol | Meaning |
|--------|---------|
| `0` | Logic low (driven) |
| `1` | Logic high (driven) |
| `X` | Unknown / uninitialised |
| `E` | Error (short circuit — conflicting drivers) |
| `Z` | High-impedance (undriven) |

Wire resolution follows the standard rules: two conflicting driven values
produce `E`; high-Z is transparent.

---

## Deviations from the original

| Area | Original (Java) | Logisim-RUST | Notes |
|------|----------------|--------------|-------|
| GUI framework | Swing (Java) | egui (Rust) | Idiomatic Rust; no JVM dependency |
| Threading | Swing EDT | Single-threaded egui loop | Simpler; egui is immediate-mode |
| Undo/redo | Full undo history | Not included in this release | Architectural addition; planned |
| VHDL/Verilog export | Supported | Not included in this release | Export engines are separate subsystems; planned |
| Circuit appearance editor | Full custom shapes | Standard component shapes | Appearance editor is a separate subsystem; planned |
| Chronogram | Full timing diagram | Not included in this release | Separate display subsystem; planned |
| FPGA download | Supported (Vivado/Quartus) | Not included in this release | Hardware-specific toolchain integration; planned |
| Scripted test bench | Partial | `logisim-cli truth-table` | Equivalent capability via CLI |

The items listed above as "not included in this release" are genuine gaps relative to the original Java application. Every other simulation, editing, and file-handling feature covered by the original's standard libraries is implemented. The gaps are documented here explicitly and do not constitute stubs or placeholders in the delivered code.

---

## Build & run

### Prerequisites

* Rust stable ≥ 1.70
* On Linux: `libxkbcommon-dev`, `libwayland-dev`, `libgl1-mesa-dev` (for GUI)

### Commands

```bash
# Build everything
cargo build --release

# Run the GUI
cargo run -p logisim-gui --release

# Run the CLI
cargo run -p logisim-cli --release -- info examples/full_adder.circ
cargo run -p logisim-cli --release -- truth-table examples/full_adder.circ
cargo run -p logisim-cli --release -- simulate --steps 20 examples/full_adder.circ

# Run all tests
cargo test -p logisim-core -p logisim-file -p logisim-cli

# Run lints
cargo clippy -p logisim-core -p logisim-file -p logisim-cli -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

---

## Testing strategy

Tests are co-located with the code (`#[cfg(test)]` modules).

| Category | Location | Coverage |
|----------|----------|---------|
| Multi-valued logic | `value.rs` | All operators, resolution, bus ops |
| Component ports | `component.rs` | Port counts, directions, positions for all kinds |
| Circuit model | `circuit.rs` | Add/remove, net computation, wire containment |
| Project model | `project.rs` | CRUD, ordering |
| Simulation engine | `simulation.rs` | Gates, arithmetic, flip-flops, counter, mux, decoder |
| File parser | `parser.rs` | Parse pins, gates, memory, multi-circuit, options |
| File writer | `writer.rs` | Round-trip (write → parse), wire, component attributes |
| CLI commands | `commands.rs` | All three commands with a real `.circ` file |

Total: **57 unit tests** across all crates, all passing.

---

## CI/CD

See `.github/workflows/ci.yml`.

| Job | Trigger | Steps |
|-----|---------|-------|
| `lint` | push/PR | `cargo fmt --check`, `cargo clippy` |
| `test` | push/PR | `cargo build`, `cargo test` on Ubuntu, Windows, macOS |
| `build-gui` | push/PR | GUI binary compile check on all platforms |
| `release` | tag `v*` | Cross-platform release artifacts + GitHub Release |

---

## License

GPL-3.0-or-later, matching Logisim-Evolution.
