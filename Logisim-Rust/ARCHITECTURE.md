# Logisim-RUST Architecture

This document provides an overview of the Logisim-RUST project architecture and crate organization.

> **Note**: This is a placeholder file for the workspace setup. For detailed architecture documentation, see [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## Workspace Structure

The Logisim-RUST project is organized as a Cargo workspace with the following crates:

### Core Crates

- **`logisim_core`**: Core simulation engine and circuit representation
  - Digital logic simulation kernel
  - Component definitions and behaviors
  - Circuit parsing and data structures
  - File I/O for `.circ` format

- **`logisim_formats`**: File format parsers and writers
  - Logisim-Evolution `.circ` file format support
  - Import/export utilities for other formats
  - File validation and error reporting

- **`logisim_ui`**: User interface components
  - GUI implementation using egui framework
  - Canvas, toolbox, and editor components
  - Feature-gated for headless operation

- **`logisim_cli`**: Command-line interface (optional)
  - Headless simulation and validation tools
  - Batch processing utilities
  - CI/CD integration support

## Design Principles

1. **Memory Safety**: Leveraging Rust's ownership system for bug-free simulation
2. **Performance**: Zero-cost abstractions and efficient algorithms
3. **Cross-Platform**: Support for Windows, macOS, Linux, and WebAssembly
4. **Compatibility**: Full compatibility with Logisim-Evolution file formats
5. **Modularity**: Clean separation of concerns between crates

## Build Configuration

- **Workspace**: Cargo workspace with shared dependencies and configuration
- **Features**: GUI components are feature-gated for headless CI/CD
- **Testing**: Comprehensive test suite across all crates
- **Documentation**: Inline documentation with examples

For detailed architecture information, see the [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) file.