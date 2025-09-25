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

## Extensibility and Advanced Modeling

### Advanced Modeling Features

The architecture includes experimental advanced modeling capabilities that provide extensibility hooks for plugins and custom components:

#### Observer Pattern for Simulation Events
```rust
pub trait SimulationObserver: Send + Sync {
    fn on_event(&mut self, event: &SimulationEvent);
    fn name(&self) -> &str;
    fn is_interested_in(&self, event: &SimulationEvent) -> bool;
}
```

**⚠️ Unstable API**: The observer pattern is experimental and subject to change.

**Key Features:**
- Real-time simulation event monitoring
- Signal change notifications
- Clock edge detection
- Component state change events
- Weak reference support to prevent memory leaks

#### Dynamic Component Registration
```rust
pub trait ComponentFactory: Send + Sync {
    fn create_component(&self, id: ComponentId) -> Box<dyn Component>;
    fn component_type(&self) -> &str;
    fn category(&self) -> &str;
}

pub struct DynamicComponentRegistry {
    // Runtime component type registration
}
```

**⚠️ Unstable API**: Dynamic registration is experimental.

**Capabilities:**
- Runtime component type registration
- Category-based organization
- Plugin-provided component factories
- Type-safe component creation

#### Extension Points
```rust
pub trait ExtensionPoint: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> ModelingResult<()>;
    fn cleanup(&mut self) -> ModelingResult<()>;
}
```

**⚠️ Unstable API**: Extension points are experimental.

**Use Cases:**
- Custom simulation algorithms
- Additional analysis tools
- External tool integration
- Debugging and profiling hooks

### Plugin System Architecture

#### Plugin Library Interface
```rust
pub trait PluginLibrary: Send + Sync {
    fn info(&self) -> &PluginInfo;
    fn components(&self) -> Vec<ComponentInfo>;
    fn create_component(&self, component_type: &str, id: ComponentId) -> PluginResult<Box<dyn Component>>;
    
    // Advanced modeling hooks (unstable)
    fn extension_points(&self) -> Vec<Box<dyn ExtensionPoint>>;
    fn observers(&self) -> Vec<Box<dyn SimulationObserver>>;
    fn setup_modeling(&mut self, context: &mut ModelingContext) -> PluginResult<()>;
}
```

**Planned Plugin Types:**
- Native Rust libraries (.so/.dll/.dylib)
- WebAssembly modules (.wasm) - future
- JAR compatibility layer - future

#### Example Plugin Implementation

A complete example plugin (`ExamplePlugin`) is provided in `logisim_core/src/integrations/stub_plugin.rs` demonstrating:

1. **Component Creation**: Example counter and monitor components
2. **Observer Implementation**: Signal change logging and clock edge detection
3. **Extension Points**: Custom functionality registration
4. **Plugin Lifecycle**: Initialization and cleanup

```rust
// Example usage
let plugin = ExamplePlugin::new();
let mut context = ModelingContext::new();
plugin.setup_modeling(&mut context)?;

// Plugin provides:
// - ExampleCounter component
// - ExampleMonitor component  
// - ExampleObserver for event logging
// - ClockTracker for clock edge detection
// - ExampleExtensionPoint for custom functionality
```

### API Stability

**⚠️ IMPORTANT: Experimental APIs**

The following APIs are marked as **unstable** and may change in future versions:

| API Area | Stability | Notes |
|----------|-----------|-------|
| `SimulationObserver` | Unstable | Event types may change |
| `ExtensionPoint` | Unstable | Registration mechanism may change |
| `DynamicComponentRegistry` | Unstable | Interface may change |
| `ComponentFactory` (dynamic) | Unstable | Parameters may change |
| `ModelingContext` | Unstable | Internal structure may change |
| Plugin `setup_modeling()` | Unstable | Parameters may change |

**Stable APIs:**
- Core `Component` trait
- Basic `PluginLibrary` interface (without modeling hooks)
- `PluginManager` discovery and loading (stub)

### Extension Point Documentation

#### Simulation Event Types
```rust
pub enum SimulationEvent {
    SignalChanged { node_id, old_signal, new_signal, timestamp, source },
    ComponentStateChanged { component_id, event_type, data },
    StepCompleted { timestamp, events_processed },
    SimulationReset { timestamp },
    ClockEdge { node_id, edge_type, timestamp },
}
```

#### Observer Registration
```rust
// Direct registration (takes ownership)
context.observer_manager().add_observer(observer);

// Weak registration (doesn't take ownership)
let weak_observer = Arc::downgrade(&observer_arc);
context.observer_manager().add_weak_observer(weak_observer);
```

#### Extension Point Registration
```rust
// Register custom extension point
context.extension_registry().register_extension(my_extension)?;

// Retrieve extension by name
let ext = context.extension_registry().get_extension("my_extension");

// Retrieve extension by type
let ext = context.extension_registry().get_extension_by_type::<MyExtension>();
```

### Migration Path

For developers wanting to use these features:

1. **Start with Stable APIs**: Use basic plugin interfaces first
2. **Experimental Usage**: Mark code using unstable APIs clearly
3. **Version Pinning**: Pin to specific versions when using unstable APIs
4. **Feedback Welcome**: Report issues and suggestions for unstable APIs

### Performance Considerations

**Observer Pattern:**
- Use `is_interested_in()` to filter events and reduce overhead
- Weak references prevent observer lifetime issues
- Events are processed synchronously in simulation thread

**Dynamic Components:**
- Runtime registration has minimal overhead
- Component creation uses standard factory pattern
- No reflection or dynamic dispatch beyond trait objects

**Extension Points:**
- Initialization/cleanup called once per plugin lifecycle
- No runtime overhead for unused extension points
- Thread-safe by design (Send + Sync bounds)

## Future Enhancements

### Planned Features
- VHDL/Verilog export
- Advanced component library
- **Plugin system stabilization**
- **WebAssembly plugin support**
- Network simulation capabilities

### Performance Optimizations
- Multi-threaded simulation
- GPU acceleration for rendering
- Memory pool allocation
- **Observer pattern optimization**

## Migration from Java

See `MIGRATION_NOTES.md` for detailed information about:
- API differences from Java implementation
- Porting guidelines for Java components
- Feature parity status
- Known limitations and workarounds