# Gray Code Module Documentation

## Overview

The Gray Code module provides implementations of Gray code components for Logisim-Rust. Gray code (also known as reflected binary code) is a binary numeral system where two successive values differ in only one bit. This property makes it particularly useful in digital circuits to minimize glitches and errors.

## Components

### Gray Code Library (`GrayComponents`)

The main library container that holds all Gray code tools available to users.

**Location**: `logisim_core/src/components/gray/components.rs`  
**Java Equivalent**: `com.cburch.gray.Components`

### Gray Code Incrementer (`GrayIncrementer`) 

Takes a multibit input and outputs the next value in the Gray code sequence.

**Location**: `logisim_core/src/components/gray/gray_incrementer.rs`  
**Java Equivalent**: `com.cburch.gray.GrayIncrementer`

**Key Features**:
- Converts binary to Gray code and vice versa
- Implements proper Gray code increment algorithm
- Supports arbitrary bit widths (up to 64 bits)
- Generates complete Gray code sequences

**Example**:
```rust
use logisim_core::components::gray::GrayIncrementer;
use logisim_core::signal::BusWidth;

let width = BusWidth::new(4);
let next = GrayIncrementer::next_gray(0b0100, width); // Returns 0b0110
```

### Simple Gray Counter (`SimpleGrayCounter`)

A fixed 4-bit Gray code counter with clock input.

**Location**: `logisim_core/src/components/gray/simple_gray_counter.rs`  
**Java Equivalent**: `com.cburch.gray.SimpleGrayCounter`

**Key Features**:
- Fixed 4-bit width
- Complete 16-step Gray code sequence
- Clock edge detection
- Position-to-Gray and Gray-to-position conversion

### Gray Counter (`GrayCounter`)

A configurable Gray code counter with advanced features.

**Location**: `logisim_core/src/components/gray/gray_counter.rs`  
**Java Equivalent**: `com.cburch.gray.GrayCounter`

**Key Features**:
- Configurable bit width
- User-editable labels
- Poke tool integration for direct value editing
- Custom icon support
- Attribute system integration

### Counter Data (`CounterData`)

State management for counter components including clock edge detection.

**Location**: `logisim_core/src/components/gray/counter_data.rs`  
**Java Equivalent**: `com.cburch.gray.CounterData`

**Key Features**:
- Clock edge detection (rising edge triggers)
- Value storage and retrieval
- State reset functionality
- Thread-safe instance data

### Counter Poker (`CounterPoker`)

Handles user interaction for direct counter value editing using the Poke Tool.

**Location**: `logisim_core/src/components/gray/counter_poker.rs`  
**Java Equivalent**: `com.cburch.gray.CounterPoker`

**Key Features**:
- Mouse event handling
- Keyboard input processing
- Edit state management
- Value validation

## Gray Code Theory

Gray code ensures that only one bit changes between consecutive values, which has several advantages:

1. **Glitch Reduction**: Eliminates spurious outputs during transitions
2. **Error Minimization**: Reduces errors in rotary encoders and mechanical systems
3. **Synchronization**: Easier to synchronize in asynchronous systems

### 4-bit Gray Code Sequence

| Decimal | Binary | Gray Code |
|---------|--------|-----------|
| 0       | 0000   | 0000      |
| 1       | 0001   | 0001      |
| 2       | 0010   | 0011      |
| 3       | 0011   | 0010      |
| 4       | 0100   | 0110      |
| 5       | 0101   | 0111      |
| 6       | 0110   | 0101      |
| 7       | 0111   | 0100      |
| 8       | 1000   | 1100      |
| 9       | 1001   | 1101      |
| 10      | 1010   | 1111      |
| 11      | 1011   | 1110      |
| 12      | 1100   | 1010      |
| 13      | 1101   | 1011      |
| 14      | 1110   | 1001      |
| 15      | 1111   | 1000      |

## Usage Examples

### Creating a Gray Code Library

```rust
use logisim_core::components::gray::GrayComponents;

let library = GrayComponents::new();
println!("Library: {}", library.display_name()); // "Gray Tools"
```

### Using the Gray Incrementer

```rust
use logisim_core::components::gray::GrayIncrementer;
use logisim_core::signal::BusWidth;

let incrementer = GrayIncrementer::new();
let width = BusWidth::new(3);

// Get complete 3-bit Gray sequence
let sequence = GrayIncrementer::get_gray_sequence(width);
// [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100]

// Convert binary to Gray
let gray = GrayIncrementer::binary_to_gray(5, width); // 0b101 -> 0b111

// Convert Gray to binary  
let binary = GrayIncrementer::gray_to_binary(0b111, width); // 0b111 -> 0b101
```

### Setting up a Counter

```rust
use logisim_core::components::gray::{GrayCounter, CounterData};
use logisim_core::signal::{BusWidth, Value};

let mut counter = GrayCounter::with_width_and_label(
    BusWidth::new(8), 
    "Main Counter".to_string()
);

let mut data = CounterData::new(None, Value::Low);

// Simulate clock edge
let output = counter.step(&mut data, Value::High);
```

## Testing

The Gray Code module includes comprehensive tests covering:

- Component creation and configuration
- Gray code arithmetic (binary ↔ Gray conversion)
- Clock edge detection
- Sequence generation and validation
- User interaction simulation
- Integration with existing Logisim infrastructure

Run tests with:
```bash
cargo test components::gray --lib
```

## Architecture Integration

The Gray Code module integrates seamlessly with Logisim-Rust's architecture:

- **Signal System**: Uses `BusWidth` and `Value` types
- **Component System**: Implements `ComponentTool` trait
- **Instance System**: Uses `InstanceData` for state management
- **Library System**: Provides `GrayComponents` as a tool library
- **UI Integration**: Supports Poke Tool interaction

## Migration from Java

This module maintains full compatibility with the original Java implementation:

| Java Class | Rust Module | Migration Status |
|------------|-------------|------------------|
| `Components.java` | `components.rs` | ✅ Complete |
| `CounterData.java` | `counter_data.rs` | ✅ Complete |
| `CounterPoker.java` | `counter_poker.rs` | ✅ Complete |
| `GrayCounter.java` | `gray_counter.rs` | ✅ Complete |
| `GrayIncrementer.java` | `gray_incrementer.rs` | ✅ Complete |
| `SimpleGrayCounter.java` | `simple_gray_counter.rs` | ✅ Complete |

All functionality has been preserved while adding Rust's memory safety and performance benefits.