# Logisim-RUST Architecture

This document describes the architecture of the Logisim-RUST project, which is a Rust implementation of the Logisim-Evolution digital logic simulator.

## Overview

The project is organized into several crates, each with specific responsibilities:

- `logisim_core`: Core simulation engine and circuit representation
- `logisim_ui`: User interface components using egui
- Example schematics and test circuits are provided in `example_schematics/`

## Core Architecture (`logisim_core`)

### Foundation Infrastructure

#### Utility Classes (`logisim_core/src/util/`)
The utility module provides essential helper functions and data structures:

- **StringUtil & StringGetter**: Trait-based string handling with hex conversion, null checking, and text resizing operations
- **CollectionUtil**: Type-safe collection operations for Vec, HashMap, HashSet with union types and null-safe operations  
- **Cache**: Generic caching system with configurable sizing and string interning capabilities for performance optimization
- **FileUtil**: Cross-platform file operations with temporary file management and comprehensive I/O utilities
- **LocaleManager**: Internationalization system with string getter patterns and locale switching support

#### Core Data Structures (`logisim_core/src/data/`)
The data module contains fundamental types used throughout the system:

- **Direction**: Four cardinal directions (North, South, East, West) with rotation logic, degree/radian conversion, and display formatting
- **Location**: Immutable 2D coordinate system with grid snapping, Manhattan distance calculations, and spatial operations
- **Bounds**: Immutable rectangular bounding box with union/intersection operations, collision detection, and rotation support
- **BitWidth**: Enhanced bit width system with UI integration, mask generation, and compatibility with existing BusWidth types
- **Attribute System**: Complete type-safe component configuration system with generics, validation, and standard attributes for component properties

#### Component Framework (`logisim_core/src/components/`)
Prepared structure for component implementations:

- **Base Module**: Foundation for component implementations with proper module organization
- **Extensible Architecture**: Ready for systematic addition of gates, memory, I/O, and specialized components

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

### GUI Framework Selection

#### Framework Comparison: egui vs iced

| Criteria | egui | iced | Winner |
|----------|------|------|--------|
| **Canvas Performance** | ✅ **Excellent** - Immediate mode, custom painting API, 60+ FPS for complex circuits, efficient partial redraws | ⚠️ **Good** - Retained mode with canvas widget, more overhead for custom drawing, potential performance issues with complex circuits | **egui** |
| **Text/i18n Support** | ⚠️ **Limited** - Basic text rendering, no built-in i18n, requires external crates (fluent-rs, sys-locale), Unicode support present | ✅ **Better** - More mature text handling, better RTL support, easier integration with i18n libraries | **iced** |
| **Packaging** | ✅ **Excellent** - Native (Windows/macOS/Linux), WebAssembly, single binary, eframe provides unified backend | ✅ **Excellent** - Native platforms, WASM support, good cross-compilation | **Tie** |
| **Widget Maturity** | ⚠️ **Developing** - Growing ecosystem, basic widgets complete, some specialized widgets missing, custom widgets easy to build | ✅ **Mature** - Rich widget ecosystem, many third-party widgets, canvas widget for custom drawing | **iced** |
| **Active Maintenance** | ✅ **Very Active** - Emil Ernerfeldt (main dev), frequent releases, active Discord community, 21k+ GitHub stars | ✅ **Very Active** - Large contributor base, regular releases, good documentation, 22k+ GitHub stars | **Tie** |
| **Learning Curve** | ✅ **Easy** - Immediate mode is intuitive, minimal boilerplate, great for prototyping | ⚠️ **Steeper** - Elm architecture, more concepts to learn, retained mode complexity | **egui** |
| **Memory Usage** | ✅ **Low** - Immediate mode means less state retention, efficient for large applications | ⚠️ **Higher** - Retained mode requires more memory for widget trees | **egui** |
| **Custom Drawing** | ✅ **Excellent** - Direct access to painter, easy custom rendering, perfect for circuit diagrams | ⚠️ **Limited** - Canvas widget exists but more constrained, harder to integrate with other widgets | **egui** |

#### Recommendation: **egui/eframe**

**Justification for Logisim-RUST:**

**Pros of egui:**
- **Superior canvas performance**: Immediate mode GUI is ideal for circuit simulation where the canvas needs frequent updates
- **Custom drawing flexibility**: Direct access to painting primitives essential for drawing circuits, components, and wires
- **Lower memory footprint**: Important for complex circuits with many components
- **Easier integration**: Already implemented and working in the current codebase
- **Web deployment**: eframe provides seamless native + WebAssembly support for browser-based Logisim
- **Rapid development**: Immediate mode reduces boilerplate for complex UI interactions

**Acknowledged tradeoffs:**
- **i18n limitations**: Will require additional work to implement comprehensive internationalization
- **Widget ecosystem**: Some specialized widgets may need custom implementation
- **Text rendering**: Less mature than iced for complex text scenarios

**Mitigation strategies:**
- Implement robust i18n system using fluent-rs (already started in codebase)
- Create custom widgets as needed (project already has good foundation)
- Leverage egui's active community for specialized requirements

The current codebase already demonstrates egui's suitability for Logisim's requirements, with working canvas, chronogram, and component systems.

