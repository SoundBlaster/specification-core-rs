# P3-T1 — Cross-language Conformance Fixtures

**Status:** INPROGRESS  
**Priority:** P1  
**Dependencies:** P1-T4  
**Date:** 2026-07-10

## Goal

Create a small, versioned, language-neutral corpus that specifies observable
behavior shared by the Rust implementation and the Swift reference. The corpus
is a behavioral contract; it is not a serialization format for executable
specifications.

## Deliverables

- `fixtures/conformance/v1/` with a manifest and JSON fixtures for boolean
  composition, ordered decisions, and evaluation context built-ins.
- A Rust integration test that reads every fixture from the manifest and
  verifies outcomes and observable evaluation order.
- An adapter contract explaining how another implementation, including Swift,
  consumes the corpus and reports unsupported capability groups.
- An ADR documenting versioning, scope, and the boundary from the optional
  `specification-core-serde` document format.
- mdBook documentation of supported, adapted, deferred, and excluded Swift
  semantics.

## Fixture Protocol

Each fixture declares `schema_version: 1`, a stable identifier, one capability
group, a candidate/context input, an expected result, and an expected trace.
The trace records only named probe evaluation order, which makes short-circuit
and first-match behavior portable without prescribing internal types.

The v1 groups are:

1. `composition` — AND, OR, NOT, empty all-of, and empty any-of.
2. `first_match` — priority, no match, and fallback behavior.
3. `context` — missing values, `MaxCount` strict bound, `Flag`, and
   `Cooldown` boundary/future-timestamp behavior using a supplied integer
   monotonic clock value.

Async behavior is excluded from v1 because a portable fixture would need an
executor and error transport convention. Its Rust contract remains covered by
the P2-T2 tests and documentation.

## Acceptance Criteria

- Composition, first-match, and context semantics have portable fixtures.
- The Rust test executes the complete manifest in CI through the existing
  `make test` target.
- Documented differences make explicit that Rust borrows candidates, does not
  reproduce Swift global/context-provider APIs, and uses injected monotonic
  time.

## Validation Plan

- `make fmt-check`
- `make lint`
- `make test`
- `make docs`
- `make coverage`
- `make miri`
- `make package`

## Risks and Non-goals

- Fixtures must not require `specification-core-serde`; it models declarative
  rule data rather than cross-language test input.
- The protocol must remain deterministic and not read wall-clock time.
- This task does not modify the Swift repository or claim that its currently
  failing package build has been repaired.
