# P2-T1 — Concurrency Contracts

**Status:** Planned

**Priority:** P1

**Dependencies:** P1-T4

## Objective

Make the concurrency model of the synchronous core explicit and executable.
Shared dynamic specifications must require `Send + Sync`, immutable contexts
must remain safe to share when their user data is safe, and mutable or
time-dependent inputs must be supplied by the caller rather than discovered
through global state.

## Scope and Deliverables

- Record the ownership and thread-safety contract in ADR-0004.
- Improve rustdoc for `SharedSpecification`, `EvaluationContext`, `Clock`, and
  `Cooldown`, including conditional auto-trait behavior and provider injection.
- Add compile-time assertions for the shared API and deterministic built-ins.
- Add multi-threaded tests that evaluate one shared specification from several
  threads and exercise an explicitly injected, thread-safe clock/provider.
- Add a concise mdBook concurrency guide and link it from the summary.

The core will not impose `Send + Sync` on the static `Specification` trait and
will not add a global mutable context registry. A provider abstraction is only
introduced if the tests demonstrate a stable API need; otherwise `Clock` and
immutable `EvaluationContext` remain the explicit dependency-injection points.

## Acceptance Tests

1. `SharedSpecification::new` visibly requires `Specification + Send + Sync +
   'static` and can be cloned and evaluated concurrently.
2. A non-thread-safe captured closure cannot be passed to `SharedSpecification`
   (compile-fail documentation or an equivalent compile-time assertion).
3. `EvaluationContext<UserData>` and built-in rules are shareable exactly when
   their generic/provider inputs satisfy the corresponding auto-traits.
4. Concurrent tests are deterministic, use no global mutable state, and pass
   under the repository's standard test and Miri gates.
5. Public docs and the architectural decision agree with the implemented API.

## Test-First Execution Plan

### Phase A — Contract and failing tests

Write compile-time Send/Sync assertions, a concurrent shared-specification
test, and a thread-safe injected-clock test before changing implementation or
documentation.

### Phase B — Minimal implementation and documentation

Adjust only the public bounds/docs needed to make the contract unambiguous.
Add ADR-0004 and the mdBook guide. Do not introduce a runtime, unsafe code, or
third-party dependency.

### Phase C — Validation and handoff

Run `make check`, coverage, Miri, rustdoc, mdBook, and package checks. Capture
commands and results in `P2-T1_Validation_Report.md`, then archive and review
the task through the remaining Flow stages.

## Decision Points

- Keep `Specification<T>` single-thread-friendly; sharing is opt-in.
- Treat `Clock` as dependency injection, not an ambient service locator.
- Use standard-library synchronization primitives in tests only.

## Notes

Public documentation remains technical English. Update the migration map only
if the context-provider destination changes; otherwise preserve its explicit
exclusion of global singleton semantics.

---
**Archived:** 2026-07-10
**Verdict:** PASS
