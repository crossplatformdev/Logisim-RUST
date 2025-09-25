# Logisim-RUST

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-GPL--3.0--or--later-green)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)

A modern Rust implementation of the Logisim digital logic simulator with native GUI using egui.

## 🎉 Version 1.0.0 Release

This is the first official release of Logisim-RUST! This version provides a solid foundation for digital logic design and simulation with modern Rust performance and safety.

### 📦 Downloads

**Pre-built binaries coming soon with release process!**

For now, build from source using the instructions below.

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+ (latest stable recommended)
- For GUI builds: Display server (X11, Wayland, or Windows/macOS native)

### Building

```bash
# Clone and navigate
git clone https://github.com/crossplatformdev/Logisim-RUST.git
cd Logisim-RUST/Logisim-Rust

# Build (headless mode - CI compatible)
cargo build --workspace

# Build with GUI support
cargo build --workspace --features gui

# Run tests
cargo test --workspace

# Run with GUI
cargo run --package logisim_ui --features gui
```

### Using Just (Recommended)

```bash
# Install just task runner
cargo install just

# Show all available commands
just

# Development workflow
just ci           # Run all CI checks (format, lint, test)
just build-gui    # Build with GUI features
just test         # Run all tests
just run-gui      # Run with GUI
just release-check # Pre-release validation
```

## Features

- **Memory Safe**: Written in Rust for guaranteed memory safety
- **High Performance**: Native compilation with zero-cost abstractions
- **Cross-Platform**: Windows, macOS, Linux, and Web (WASM) support
- **Modern GUI**: Immediate mode GUI with egui
- **Headless Mode**: For automation and testing
- **File Compatibility**: Reads Logisim-Evolution circuit files

## Architecture

- `logisim_core/`: Core simulation engine and circuit representation
- `logisim_ui/`: User interface components using egui
- `example_schematics/`: Example circuits for testing

## Contributing

### For Contributors

We welcome contributions to Logisim-RUST! Please see our contribution guidelines:

- [BUILD.md](./BUILD.md#contributing) - Build instructions and development setup
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Architecture overview and design principles

#### Kernel Hacking

**Working on the simulation kernel?** The core simulation engine is in `logisim_core/` and uses an event-driven architecture. Key areas for kernel development:

- **Event Processing**: `src/event.rs` - Priority queue-based event scheduling
- **Simulation Loop**: `src/simulation.rs` - Main simulation orchestration  
- **Signal Propagation**: `src/netlist.rs` - Circuit connectivity and signal routing
- **Component Integration**: `src/component.rs` - Component trait and extensibility points

See the [Simulation Kernel Control Flow](./docs/ARCHITECTURE.md#simulation-kernel-control-flow) section for detailed architecture information, event processing diagrams, and extensibility points.

**Debugging Simulation Issues:**
```bash
# Enable debug logging
RUST_LOG=debug cargo run --features gui

# Run specific simulation tests
cargo test --package logisim_core simulation

# Profile simulation performance
cargo build --release --features gui
```

## Documentation

- [BUILD.md](./BUILD.md) - Detailed build instructions
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Architecture overview
- [docs/MIGRATION_NOTES.md](./docs/MIGRATION_NOTES.md) - Migration from Java

## License

This project is licensed under GPL-3.0. See [LICENSE.md](./LICENSE.md) for details.