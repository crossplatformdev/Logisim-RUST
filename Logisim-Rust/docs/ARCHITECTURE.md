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

## Extension Points and Plugin System

### Overview

Logisim-RUST provides a comprehensive extensibility framework that allows developers to extend the simulator with custom components, advanced modeling features, and specialized tools. The plugin system is designed with flexibility and performance in mind while maintaining API stability where possible.

**⚠️ UNSTABLE API WARNING**: All plugin and extensibility APIs are currently experimental and marked as unstable. Breaking changes may occur without major version increments as the system evolves.

### Plugin Architecture

#### Core Plugin Traits

The plugin system is built around several key traits:

```rust
/// Main plugin library trait
pub trait PluginLibrary: Send + Sync {
    fn info(&self) -> &PluginInfo;
    fn components(&self) -> Vec<ComponentInfo>;
    fn create_component(&self, component_type: &str, id: ComponentId) -> PluginResult<Box<dyn Component>>;
    fn initialize(&mut self) -> PluginResult<()>;
    fn cleanup(&mut self) -> PluginResult<()>;
    
    // Extensibility hooks
    fn register_hooks(&mut self, registry: &mut ExtensionRegistry) -> PluginResult<()>;
    fn config_schema(&self) -> Option<ConfigSchema>;
    fn on_plugin_event(&mut self, event: &PluginEvent) -> PluginResult<()>;
}
```

#### Dynamic Component Registration

Components can be registered dynamically at runtime:

```rust
/// Factory for creating component instances
pub trait ComponentFactory: Send + Sync {
    fn create(&self, id: ComponentId, location: Location) -> PluginResult<Box<dyn Component>>;
    fn component_info(&self) -> ComponentInfo;
    fn validate_placement(&self, location: Location) -> bool;
}

/// Component registry for dynamic registration
pub struct ComponentRegistry {
    // Manages component factories by type and category
}
```

#### Extension Registry System

The extension registry provides multiple extension points:

```rust
pub struct ExtensionRegistry {
    component_factories: HashMap<String, Box<dyn ComponentFactory>>,
    modeling_extensions: HashMap<String, Box<dyn ModelingExtension>>,
    ui_extensions: HashMap<String, Box<dyn UiExtension>>,
    simulation_hooks: Vec<Box<dyn SimulationHook>>,
    circuit_observers: Vec<Arc<Mutex<dyn Observer<CircuitEvent>>>>,
    simulation_observers: Vec<Arc<Mutex<dyn Observer<SimulationEvent>>>>,
}
```

### Extension Points

#### 1. Component Extension Point

**Purpose**: Add custom digital logic components
**Stability**: ⚠️ Unstable - Component trait may be extended

**How to extend**:
```rust
// Implement the Component trait
impl Component for MyCustomGate {
    fn id(&self) -> ComponentId { /* ... */ }
    fn name(&self) -> &str { /* ... */ }
    fn pins(&self) -> &HashMap<String, Pin> { /* ... */ }
    fn update(&mut self, current_time: Timestamp) -> UpdateResult { /* ... */ }
    fn reset(&mut self) { /* ... */ }
}

// Create a factory
struct MyGateFactory;
impl ComponentFactory for MyGateFactory {
    fn create(&self, id: ComponentId, location: Location) -> PluginResult<Box<dyn Component>> {
        Ok(Box::new(MyCustomGate::new(id, location)))
    }
}

// Register with the system
plugin_manager.register_component_type("MyGate".to_string(), Box::new(MyGateFactory), ComponentCategory::Gates)?;
```

#### 2. Modeling Extension Point

**Purpose**: Advanced simulation features like timing analysis, fault injection, or custom signal processing
**Stability**: ⚠️ Unstable - Interface is experimental

**How to extend**:
```rust
pub trait ModelingExtension: Send + Sync {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> PluginResult<()>;
    fn process_step(&mut self, step_data: &SimulationStepData) -> PluginResult<()>;
    fn cleanup(&mut self) -> PluginResult<()>;
}
```

#### 3. UI Extension Point

