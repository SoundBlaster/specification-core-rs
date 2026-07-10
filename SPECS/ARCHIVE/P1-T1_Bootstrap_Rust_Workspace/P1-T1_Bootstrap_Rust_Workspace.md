# P1-T1 — Bootstrap Rust Workspace

**Status:** Planned

**Priority:** P0

**Dependencies:** P0-T2, P0-T3, P0-T4

## Objective

Create the first compilable Rust workspace and make its CI quality surface
comparable to the Swift `SpecificationCore` reference: multi-platform testing,
multiple compiler baselines, formatting and lint checks, release build,
documentation build, and a memory-safety-oriented validation path.

## Scope and Deliverables

- Add a Rust 2024 workspace with the `specification-core` library crate,
  MSRV 1.85 metadata, inherited lints, and a minimal documented crate root.
- Add local quality commands through a Makefile and update Flow params and
  contributor documentation.
- Add CI for push, PR, and manual runs: Linux/macOS/Windows stable tests, Linux
  MSRV verification, formatting, Clippy, release build, docs, coverage, and
  Miri.
- Add a GitHub Pages mdBook workflow analogous to the Swift DocC workflow.
- Add `.gitignore` and Rust toolchain configuration.

## Acceptance Criteria

- `cargo fmt`, Clippy, tests, doctests, rustdoc, mdBook build/test, release
  build, Miri, and coverage commands have local or CI execution paths.
- CI tests Rust stable on Linux, macOS, and Windows, and Rust 1.85 on Linux.
- Docs workflow builds and tests mdBook for PRs and deploys it only from `main`.
- The crate is publish-ready structurally: metadata, license, README, and
  `forbid(unsafe_code)` are present.
- All locally runnable quality gates pass and unavailable tools are recorded.

## Test-First Verification Plan

1. Create the manifest and minimal crate before running Cargo commands.
2. Add CI workflows before validating YAML and workflow triggers statically.
3. Run formatting, Clippy, tests, doctests, rustdoc, mdBook build/test, release
   build, Miri, and coverage where installed locally.
4. Run `cargo package --allow-dirty` as a package-layout check without
   publishing; record any intentionally unavailable tool.

## Execution Plan

### Phase A — Workspace and Local Commands

Create Cargo manifests, crate documentation, toolchain configuration, ignore
rules, Makefile, and contributor command references according to ADR-0001 and
ADR-0002.

### Phase B — CI and Documentation Delivery

Create the quality matrix and mdBook Pages workflows. Keep runner-specific
setup explicit and avoid platform-dependent repository scripts.

### Phase C — Validation and Handoff

Run all local gates available on the developer machine, validate workflow YAML
and package metadata, then record evidence. Archive and review via Flow.

## Documentation Notes

This task creates the quality gate infrastructure but does not introduce the
Specification API; P1-T2 owns the first executable behavior and coverage
threshold enforcement.
