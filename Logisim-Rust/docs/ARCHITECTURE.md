# Logisim-RUST Architecture

This document describes the architecture of the Logisim-RUST project, which is a Rust implementation of the Logisim-Evolution digital logic simulator.

## Overview

The project is organized into several crates, each with specific responsibilities:

- `logisim_core`: Core simulation engine and circuit representation
- `logisim_ui`: User interface components using egui
- Example schematics and test circuits are provided in `example_schematics/`

## Core Architecture (`logisim_core`)

### Circuit Representation
- **Circuit Format (.circ)**: XML-based format for storing circuit designs
- **Netlist**: Represents the connectivity between components
- **Components**: Individual logic elements (gates, latches, etc.)
- **Signals**: Digital values and timing information

### Simulation Engine
- **Event-driven simulation**: Uses an event queue to process signal changes
- **Component abstraction**: Generic trait for all circuit components
- **Time-based simulation**: Supports precise timing simulation

### Key Modules
- `circ_format.rs`: Circuit file parsing and serialization
- `simulation.rs`: Main simulation engine
- `netlist.rs`: Network connectivity management
- `signal.rs`: Signal and value representations
- `component.rs`: Component trait and implementations

## User Interface (`logisim_ui`)

### GUI Framework
The UI is built using **egui**, a modern immediate-mode GUI framework for Rust that provides:
- Cross-platform compatibility
- High performance rendering
- Integration with wgpu/OpenGL backends

### Main Components
- **MainFrame**: Primary application window
- **Canvas**: Circuit editing and display area
- **Toolbox**: Component palette
- **ProjectExplorer**: Circuit hierarchy navigation
- **ChronogramPanel**: Timing diagram display

## Chronogram (Waveform/Timing View) Feature

### Overview
The chronogram feature provides timing diagram visualization, equivalent to the Java Logisim-Evolution's chronogram functionality. It displays signal states over time for wires, buses, and components.

### Architecture

#### Module Structure
```
logisim_ui/src/gui/chronogram/
├── mod.rs          # Module exports and constants
├── model.rs        # Data model for signal tracking
├── panel.rs        # Main chronogram UI panel
├── timeline.rs     # Time axis and navigation
└── waveform.rs     # Signal waveform rendering
```

#### Key Components

##### ChronogramModel (`model.rs`)
- **SignalInfo**: Metadata about tracked signals (name, width, selection state)
- **SignalData**: Time-series data for signal value changes
- **ChronogramModel**: Main data container managing all signals and timing

Features:
- Efficient signal value storage using `BTreeMap<Timestamp, Signal>`
- Support for single-bit and multi-bit (bus) signals
- Time range tracking and cursor positioning
- Signal selection management

##### Timeline (`timeline.rs`)
- Time axis rendering with automatic tick spacing
- Zoom and scroll navigation
- Cursor positioning and time marker display
- Interactive time selection

Features:
- Automatic "nice" interval calculation for tick marks
- Pixel-to-time coordinate conversion
- Zoom limits and smooth zooming around cursor position
- Horizontal scrolling for long simulations

##### Waveform (`waveform.rs`)
- Individual signal waveform rendering
- Support for digital signals and bus values
- Color-coded value representation
- Text labels for bus values

Features:
- Digital signal rendering (high/low/unknown/error states)
- Bus signal rendering with hex/decimal value display
- Transition edge visualization
- Customizable color schemes
- Selection highlighting

##### ChronogramPanel (`panel.rs`)
- Main UI panel integrating all components
- Signal list display with current values
- Recording control and simulation integration
- Export functionality

Features:
- Splitter between signal names and waveforms
- Signal selection dialog
- Toolbar with zoom, recording, and export controls
- Real-time update during simulation
- Text export of waveform data

### Integration with Simulation

#### Signal Capture
The chronogram integrates with the simulation engine through:
1. **Automatic signal discovery**: Extracts all nodes from the netlist
2. **Real-time monitoring**: Updates signal values during simulation steps
3. **Callback system** (planned): Direct notification of signal changes

#### Required Signals
Following the Java implementation, the chronogram requires:
- **sysclk**: System clock signal (mandatory)
- **clk**: Optional secondary clock
- Any user-selected signals from the circuit

### Usage

#### Basic Workflow
1. Load a circuit file containing clock signals
2. Open the chronogram via the "📊 Chronogram" button
3. Start simulation recording
4. Step or run the simulation to capture timing data
5. Use zoom/scroll to navigate the timing diagram
6. Export timing data if needed

#### Navigation Controls
- **Zoom**: Mouse wheel or toolbar buttons
- **Scroll**: Click and drag on timeline
- **Cursor**: Click on timeline to set time cursor
- **Signal Selection**: Click on signal names to highlight

### Export Capabilities

#### Text Export
The chronogram supports text export with:
- Time range information
- Signal count and names
- Timestamped value changes for each signal
- Human-readable format

#### Future Enhancements
- Image export (PNG/SVG)
- CSV format export
- Custom time range selection

### Performance Considerations

#### Memory Management
- Efficient storage using `BTreeMap` for sparse signal data
- Lazy waveform rendering only for visible time ranges
- Configurable maximum recording duration

#### Rendering Optimization
- Culling of off-screen waveforms
- Efficient timeline tick calculation
- Minimized redraw on zoom/scroll operations

### Testing

#### Unit Tests
Each module includes comprehensive unit tests:
- Signal data operations
- Timeline coordinate conversions
- Waveform value formatting
- Export functionality

#### Integration Tests
- Chronogram panel creation and lifecycle
- Simulation integration
- Signal recording and playback

### Compatibility with Java Implementation

The Rust chronogram implementation maintains compatibility with the Java Logisim-Evolution by:
- Following the same signal naming conventions (sysclk requirement)
- Using equivalent timing semantics
- Providing similar UI layout and controls
- Supporting the same export formats

### Configuration Constants

Key configuration values in `chronogram/mod.rs`:
```rust
pub const SIGNAL_HEIGHT: f32 = 30.0;      // Height of each waveform
pub const HEADER_HEIGHT: f32 = 20.0;       // Timeline header height
pub const GAP: f32 = 2.0;                  // Gap between traces
pub const DEFAULT_TICK_WIDTH: f32 = 10.0;  // Default time scale
```

## Dependencies

### Core Dependencies
- `serde`: Serialization for circuit files
- `thiserror`: Error handling
- `quick-xml`: XML parsing
- `roxmltree`: XML tree processing

### UI Dependencies (feature-gated)
- `eframe`: egui application framework
- `egui`: Immediate mode GUI
- Graphics backends (automatically selected)

## Build Configuration

### Features
- `gui`: Enables GUI components (default)
- No GUI: Headless simulation mode for testing/automation

### Platform Support
- Windows, macOS, Linux desktop platforms
- Web assembly target (via eframe web backend)

## Testing Strategy

### Unit Tests
- Individual module functionality
- Algorithm correctness
- Error handling

### Integration Tests
- Cross-module interactions
- File format compatibility
- Simulation accuracy

### Example Circuits
- Comprehensive test suite using `example_schematics/`
- Verification against Java implementation behavior
- Performance benchmarking

## Future Enhancements

### Planned Features
- VHDL/Verilog export
- Advanced component library
- Plugin system
- Network simulation capabilities

### Performance Optimizations
- Multi-threaded simulation
- GPU acceleration for rendering
- Memory pool allocation

## Migration from Java

See `MIGRATION_NOTES.md` for detailed information about:
- API differences from Java implementation
- Porting guidelines for Java components
- Feature parity status
- Known limitations and workarounds