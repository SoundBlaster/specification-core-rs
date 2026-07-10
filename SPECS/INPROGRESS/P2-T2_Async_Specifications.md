# P2-T2 — Async Specifications

**Status:** Planned

**Priority:** P1

**Dependencies:** P2-T1

## Objective

Add an async specification layer that is independent of Tokio, async-std, or
any other executor. Async rules must preserve borrowed candidates, typed
errors, deterministic boolean short-circuiting, and the explicit concurrency
contracts established by P2-T1.

## Scope and Deliverables

- Add an ADR documenting the async future, `Send`, object-safety, and runtime
  neutrality decisions.
- Add `AsyncSpecification<T>` using a stable Rust 1.85 return-position `impl
  Future` method with `Send` futures and a `Sync` specification boundary.
- Add `AsyncAnd`, `AsyncOr`, and `AsyncNot` with typed error propagation and
  sequential short-circuiting.
- Add `BoxedAsyncSpecification<T, Error>` as the explicit object-safe dynamic
  adapter; keep allocation and erasure out of the static trait path.
- Add executor-free tests using `std::task::Waker::noop` and ready futures.
- Update rustdoc, mdBook, and the Swift migration map to describe the shipped
  runtime-neutral API accurately.

`AsyncSpecification` intentionally uses RPITIT and is therefore not directly
dyn-compatible. Callers needing heterogeneous async rules use
`BoxedAsyncSpecification`, whose constructor requires `Send + Sync + 'static`.
No executor or third-party dependency is added to the core crate.

## Acceptance Tests

1. Async public API compiles on MSRV 1.85 without a specific runtime.
2. Object-safety and `Send` behavior are stated in rustdoc and ADR-0005.
3. `and`, `or`, and `not` preserve short-circuit order and typed errors.
4. The dynamic adapter evaluates a concrete async rule through a trait object.
5. Tests prove skipped branches are not polled/evaluated and errors stop
   composition deterministically.

## Test-First Execution Plan

### Phase A — Contract tests and design

Write ready-future fixtures, compile-time Send/Sync assertions, dynamic
adapter tests, and short-circuit/error tests before implementation. Record the
RPITIT/object-safe adapter decision in ADR-0005.

### Phase B — Async implementation

Implement the trait, boxed adapter, and boolean combinators with no runtime
dependency. Keep error types generic and require matching error types across a
composition.

### Phase C — Documentation and validation

Document executor integration as consumer-side behavior, cancellation and
evaluation order, then run all repository gates plus MSRV, coverage, Miri, and
package verification. Capture results in `P2-T2_Validation_Report.md`.

## Decision Points

- Use stable RPITIT rather than `async fn` in the public trait so `Send` can be
  expressed explicitly and `async_fn_in_trait` warnings are avoided.
- Keep the static API runtime-neutral; the boxed adapter is the only allocation
  and dynamic-dispatch boundary.
- Require `T: Sync` for async specifications because returned futures are
  `Send` and may hold a borrowed candidate across an await point.

## Notes

Public docs remain technical English. Do not claim a built-in executor,
cancellation runtime, or automatic Swift property-wrapper equivalent.
