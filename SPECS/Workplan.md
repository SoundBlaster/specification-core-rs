# specification-core-rs Workplan

This workplan tracks delivery of an idiomatic Rust implementation of the
Specification Pattern. The Swift `SpecificationCore` library is a behavioral
reference, not an API template: Rust ownership, concurrency, and trait idioms
take precedence over source-level parity.

---

## Phase 0: Project Foundations

#### P0-T1: Project Charter and Architecture Baseline
- **Description:** Turn the accepted product direction into a durable charter, explicit goals and non-goals, initial crate boundaries, and an ADR process.
- **Priority:** P0
- **Dependencies:** None
- **Parallelizable:** no
- **Status:** Complete
- **Acceptance Criteria:**
  - Project goals, audiences, use cases, and non-goals are documented
  - Idiomatic Rust design is explicitly prioritized over source-level Swift API parity
  - Initial crate/workspace boundaries and extension points are documented
  - Accepted decisions and open design questions are separated clearly
  - An ADR location and lightweight decision template are established

#### P0-T2: Contribution and Agent Governance
- **Description:** Define the operating contract for AI agents and human contributors before implementation begins.
- **Priority:** P0
- **Dependencies:** P0-T1
- **Parallelizable:** yes
- **Status:** Complete
- **Acceptance Criteria:**
  - Root `AGENTS.md` defines mandatory repository and documentation rules
  - `CONTRIBUTING.md` defines development, review, and commit expectations
  - Security reporting, licensing, and third-party contribution expectations are documented
  - Governance documents link to detailed policies instead of duplicating them
  - Flow remains the source of truth for task execution and archival

#### P0-T3: Documentation System and DocC Migration Plan
- **Description:** Establish the rustdoc/mdBook information architecture and a traceable migration plan for the existing Swift DocC corpus.
- **Priority:** P0
- **Dependencies:** P0-T1
- **Parallelizable:** yes
- **Status:** Complete
- **Acceptance Criteria:**
  - rustdoc and mdBook responsibilities are documented without API-reference duplication
  - Documentation style guide defines language, examples, links, and required sections
  - mdBook structure and `SUMMARY.md` are designed for incremental migration
  - Every Swift DocC tutorial and API article has a destination or explicit exclusion
  - Documentation examples have executable doctest or `examples/` validation paths

#### P0-T4: Toolchain, Quality, and Release Policy
- **Description:** Decide the Rust baseline and enforceable engineering policies that the workspace bootstrap must implement.
- **Priority:** P0
- **Dependencies:** P0-T1
- **Parallelizable:** yes
- **Status:** Complete
- **Acceptance Criteria:**
  - Rust edition, MSRV, supported targets, and CI compatibility matrix are decided
  - Formatting, Clippy, tests, rustdoc, mdBook, and coverage gates are specified
  - Dependency, Cargo feature, unsafe-code, and license policies are documented
  - SemVer, changelog, crate publication, and release ownership rules are documented
  - `P1-T1` has an implementation-ready tooling contract

#### P0-T5: Swift Reference Audit and Porting Matrix
- **Description:** Audit the actual Swift implementation, tests, and documentation to define the behavioral reference before Rust API implementation.
- **Priority:** P0
- **Dependencies:** P0-T1, P0-T3
- **Parallelizable:** no
- **Status:** Complete
- **Acceptance Criteria:**
  - Actual Swift public API and test coverage are inventoried independently of README claims
  - Swift build/test baseline and known failures are recorded with toolchain context
  - Features are classified as adopt, adapt, defer, or exclude for Rust
  - Core composition, decision, context, time, and async semantics have a porting matrix
  - Source provenance and MIT licensing assumptions are verified
  - `P1-T2` has an implementation-ready behavioral reference

---

## Phase 1: Foundation