**Purpose**: Custom user interface elements and tools
**Stability**: ⚠️ Unstable - Will be redesigned when GUI system is finalized

**How to extend**:
```rust
pub trait UiExtension: Send + Sync {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> PluginResult<()>;
    fn render(&mut self, ui_context: &mut UiContext) -> PluginResult<()>;
    fn handle_event(&mut self, event: &UiEvent) -> PluginResult<()>;
    fn cleanup(&mut self) -> PluginResult<()>;
}
```

#### 4. Simulation Hook Extension Point

**Purpose**: Intercept simulation lifecycle events
**Stability**: ⚠️ Unstable - Hook interface is experimental

**How to extend**:
```rust
pub trait SimulationHook: Send + Sync {
    fn before_simulation_start(&mut self) -> PluginResult<()> { Ok(()) }
    fn after_simulation_stop(&mut self) -> PluginResult<()> { Ok(()) }
    fn before_step(&mut self, step_count: u64) -> PluginResult<()> { Ok(()) }
    fn after_step(&mut self, step_count: u64) -> PluginResult<()> { Ok(()) }
}
```

#### 5. Observer Pattern Extension Point

**Purpose**: React to circuit and simulation events
**Stability**: ⚠️ Unstable - Event types may be extended

**How to extend**:
```rust
pub trait Observer<E: Event>: Send + Sync {
    fn on_event(&mut self, event: &E) -> EventResult<()>;
    fn name(&self) -> &str { "UnnamedObserver" }
    fn should_handle(&self, event: &E) -> bool { true }
}

// Example usage
struct MyCircuitObserver;
impl Observer<CircuitEvent> for MyCircuitObserver {
    fn on_event(&mut self, event: &CircuitEvent) -> EventResult<()> {
        match event {
            CircuitEvent::ComponentAdded { component_id, location, .. } => {
                println!("Component {:?} added at {:?}", component_id, location);
            }
            _ => {}
        }
        Ok(())
    }
}
```

### Event System

The event system provides a comprehensive observer pattern implementation:

#### Event Types

```rust
/// Circuit-related events
pub enum CircuitEvent {
    ComponentAdded { component_id: ComponentId, location: Location, timestamp: u64 },
    ComponentRemoved { component_id: ComponentId, timestamp: u64 },
    ComponentMoved { component_id: ComponentId, old_location: Location, new_location: Location, timestamp: u64 },
    // ... more event types
}

/// Simulation-related events  
pub enum SimulationEvent {
    SimulationStarted { timestamp: u64 },
    SimulationStopped { timestamp: u64 },
    SignalChanged { component_id: ComponentId, signal: Signal, timestamp: u64 },
    // ... more event types
}
```

#### Event Dispatching

```rust
/// Event dispatcher with observer management
pub struct EventDispatcher<E: Event> {
    // Manages weak references to observers to prevent memory leaks
    // Supports both synchronous and asynchronous event delivery
}

/// Global event system
pub struct EventSystem {
    circuit_dispatcher: EventDispatcher<CircuitEvent>,
    simulation_dispatcher: EventDispatcher<SimulationEvent>,
}
```

### Plugin Configuration

Plugins can define configuration schemas:

```rust
pub struct ConfigSchema {
    pub fields: Vec<ConfigField>,
    pub version: String,
}

pub struct ConfigField {
    pub name: String,
    pub field_type: ConfigFieldType,
    pub default_value: Option<String>,
    pub description: String,
    pub required: bool,
}

pub enum ConfigFieldType {
    String,
    Integer,
    Float,
    Boolean,
    Choice(Vec<String>),
    Path,
}
```

### Plugin Discovery and Loading

#### File System Discovery

```rust
impl PluginManager {
    pub fn add_search_path(&mut self, path: PathBuf);
    pub fn discover_plugins(&mut self) -> PluginResult<Vec<PluginInfo>>;
    pub fn load_plugin(&mut self, plugin_name: &str) -> PluginResult<()>;
}
```

#### Supported Plugin Formats

