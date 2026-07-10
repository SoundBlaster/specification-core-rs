# ADR-0005: Runtime-Neutral Async Specifications

**Status:** Accepted

**Date:** 2026-07-10

**Deciders:** SoundBlaster maintainers

**Related Workplan Tasks:** P2-T2

## Context

Async specifications need to preserve borrowed candidates, typed failures, and
boolean short-circuiting without forcing applications to adopt one executor.
Rust's public `async fn` in traits does not let the API state `Send` bounds on
the hidden future, and return-position `impl Future` (RPITIT) is deliberately
not dyn-compatible.

## Decision

Define `AsyncSpecification<T>` for `T: Sync` and `Sync` specification values.
Its evaluation method returns a `Send` future with `Result<bool, Error>`, where
`Error: Send`. The trait uses RPITIT, keeping static implementations and
combinators concrete while remaining independent of Tokio, async-std, and
other runtimes.

Provide `AsyncAnd`, `AsyncOr`, and `AsyncNot` with sequential short-circuiting:
the right operand is evaluated only when the boolean identity requires it, and
the first error is returned immediately. Provide `BoxedAsyncSpecification` as
the explicit object-safe adapter; it requires `Send + Sync + 'static` and boxes
the future at that boundary.

## Consequences

Static async rules can be consumed by any compatible executor and retain typed
borrowed inputs. Dynamic async collections pay an explicit allocation and
erasure cost. A candidate held across an await point must be `Sync`, and the
core does not promise cancellation behavior beyond what the consuming runtime
provides.

## Alternatives Considered

- Public `async fn` in the trait: rejected because the hidden future's `Send`
  contract cannot be expressed without a warning-suppression policy.
- Box every future in the primary trait: rejected because it makes allocation
  and dynamic dispatch unavoidable for static composition.
- Select Tokio or async-std: rejected because runtime choice belongs to the
  application and would expand the core dependency surface.

## Follow-up

Future API revisions must preserve the runtime-neutral contract or supersede
this ADR. Executor-specific helpers belong in extension crates or examples.
