# P1-T4 — Evaluation Context and Built-ins

**Status:** Planned

**Priority:** P1

**Dependencies:** P1-T3

## Objective

Add an immutable, typed evaluation context and the first deterministic built-in
rules without reintroducing Swift's global mutable provider or unbounded
`Any`-style user data.

## Scope and Deliverables

- Add `EvaluationContext<UserData>` with typed user data, counters, flags, and
  an optional injected evaluation time.
- Make the context immutable after construction and expose absence semantics:
  missing counters are zero and missing flags are false.
- Add `MaxCount`, `Flag`, and time-based `Cooldown` specifications.
- Add a small `Clock` abstraction and a `FixedClock` test implementation; time
  rules receive their clock explicitly and never read global wall-clock state.
- Document strict counter and inclusive cooldown boundary semantics, then test
  them with deterministic timestamps.

## API Contract

- `EvaluationContext<UserData>` stores user data as its generic parameter; no
  runtime downcasting or string-to-`Any` bag is exposed.
- Context construction may use `BTreeMap` for deterministic key order, but
  evaluation only reads immutable data.
- `MaxCount` is satisfied only when `counter < maximum_count`.
- `Flag` requires an explicitly named flag to be true.
- `Cooldown` allows a missing timestamp and allows evaluation at exactly the
  configured duration (`elapsed >= duration`).

## Exclusions

This task excludes shared mutable providers, event/date range rules,
serialization, observation, async evaluation, and optional third-party time
dependencies.

## Acceptance Criteria

- User data is generic rather than based on unbounded runtime downcasting.
- Built-in specifications are deterministic under an injected clock/context.
- Optional dependencies are isolated behind Cargo features (none are needed
  for this core implementation).

## Test-First Verification Plan

1. Define boundary tests for missing values, strict maximum count, and exact
   cooldown duration before implementing each built-in.
2. Verify a fixed clock makes time rules independent of wall-clock state.
3. Run full quality gates and retain the 90% coverage threshold.

## Execution Plan

### Phase A — Immutable Context and Clock

Implement typed context construction/read APIs and an injected clock contract.

### Phase B — Deterministic Built-ins

Implement and test counter, flag, and cooldown specifications against explicit
context data and fixed times.

### Phase C — Validation and Handoff

Update docs, validate, archive/review P1-T4, and open the final Phase 1 PR on
top of P1-T3.
