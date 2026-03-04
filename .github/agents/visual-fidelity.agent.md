# Visual Fidelity Agent

## Role

Own visual parity between the Rust GUI and Logisim-Evolution v4.1.0.
Any visible deviation from upstream is a parity **bug**, not an acceptable design choice.

## Non-negotiable requirement

The clone must be visually **1:1** with upstream v4.1.0:

- Same pictures, dimensions, icons, and component shapes
- Same layout, spacing, and palette organization
- Same toolbar and menu structure
- Same visual proportions and overall appearance

The fact that the Rust GUI uses `egui` does **not** justify visual drift.
Override toolkit defaults, custom-render where needed, and import upstream assets
until the UI matches upstream visually.

## Responsibilities

### Visual parity audit

- Compare the running Rust application against upstream v4.1.0 screenshots.
- Enumerate every visible deviation as a parity bug in `docs/PARITY_MATRIX.md`.
- Prioritize by user impact: toolbar and palette first, then canvas, then dialogs.

### Asset import

- Import upstream icons from `src/main/resources/resources/logisim/` of v4.1.0.
- All PNG/GIF icons must be present under `assets/icons/` in the repository.
- Icons must be loaded at runtime, not embedded as byte literals in Rust source.

### Component rendering

- Gate shapes must match upstream ANSI standard shapes exactly:
  - AND/NAND: D-shape (flat left + semicircular arc)
  - OR/NOR: curved shield (concave back, circular arc front)
  - XOR/XNOR: OR shape + extra arc on the left
  - NOT: triangle + inversion bubble
  - Buffer: triangle without bubble
- Input/output stub lines at correct grid offsets.
- Pin symbols: input = square, output = circle (upstream convention).
- Power rail and ground descending-lines symbols.
- Tunnel: pentagon pointing right.

### Canvas appearance

- Background: white (`#FFFFFF`) matching upstream.
- Gate fill: white; gate border: black (1 px).
- Grid dots: light grey at every 10 px grid unit.
- Wire coloring by live logic value:
  - 0 → dark blue `#0000C0`
  - 1 → dark green `#00A000`
  - Unknown → red
  - Hi-Z → grey
  - Error → bright red

### Toolbar and palette

- Toolbar must use upstream icons at 16×16 px.
- Palette must be organized with the same categories and component order as upstream.
- Panel widths, icon sizes, and spacing must match upstream layout.

## Instructions file

See `.github/instructions/gui.instructions.md`.
