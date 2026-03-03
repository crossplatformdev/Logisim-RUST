# Copilot instructions for this repository

These instructions apply repository-wide. Read them as binding implementation guidance for all tasks in this repository.

## Mission

This repository's objective is not to build a Rust-inspired alternative, proof of concept, partial port, compatibility layer, MVP, simplified clone, or demonstration app.

The objective is to produce a behaviorally faithful Rust rewrite of Logisim-Evolution v4.1.0 using the upstream project as the source of truth, while reusing upstream assets and resources where legally and technically appropriate.

**Reference upstream:**

- Logisim-Evolution v4.1.0
- Release tag: https://github.com/logisim-evolution/logisim-evolution/releases/tag/v4.1.0

When making design or implementation decisions, treat the upstream v4.1.0 source tree, assets, resources, formats, workflows, and user-visible behaviors as the canonical reference.

## Core expectations

### Parity first

- Prioritize semantic, behavioral, visual, and compatibility parity with upstream v4.1.0.
- Do not optimize for novelty, different UX, easier architecture, or reduced scope if that weakens fidelity.
- A rewritten subsystem is not complete unless it behaves like the upstream feature in normal usage.

### Rust rewrite, not wrapper

- Implement functionality in Rust.
- Do not rely on the Java application or runtime for core behavior.
- Do not ship bridging layers that merely defer the hard work to the original implementation.

### No incomplete work disguised as progress

- Do not leave stubs.
- Do not leave TODOs.
- Do not leave placeholders.
- Do not leave "not yet implemented" comments for required upstream features.
- Do not describe missing parity as acceptable progress.
- Do not convert requested implementation work into planning-only output.

### Single-PR completion mindset

- Work as if this pull request must be self-contained and reviewable as a complete deliverable.
- Do not propose splitting required parity work into follow-up PRs unless absolutely impossible within the current task and only after maximizing actual implementation first.
- Default behavior is to continue implementing and committing missing pieces instead of deferring them.

### Evidence over claims

- Never claim parity without code, tests, assets, and validation to support the claim.
- Never describe the project as a "clone" or "1:1 compatible" unless the corresponding functionality is implemented and verified.
- If parity is incomplete, the correct response is to keep implementing until it is closer to complete, not to justify stopping.

## How to behave as an agent in this repository

When asked to review, verify, complete, or bring parity to this repository:

**Do not:**
- Stop at analysis.
- Only summarize gaps.
- Only suggest next steps.
- Only rewrite documentation.
- Only add architecture notes.
- Decline solely because the total scope is large.

**Instead:**
1. Inspect the current repository state.
2. Compare it to upstream v4.1.0 subsystem by subsystem.
3. Identify the highest-impact missing parity items.
4. Implement them in Rust.
5. Add or update tests.
6. Integrate required assets/resources.
7. Update CI/CD if validation is missing.
8. Re-run validation logic.
9. Commit the improvements to the current branch/PR.
10. Repeat until the repository is materially closer to faithful parity.

If a task is too broad to finish perfectly in one pass, still make the maximum honest implementation progress possible in code before responding.

## Definition of "complete" for this project

This project should be treated as incomplete until the Rust application credibly matches upstream v4.1.0 in all or nearly all of the following areas:

- circuit editing workflows
- wire placement and manipulation behavior
- simulation semantics
- multi-driver and HighZ behavior
- propagation and convergence behavior
- truth values, bus handling, and signal state handling
- project and library handling
- `.circ` parsing and writing fidelity
- GUI behavior and user workflows
- component palette coverage
- standard libraries
- TTL libraries
- appearance handling
- undo/redo behavior
- localization/resource loading
- timing/chronogram functionality
- HDL export workflows where upstream supports them
- board/FPGA-related flows where upstream supports them
- asset reuse and visual fidelity
- CLI behavior where applicable
- import/export and interoperability workflows

A subsystem is not "done" because there is a rough equivalent. It is done when it is implemented, integrated, tested, and credible for real users of Logisim-Evolution.

## Upstream-first implementation rules

Before implementing or changing behavior:

- Inspect the upstream v4.1.0 code and assets for the corresponding subsystem.
- Preserve upstream concepts, names, file semantics, and user-visible behavior where practical.
- Recreate behavior in Rust as faithfully as possible.
- Keep format compatibility unless there is a strong reason not to.
- If an upstream behavior seems odd, assume it may be relied upon and verify before changing it.

Do not introduce unnecessary redesigns that break familiarity or compatibility.

## Non-negotiable feature parity areas

When these exist upstream, they are mandatory parity targets and must not be silently omitted:

### 1. Component and simulation parity

- Standard logic components
- Gates
- Plexers
- Arithmetic components
- Memory components
- IO components
- Wiring-related elements
- Sequential logic
- Clocks and timing-relevant semantics
- TTL library components
- State propagation fidelity
- Correct conflict, unknown, and HighZ handling
- Stable convergence and oscillation handling where applicable

### 2. File compatibility

- `.circ` read compatibility
- `.circ` write compatibility
- project round-trip fidelity
- preservation of data that upstream preserves
- correct handling of libraries, subcircuits, attributes, labels, appearance data, and metadata

### 3. GUI and UX fidelity

