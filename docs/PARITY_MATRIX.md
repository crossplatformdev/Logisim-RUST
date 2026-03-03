# Parity Matrix — Logisim-RUST vs Logisim-Evolution v4.1.0

Upstream reference: <https://github.com/logisim-evolution/logisim-evolution/releases/tag/v4.1.0>

**Status key**

| Status | Meaning |
|--------|---------|
| `implemented` | Behavior matches upstream in normal usage |
| `tested` | At least one automated test covers the behavior |
| `asset-complete` | Required upstream assets are present and wired in |
| `compatible` | File format / protocol interoperable with upstream |
| `partial` | Core present, gaps remain (described in Notes) |
| `missing` | Not yet implemented |

---

## 1. Simulation semantics

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| Multi-value logic (0, 1, X, Z, Error) | ✅ | ✅ | ✅ | `Value` enum |
| Multi-driver conflict resolution | ✅ | ✅ | ✅ | Error on conflict, HighZ when undriven |
| Iterative propagation / convergence loop | ✅ | ✅ | ✅ | `Simulator::propagate` |
| Oscillation detection | ✅ | ✅ | ✅ | Detected after 1 000 iterations |
| Clock tick | ✅ | ✅ | ✅ | `Simulator::tick` |
| Continuous simulation (run mode) | ✅ | ✅ | ✅ | GUI run/stop |
| Single-step simulation | ✅ | ✅ | ✅ | Step button |
| Input pin forcing (Poke tool) | ✅ | ✅ | ✅ | `set_pin_value` |
| Subcircuit simulation | ✅ | ✅ | ✅ | Recursive evaluation |
| Bus width mismatch handling | ✅ | ✅ | ✅ | Per-bit `Bus` type |

---

## 2. Component library coverage

### 2a. Wiring (lib `"0"`)

| Component | Implemented | Tested | Compatible | Notes |
|-----------|-------------|--------|------------|-------|
| Pin (input / output) | ✅ | ✅ | ✅ | |
| Clock | ✅ | ✅ | ✅ | |
| Constant | ✅ | ✅ | ✅ | |
| Power / Ground | ✅ | ✅ | ✅ | |
| Splitter | ✅ | ✅ | ✅ | |
| Tunnel | ✅ | ✅ | ✅ | Label-based merging |
| Probe | ✅ | ✅ | ✅ | |
| Pull Resistor | ✅ | ✅ | ✅ | |
| Tristate Buffer | ✅ | ✅ | ✅ | Parser + writer |
| Bit Extender | ✅ | ✅ | ✅ | Zero-extend; parser + writer + simulation |
| Transistor | missing | — | — | |
| Transmission Gate | missing | — | — | |

### 2b. Gates (lib `"1"`)

| Component | Implemented | Tested | Compatible | Notes |
|-----------|-------------|--------|------------|-------|
| AND Gate | ✅ | ✅ | ✅ | |
| OR Gate | ✅ | ✅ | ✅ | |
| NAND Gate | ✅ | ✅ | ✅ | |
| NOR Gate | ✅ | ✅ | ✅ | |
| XOR Gate | ✅ | ✅ | ✅ | |
| XNOR Gate | ✅ | ✅ | ✅ | |
| NOT Gate | ✅ | ✅ | ✅ | |
| Buffer | ✅ | ✅ | ✅ | |
| Controlled Buffer | ✅ | ✅ | ✅ | Parser + writer round-trip fixed |
| Odd Parity | ✅ | ✅ | ✅ | Parser + writer + simulation |
| Even Parity | ✅ | ✅ | ✅ | Parser + writer + simulation |

### 2c. Plexers (lib `"2"`)

| Component | Implemented | Tested | Compatible | Notes |
|-----------|-------------|--------|------------|-------|
| Multiplexer | ✅ | ✅ | ✅ | |
| Demultiplexer | ✅ | ✅ | ✅ | |
| Decoder | ✅ | ✅ | ✅ | |
| Priority Encoder | ✅ | ✅ | ✅ | |
| Bit Selector | ✅ | ✅ | ✅ | |