- **Native Rust Libraries** (.so, .dll, .dylib) - Future implementation
- **WebAssembly Plugins** (.wasm) - Future implementation  
- **JAR Libraries** (Java compatibility) - Future implementation
- **Dynamic Registration** (In-process) - Currently implemented

### Example Plugin Implementation

A complete example plugin is provided in `logisim_core/src/integrations/plugin_examples.rs`:

```rust
pub struct ExamplePlugin {
    info: PluginInfo,
    components: Vec<ComponentInfo>,
    initialized: bool,
}

impl PluginLibrary for ExamplePlugin {
    fn register_hooks(&mut self, registry: &mut ExtensionRegistry) -> PluginResult<()> {
        // Register component factories
        registry.register_component_factory("ExampleGate".to_string(), Box::new(ExampleGateFactory))?;
        
        // Register modeling extensions
        registry.register_modeling_extension("TimingAnalysis".to_string(), Box::new(TimingExtension::new()))?;
        
        // Add simulation hooks
        registry.add_simulation_hook(Box::new(LoggingHook::new()));
        
        // Add event observers
        registry.add_circuit_observer(Arc::new(Mutex::new(CircuitDebugObserver::new())));
        
        Ok(())
    }
}
```

### API Stability Guarantees

#### Stable APIs (✅)
- None currently - all extensibility APIs are experimental

#### Unstable APIs (⚠️)
- **All plugin traits and interfaces** - May be extended or redesigned
- **Event system** - Event types may be added or modified
- **Extension registry** - Registration methods may change
- **Component factory system** - Interface may be refined
- **Configuration system** - Schema format may evolve

#### Deprecated APIs (❌)  
- None currently

### Migration Path

When APIs change, migration will be supported through:

1. **Versioned plugin interfaces** - Multiple interface versions supported simultaneously
2. **Compatibility layers** - Automatic translation between versions where possible
3. **Migration tools** - Automated code transformation utilities
4. **Clear deprecation notices** - Advance warning of breaking changes

### Performance Considerations

#### Plugin Loading
- Lazy loading of plugin libraries
- Dependency resolution and caching  
- Isolated plugin environments

#### Event System
- Weak references prevent memory leaks
- Asynchronous event delivery for performance-critical paths
- Event filtering and batching

#### Component Registration
- Efficient component lookup by type and category
- Factory caching and reuse
- Validation at registration time

### Security Considerations

#### Plugin Sandboxing
- Future implementation will include plugin sandboxing
- Resource limits and access controls
- Digital signature verification for distributed plugins

#### Memory Safety
- All plugin interfaces use Rust's ownership system
- Automatic cleanup of plugin resources
- Protection against plugin crashes affecting core system

### Development Tools

#### Plugin Template Generator
```rust
use logisim_core::integrations::dev_utils;
dev_utils::generate_plugin_template("MyPlugin", output_dir)?;
```

#### Plugin Validation
```rust
let plugin_info = dev_utils::validate_plugin(plugin_path)?;
```

#### Testing Framework
- Unit test helpers for plugin development
- Integration test utilities
- Mock event system for testing observers

### Future Roadmap

#### Phase 1 (Current)
- ✅ Basic plugin trait definitions
- ✅ Dynamic component registration
- ✅ Event system infrastructure  
- ✅ Extension registry framework

#### Phase 2 (Next)
- Native dynamic library loading
- WebAssembly plugin support
- Configuration management system
- Plugin discovery improvements

#### Phase 3 (Future)
- Plugin marketplace integration
- Advanced sandboxing
- Hot-reload capabilities
- Distributed plugin systems

### Example Usage

See `examples/plugin_demo.rs` for a complete demonstration of the extensibility features:

```bash
cargo run --example plugin_demo
```

This example shows:
- Event system creation and usage
- Plugin manager configuration
- Dynamic component registration
- Extension registry interaction

The extensibility system provides a solid foundation for future plugin development while maintaining the flexibility to evolve as requirements become clearer.

## Migration from Java

See `MIGRATION_NOTES.md` for detailed information about:
- API differences from Java implementation
- Porting guidelines for Java components
- Feature parity status
- Known limitations and workarounds