# Changelog

All notable changes to the Logisim-RUST project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-12-19

### Added
- **Core simulation engine** with Rust implementation
  - Component system with built-in logic gates (AND, OR, NOT, NAND, NOR, XOR, XNOR)
  - Pin system for component I/O
  - Clocked latches and sequential logic support
  - Event-driven simulation with proper timing
  - Signal propagation with support for multiple bit widths
  - Netlist management for component connections

- **Circuit file format support**
  - Full `.circ` file parsing and serialization
  - Support for Logisim-Evolution circuit format
  - Component attributes and properties
  - Project-level settings and configurations
  - ROM content parsing with run-length encoding

- **GUI Application (egui-based)**
  - Modern cross-platform GUI using egui framework
  - Circuit editor with component placement and wiring
  - Simulation controls and visualization
  - Chronogram/timing diagram display
  - File operations (open, save, export)
  - Headless mode for CI/CD compatibility

- **Build and Development System**
  - Comprehensive `justfile` with development tasks
  - Cross-platform build support (Linux, Windows, macOS)
  - Automated testing with 77+ test cases
  - Code quality tools (clippy, rustfmt)
  - Release packaging system

- **CI/CD Pipeline**
  - GitHub Actions workflows for testing and releases
  - Cross-platform automated builds
  - Code quality and security checks
  - Automated release packaging (AppImage, DMG, ZIP)
  - License compliance verification

- **Documentation**
  - Comprehensive build instructions
  - Developer contribution guidelines
  - API documentation with examples
  - Platform-specific installation guides

### Technical Details
- **Language**: Rust 2021 edition
- **Minimum Supported Rust Version**: 1.70.0
- **GUI Framework**: egui 0.30
- **License**: GPL-3.0-or-later
- **Test Coverage**: 77 tests covering core functionality
- **Platform Support**: Linux, Windows, macOS (GUI requires display server)

### Design Principles
- **Memory Safety**: Leveraging Rust's ownership system
- **Performance**: Native compilation for optimal speed
- **Compatibility**: Supporting existing Logisim circuit files
- **Modularity**: Clean separation between core logic and UI
- **Testing**: Comprehensive test suite for reliability

[1.0.0]: https://github.com/crossplatformdev/Logisim-RUST/releases/tag/v1.0.0