### 2d. Arithmetic (lib `"3"`)

| Component | Implemented | Tested | Compatible | Notes |
|-----------|-------------|--------|------------|-------|
| Adder | ✅ | ✅ | ✅ | |
| Subtractor | ✅ | ✅ | ✅ | |
| Multiplier | ✅ | ✅ | ✅ | |
| Divider | ✅ | ✅ | ✅ | |
| Negator | ✅ | ✅ | ✅ | |
| Comparator | ✅ | ✅ | ✅ | |
| Shifter | ✅ | ✅ | ✅ | |
| Bit Adder | ✅ | ✅ | ✅ | Popcount |
| Bit Finder | ✅ | ✅ | ✅ | LSB/MSB finder |

### 2e. Memory (lib `"4"`)

| Component | Implemented | Tested | Compatible | Notes |
|-----------|-------------|--------|------------|-------|
| D Flip-Flop | ✅ | ✅ | ✅ | |
| T Flip-Flop | ✅ | ✅ | ✅ | |
| JK Flip-Flop | ✅ | ✅ | ✅ | |
| SR Flip-Flop | ✅ | ✅ | ✅ | |
| Register | ✅ | ✅ | ✅ | |
| Counter | ✅ | ✅ | ✅ | |
| Shift Register | ✅ | ✅ | ✅ | |
| RAM | ✅ | ✅ | ✅ | |
| ROM | ✅ | ✅ | ✅ | |

### 2f. I/O (lib `"5"`)

| Component | Implemented | Tested | Compatible | Notes |
|-----------|-------------|--------|------------|-------|
| Button | ✅ | ✅ | ✅ | |
| LED | ✅ | ✅ | ✅ | |
| 7-Segment Display | ✅ | ✅ | ✅ | |
| Hex Digit Display | ✅ | ✅ | ✅ | |
| DotMatrix Display | ✅ | ✅ | ✅ | |
| Keyboard | ✅ | ✅ | ✅ | |
| TTY | ✅ | ✅ | ✅ | |
| Joystick | ✅ | ✅ | ✅ | |
| Port | ✅ | ✅ | ✅ | |

### 2g. TTL libraries

| Library | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| 74xx TTL series | missing | — | — | Upstream ships 74xx component library |

---

## 3. File format compatibility

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| Parse `<circuit>` elements | ✅ | ✅ | ✅ | |
| Parse `<wire>` elements | ✅ | ✅ | ✅ | |
| Parse `<comp>` with lib/name | ✅ | ✅ | ✅ | All libs 0–5 |
| Parse component attributes (loc, facing, width, label) | ✅ | ✅ | ✅ | |
| Write `<circuit>` elements | ✅ | ✅ | ✅ | |
| Write `<wire>` elements | ✅ | ✅ | ✅ | |
| Write `<comp>` with lib/name | ✅ | ✅ | ✅ | |
| Round-trip fidelity (parse → write → parse) | partial | ✅ | partial | Bit Extender, Transistor, TTL missing |
| Project-level metadata (`<project>`, `<lib>` declarations) | partial | — | partial | Read-only; not written back |
| Circuit appearance data (`<appear>`) | missing | — | — | Custom component shapes not stored |
| `main` circuit attribute | partial | — | partial | Parsed but not enforced |
| `.circ` file version attribute | partial | — | partial | Parsed; not validated for compatibility |

---

