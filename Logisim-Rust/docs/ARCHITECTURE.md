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

## Extensibility Framework

### Overview

Logisim-RUST provides a comprehensive extensibility framework that allows developers to extend the simulator with custom components, observers, and plugins. The framework is designed to be type-safe, performant, and maintainable while providing flexibility for advanced users.

**⚠️ API Stability Warning**: The extensibility APIs are currently **UNSTABLE** and may change in future versions. Plugin developers should expect breaking changes and plan for migration.

### Extension Points

#### 1. Observer Pattern Integration

The observer pattern allows external code to monitor and react to simulation events, component state changes, and system events without modifying core simulator code.

##### Simulation Observers
Monitor simulation lifecycle events:
```rust
use logisim_core::{SimulationObserver, SimulationEvent, ObserverResult, ObserverId};

pub struct MySimulationObserver {
    id: ObserverId,
}

impl SimulationObserver for MySimulationObserver {
    fn id(&self) -> ObserverId { self.id }
    fn name(&self) -> &str { "My Simulation Observer" }
    
    fn on_simulation_event(&mut self, event: &SimulationEvent) -> ObserverResult<()> {
        match event {
            SimulationEvent::Started { timestamp } => {
                println!("Simulation started at time {}", timestamp.0);
            }
            SimulationEvent::Stopped { timestamp } => {
                println!("Simulation stopped at time {}", timestamp.0);
            }
            // Handle other events...
            _ => {}
        }
        Ok(())
    }
}
```

##### Component Observers
Monitor component behavior and state changes:
```rust
impl ComponentObserver for MyComponentObserver {
    fn on_component_event(&mut self, event: &ComponentEvent) -> ObserverResult<()> {
        match event {
            ComponentEvent::StateChanged { component_id, timestamp } => {
                println!("Component {} changed state at time {}", component_id, timestamp.0);
            }
            ComponentEvent::OutputChanged { component_id, pin_name, new_signal, .. } => {
                println!("Component {} output {} changed to {:?}", component_id, pin_name, new_signal);
            }
            _ => {}
        }
        Ok(())
    }
}
```

##### System Observers
Monitor system-wide events like plugin loading:
```rust
impl SystemObserver for MySystemObserver {
    fn on_plugin_loaded(&mut self, plugin_name: &str) -> ObserverResult<()> {
        println!("Plugin {} loaded successfully", plugin_name);
        Ok(())
    }
}
```

#### 2. Dynamic Component Registration

The component registry allows plugins to register custom component types that can be instantiated at runtime.

##### Component Factory Implementation
```rust
use logisim_core::{DynamicComponentFactory, ComponentInfo, ComponentId, Component, PluginResult};

pub struct MyComponentFactory;

impl DynamicComponentFactory for MyComponentFactory {
    fn component_type(&self) -> &str {
        "MyCustomComponent"
    }
    
    fn component_info(&self) -> ComponentInfo {
        ComponentInfo {
            name: "My Custom Component".to_string(),
            category: "Custom Logic".to_string(),
            description: "A custom component with special features".to_string(),
            icon_path: None,
            input_count: Some(2),
            output_count: Some(1),
        }
    }
    
    fn create_component(&self, id: ComponentId) -> PluginResult<Box<dyn Component>> {
        Ok(Box::new(MyCustomComponent::new(id)))
    }
}
```

##### Registration with Plugin Manager
```rust
// Register the factory with the plugin manager
plugin_manager.register_component_factory(
    Box::new(MyComponentFactory), 
    "my_plugin"
)?;

// Create components dynamically
let component = plugin_manager.create_dynamic_component("MyCustomComponent")?;
```

#### 3. Plugin System Architecture

The plugin system provides a framework for loading and managing external code that extends Logisim-RUST functionality.

##### Plugin Library Implementation
```rust
use logisim_core::{PluginLibrary, PluginInfo, PluginResult, PluginCapabilities};

pub struct MyPlugin {
    info: PluginInfo,
}

impl PluginLibrary for MyPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn components(&self) -> Vec<ComponentInfo> {
        vec![/* component definitions */]
    }
    
    fn create_component(&self, component_type: &str, id: ComponentId) -> PluginResult<Box<dyn Component>> {
        match component_type {
            "MyCustomComponent" => Ok(Box::new(MyCustomComponent::new(id))),
            _ => Err(PluginError::PluginNotFound(format!("Unknown component: {}", component_type)))
        }
    }
    
    fn initialize(&mut self) -> PluginResult<()> {
        // Plugin initialization logic
        Ok(())
    }
    
    fn cleanup(&mut self) -> PluginResult<()> {
        // Plugin cleanup logic
        Ok(())
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities {
            observer_support: true,
            custom_events: true,
            custom_rendering: false,
            ui_extensions: false,
            custom_formats: false,
            ..Default::default()
        }
    }
}
```

