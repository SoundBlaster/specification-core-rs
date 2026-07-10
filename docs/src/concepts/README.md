# Core Concepts

The library applies the Specification Pattern: a small rule evaluates a
candidate, and larger rules are built by composing those values. Rust's trait
system makes the static path the default; dynamic dispatch, heap allocation,
and thread sharing are explicit choices.

## Static and dynamic rules

Choose the smallest boundary that matches the problem:

| Need | Rust type | Cost and guarantee |
|---|---|---|
| Known rules composed at compile time | `And`, `Or`, `Not` | No library allocation or trait-object dispatch |
| Homogeneous collection | `AllOf`, `AnyOf` | Borrows a slice and short-circuits |
| Heterogeneous/runtime-selected rules | `BoxedSpecification<T>` | Owns a `Box<dyn Specification<T>>` |
| Rule shared between threads | `SharedSpecification<T>` | Owns an `Arc` and requires `Send + Sync + 'static` |

```rust,ignore
use specification_core::{BoxedSpecification, Specification};

let rules: Vec<BoxedSpecification<u64>> = vec![
    BoxedSpecification::new(|value: &u64| *value > 0),
    BoxedSpecification::new(|value: &u64| *value % 2 == 0),
];

assert!(rules[1].is_satisfied_by(&42));
```

The complete thread-sharing scenario is in
[`dynamic_and_concurrency.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core/examples/dynamic_and_concurrency.rs).

## Boolean identities and short-circuiting

`and`, `or`, and `not` preserve left-to-right short-circuit evaluation. An
empty `AllOf` is `true`, and an empty `AnyOf` is `false`. These identities let
an application construct a rule from a possibly empty collection without
special-case control flow.

## Typed decisions

`FirstMatch<T, Decision>` is the explicit dynamic boundary for ordered routing.
It stores heterogeneous rules with one concrete decision type, evaluates them
in insertion order, and returns the first matching `Option<&Decision>`. Use
`decide_or` when a fallback is required. See [Decisions and Evaluation
Context](../guides/decisions-context.md) for a complete route-selection
example.

## Immutable context and dependencies

`EvaluationContext<UserData>` is an immutable input snapshot. Application data
is typed through `UserData`; counters, flags, and timestamps have deliberate
defaults. Time rules receive a `Clock` explicitly, so `FixedClock` can make a
boundary deterministic in tests. There is no global context registry.

See [Concurrency Contracts](concurrency.md) for `Send + Sync` behavior and
[Decisions and Evaluation Context](../guides/decisions-context.md) for the
built-in rules.

The runtime-neutral async layer is described in [Async
Specifications](../guides/async.md). Optional wire-format and predicate-macro
support are described in [Serialization and Macros](../guides/serialization-macros.md).
