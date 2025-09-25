# Migration Notes: Java Logisim-Evolution to Rust

This document provides detailed notes about migrating from the Java Logisim-Evolution implementation to the Rust version, including the foundational infrastructure and features like the chronogram.

## Overview

The Rust port maintains API compatibility and behavioral equivalence with the Java implementation while leveraging Rust's memory safety and performance benefits.

## Foundation Infrastructure Migration

### Utility Classes Migration

#### Java to Rust Mapping

| Java Class | Rust Module | Key Changes |
|------------|-------------|-------------|
| `StringUtil.java` | `util/string_util.rs` | Trait-based StringGetter, null → Option |
| `CollectionUtil.java` | `util/collection_util.rs` | Generic collections, type safety |
| `Cache.java` | `util/cache.rs` | Thread-safe caching, ownership semantics |
| `FileUtil.java` | `util/file_util.rs` | Cross-platform I/O, Result error handling |
| `LocaleManager.java` | `util/locale_manager.rs` | Simplified i18n, global state management |

#### Key Migration Patterns

**Null Safety:**
```java
// Java - null pointer risks
String value = getValue();
if (value != null) {
    return value.toUpperCase();
}
```

```rust
// Rust - compile-time null safety
let value = get_value();
value.map(|s| s.to_uppercase())
```

**Memory Management:**
```java
// Java - garbage collection
private Map<String, Object> cache = new HashMap<>();
```

```rust
// Rust - ownership and borrowing
use std::collections::HashMap;
let mut cache: HashMap<String, Box<dyn Any + Send + Sync>> = HashMap::new();
```

### Core Data Structures Migration

#### Geometric Types

| Java Class | Rust Module | Key Features |
|------------|-------------|--------------|
| `Direction.java` | `data/direction.rs` | Enum-based, rotation logic, display formatting |
| `Location.java` | `data/location.rs` | Immutable coordinates, grid snapping, spatial operations |
| `Bounds.java` | `data/bounds.rs` | Immutable rectangles, collision detection, transformations |
| `BitWidth.java` | `data/bit_width.rs` | Enhanced bit width, UI integration, mask generation |

#### Attribute System

**Java Implementation:**
```java
public abstract class Attribute<V> {
    public abstract V parse(String value);
    public String toDisplayString(V value);
}

public class AttributeSet {
    private Map<Attribute<?>, Object> values;
    public <V> V getValue(Attribute<V> attr);
}
```

**Rust Implementation:**
```rust
pub trait AttributeValue: Any + Debug + Clone + Send + Sync {
    fn to_display_string(&self) -> String;
    fn parse_from_string(s: &str) -> Result<Self, String> where Self: Sized;
}

pub struct Attribute<T: AttributeValue> {
    id: AttributeId,
    display_name: Option<String>,
}

pub struct AttributeSet {
    values: HashMap<AttributeId, Box<dyn Any + Send + Sync>>,
}
```

**Benefits of Rust Approach:**
- **Type Safety**: Compile-time verification of attribute types
- **Memory Safety**: No null pointer exceptions, automatic resource management
- **Performance**: Zero-cost abstractions, no runtime type checking overhead
- **Thread Safety**: Safe concurrent access with Send + Sync bounds

## Chronogram Feature Migration

### Java Implementation Analysis

#### Key Java Classes
- `ChronoPanel.java`: Main chronogram panel with split view
- `LeftPanel.java`: Signal list and controls  
- `RightPanel.java`: Timeline and waveform display
- `Signal.java`: Signal data representation
- `Model.java`: Data model for logging

#### Java Architecture Patterns
```java
public class ChronoPanel extends LogPanel implements Model.Listener {
    private Model model;
    private RightPanel rightPanel; 
    private LeftPanel leftPanel;
    // Swing/AWT UI components
}
```

### Rust Implementation Mapping

