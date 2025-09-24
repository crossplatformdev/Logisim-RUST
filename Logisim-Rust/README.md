# Logisim-RUST

A modern Rust implementation of the Logisim digital logic simulator with native GUI using egui.

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- For GUI builds: Display server (X11, Wayland, or Windows/macOS native)

### Building

```bash
# Clone and navigate
git clone https://github.com/crossplatformdev/Logisim-RUST.git
cd Logisim-RUST/Logisim-Rust

# Build (headless mode)
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
# Show all available commands
just

# Development workflow
just ci           # Run all CI checks
just build-gui    # Build with GUI
just test         # Run tests
just run-gui      # Run with GUI
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

## Documentation

- [BUILD.md](./BUILD.md) - Detailed build instructions
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Architecture overview
- [docs/MIGRATION_NOTES.md](./docs/MIGRATION_NOTES.md) - Migration from Java

## License

This project is licensed under GPL-3.0. See [LICENSE.md](./LICENSE.md) for details.