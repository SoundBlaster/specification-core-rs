# P1-T3 — Dynamic Specifications and Decisions

**Status:** Planned

**Priority:** P0

**Dependencies:** P1-T2

## Objective

Add explicit type erasure and typed decision evaluation without weakening the
static `Specification` core or introducing implicit global concurrency policy.

## Scope and Deliverables

- Add an owned `BoxedSpecification<T>` wrapper around `Box<dyn
  Specification<T>>` for heterogeneous collections and runtime-selected rules.
- Add a `SharedSpecification<T>` wrapper around `Arc<dyn Specification<T> +
  Send + Sync>`; the shared API must make its concurrency bounds visible.
- Add `DecisionSpecification<T>` with a strongly typed decision result.
- Add ordered `FirstMatch<T, Decision>` evaluation over boxed specifications,
  plus explicit no-match and fallback behavior.
- Add rustdoc, unit tests, and mdBook narrative for static versus dynamic
  dispatch.

## API Contract

- Type erasure is opt-in through a named wrapper; static composition remains
  the default path from P1-T2.
- `BoxedSpecification` accepts a concrete specification and owns it. Its
  evaluation does not require `Send + Sync`.
- `SharedSpecification` requires its input to be `Send + Sync` and can be
  cloned for sharing across threads.
- `FirstMatch` evaluates pairs in insertion order, stops after the first
  satisfied specification, returns `None` when nothing matches, and offers an
  explicit fallback method.

## Exclusions

P1-T3 does not add evaluation contexts, providers, counter/flag/time rules,
async evaluation, serialization, wrappers, macros, or mutable global state.

## Acceptance Criteria

- Dynamic specifications require explicit type erasure.
- Shared dynamic specifications enforce `Send + Sync`.
- Decision results remain strongly typed.
- First-match ordering is deterministic and tested.

## Test-First Verification Plan

1. Write heterogeneous boxed-rule and shared-rule tests before wrapper code.
2. Write first-match ordering, no-match, and fallback tests before its API.
3. Run the full P1 quality gates; retain the 90% coverage requirement.

## Execution Plan

### Phase A — Explicit Type Erasure

Implement owned and shared wrappers with rustdoc examples and heterogeneous
collection tests.

### Phase B — Typed Decisions

Implement first-match decision evaluation and tests for ordering,
short-circuiting, no-match, and fallback semantics.

### Phase C — Validation and Handoff

Update narrative docs, validate locally and in CI, archive/review P1-T3, and
open it as the second stacked PR based on P1-T2.