#### Structural Translation
| Java Class | Rust Module | Purpose |
|------------|-------------|---------|
| `ChronoPanel` | `panel.rs` | Main UI coordination |
| `LeftPanel` | `panel.rs` (signal list) | Signal names/values |
| `RightPanel` | `timeline.rs` + `waveform.rs` | Time axis + waveforms |
| `Signal` | `model.rs` (SignalData) | Signal storage |
| `Model` | `model.rs` (ChronogramModel) | Data management |

#### Key Architectural Changes

##### Event System
**Java (Observer Pattern):**
```java
public class ChronoPanel implements Model.Listener {
    @Override
    public void signalChanged(Signal signal, long time) {
        // Update display
    }
}
```

**Rust (Direct Integration):**
```rust
impl ChronogramPanel {
    pub fn update_from_simulation(&mut self, simulation: &Simulation) {
        // Extract current signal values directly
        let current_time = simulation.current_time();
        for signal_info in self.model.signals() {
            if let Some(signal) = simulation.get_node_signal(signal_info.id) {
                self.model.record_signal_change(signal_info.id, current_time, signal);
            }
        }
    }
}
```

##### UI Framework Migration
**Java (Swing/AWT):**
- Complex layout managers
- Manual paint methods
- Event listener registration
- Thread-unsafe UI updates

**Rust (egui):**
- Immediate mode rendering
- Automatic layout
- Built-in event handling
- Thread-safe by design

### Feature Parity Status

#### ✅ Fully Implemented
- [x] Signal data model with time-series storage
- [x] Timeline rendering with automatic tick spacing
- [x] Digital signal waveform display
- [x] Bus signal rendering with value labels
- [x] Zoom and scroll navigation
- [x] Signal selection and highlighting
- [x] Time cursor positioning
- [x] Text export functionality
- [x] Integration with simulation engine

#### ⚠️ Partially Implemented
- [ ] Signal selection dialog (placeholder UI)
- [ ] Bus width detection from netlist
- [ ] Named signal identification
- [ ] Drag-and-drop signal reordering

#### ❌ Not Yet Implemented
- [ ] Image export (PNG/SVG)
- [ ] Print functionality
- [ ] Signal grouping/hierarchies
- [ ] Custom color schemes
- [ ] Measurement cursors
- [ ] Signal value search

### API Differences

#### Signal Creation
**Java:**
```java
Signal signal = new Signal(node, name, width);
model.addSignal(signal);
```

**Rust:**
```rust
let signal_info = SignalInfo::new(node_id, name.to_string(), width, index);
model.add_signal(signal_info);
```

#### Signal Value Recording
**Java:**
```java
signal.setValue(time, value);
```

**Rust:**
```rust
model.record_signal_change(node_id, time, signal);
```

#### Timeline Navigation
**Java:**
```java
rightPanel.setZoom(zoomLevel);
rightPanel.setScrollOffset(offset);
```

**Rust:**
```rust
timeline.set_zoom(zoom_level);
timeline.set_scroll_offset(offset);
```

### Memory Management Differences

#### Java Approach
- Garbage collection handles memory automatically
- Potential memory leaks with long simulations
- No explicit cleanup required

#### Rust Approach
- Zero-cost abstractions with compile-time safety
- Explicit ownership model prevents leaks
- RAII ensures proper resource cleanup

```rust
// Rust automatically cleans up when ChronogramModel is dropped
impl Drop for ChronogramModel {
    fn drop(&mut self) {
        // Cleanup happens automatically due to ownership
    }
}
```

### Performance Characteristics

#### Java Implementation
- JVM warm-up time
- Garbage collection pauses
- Reflection overhead for component updates
- Swing EDT bottlenecks

#### Rust Implementation
- Zero-cost abstractions
- No garbage collection pauses
- Compile-time optimization
- Efficient memory layout

### Error Handling Differences

#### Java (Exceptions)
```java
try {
    model.addSignal(signal);
} catch (InvalidSignalException e) {
    showError("Invalid signal: " + e.getMessage());
}
```

#### Rust (Result Types)
```rust
match model.add_signal(signal_info) {
    Ok(()) => {/* success */},
    Err(e) => show_error(&format!("Invalid signal: {}", e)),
}
```