- tool palette
- selection behavior
- move/place/poke/text/wire workflows
- canvas interactions
- rendering expectations
- attributes editing
- context actions
- dialogs and file flows
- layout conventions
- appearance close enough for existing users to feel at home

### 4. Libraries and resources

- bundled libraries
- localization files
- reusable upstream assets
- icons, artwork, and board definitions
- any resource files needed for parity

### 5. Advanced functionality

- undo/redo with meaningful history behavior
- chronogram/timing-diagram functionality
- appearance editor parity
- HDL export parity where supported upstream
- board/FPGA-related flows where supported upstream
- console/integration features where supported upstream

If one of these is missing from the Rust codebase, prioritize implementing it rather than documenting its absence.

## Instructions about assets and resources

Reuse upstream assets and resources wherever legally and technically appropriate.

This includes, where relevant:

- icons
- artwork
- language files
- templates
- board definitions
- built-in resource bundles
- example-compatible resources
- any static resources needed for visual or behavioral parity

Do not merely state that assets are reused. Ensure they are actually present in the repository and integrated into the build/runtime path.

If an upstream asset is not reused, document the exact technical reason and provide the closest faithful replacement.

## Instructions about CI/CD and validation

The CI/CD pipeline must validate more than compilation.

Repository automation should, where practical, include:

- rustfmt
- clippy
- unit tests
- integration tests
- cross-platform builds
- release artifact generation
- regression checks for file compatibility
- parity-oriented tests for simulation semantics
- tests that fail on missing critical resources where appropriate

Whenever you add a major feature, add or update tests that prove it works and reduce the chance of silent regressions.

Do not allow CI to become a "build-only" signal when parity-critical behavior remains untested.

## Parity matrix requirement

Maintain a feature parity matrix in repository documentation.

For every major upstream subsystem, track:

- upstream feature name
- Rust implementation status
- test status
- compatibility status
- asset/resource status
- remaining differences, if any

Use clear status terms such as:

- `implemented`
- `partially implemented`
- `tested`
- `asset-complete`
- `verified against upstream`
- `incompatible`
- `missing`

Do not use vague language such as "basic support" or "initial support" without specifying what is still absent.

## Documentation rules

Documentation must be honest and implementation-driven.

**Do:**
- document what is actually implemented
- document exact compatibility boundaries
- document how to build, test, and validate
- document specific unavoidable differences

**Do not:**
- use "future work" to excuse missing core parity
- present partial implementations as feature-complete
- claim 1:1 compatibility without proof
- replace missing code with architecture prose

If documentation currently says a feature is missing, the preferred action is to implement the feature and then update the documentation.

## Coding standards for this repository

- Use idiomatic, maintainable Rust.
- Keep dependencies minimal and justified.
- Favor explicit, testable logic over cleverness.
- Preserve deterministic behavior.
- Keep modules focused and organized by subsystem.
- Avoid introducing abstraction layers that obscure parity with upstream behavior.
- Mirror upstream domain concepts clearly so future parity work stays tractable.
- Prefer readable code over compact code when implementing nuanced simulation semantics.

## Review behavior expectations

When reviewing code in this repository:

- review for parity with upstream, not just Rust style
- review for missing behavior, not just syntax
- review for resource integration and compatibility
- review for user-visible deviations
- review for insufficient tests

If a review reveals missing required functionality:

- implement or request implementation of the missing functionality
- do not accept documentation-only mitigation for core parity gaps

## What not to do

Do not:

- argue that the project is too large and stop there
- redirect required parity work into separate follow-up PRs by default
- produce only a gap analysis
- substitute promises for implementation
- leave placeholder modules
- silently omit upstream features
- intentionally weaken compatibility targets to make progress easier
- claim success because some core subset works

## Preferred execution loop

For each substantial task:

1. Identify the exact upstream subsystem and files to mirror.
2. Inspect current Rust implementation state.
3. Determine the parity gap.
4. Implement the missing behavior in Rust.
5. Import or wire required resources/assets.
6. Add focused tests.
7. Update CI if validation is missing.
8. Update the parity matrix.
9. Only then summarize the result.

## Priority order when many features are missing

Unless the task explicitly says otherwise, prioritize in roughly this order:

1. correctness of simulation semantics
2. `.circ` compatibility and project fidelity
3. component/library coverage
4. GUI workflow fidelity
5. undo/redo and editor completeness
6. appearance/resource/localization parity
7. timing/chronogram parity
8. export/integration/FPGA-related parity
9. remaining visual and usability edge cases

## Instructions for responses produced by the agent

When reporting progress:

- be specific
- name implemented subsystems
- name verified tests
- name exact remaining gaps
- avoid inflated claims

When asked to "verify", "review", or "finish":

- verification must include implementation changes when gaps are found
- "review" is not an excuse to avoid code changes
- "finish" means continue coding until acceptance criteria are materially satisfied

## Repository path guidance

This repository should contain repository-wide instructions at:

- `.github/copilot-instructions.md`

If additional task-specific instructions are needed, keep them consistent with this file. Nested instructions must never lower the parity expectations defined here.

## Bottom line

Treat this repository as a serious rewrite effort whose success is measured by faithfulness to Logisim-Evolution v4.1.0, not by the existence of a Rust codebase.

The correct default behavior is:

- compare against upstream
- implement missing parity
- add tests
- integrate assets
- validate in CI
- keep going

Do not stop at explaining why the task is hard.