### API Contracts

#### Version Compatibility

All plugins must specify the API version they were built against:
```rust
pub const API_VERSION: u32 = 1;

impl PluginLibrary for MyPlugin {
    fn api_version(&self) -> u32 {
        API_VERSION
    }
}
```

The simulator will check API compatibility during plugin loading and may reject incompatible plugins.

#### Resource Management

Plugins must implement proper resource management:
- **Initialization**: Set up resources in `initialize()`
- **Cleanup**: Release resources in `cleanup()`
- **Error Handling**: Use `PluginResult<T>` for error propagation
- **Thread Safety**: All plugin interfaces must be `Send + Sync`

#### Component Lifecycle

Custom components must follow the standard component lifecycle:
1. **Creation**: Implement `Component` trait
2. **Reset**: Handle `reset()` calls properly  
3. **Update**: Implement `update()` for signal propagation
4. **Clock Handling**: Implement `clock_edge()` for sequential components

### Unstable APIs

The following APIs are explicitly marked as **UNSTABLE** and may change:

#### Observer System (`logisim_core::observers`)
- `SimulationObserver` trait
- `ComponentObserver` trait  
- `SystemObserver` trait
- Observer manager implementations
- Event structures (`SimulationEvent`, `ComponentEvent`)

#### Plugin System (`logisim_core::integrations::plugins`)
- `PluginLibrary` trait extensions (beyond basic interface)
- `DynamicComponentFactory` trait
- `ComponentRegistry` structure
- `PluginCapabilities` structure
- `PluginConfig` and `ResourceLimits`

#### Extension Utilities
- Observer registration and management functions
- Component factory registration system
- Plugin lifecycle management

### Stable APIs

The following APIs are considered **STABLE** and will maintain backward compatibility:

#### Core Component System
- `Component` trait (basic interface)
- `ComponentId`, `Pin`, `UpdateResult` types
- `Signal`, `Value`, `BusWidth`, `Timestamp` types

#### Basic Plugin Interface
- `PluginInfo` structure
- `ComponentInfo` structure  
- `PluginError` and `PluginResult` types
- Basic `PluginLibrary` methods (`info()`, `components()`, `create_component()`)

### Migration Guidelines

When API changes occur:

1. **Monitor API Version**: Check `logisim_core::API_VERSION` for compatibility
2. **Use Feature Flags**: Check `logisim_core::is_feature_enabled()` for optional features
3. **Handle Errors Gracefully**: Use `PluginResult` error handling throughout
4. **Follow Deprecation Notices**: Watch for deprecation warnings in logs
5. **Test Compatibility**: Regularly test plugins against new simulator versions

### Performance Considerations

#### Observer Performance
- Observers are called synchronously during simulation
- Use `interested_in_event()` to filter unnecessary events
- Keep observer logic lightweight to avoid simulation slowdown
- Consider async processing for heavy operations

#### Component Performance  
- Minimize computation in `update()` method
- Use appropriate propagation delays
- Cache expensive calculations when possible
- Implement efficient state management

#### Memory Management
- Release resources in `cleanup()` methods
- Avoid memory leaks in long-running simulations
- Use appropriate data structures for component state
- Monitor memory usage in complex plugins

### Security Considerations

#### Plugin Sandboxing
- Plugins run in the same process as the simulator
- No current sandboxing mechanism (planned for future versions)
- Plugin developers are responsible for security

#### Resource Limits
- Configure `ResourceLimits` for plugin instances
- Monitor CPU and memory usage
- Implement timeouts for long-running operations

#### Input Validation
- Validate all plugin inputs and parameters
- Use type-safe interfaces where possible
- Handle malformed data gracefully

### Testing Framework

#### Plugin Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_initialization() {
        let mut plugin = MyPlugin::new();
        assert!(plugin.initialize().is_ok());
        assert!(plugin.cleanup().is_ok());
    }
    
    #[test]
    fn test_component_creation() {
        let plugin = MyPlugin::new();
        let result = plugin.create_component("MyCustomComponent", ComponentId::new(1));
        assert!(result.is_ok());
    }
}
```

#### Observer Testing
```rust
#[test]
fn test_observer_events() {
    let mut observer = MyObserver::new();
    let event = SimulationEvent::Started { timestamp: Timestamp(0) };
    assert!(observer.on_simulation_event(&event).is_ok());
}
```

### Example Plugin Structure

See `examples/stub_plugin/` for a complete example plugin implementation that demonstrates:
- Custom component creation (`CustomXOR`, `CustomCounter`)
- Observer implementation (`PluginEventLogger`, `ComponentStateTracker`)
- Dynamic component factories
- Plugin lifecycle management
- Parameter validation and configuration

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