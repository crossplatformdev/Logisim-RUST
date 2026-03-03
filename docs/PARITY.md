# Feature Parity Matrix — Logisim-RUST vs Logisim-Evolution v4.1.0

This document tracks the implementation status of every major upstream subsystem.
Each subsystem is evaluated against the Logisim-Evolution v4.1.0 release
(https://github.com/logisim-evolution/logisim-evolution/releases/tag/v4.1.0).

**Status key**

| Symbol | Meaning |
|--------|---------|
| ✅ | Implemented, tested, and verified against upstream |
| 🟡 | Partially implemented — core behaviour present, gaps remain |
| ❌ | Missing — not yet implemented |

---

## 1. Simulation semantics

| Feature | Status | Notes |
|---------|--------|-------|
| Multi-value logic (0, 1, X, Z) | ✅ | `Value` enum; HighZ, Unknown, Error, True, False |
| Multi-driver conflict resolution | ✅ | Error value on conflict, HighZ when undriven |
| Iterative propagation (convergence loop) | ✅ | `Simulator::propagate`; max-iteration guard |
| Oscillation detection | ✅ | Detects non-converging circuits after 1 000 iterations |
| Clock tick | ✅ | `Simulator::tick` advances all clocks |
| Continuous simulation (run mode) | ✅ | GUI run/stop with configurable Hz rate |
| Single-step simulation | ✅ | Step button and toolbar |
| Input pin forcing | ✅ | `set_pin_value`; Poke tool |
| Subcircuit simulation | ✅ | Recursive evaluation via `Subcircuit` component |
| Bus width mismatch handling | ✅ | `Bus` type with per-bit values |

---

## 2. Component library coverage

### 2a. Wiring (#0)

| Component | Status | Notes |
|-----------|--------|-------|
| Pin (input / output) | ✅ | |
| Clock | ✅ | |
| Constant | ✅ | |
| Power / Ground | ✅ | |
| Splitter | ✅ | |
| Tunnel | ✅ | Label-based bus merging |
| Probe | ✅ | |
| Pull Resistor | ✅ | |
| Tristate Buffer | ✅ | |
| Bit Extender | ❌ | Missing |
| Transistor | ❌ | Missing |
| Transmission Gate | ❌ | Missing |

### 2b. Gates (#1)

| Component | Status | Notes |
|-----------|--------|-------|
| AND Gate | ✅ | |
| OR Gate | ✅ | |
| NAND Gate | ✅ | |
| NOR Gate | ✅ | |
| XOR Gate | ✅ | |
| XNOR Gate | ✅ | |
| NOT Gate | ✅ | |
| Buffer | ✅ | |
| Controlled Buffer | ✅ | Parser + writer round-trip |
| Odd / Even Parity | ❌ | Missing |

### 2c. Plexers (#2)

| Component | Status | Notes |
|-----------|--------|-------|
| Multiplexer | ✅ | |
| Demultiplexer | ✅ | |
| Decoder | ✅ | |
| Priority Encoder | ✅ | |
| Bit Selector | ✅ | |

### 2d. Arithmetic (#3)

| Component | Status | Notes |
|-----------|--------|-------|
| Adder | ✅ | |
| Subtractor | ✅ | |
| Multiplier | ✅ | |
| Divider | ✅ | |
| Negator | ✅ | |
| Comparator | ✅ | |
| Shift Register (arithmetic) | ✅ | |
| Bit Adder | ✅ | |
| Bit Finder | ✅ | |

### 2e. Memory (#4)

| Component | Status | Notes |
|-----------|--------|-------|
| D Flip-Flop | ✅ | |
| T Flip-Flop | ✅ | |
| JK Flip-Flop | ✅ | |
| SR Flip-Flop | ✅ | |
| Register | ✅ | |
| RAM | ✅ | |
| ROM | ✅ | |
| Counter | ✅ | |
| Shift Register (memory) | ✅ | |

### 2f. I/O (#5)

| Component | Status | Notes |
|-----------|--------|-------|
| LED | ✅ | Rendered |
| RGB LED | ✅ | Rendered |
| 7-Segment Display | ✅ | |
| Hex Digit Display | ✅ | |
| Dot Matrix | ✅ | |
| Button | ✅ | |
| DIP Switch | ✅ | |
| Keyboard | ✅ | |
| TTY | ✅ | |

### 2g. TTL Libraries

| Library | Status | Notes |
|---------|--------|-------|
| TTL 7400 series | ❌ | Not yet implemented |

---

## 3. File format compatibility (`.circ`)

| Feature | Status | Notes |
|---------|--------|-------|
| Parse v4.1.0 `.circ` files | 🟡 | Core elements supported; some exotic attributes skipped |
| Write v4.1.0-compatible `.circ` files | 🟡 | Core elements written; some attributes omitted |
| Round-trip component positions | ✅ | |
| Round-trip wire segments | ✅ | |
| Round-trip component attributes | 🟡 | Common attributes yes; appearance data no |
| Round-trip multi-circuit projects | ✅ | |
| Round-trip subcircuit references | ✅ | |
| Library declarations in header | ✅ | |
| Appearance data (`<appear>`) | ❌ | Not read/written |
| Description / metadata attributes | 🟡 | Partially preserved |

---

## 4. GUI and editor workflow

| Feature | Status | Notes |
|---------|--------|-------|
| Component palette | ✅ | All standard categories |
| Wire placement (L-routed) | ✅ | |
| Component placement | ✅ | |
| Component selection | ✅ | Click to select |
| Component drag-to-move | ✅ | Drag with Select tool |
| Component deletion | ✅ | Delete key |
| Select All | ✅ | Ctrl+A via menu |
| Multi-select | ❌ | Rubber-band selection not yet implemented |
| Undo / Redo | ✅ | Ctrl+Z / Ctrl+Y; 200-step history |
| Copy / Paste | ❌ | Not yet implemented |
| Canvas pan | ✅ | Middle mouse / Alt+right-drag |
| Canvas zoom | ✅ | Scroll wheel; menu buttons |
| Grid display | ✅ | Toggleable |
| Multi-circuit navigation | ✅ | Right-side circuit list; "+ New Circuit" |
| Circuit rename | ❌ | Not yet implemented |
| Component labels | ✅ | Rendered on canvas |
| Wire value overlay | ✅ | Colour-coded by simulation value |
| Poke tool | ✅ | Toggle input pins at runtime |
| Simulation run/stop/step | ✅ | Toolbar + menu |
| File open / save / save-as | ✅ | Native file dialogs |
| About dialog | ✅ | |
| Preferences / Options dialog | ❌ | Not yet implemented |
| Appearance editor | ❌ | Not yet implemented |
| Print / export image | ❌ | Not yet implemented |

---

## 5. Undo/Redo

| Feature | Status | Notes |
|---------|--------|-------|
| Undo add component | ✅ | |
| Undo remove component | ✅ | |
| Undo add wire | ✅ | |
| Undo remove wire | ✅ | |
| Undo move component | ✅ | |
| Batched undo (multi-action) | ✅ | `UndoAction::Batch` |
| Bounded history (200 steps) | ✅ | Oldest entries evicted |
| History cleared on file load | ✅ | |
| Undo rename circuit | ❌ | Circuit rename not yet implemented |

---

## 6. Chronogram / timing diagram

| Feature | Status | Notes |
|---------|--------|-------|
| Signal recording | ❌ | Not yet implemented |
| Timeline rendering | ❌ | Not yet implemented |

---

## 7. HDL export

| Feature | Status | Notes |
|---------|--------|-------|
| VHDL export | ❌ | Not yet implemented |
| Verilog export | ❌ | Not yet implemented |

---

## 8. FPGA / board integration

| Feature | Status | Notes |
|---------|--------|-------|
| Board definitions | ❌ | Not yet implemented |
| Pin mapping | ❌ | Not yet implemented |
| Synthesis flow | ❌ | Not yet implemented |

---

## 9. Localization

| Feature | Status | Notes |
|---------|--------|-------|
| English UI strings | ✅ | Hard-coded in source |
| Translated resource bundles | ❌ | Not yet implemented (upstream has 20+ languages) |

---

## 10. CLI

| Feature | Status | Notes |
|---------|--------|-------|
| Load `.circ` file | ✅ | |
| Simulate truth table | ✅ | `simulate` subcommand |
| JSON output | ✅ | `--format json` |
| REPL / interactive mode | ❌ | Not yet implemented |

---

## Summary

| Category | Implemented | Partial | Missing |
|----------|-------------|---------|---------|
| Simulation semantics | 10 | 0 | 0 |
| Wiring components | 8 | 0 | 3 |
| Gate components | 9 | 0 | 1 |
| Plexer components | 5 | 0 | 0 |
| Arithmetic components | 9 | 0 | 0 |
| Memory components | 9 | 0 | 0 |
| I/O components | 9 | 0 | 0 |
| TTL libraries | 0 | 0 | 1 |
| File format | 4 | 4 | 2 |
| GUI / editor | 13 | 0 | 9 |
| Undo/Redo | 8 | 0 | 1 |
| Chronogram | 0 | 0 | 2 |
| HDL export | 0 | 0 | 2 |
| FPGA / board | 0 | 0 | 3 |
| Localization | 1 | 0 | 1 |
| CLI | 3 | 0 | 1 |