#### P1-T1: Bootstrap Rust Workspace
- **Description:** Create the Cargo workspace, core crate, repository tooling, and CI quality gates.
- **Priority:** P0
- **Dependencies:** P0-T2, P0-T3, P0-T4
- **Parallelizable:** no
- **Status:** Complete
- **Acceptance Criteria:**
  - Cargo workspace contains the `specification-core` library crate
  - Formatting, Clippy, tests, and rustdoc gates pass locally
  - mdBook builds and its Rust examples are tested
  - GitHub Actions validates the supported Rust toolchain
  - README and contributor documentation describe development and verification commands

#### P1-T2: Core Specification API
- **Description:** Implement the generic `Specification` trait and allocation-free boolean combinators.
- **Priority:** P0
- **Dependencies:** P1-T1, P0-T5
- **Parallelizable:** no
- **Status:** Complete
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
- **Status:** Complete
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
- **Status:** Complete
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
- **Status:** Complete
- **Acceptance Criteria:**
  - Shared APIs expose explicit `Send + Sync` bounds
  - Mutable providers use dependency injection rather than global singletons
  - Concurrent behavior is covered by tests

#### P2-T2: Async Specifications
- **Description:** Design a runtime-neutral async specification API after the synchronous API stabilizes.
- **Priority:** P1
- **Dependencies:** P2-T1
- **Parallelizable:** no
- **Status:** Complete
- **Acceptance Criteria:**
  - Public API does not require a specific async runtime
  - Object-safety and `Send` behavior are documented
  - Async composition and error propagation are tested

#### P2-T3: Serialization and Macros Evaluation
- **Description:** Prototype optional serialization and procedural macros, retaining only features with demonstrated ergonomic value.
- **Priority:** P2
- **Dependencies:** P1-T4
- **Parallelizable:** yes
- **Status:** Complete
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
- **Status:** Complete
- **Acceptance Criteria:**
  - Composition, first-match, and context rules have portable fixtures
  - Rust conformance tests run in CI
  - Behavioral differences from Swift are documented

#### P3-T2: Documentation Completion and Benchmarks
- **Description:** Complete the incrementally maintained guides, examples, API documentation, and baseline performance benchmarks.
- **Priority:** P1
- **Dependencies:** P2-T1, P3-T1
- **Parallelizable:** yes
- **Status:** Complete
- **Acceptance Criteria:**
  - Getting-started and design guides are complete
  - Examples compile in CI
  - Benchmarks distinguish static and dynamic dispatch costs

#### P3-T3: Publish 0.1.0
- **Description:** Validate packaging and publish the first public crate release.
- **Priority:** P1
- **Dependencies:** P2-T2, P3-T2
- **Parallelizable:** no
- **Status:** Complete
- **Acceptance Criteria:**
  - MSRV and supported platforms are documented
  - `cargo package` succeeds from a clean checkout
  - License and third-party dependency review is complete
  - Release notes describe stability guarantees and known limitations

---

## Phase 4: Documentation Editorial Quality

#### P4-T1: Documentation Editorial Rewrite
- **Description:** Convert the mechanically complete documentation set into a reader-first 0.1.0 guide with a coherent learning path, executable companion examples, discoverable API reference, and an explicit Swift-to-Rust migration walkthrough.
- **Priority:** P1
- **Dependencies:** P3-T3
- **Parallelizable:** no
- **Status:** INPROGRESS
- **Acceptance Criteria:**
  - Landing page and Getting Started describe the released 0.1.0 state without stale future-work claims
  - A single domain example explains static composition, dynamic dispatch, typed decisions, immutable context, and injected time
  - Async, serialization, macros, concurrency, and Swift migration each have concrete examples or links to runnable examples
  - Cargo examples and rustdoc remain the executable validation sources; docs clearly identify illustrative snippets
  - GitHub Pages links to the published API reference and includes a reader-first navigation separate from maintainer audit material
  - Pages workflow publishes or links the API reference, fixes edit links, and passes the repository quality gates
  - Review report confirms no stale claims, broken navigation, or undocumented public workflow remains in the user path

---

## Task Status Legend

- **Not Started** — task is available or waiting on dependencies
- **INPROGRESS** — task is selected and being executed through Flow
- **Blocked** — task cannot proceed until its blocker is resolved
- **Complete** — task passed validation and was archived
