# Logisim-RUST build automation with Just
# https://github.com/casey/just

# List all available commands
default:
    @just --list

# Clean all build artifacts
clean:
    cargo clean

# Format all code
fmt:
    cargo fmt --all

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Run clippy with pedantic lints (warnings only)
clippy-pedantic:
    cargo clippy --workspace --all-targets -- -W clippy::pedantic

# Build the project (headless mode by default)
build:
    cargo build --workspace

# Build with GUI features (requires display server)
build-gui:
    cargo build --workspace --features gui

# Build in release mode
build-release:
    cargo build --workspace --release

# Build release with GUI features
build-release-gui:
    cargo build --workspace --release --features gui

# Run all tests (headless mode)
test:
    cargo test --workspace

# Run tests with GUI features (requires display server)
test-gui:
    cargo test --workspace --features gui

# Run a specific test
test-one test_name:
    cargo test --workspace {{test_name}}

# Run tests for a specific package
test-package package:
    cargo test --package {{package}}

# Generate and open documentation
doc:
    cargo doc --workspace --open --no-deps

# Generate documentation with GUI features
doc-gui:
    cargo doc --workspace --open --no-deps --features gui

# Run the application in headless mode
run:
    cargo run --package logisim_ui

# Run the application with GUI
run-gui:
    cargo run --package logisim_ui --features gui

# Run with a specific circuit file (headless)
run-file file:
    cargo run --package logisim_ui -- "{{file}}"

# Run with a specific circuit file (GUI)
run-file-gui file:
    cargo run --package logisim_ui --features gui -- "{{file}}"

# Check code without building
check:
    cargo check --workspace

# Check with GUI features
check-gui:
    cargo check --workspace --features gui

# Run all quality checks (format, clippy, tests)
ci: fmt-check clippy test
    @echo "All CI checks passed!"

# Run all quality checks including GUI
ci-gui: fmt-check clippy test-gui
    @echo "All CI checks (including GUI) passed!"

# Install development dependencies
install-deps:
    cargo install cargo-tarpaulin cargo-license cargo-audit

# Generate code coverage report
coverage:
    cargo tarpaulin --workspace --out html --output-dir coverage/

# Audit dependencies for security vulnerabilities
audit:
    cargo audit

# Check licenses of dependencies
licenses:
    cargo license

# Update dependencies
update:
    cargo update

# Set up pre-commit hooks
setup-hooks:
    cp .pre-commit-config.yaml.dist .pre-commit-config.yaml
    pre-commit install

# Run benchmarks (if any)
bench:
    cargo bench --workspace

# Profile the application (requires additional setup)
profile:
    cargo build --workspace --release
    # Add profiling commands here

# Package for distribution (Linux AppImage)
package-linux:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --release --features gui --package logisim_ui
    mkdir -p dist/AppDir/usr/bin
    cp target/release/logisim_ui dist/AppDir/usr/bin/logisim-rust
    echo "Linux package created in dist/"

# Package for distribution (macOS)
package-macos:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --release --features gui --package logisim_ui
    mkdir -p dist/Logisim-RUST.app/Contents/MacOS
    cp target/release/logisim_ui dist/Logisim-RUST.app/Contents/MacOS/logisim-rust
    echo "macOS package created in dist/"

# Package for distribution (Windows)
package-windows:
    cargo build --release --features gui --package logisim_ui
    mkdir -p dist
    copy target\\release\\logisim_ui.exe dist\\logisim-rust.exe
    echo "Windows package created in dist/"

# Example circuit tests
test-examples:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Testing example circuit files..."
    for file in example_schematics/logisim_evolution/*.circ; do
        if [ -f "$file" ]; then
            echo "Testing: $file"
            cargo run --package logisim_ui -- "$file" || echo "Failed to load $file"
        fi
    done

# Development server (watch mode)
dev:
    cargo watch -x "build --workspace" -x "test --workspace"

# Development server with GUI
dev-gui:
    cargo watch -x "build --workspace --features gui" -x "test --workspace"

# Release preparation checklist
release-check:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🔍 Running pre-release checks..."
    
    echo "✅ Checking code format..."
    just fmt-check
    
    echo "✅ Running clippy..."
    just clippy
    
    echo "✅ Running tests..."
    just test
    
    echo "✅ Checking licenses..."
    just licenses
    
    echo "✅ Auditing dependencies..."
    just audit
    
    echo "✅ Building release..."
    just build-release
    
    echo "🎉 Release checks completed successfully!"