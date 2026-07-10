# Core Concepts

The library applies the Specification Pattern: small, deterministic rules can
be evaluated against a candidate and composed into larger rules. Rust's trait
system makes static composition the normal path; dynamic dispatch remains an
explicit opt-in boundary.

The core boolean algebra has predictable identities: an empty `AllOf` is true,
and an empty `AnyOf` is false. This lets callers construct rules from possibly
empty collections without special-case control flow.

## Dynamic dispatch and decisions

Static composition is the default. When a program truly needs heterogeneous or
runtime-selected rules, use `BoxedSpecification<T>` explicitly. Use
`SharedSpecification<T>` only when the stored rule must cross thread
boundaries; its constructor requires `Send + Sync`.

`FirstMatch<T, Decision>` associates each boxed rule with one concrete decision
type. It evaluates rules in insertion order, returns the first matching
decision, and otherwise returns `None` (or an explicitly supplied fallback).
This keeps dynamic rule selection visible while preserving strongly typed
business outcomes.

## Evaluation context and built-ins

`EvaluationContext<UserData>` is immutable after construction and keeps
application data in its generic `UserData` parameter. Counters, flags, and
timestamps are optional inputs with deliberate defaults: missing counters are
zero and missing flags are false.

`MaxCount` uses the strict rule `counter < maximum`. `Flag` checks a named
true flag. `Cooldown` receives a `Clock` explicitly; `FixedClock` makes the
boundary `elapsed >= duration` deterministic in tests and in applications that
already own a clock abstraction.

See [Concurrency Contracts](concurrency.md) for the explicit `Send + Sync`
boundary, immutable context behavior, and dependency-injected providers.

The runtime-neutral async layer is described in [Async
Specifications](../guides/async.md); it keeps async evaluation separate from
executor selection.

The project prioritizes typed evaluation data, explicit ownership, visible
concurrency contracts, and runtime-neutral design. Detailed API semantics are
published in rustdoc as implementation tasks complete.
