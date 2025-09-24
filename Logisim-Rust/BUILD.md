# Building Logisim-RUST

This document explains how to build, test, and package Logisim-RUST across different platforms.

## Prerequisites

### Required Software

- **Rust** 1.70.0 or later (stable toolchain recommended)
- **Git** for version control
- **Just** (optional but recommended for automation) - install with `cargo install just`

### Platform-Specific Dependencies

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev

# Fedora
sudo dnf install libxcb-devel libxkbcommon-devel openssl-devel

# Arch Linux
sudo pacman -S libxcb libxkbcommon
```

#### macOS
```bash
# Install Xcode command line tools
xcode-select --install

# Optional: Homebrew for development tools
brew install just
```

#### Windows
```bash
# Install Visual Studio Build Tools or Visual Studio with C++ support
# Rust will automatically detect and use the appropriate toolchain

# Optional: Package managers
# Chocolatey: choco install just
# Scoop: scoop install just
```

## Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/crossplatformdev/Logisim-RUST.git
cd Logisim-RUST/Logisim-Rust
```

### 2. Build and Test (Headless Mode)
```bash
# Using Just (recommended)
just ci

# Or using Cargo directly
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

### 3. Build with GUI Support
```bash
# Using Just
just build-gui

# Or using Cargo
cargo build --workspace --features gui
```

## Build Modes

### Headless Mode (Default)
The default build mode excludes GUI components and is suitable for:
- CI/CD environments
- Server deployments
- Automated testing
- Command-line usage

```bash
cargo build --workspace
cargo test --workspace
```

### GUI Mode
Includes the full graphical user interface:
```bash
cargo build --workspace --features gui
cargo run --package logisim_ui --features gui
```

## Development Workflow

### Using Just (Recommended)

Just provides convenient commands for common development tasks:

```bash
# Show all available commands
just

# Development workflow
just fmt          # Format code
just clippy       # Run lints
just test         # Run tests
just build        # Build project
just ci           # Run all CI checks

# With GUI features
just build-gui    # Build with GUI
just test-gui     # Test with GUI
just run-gui      # Run with GUI

# Documentation
just doc          # Generate and open docs

# Quality assurance
just coverage     # Generate coverage report
just audit        # Security audit
just licenses     # Check dependency licenses
```

### Using Cargo Directly

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy lints
cargo clippy --workspace --all-targets -- -D warnings

# Build
cargo build --workspace                    # Debug build
cargo build --workspace --release         # Release build
cargo build --workspace --features gui    # With GUI features

# Test
cargo test --workspace                     # All tests
cargo test --package logisim_core         # Specific package
cargo test test_name                       # Specific test

# Documentation
cargo doc --workspace --open --no-deps

# Run
cargo run --package logisim_ui                           # Headless mode
cargo run --package logisim_ui --features gui           # GUI mode
cargo run --package logisim_ui -- circuit_file.circ     # With file
```

## Testing

### Test Organization

- **Unit tests**: Located alongside source code (`src/`)
- **Integration tests**: Located in `tests/` directories
- **Doc tests**: Embedded in documentation comments

### Running Tests

```bash
# All tests (headless mode)
just test
# or
cargo test --workspace

# Specific package
cargo test --package logisim_core

# Specific test
cargo test test_mainboard_circ_exists

# With output
cargo test -- --nocapture

# Test with GUI features (requires display server)
just test-gui
# or
cargo test --workspace --features gui
```

### Test Coverage

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
just coverage
# or
cargo tarpaulin --workspace --out html --output-dir coverage/
```

## Packaging and Distribution

### Development Packages

```bash
# Linux AppImage
just package-linux

# macOS App Bundle
just package-macos

# Windows Executable
just package-windows
```

### Release Builds

```bash
# Release build
just build-release
# or
cargo build --workspace --release --features gui

# Full release check
just release-check
```

### CI/CD Integration

The project includes comprehensive GitHub Actions workflows:

- **rust-build-test.yml**: Build and test on all platforms
- **rust-clippy.yml**: Linting and code quality
- **rust-fmt.yml**: Code formatting checks
- **rust-codeql.yml**: Security analysis
- **rust-nightly.yml**: Nightly Rust compatibility
- **rust-release.yml**: Automated releases and packaging

## Troubleshooting

### Common Issues

#### GUI Build Fails on Headless Systems
```bash
# Error: winit platform not supported
# Solution: Build without GUI features
cargo build --workspace  # Excludes GUI by default
```

#### Missing System Dependencies (Linux)
```bash
# Error: Could not find X11 libraries
# Solution: Install development packages
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
```

#### Test Failures
```bash
# Check test file paths
ls -la logisim_core/test_resources/

# Run specific failing test with output
cargo test test_name -- --nocapture
```

### Build Environment

#### Rust Version
```bash
# Check Rust version
rustc --version
cargo --version

# Update Rust
rustup update
```

#### Feature Detection
```bash
# Check available features for each package
cargo metadata --format-version 1 | jq '.packages[] | select(.name == "logisim_ui") | .features'
```

## IDE Integration

### VS Code
Recommended extensions:
- rust-analyzer
- CodeLLDB (debugging)
- Better TOML
- GitLens

### CLion/IntelliJ
- Rust plugin
- TOML plugin

### Vim/Neovim
- rust.vim
- coc-rust-analyzer (for coc.nvim)

## Contributing

Before submitting contributions:

1. Run the full CI check: `just ci`
2. Ensure tests pass: `just test`
3. Check code formatting: `just fmt-check`
4. Run lints: `just clippy`
5. Update documentation if needed

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for detailed contribution guidelines.

## License Compliance

This project is licensed under GPL-3.0. All dependencies are checked for license compatibility:

```bash
# Check dependency licenses
just licenses
# or
cargo license

# Generate license report
cargo license --json > licenses.json
```

## Performance and Profiling

### Release Builds
Always use release builds for performance testing:
```bash
cargo build --workspace --release --features gui
```

### Profiling (Linux)
```bash
# Install perf
sudo apt-get install linux-perf

# Profile application
cargo build --workspace --release --features gui
perf record target/release/logisim_ui
perf report
```

### Memory Analysis
```bash
# Install valgrind
sudo apt-get install valgrind

# Run with valgrind
cargo build --workspace --features gui
valgrind --tool=memcheck target/debug/logisim_ui
```

## Cross-Compilation

For advanced users who need to build for different targets:

```bash
# Install target
rustup target add x86_64-unknown-linux-musl

# Build for target
cargo build --workspace --target x86_64-unknown-linux-musl
```

Note: GUI features may require platform-specific dependencies and may not be available for all targets.