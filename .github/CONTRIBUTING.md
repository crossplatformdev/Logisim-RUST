# Contributing to Logisim-RUST

Thank you for your interest in contributing to Logisim-RUST! This document provides guidelines for contributing to the project.

## Getting Started

1. **Open an issue first**: Before working on any significant changes, please open an issue to discuss your intent. This helps avoid duplicate work and ensures your contribution aligns with the project goals.

2. **Fork the repository**: Create your own fork of the repository to work in.

3. **Create a branch**: You **MUST** put your work either in the **main** branch or a branch of **main**.

4. **Make your changes**: Follow the development guidelines below.

5. **Create a pull request**: Once done, create a pull request against the **main** branch.

6. **Update changelog**: Ensure your change is also listed in `CHANGES.md` (minor changes may be omitted).

## Development Environment

### Prerequisites

See [BUILD.md](../BUILD.md) for detailed setup instructions. At minimum you need:

- Rust 1.70.0 or later
- Platform-specific GUI dependencies (for GUI development)
- Just (recommended): `cargo install just`

### Quick Setup

```bash
git clone https://github.com/crossplatformdev/Logisim-RUST.git
cd Logisim-RUST

# Install development dependencies
just install-deps

# Run all checks
just ci
```

## Development Workflow

### Code Quality Standards

Before submitting any pull request, ensure your code passes all quality checks:

```bash
# Run the full CI pipeline locally
just ci

# This includes:
# - Code formatting (rustfmt)
# - Linting (clippy) 
# - All tests
# - Documentation builds
```

### Individual Commands

```bash
# Format code
just fmt

# Check formatting
just fmt-check

# Run lints
just clippy

# Run tests
just test

# Build project
just build

# Build with GUI features (requires display server)
just build-gui
```

### Testing

#### Test Coverage Requirements

- **Unit tests**: All new functions should have unit tests
- **Integration tests**: New features should have integration tests
- **No disabled tests**: Do not disable or skip existing tests without justification
- **Platform compatibility**: Tests must pass on Windows, macOS, and Linux

#### Running Tests

```bash
# Run all tests (headless mode - works in CI)
just test

# Run specific test
just test-one test_name

# Run tests for specific package
just test-package logisim_core

# Generate coverage report
just coverage
```

### Code Style

#### Rust Style Guidelines

- **Format**: Use `rustfmt` with default settings (`just fmt`)
- **Lints**: Fix all `clippy` warnings (`just clippy`)
- **Documentation**: Document public APIs with doc comments
- **Error handling**: Use `thiserror` for custom error types
- **Naming**: Follow Rust naming conventions

#### Comments and Documentation

- Add comments for complex logic
- Document public APIs thoroughly
- Include examples in doc comments where helpful
- Keep comments up-to-date with code changes

```rust
/// Parse a circuit file and return the parsed representation
/// 
/// # Arguments
/// 
/// * `file_path` - Path to the .circ file to parse
/// 
/// # Returns
/// 
/// Returns `Ok(Circuit)` on success, or `Err(ParseError)` if the file
/// cannot be parsed or contains invalid data.
/// 
/// # Examples
/// 
/// ```rust
/// use logisim_core::CircParser;
/// 
/// let circuit = CircParser::load_file("example.circ")?;
/// println!("Loaded circuit: {}", circuit.name());
/// ```
pub fn load_file<P: AsRef<Path>>(file_path: P) -> Result<Circuit, ParseError> {
    // Implementation...
}
```

## Project Structure

### Workspace Organization

The project uses a Cargo workspace with two main packages:

- **`logisim_core`**: Core simulation engine (no GUI dependencies)
- **`logisim_ui`**: User interface components (optional GUI features)

### Feature Flags

The project uses feature flags to manage optional dependencies:

- **Default**: Headless mode, no GUI dependencies
- **`gui`**: Enables GUI components (egui/eframe)

This design ensures the project builds in CI environments without display servers.

### Directory Structure

```
Logisim-RUST/
├── logisim_core/           # Core simulation engine
│   ├── src/
│   ├── tests/
│   └── test_resources/
├── logisim_ui/             # UI components
│   ├── src/
│   └── tests/
├── example_schematics/     # Example circuit files
├── .github/                # CI/CD workflows
│   └── workflows/
├── docs/                   # Documentation
├── BUILD.md               # Build instructions
├── justfile               # Build automation
└── Cargo.toml            # Workspace configuration
```

## Contributing Guidelines

### Types of Contributions

#### Bug Fixes
- Include test case that reproduces the bug
- Verify fix works on all supported platforms
- Add regression tests if applicable

#### New Features
- Discuss the feature in an issue first
- Implement with appropriate abstractions
- Include comprehensive tests
- Update documentation

#### Documentation
- Improve existing documentation
- Add examples and tutorials
- Fix typos and unclear explanations

#### CI/CD Improvements
- Enhance build processes
- Add new quality checks
- Improve cross-platform compatibility

### Code Review Process

1. **Automated checks**: All CI checks must pass
2. **Manual review**: Maintainers will review code quality, design, and test coverage
3. **Testing**: Verify changes work as expected on multiple platforms
4. **Documentation**: Ensure documentation is updated if needed

### Git Commit Guidelines

#### Commit Messages

Use clear, descriptive commit messages:

```
feat: add chronogram visualization component

