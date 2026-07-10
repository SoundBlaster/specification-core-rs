# P1-T2 — Core Specification API

**Status:** Planned

**Priority:** P0

**Dependencies:** P1-T1, P0-T5

## Objective

Implement the synchronous, statically dispatched boolean core established by
the Swift behavioral audit, using idiomatic Rust ownership and borrowed
candidates rather than source-level Swift API parity.

## Scope and Deliverables

- Define `Specification<T>` with `is_satisfied_by(&self, &T) -> bool`.
- Implement the trait for compatible `Fn(&T) -> bool` closures.
- Provide allocation-free `And`, `Or`, and `Not` combinator types and ergonomic
  trait methods with left-to-right short-circuit evaluation.
- Provide `Always` and `Never` constant specifications and borrowed
  collection adapters whose empty identities are true for all-of and false for
  any-of.
- Add rustdoc examples, unit tests, and a concise mdBook getting-started guide.

## Exclusions

P1-T2 does not introduce type erasure, trait objects, decisions, evaluation
contexts, providers, built-in counter/flag/time rules, async evaluation,
wrappers, macros, or custom operators. Those boundaries belong to P1-T3 and
later tasks.

## API Contract

- Candidates are borrowed, so evaluating a specification never consumes its
  input.
- `and`, `or`, and `not` consume their concrete operands and return concrete
  wrapper types; composition allocates no heap storage.
- `And` evaluates its left side first and does not evaluate its right side when
  the left side is false. `Or` does not evaluate its right side when the left
  side is true.
- `AllOf` and `AnyOf` borrow homogeneous slices of specifications. Empty
  all-of is true and empty any-of is false.

## Acceptance Criteria

- Closures can act as specifications.
- `and`, `or`, and `not` preserve short-circuit behavior.
- Public API has executable rustdoc examples.
- Unit tests cover success, failure, short-circuit paths, constants, and
  collection identities.

## Test-First Verification Plan

1. Write tests that record evaluation order through `Cell` before implementing
   the composite types.
2. Run the focused crate tests while adding each behavior.
3. Run `make check`, `make coverage`, `make miri`, and the locked package
   verification after implementation.
4. Confirm that the core crate reaches the documented 90% line-coverage gate
   once executable behavior exists.

## Execution Plan

### Phase A — Trait and Static Composition

Implement the trait, closure implementation, constants, and concrete
allocation-free composition types with focused behavior tests.

### Phase B — Collection and Documentation Surface

Add borrowed all-of/any-of adapters, API rustdoc examples, and a mdBook
getting-started narrative that uses the public API.

### Phase C — Validation and Handoff

Run all quality gates, enforce the coverage threshold, document evidence,
archive the task, review the branch, and prepare P1-T3 as the next stacked
PR.
