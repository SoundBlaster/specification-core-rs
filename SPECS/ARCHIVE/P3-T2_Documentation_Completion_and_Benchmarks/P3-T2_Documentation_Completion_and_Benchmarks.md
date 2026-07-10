# P3-T2 — Documentation Completion and Benchmarks

**Status:** INPROGRESS
**Priority:** P1
**Dependencies:** P2-T1, P3-T1
**Date:** 2026-07-10

## Goal

Finish the user-facing documentation for the released API, provide runnable
end-to-end examples, and establish a reproducible benchmark baseline that
compares static and explicit dynamic dispatch without making performance
guarantees.

## Deliverables

- Complete getting-started, decision/context, async, and integration guidance.
- Add runnable Cargo examples for static composition and context/decision use.
- Add a Criterion benchmark for equivalent static and boxed dynamic predicates.
- Document benchmark scope, environment reporting, and non-guarantee policy.
- Ensure examples compile through the existing `--all-targets` CI test matrix.

## Validation

- `make check`, `make coverage`, `make miri`, and `make package`.
- `cargo bench -p specification-core --bench dispatch --no-run`.

## Non-goals

- No performance target or regression gate is introduced in CI.
- No async runtime is selected merely to produce an example.