### Current Implementation
The UI is built using **egui/eframe**, chosen for its superior canvas performance and custom drawing capabilities essential for circuit design:
- Cross-platform compatibility (Windows, macOS, Linux, WebAssembly)
- High performance immediate-mode rendering
- Direct integration with GPU backends (OpenGL/wgpu)
- Efficient handling of complex circuit diagrams with real-time updates

### UI Crate Structure (`logisim_ui`)

#### Module Organization
```
logisim_ui/src/
├── main.rs                   # Binary entry point
├── lib.rs                    # Library root with feature gates
├── gui/                      # GUI components (feature-gated)
│   ├── mod.rs               # GUI module exports
│   ├── app.rs               # Main LogisimApp struct and eframe integration
│   ├── frame.rs             # MainFrame - primary window layout
│   ├── startup.rs           # Command line parsing and startup logic
│   ├── canvas.rs            # Circuit editing canvas with custom painting
│   ├── menu.rs              # Application menu bar
│   ├── toolbar.rs           # Main toolbar with tools
│   ├── toolbox.rs           # Component palette/toolbox panel
│   ├── project_explorer.rs  # Circuit hierarchy tree view
│   ├── properties.rs        # Component properties panel
│   ├── selection.rs         # Selection management system
│   ├── edit_handler.rs      # Clipboard and edit operations
│   ├── i18n.rs              # Internationalization system
│   ├── tests.rs             # GUI unit tests
│   ├── generic/             # Generic UI utilities
│   │   ├── mod.rs          # Generic components module
│   │   └── option_pane.rs  # Dialog and popup utilities
│   └── chronogram/          # Timing diagram subsystem
│       ├── mod.rs          # Chronogram module exports and constants
│       ├── model.rs        # Signal data model and storage
│       ├── panel.rs        # Main chronogram UI panel
│       ├── timeline.rs     # Time axis and navigation controls
│       └── waveform.rs     # Individual signal waveform rendering
├── hex/                     # Hex editor components
│   ├── mod.rs              # Hex editor module
│   ├── hex_editor.rs       # Main hex editor widget
│   ├── hex_model.rs        # Data model for hex editing
│   ├── caret.rs            # Text cursor management
│   ├── measures.rs         # Layout and measurement utilities
│   └── highlighter.rs     # Syntax highlighting
└── headless/               # Headless mode components
    ├── mod.rs              # Headless module
    └── runner.rs           # CLI simulation runner
```

#### Key Architectural Decisions

##### Feature-Gated Architecture
```rust
// Conditional compilation for different deployment scenarios
#[cfg(feature = "gui")]
pub mod gui;

#[cfg(not(feature = "gui"))]
pub mod headless;

// Platform-specific implementations
#[cfg(all(feature = "gui", target_arch = "wasm32"))]
pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // WebAssembly-specific initialization
}

#[cfg(all(feature = "gui", not(target_arch = "wasm32")))]
pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // Native desktop initialization
}
```

##### Component Lifecycle Management
- **App Level**: `LogisimApp` manages global state and eframe integration
- **Frame Level**: `MainFrame` handles window layout and panel management
- **Panel Level**: Individual panels (`Canvas`, `Toolbox`, etc.) manage their own state
- **Widget Level**: Custom widgets for specialized circuit editing functionality

##### State Management Pattern
```rust
// Centralized state in MainFrame with panel delegation
pub struct MainFrame {
    canvas: Canvas,
    toolbox: Toolbox,
    project_explorer: ProjectExplorer,
    chronogram_panel: ChronogramPanel,
    simulation: Option<Simulation>,
    // ... other components
}

impl MainFrame {
    pub fn update(&mut self, ctx: &egui::Context) {
        // Coordinate updates between panels
        self.handle_menu_bar(ctx);
        self.handle_central_panel(ctx);
        self.handle_side_panels(ctx);
    }
}
```

##### Internationalization Integration
```rust
// Global i18n system accessible throughout GUI
use crate::gui::i18n::{tr, tr_args, Language};

// Usage in UI components
egui::Button::new(tr("menu.file.open"))
egui::Label::new(tr_args("status.components_selected", &[&count.to_string()]))
```

### Main Components

#### Core UI Components
- **LogisimApp**: Top-level application struct implementing `eframe::App`
- **MainFrame**: Primary window layout manager with panel coordination
- **Canvas**: Interactive circuit editing surface with custom painting
- **Toolbox**: Component palette with drag-and-drop support
- **ProjectExplorer**: Hierarchical circuit navigation tree
- **ChronogramPanel**: Real-time timing diagram visualization
- **Properties**: Context-sensitive component property editor
- **MenuBar**: Application menu with keyboard shortcuts
- **Toolbar**: Quick-access tool selection and actions

#### Specialized Subsystems
- **I18n System**: Complete internationalization with 9+ language support
- **Selection Manager**: Multi-object selection with rectangular and individual selection
- **Edit Handler**: Clipboard operations (cut/copy/paste) with undo/redo
- **Hex Editor**: Memory viewing and editing with syntax highlighting
- **Dialog System**: Modal dialogs and popup windows

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