## 4. GUI / editor

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| Tool palette (Select, Wire, Place, Poke, Text, Delete) | ✅ | — | ✅ | |
| Canvas pan (drag) | ✅ | — | ✅ | |
| Canvas zoom (scroll / buttons) | ✅ | — | ✅ | |
| Grid display | ✅ | — | ✅ | Toggleable |
| Component placement with ghost preview | ✅ | — | ✅ | |
| Wire drawing (click-drag, T-junction) | ✅ | — | ✅ | |
| Rubber-band selection | ✅ | — | ✅ | |
| Component drag-to-move | ✅ | — | ✅ | Commits MoveComponent undo action |
| Delete selected (Delete key) | ✅ | — | ✅ | |
| Poke tool (toggle input pins) | ✅ | — | ✅ | |
| Wire value overlay (colour by logic value) | ✅ | — | ✅ | |
| Component labels rendered | ✅ | — | ✅ | |
| Multi-circuit navigation (circuit list) | ✅ | — | ✅ | |
| Add new circuit | ✅ | — | ✅ | |
| Circuit rename | missing | — | — | |
| Simulation run / stop / step toolbar | ✅ | — | ✅ | |
| File open / save / save-as | ✅ | — | ✅ | Native file dialogs |
| Undo / Redo (Edit menu + Ctrl+Z/Y) | ✅ | ✅ | ✅ | |
| About dialog | ✅ | — | ✅ | |
| Preferences / Options dialog | missing | — | — | |
| Appearance editor | missing | — | — | |
| Print / export image | missing | — | — | |
| Find / search | missing | — | — | |
| Component attribute panel (sidebar) | partial | — | partial | Properties shown; not fully editable |

---

## 5. Undo / Redo

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| Undo add component | ✅ | ✅ | ✅ | |
| Undo remove component | ✅ | ✅ | ✅ | |
| Undo add wire | ✅ | ✅ | ✅ | |
| Undo remove wire | ✅ | ✅ | ✅ | |
| Undo move component | ✅ | ✅ | ✅ | |
| Batched undo (multi-action) | ✅ | ✅ | ✅ | `UndoAction::Batch` |
| Bounded history (200 steps) | ✅ | ✅ | ✅ | Oldest entries evicted |
| History cleared on file load | ✅ | ✅ | ✅ | |
| Undo rename circuit | missing | — | — | Circuit rename not yet implemented |

---

## 6. Chronogram / timing diagram

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| Signal recording during simulation | missing | — | — | |
| Timeline / waveform rendering | missing | — | — | |
| Export timing diagram | missing | — | — | |

---

## 7. HDL export

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| VHDL export | missing | — | — | |
| Verilog export | missing | — | — | |
| Testbench generation | missing | — | — | |

---

## 8. FPGA / board integration

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| Board definition files | missing | — | — | Upstream ships Basys3, Arty S7, DE2-115, etc. |
| Pin mapping editor | missing | — | — | |
| Synthesis / download flow | missing | — | — | |
| FPGA component library | missing | — | — | |

---

## 9. Localization

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| English UI strings | ✅ | — | ✅ | Hard-coded in Rust source |
| Translated resource bundles (20+ languages) | missing | — | — | Upstream ships `*.properties` bundles |

---

## 10. CLI

| Feature | Implemented | Tested | Compatible | Notes |
|---------|-------------|--------|------------|-------|
| Load `.circ` file | ✅ | ✅ | ✅ | |
| Simulate truth table (`simulate` subcommand) | ✅ | ✅ | ✅ | |
| JSON output (`--format json`) | ✅ | ✅ | ✅ | |
| REPL / interactive mode | missing | — | — | |

---

## Summary

| Category | Implemented | Partial | Missing |
|----------|-------------|---------|---------|
| Simulation semantics | 10 | 0 | 0 |
| Wiring components | 10 | 0 | 2 |
| Gate components | 11 | 0 | 0 |
| Plexer components | 5 | 0 | 0 |
| Arithmetic components | 9 | 0 | 0 |
| Memory components | 9 | 0 | 0 |
| I/O components | 9 | 0 | 0 |
| TTL libraries | 0 | 0 | 1 |
| File format | 8 | 4 | 2 |
| GUI / editor | 14 | 1 | 7 |
| Undo / Redo | 8 | 0 | 1 |
| Chronogram | 0 | 0 | 3 |
| HDL export | 0 | 0 | 3 |
| FPGA / board | 0 | 0 | 4 |
| Localization | 1 | 0 | 1 |
| CLI | 3 | 0 | 1 |

_Last updated: 2026-03-03_
