# Project Charter

## Purpose

`specification-core` is a Rust library for expressing composable, strongly
typed business rules using the Specification Pattern. It provides a small,
runtime-neutral core that applications and higher-level libraries can use to
evaluate predicates, compose rules, and produce ordered decisions.

## Audience

The primary audience is Rust application and library developers building
backend services, command-line tools, domain models, policy layers, embedded
software, or WebAssembly applications. The library is not specific to an Apple
platform, UI framework, or asynchronous runtime.

## Product Goals

- Provide an idiomatic Rust API for synchronous specifications and composition.
- Preserve relevant behavioral semantics from Swift `SpecificationCore` where
  they improve interoperability and can be expressed safely in Rust.
- Make ownership, dynamic dispatch, error handling, and concurrency guarantees
  visible in the API rather than implicit.
- Keep the core small, testable, dependency-conscious, and suitable for a
  published crate.
- Treat documentation and executable examples as part of the public API.

## Non-Goals

The initial synchronous-core milestone does not include:

- source-level compatibility with Swift APIs, property wrappers, or macros;
- a global mutable context singleton;
- mandatory asynchronous runtime support;
- Swift, C, or other FFI bindings;
- procedural macros, serialization, or platform-specific integrations;
- a policy language, remote evaluation service, or GUI framework.

These capabilities may be proposed in later ADRs and implemented in optional
crates only when they have a demonstrated use case.

## Compatibility Policy

Swift `SpecificationCore` is a behavioral and conceptual reference, not an API
template. Rust design follows Rust ownership, trait, naming, and error-handling
idioms even when that changes surface syntax. Portable conformance fixtures will
describe behavior shared by both implementations; documented differences are
acceptable when required by language semantics or safety.

The Rust crate has its own SemVer versions and release schedule. A Swift release
number does not imply a matching Rust API or release.

## Architectural Principles

- Prefer static composition and allocation-free evaluation in the default path.
- Require explicit type erasure for dynamic dispatch.
- Require explicit `Send + Sync` contracts for shared dynamic specifications.
- Model evaluation data with typed or generic contexts instead of unrestricted
  runtime downcasting.
- Use dependency injection for mutable providers; avoid ambient global state.
- Keep async design runtime-neutral if it is introduced after the synchronous
  API stabilizes.

## Documentation Policy

Public API documentation, examples, and user guides are written in English.
rustdoc owns API reference; mdBook owns tutorials, concepts, migration guides,
and architecture explanations. Public behavior changes must update the relevant
documentation and executable examples in the same task.

## Governance and Decisions

The Workplan is the delivery source of truth. Durable architectural choices are
recorded as ADRs under [`docs/adr/`](adr/README.md). Initial context and open
questions are retained in [planning inputs](planning/README.md) until the
relevant Phase 0 task promotes or supersedes them.
