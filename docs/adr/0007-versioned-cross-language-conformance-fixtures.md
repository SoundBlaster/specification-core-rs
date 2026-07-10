# ADR-0007: Versioned Cross-language Conformance Fixtures

**Status:** Accepted

**Date:** 2026-07-10

**Deciders:** SoundBlaster maintainers

**Related Workplan Tasks:** P3-T1

## Context

The Swift project is the behavioral reference for the port, while Rust APIs and
implementation details intentionally differ. Source-level tests cannot be
shared between the languages, and the optional `specification-core-serde` rule
document is not a suitable substitute: it represents declarative rules rather
than observable compatibility scenarios.

## Decision

Adopt a repository-owned, JSON conformance corpus under
`fixtures/conformance/v1/`. A manifest lists fixture files and every document
has `schema_version: 1`. The version is part of the fixture contract; consumers
must reject unknown versions rather than guessing their meaning.

Fixtures record inputs, expected results, and named-probe traces. Traces make
evaluation order observable while keeping the format independent of concrete
types and allocation strategies. The first version covers synchronous boolean
composition, ordered first-match decisions, and immutable context built-ins
with a supplied monotonic time value.

The Rust integration test is the reference consumer and runs through `make
test`. A Swift adapter may consume the same files after mapping candidates and
context into its own public APIs.

## Consequences

The corpus provides a stable, reviewable behavioral seam without imposing FFI,
identical naming, or identical serialization on either implementation. New
semantics require a new fixture version or an explicitly backward-compatible
addition documented in the adapter contract.

Async execution, errors, global providers, property wrappers, observation, and
platform-specific time types are intentionally outside v1. They need extra
portable conventions and must not be implied by a passing synchronous corpus.

## Alternatives Considered

- Share Swift XCTest and Rust test code: rejected because toolchains, types,
  ownership, and test frameworks differ.
- Reuse `RuleDocument` from `specification-core-serde`: rejected because its
  schema models declarative rules and is an optional integration boundary.
- Compare internal evaluation implementations: rejected because compatibility
  concerns observable behavior, not implementation structure.