- Implement ChronogramModel for signal data storage
- Add ChronogramPanel for UI display
- Include tests for signal recording and playback
- Update documentation with usage examples

Closes #123
```

#### Commit Structure

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **test**: Adding or updating tests
- **chore**: Maintenance tasks, dependency updates

### Platform-Specific Considerations

#### GUI Development

GUI features are optional and gated behind the `gui` feature flag:

```rust
#[cfg(feature = "gui")]
use eframe::egui;

#[cfg(feature = "gui")]
pub fn create_gui() -> Result<(), GuiError> {
    // GUI implementation
}

#[cfg(not(feature = "gui"))]
pub fn create_gui() -> Result<(), GuiError> {
    Err(GuiError::NotSupported("GUI features not enabled".to_string()))
}
```

#### Cross-Platform Testing

Ensure your changes work on all supported platforms:

- **Linux**: Ubuntu 20.04+ (primary CI environment)
- **Windows**: Windows 10+ with MSVC toolchain
- **macOS**: macOS 10.15+ (x86_64 and ARM64)

### Dependencies

#### Adding Dependencies

Before adding new dependencies:

1. Check if existing dependencies can solve the problem
2. Ensure the dependency is actively maintained
3. Verify license compatibility (GPL-3.0 compatible)
4. Consider impact on build times and binary size

#### License Compatibility

All dependencies must be compatible with GPL-3.0:

```bash
# Check dependency licenses
just licenses

# Audit for security issues
just audit
```

Acceptable licenses include:
- MIT
- Apache-2.0
- BSD-2-Clause, BSD-3-Clause
- ISC
- GPL-3.0 (same license)

## Issue Reporting

### Bug Reports

Include the following information:

- **Environment**: Operating system, Rust version
- **Steps to reproduce**: Detailed steps
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Error messages**: Full error output if applicable
- **Circuit file**: If the bug involves a specific circuit file

### Feature Requests

- **Use case**: Describe the problem you're trying to solve
- **Proposed solution**: How you envision the feature working
- **Alternatives**: Other solutions you've considered
- **Impact**: Who would benefit from this feature

### Questions and Support

- Check existing documentation first
- Search existing issues
- Use GitHub Discussions for general questions
- Be specific about what you're trying to achieve

## License

By contributing to Logisim-RUST, you agree that your contributions will be licensed under the same GPL-3.0 license that covers the project.

All contributions must include appropriate license headers:

```rust
/*
 * This file is part of Logisim-RUST.
 *
 * Logisim-RUST is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Logisim-RUST is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Logisim-RUST. If not, see <https://www.gnu.org/licenses/>.
 */
```

## Getting Help

- **Documentation**: Check [BUILD.md](../BUILD.md) and `docs/` directory
- **Issues**: Search existing issues or create a new one
- **Discussions**: Use GitHub Discussions for general questions
- **Code Review**: Maintainers will provide feedback on pull requests

Thank you for contributing to Logisim-RUST!
