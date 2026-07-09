# Initial Project Decisions

**Status:** Planning input

**Last updated:** 2026-07-10

This document records the direction accepted during initial repository setup.
It prevents decisions from remaining only in chat history. Normative decisions
must later be promoted into the project charter or individual ADRs.

## Accepted Direction

### Repository and releases

- Rust is developed in the separate `SoundBlaster/specification-core-rs`
  repository.
- The intended public package name is `specification-core`, subject to registry
  availability verification before publication.
- Swift and Rust implementations use independent SemVer versions and release
  schedules.
- The repository starts as a Cargo workspace with one core library crate.
- Procedural macros, FFI, and optional integrations belong in separate crates
  if their value is demonstrated later.

### Product and compatibility

- The Swift `SpecificationCore` library is a behavioral and documentation
  reference, not a source-level API template.
- Idiomatic Rust ownership, traits, error handling, concurrency, and naming take
  priority over matching Swift syntax.
- Behavioral compatibility is maintained where practical and made explicit
  through portable conformance fixtures.
- A first useful release should prioritize the synchronous core; async,
  procedural macros, serialization, and FFI are not on the critical path unless
  re-approved.

### Initial API direction

- Static specification composition should be allocation-free.
- Dynamic dispatch should be explicit rather than hidden behind the default API.
- Shared dynamic specifications should expose explicit `Send + Sync` contracts.
- Evaluation context user data should be typed or generic, not an unrestricted
  string-to-`Any` store.
- Mutable context providers should use dependency injection rather than a global
  singleton.
- Async design should remain runtime-neutral if and when it is introduced.

### Documentation

- Public API documentation, examples, and user guides are written in English.
- rustdoc owns API reference; mdBook owns concepts, tutorials, migration, and
  architectural explanations.
- The Swift DocC corpus is reusable conceptual source material, but Swift code
  and Swift-specific abstractions must be rewritten into idiomatic Rust.
- Documentation code is executable through doctests, examples, or `mdbook test`.
- Public API and behavioral changes update relevant documentation in the same
  task.

### Workflow

- Flow `1.5.0` is installed as the repository workflow framework.
- `SPECS/Workplan.md` is the task source of truth.
- Implementation tasks follow the complete Flow lifecycle with validation and
  review artifacts.

## Open Decisions for Phase 0

- Final crates.io package-name availability.
- Exact `DynSpec` ownership form and object-safety boundaries.
- Initial `0.1.0` feature boundary, especially async and conformance fixtures.

## Resolved by P0-T4

- Rust 2024 with MSRV 1.85 and resolver 3.
- Tested Linux, Apple Silicon macOS, and Windows MSVC target matrix.
- Zero required third-party dependencies in the synchronous core by default.
- Additive optional Cargo features with documented dependency and license review.
- `forbid(unsafe_code)` for the core crate; exceptions require an ADR.
- `cargo llvm-cov` provisioning in P1-T1 and a 90% core line gate in P1-T2.
- SoundBlaster maintainer-owned SemVer releases, package checks, and publication
  approval process.

## Known Baseline Observations

- The Swift repository documentation contains performance, coverage, and
  concurrency claims that must be revalidated rather than copied as Rust
  requirements.
- On the currently installed Xcode beta toolchain, the Swift baseline fails to
  compile because `FirstMatchSpec.Builder.build()` resolves an initializer
  ambiguously. `P0-T5` must record the exact toolchain and distinguish reference
  implementation defects from intended behavior.

## Resolved by P0-T5

- The Swift source at revision `7909e62` is the initial behavioral reference,
  with its current build defect explicitly excluded from Rust requirements.
- P1-T2 is limited to the synchronous boolean core; decisions, context, time,
  async, wrappers, macros, and dynamic dispatch remain in later tasks.
- Source and documentation compatibility are evaluated through the porting
  matrix, not README performance or coverage claims.

## Promotion Rule

When an open question is resolved, record the rationale and consequences in an
ADR. Remove ambiguity from this planning document, but preserve its historical
role rather than treating it as the final specification.
