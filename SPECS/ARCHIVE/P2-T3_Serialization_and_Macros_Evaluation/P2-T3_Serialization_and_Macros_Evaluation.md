# P2-T3 — Serialization and Macros Evaluation

**Status:** Planned

**Priority:** P2

**Dependencies:** P1-T4

## Objective

Evaluate two optional integrations against concrete Rust use cases without
polluting `specification-core`: a versioned declarative rule document for
transport/storage and a small attribute macro that turns a predicate function
into a named specification type.

## Scope and Deliverables

- Add `specification-core-serde` as a separate integration crate with a
  versioned `RuleDocument`/`RuleNode` schema, serde derives, JSON round-trip
  tests, and rejection tests for unsupported schema versions.
- Add `specification-core-macros` as a separate proc-macro crate with a
  documented `#[specification(SpecName)]` prototype. Validate its input and
  generate a normal `Specification<T>` implementation.
- Keep the core crate's default dependency graph unchanged and verify that it
  builds without either integration.
- Record adoption, limitations, dependency licenses, and future compatibility
  policy in ADR-0006 and update architecture/mdBook documentation.

The serializable document represents declarative rules only; arbitrary Rust
closures, captured state, and executable trait objects are not serializable.
The macro is accepted only for the narrow predicate-to-named-type use case; it
does not create implicit context or alter async/concurrency semantics.

## Acceptance Tests

1. Serialization schema carries an explicit version discriminator and has
   deterministic JSON round-trip tests.
2. Unknown schema versions fail with a typed error rather than silently
   changing meaning.
3. The proc macro lives in its own crate and generates a usable specification;
   malformed signatures produce compile-time diagnostics.
4. `specification-core` remains buildable/testable with no optional integration
   dependencies.
5. Workspace package, docs, coverage, and all-features checks cover every
   adopted crate.

## Test-First Execution Plan

### Phase A — Contract fixtures

Write schema round-trip/version rejection tests and a macro integration test
before implementing either crate. Define the public JSON shape and macro
signature constraints in ADR-0006.

### Phase B — Isolated prototypes

Implement the serde AST and proc-macro expansion in separate workspace crates.
Use only the dependencies needed by those crates; keep the core package free
of optional transitive requirements. Add compile-fail coverage for malformed
macro input where practical.

### Phase C — Evaluation and validation

Review ergonomics, diagnostics, schema evolution, compile-time cost, and
dependency/license impact. Record adopt/defer decisions, run all workspace
quality gates, package each crate, and capture evidence in the validation
report.

## Decision Points

- Version the schema independently from crate SemVer (`schema_version: 1`).
- Keep declarative serialization separate from executable `Specification<T>`.
- Keep macro expansion explicit and opt-in; no global context or hidden runtime.
- If either prototype lacks demonstrated value, retain its ADR evidence and
  defer the integration rather than enlarging the stable core API.

## Notes

Public docs remain technical English. Dependency license review must cover
serde, syn, quote, and proc-macro2 before publication.

---
**Archived:** 2026-07-10
**Verdict:** PASS
