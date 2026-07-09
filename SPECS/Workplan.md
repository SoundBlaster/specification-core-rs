# specification-core-rs Workplan

This workplan tracks delivery of an idiomatic Rust implementation of the
Specification Pattern. The Swift `SpecificationCore` library is a behavioral
reference, not an API template: Rust ownership, concurrency, and trait idioms
take precedence over source-level parity.

---

## Phase 1: Foundation

#### P1-T1: Bootstrap Rust Workspace
- **Description:** Create the Cargo workspace, core crate, repository tooling, and CI quality gates.
- **Priority:** P0
- **Dependencies:** None
- **Parallelizable:** no
- **Status:** Not Started
- **Acceptance Criteria:**
  - Cargo workspace contains the `specification-core` library crate
  - Formatting, Clippy, tests, and rustdoc gates pass locally
  - GitHub Actions validates the supported Rust toolchain
  - README documents development and verification commands

#### P1-T2: Core Specification API
- **Description:** Implement the generic `Specification` trait and allocation-free boolean combinators.
- **Priority:** P0
- **Dependencies:** P1-T1
- **Parallelizable:** no
- **Status:** Not Started
- **Acceptance Criteria:**
  - Closures can act as specifications
  - `and`, `or`, and `not` preserve short-circuit behavior
  - Public API has rustdoc examples
  - Unit tests cover success, failure, and short-circuit paths

#### P1-T3: Dynamic Specifications and Decisions
- **Description:** Add explicit dynamic dispatch, typed decisions, and ordered first-match evaluation.
- **Priority:** P0
- **Dependencies:** P1-T2
- **Parallelizable:** no
- **Status:** Not Started
- **Acceptance Criteria:**
  - Dynamic specifications require explicit type erasure
  - Shared dynamic specifications enforce `Send + Sync`
  - Decision results remain strongly typed
  - First-match ordering is deterministic and tested

#### P1-T4: Evaluation Context and Built-ins
- **Description:** Add a typed immutable evaluation context and initial counter, flag, and time specifications.
- **Priority:** P1
- **Dependencies:** P1-T3
- **Parallelizable:** no
- **Status:** Not Started
- **Acceptance Criteria:**
  - User data is generic rather than based on unbounded runtime downcasting
  - Built-in specifications are deterministic under an injected clock/context
  - Optional dependencies are isolated behind Cargo features

---

## Phase 2: Extended Capabilities

#### P2-T1: Concurrency Contracts
- **Description:** Define and validate thread-safety guarantees for specifications and context providers.
- **Priority:** P1
- **Dependencies:** P1-T4
- **Parallelizable:** no
- **Status:** Not Started
- **Acceptance Criteria:**
  - Shared APIs expose explicit `Send + Sync` bounds
  - Mutable providers use dependency injection rather than global singletons
  - Concurrent behavior is covered by tests

#### P2-T2: Async Specifications
- **Description:** Design a runtime-neutral async specification API after the synchronous API stabilizes.
- **Priority:** P1
- **Dependencies:** P2-T1
- **Parallelizable:** no
- **Status:** Not Started
- **Acceptance Criteria:**
  - Public API does not require a specific async runtime
  - Object-safety and `Send` behavior are documented
  - Async composition and error propagation are tested

#### P2-T3: Serialization and Macros Evaluation
- **Description:** Prototype optional serialization and procedural macros, retaining only features with demonstrated ergonomic value.
- **Priority:** P2
- **Dependencies:** P1-T4
- **Parallelizable:** yes
- **Status:** Not Started
- **Acceptance Criteria:**
  - Serialization schema is explicitly versioned
  - Procedural macros remain in a separate crate if adopted
  - Core crate remains usable without optional integrations

---

## Phase 3: Compatibility and Release

#### P3-T1: Cross-language Conformance Fixtures
- **Description:** Define behavioral fixtures shared conceptually with the Swift implementation.
- **Priority:** P1
- **Dependencies:** P1-T4
- **Parallelizable:** yes
- **Status:** Not Started
- **Acceptance Criteria:**
  - Composition, first-match, and context rules have portable fixtures
  - Rust conformance tests run in CI
  - Behavioral differences from Swift are documented

#### P3-T2: Documentation and Benchmarks
- **Description:** Complete guides, examples, API documentation, and baseline performance benchmarks.
- **Priority:** P1
- **Dependencies:** P2-T1, P3-T1
- **Parallelizable:** yes
- **Status:** Not Started
- **Acceptance Criteria:**
  - Getting-started and design guides are complete
  - Examples compile in CI
  - Benchmarks distinguish static and dynamic dispatch costs

#### P3-T3: Publish 0.1.0
- **Description:** Validate packaging and publish the first public crate release.
- **Priority:** P1
- **Dependencies:** P2-T2, P3-T2
- **Parallelizable:** no
- **Status:** Not Started
- **Acceptance Criteria:**
  - MSRV and supported platforms are documented
  - `cargo package` succeeds from a clean checkout
  - License and third-party dependency review is complete
  - Release notes describe stability guarantees and known limitations

---

## Task Status Legend

- **Not Started** — task is available or waiting on dependencies
- **INPROGRESS** — task is selected and being executed through Flow
- **Blocked** — task cannot proceed until its blocker is resolved
- **Complete** — task passed validation and was archived