### Threading Model

#### Java Chronogram
- Swing Event Dispatch Thread for UI
- Background simulation thread
- Manual synchronization required
- Risk of deadlocks

#### Rust Chronogram
- egui handles threading automatically
- Send/Sync traits ensure thread safety
- Compiler prevents data races
- No explicit synchronization needed

### Configuration and Constants

#### Migrated Constants
```rust
// Equivalent to Java ChronoPanel constants
pub const SIGNAL_HEIGHT: f32 = 30.0;    // ChronoPanel.SIGNAL_HEIGHT
pub const HEADER_HEIGHT: f32 = 20.0;     // ChronoPanel.HEADER_HEIGHT  
pub const GAP: f32 = 2.0;                // ChronoPanel.GAP
```

### Testing Strategy Migration

#### Java Testing
- JUnit test framework
- Mock objects for simulation
- UI testing with fest-swing
- Manual integration testing

#### Rust Testing
- Built-in test framework
- Property-based testing with proptest
- Mock-free testing with controlled simulations
- Automated integration tests

### Build System Migration

#### Java (Gradle)
```gradle
dependencies {
    implementation 'javax.swing:swing'
    implementation 'java.awt:awt'
}
```

#### Rust (Cargo)
```toml
[dependencies]
egui = { version = "0.30", optional = true }
eframe = { version = "0.30", optional = true }

[features]
default = ["gui"]
gui = ["egui", "eframe"]
```

### Cross-Platform Considerations

#### Java
- "Write once, run anywhere" philosophy
- Platform-specific look and feel
- JVM dependency

#### Rust
- Native compilation for each platform
- Consistent look across platforms
- No runtime dependencies

### Migration Best Practices

#### 1. Preserve Behavioral Compatibility
- Test outputs match Java implementation
- Maintain same file format support
- Keep equivalent user interactions

#### 2. Leverage Rust Strengths
- Use ownership for automatic resource management
- Employ type system for correctness
- Optimize with zero-cost abstractions

#### 3. Gradual Migration Strategy
- Start with core data structures
- Add UI components incrementally
- Maintain Java version for reference

#### 4. Testing Equivalence
- Compare outputs with Java version
- Verify timing accuracy
- Test edge cases and error conditions

### Known Limitations

#### Current Rust Implementation
1. **GUI Backend**: Limited to egui (vs Java's full Swing)
2. **Platform Integration**: Less native feel than Java LAF
3. **Plugin System**: Not yet implemented
4. **Advanced Features**: Some Java features pending

#### Workarounds
1. **Custom Rendering**: Implement missing widgets in egui
2. **Platform APIs**: Use platform-specific crates where needed
3. **Extensibility**: Design for future plugin architecture
4. **Feature Parity**: Prioritize most-used features first

### Future Migration Tasks

#### Short Term
- [ ] Complete signal selection dialog
- [ ] Add image export capability
- [ ] Implement measurement cursors
- [ ] Add signal search functionality

#### Medium Term
- [ ] Plugin architecture design
- [ ] Advanced component library
- [ ] VHDL/Verilog export
- [ ] Network simulation

#### Long Term
- [ ] Web-based version
- [ ] Mobile platform support
- [ ] Cloud simulation backend
- [ ] Collaborative editing

### Resources

#### Java Codebase Reference
- Original repository: https://github.com/logisim-evolution/logisim-evolution
- Chronogram code: `src/main/java/com/cburch/logisim/gui/chrono/`
- Documentation: User manual chronogram section

#### Rust Implementation
- Current implementation: `logisim_ui/src/gui/chronogram/`
- Tests: `logisim_ui/tests/chronogram_tests.rs`
- Documentation: This file and `ARCHITECTURE.md`

#### Community Resources
- Rust GUI development: https://areweguiyet.com/
- egui documentation: https://docs.rs/egui/
- Digital simulation in Rust: Community forums and crates.io