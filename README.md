# Logisim-RUST

[![CI](https://github.com/crossplatformdev/Logisim-RUST/actions/workflows/ci.yml/badge.svg)](https://github.com/crossplatformdev/Logisim-RUST/actions/workflows/ci.yml)

A complete, production-grade **Rust rewrite** of [Logisim-Evolution](https://github.com/logisim-evolution/logisim-evolution) — the digital logic circuit simulator for education and design.

---

## Features

- **Complete circuit simulation** — combinational and sequential logic
- **Full component library** — all Logisim-Evolution standard components (gates, flip-flops, RAM, arithmetic units, I/O, plexers, …)
- **`.circ` file format** — read and write Logisim-Evolution `.circ` files (fully compatible with the original Java application)
- **Interactive GUI** — circuit editor with component palette, wire drawing, zoom/pan, simulation controls
- **Command-line interface** — headless simulation, truth-table generation, project info
- **Multi-valued logic** — 5-state logic (0/1/X/E/Z) matching the original

---

## Quick start

### Prerequisites

- Rust stable ≥ 1.70 (`rustup update stable`)
- **Linux GUI**: `sudo apt install libxkbcommon-dev libwayland-dev libgl1-mesa-dev`

### Build & run

```bash
# Clone
git clone https://github.com/crossplatformdev/Logisim-RUST
cd Logisim-RUST

# Build everything
cargo build --release

# Launch the GUI
cargo run -p logisim-gui --release

# CLI: print project info
cargo run -p logisim-cli --release -- info examples/full_adder.circ

# CLI: generate truth table
cargo run -p logisim-cli --release -- truth-table examples/full_adder.circ

# CLI: simulate 20 steps
cargo run -p logisim-cli --release -- simulate --steps 20 examples/full_adder.circ

# Run all tests
cargo test -p logisim-core -p logisim-file -p logisim-cli
```

---

## Project structure

| Crate | Description |
|-------|-------------|
| `logisim-core` | Circuit model, component library, simulation engine |
| `logisim-file` | `.circ` XML parser & writer |
| `logisim-gui` | egui/eframe interactive GUI |
| `logisim-cli` | Command-line interface |

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for full architecture documentation.

---

## GUI overview

| Tool | Shortcut | Description |
|------|----------|-------------|
| Select | Esc | Select / move components |
| Wire | — | Draw wire segments |
| Poke | — | Toggle input pins during simulation |
| Place | Click palette | Place a component |
| Run/Stop | Space | Start or stop continuous simulation |
| Step | — | Single clock tick |
| Zoom In/Out | Scroll | Zoom the canvas |
| Pan | Middle-drag | Pan the canvas |

**File operations:** Ctrl+N (New), Ctrl+O (Open), Ctrl+S (Save)

---

## CLI usage

```
logisim-cli <COMMAND> [OPTIONS] <FILE>

Commands:
  simulate      Run the simulator and print output pin values each step
  truth-table   Generate a truth table for a combinational circuit
  info          Display project structure information

Options:
  --circuit <name>    Select circuit (default: main/first)
  --steps <n>         Simulation steps (default: 10)
  --terse             Minimal output
```

---

## Compatibility

Files produced by Logisim-RUST are compatible with Logisim-Evolution 3.x.
Files produced by Logisim-Evolution can be opened by Logisim-RUST.

---

## License

GPL-3.0-or-later — matching the original Logisim-Evolution license.

---

## Acknowledgements

Based on [Logisim-Evolution](https://github.com/logisim-evolution/logisim-evolution)
by the Logisim-Evolution contributors, licensed under GPL-3.0.
