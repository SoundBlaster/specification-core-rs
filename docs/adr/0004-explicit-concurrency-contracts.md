# ADR-0004: Explicit Concurrency Contracts

**Status:** Accepted

**Date:** 2026-07-10

**Deciders:** SoundBlaster maintainers

**Related Workplan Tasks:** P2-T1

## Context

The synchronous core has two different use cases: inexpensive static rules
that may remain local to one thread, and explicitly shared rules that cross
thread boundaries. Treating every specification as thread-safe would impose
unnecessary bounds, while ambient context or clock providers would make
evaluation order and tests depend on global mutable state.

## Decision

Keep `Specification<T>` free of `Send + Sync` supertraits. Make
`SharedSpecification<T>` the explicit thread-sharing boundary: its erased
object and constructor require `Send + Sync + 'static`, and the wrapper is
cloneable through `Arc`.

Keep `EvaluationContext<UserData>` immutable and generic. Its auto-traits are
determined by `UserData` and its standard-library fields. Keep time and other
mutable inputs caller-owned and dependency-injected; `Clock` is passed into a
rule such as `Cooldown` and is never obtained from a global singleton.

## Consequences

Single-threaded specifications remain easy to write, including closures that
capture local state. Applications opt into sharing and receive compile-time
errors when captured state is not thread-safe. Deterministic providers are
slightly more explicit at call sites, but concurrent behavior is auditable and
testable without an ambient runtime service.

## Alternatives Considered

- Require `Send + Sync` on `Specification`: rejected because it would constrain
  valid local rules and static composition for no benefit.
- Add a global default context or clock: rejected because it introduces hidden
  mutable state and makes tests order-dependent.
- Add a mandatory synchronization wrapper around every provider: rejected;
  callers choose the appropriate `Arc`, lock, or immutable value for their
  domain and inject it explicitly.

## Follow-up

`P2-T2` must document the corresponding async future bounds and object-safety
trade-offs. Any future provider abstraction requires a separate API decision
if it changes the current injection boundary.
