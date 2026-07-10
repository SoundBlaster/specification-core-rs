# Migrating from Swift to Rust

The Rust crate preserves the observable Specification Pattern behavior, not
the Swift source API. The Swift package is a behavioral reference; ownership,
trait bounds, and concurrency guarantees follow Rust conventions.

## Core mapping

| Swift `SpecificationCore` | Rust `specification-core` | Practical difference |
|---|---|---|
| `Specification` | `Specification<T>` | Swift receives `T`; Rust borrows `&T` |
| `PredicateSpec` / `spec {}` | `Fn(&T) -> bool` | Closures implement the trait directly |
| `AndSpecification` / `OrSpecification` / `NotSpecification` | `And` / `Or` / `Not` | Both short-circuit; Rust uses named methods |
| `AnySpecification<T>` | `BoxedSpecification<T>` | Explicit owned trait-object boundary |
| — | `SharedSpecification<T>` | Explicit `Arc + Send + Sync` sharing boundary |
| `DecisionSpec` | `DecisionSpecification<T>` | Rust returns `Option<&Decision>` |
| `FirstMatchSpec` | `FirstMatch<T, Decision>` | Same ordered first-match behavior |
| `EvaluationContext` | `EvaluationContext<UserData>` | Generic typed data instead of `[String: Any]` |
| `DefaultContextProvider.shared` | application-owned snapshot | No global mutable context provider |
| `Date` in context | injected `Clock` + `Duration` | Deterministic, application-defined epoch |
| `async throws` | `Future<Output = Result<bool, Error>>` | Typed errors and `Send` futures |
| `@Satisfies`, `@Maybe`, `@Decides` | explicit method calls | No hidden context acquisition in property access |
| `@specs`, `@AutoContext` | `#[specification(Name)]` | Rust macro is intentionally narrower |

## Composition

Swift:

```swift
let adult = PredicateSpec<User> { $0.age >= 18 }
let verified = PredicateSpec<User> { $0.verified }
let eligible = adult.and(verified)

let result = eligible.isSatisfiedBy(user)
```

Rust:

```rust,ignore
use specification_core::Specification;

let adult = |user: &User| user.age >= 18;
let verified = |user: &User| user.verified;
let eligible = adult.and(verified);

let result = eligible.is_satisfied_by(&user);
```

Swift's protocol method is by-value at the API boundary. Rust's borrowed
candidate makes ownership explicit and lets a rule inspect a large value
without taking it from the caller.

## Type erasure and concurrency

`AnySpecification` is the usual Swift answer for heterogeneous collections. In
Rust choose deliberately:

```rust,ignore
use specification_core::{BoxedSpecification, SharedSpecification};

let local = BoxedSpecification::new(|user: &User| user.age >= 18);
let shared = SharedSpecification::new(|user: &User| user.age >= 18);
```

`BoxedSpecification` owns a local `Box<dyn Specification<T>>`. The shared form
owns `Arc<dyn Specification<T> + Send + Sync>` and rejects non-thread-safe
captures at compile time. A regular specification does not need to be `Send`
or `Sync`, which keeps single-threaded rules flexible.

## Context providers and property wrappers

Swift commonly obtains a fresh snapshot through `DefaultContextProvider.shared`
and evaluates it through a property wrapper. Rust keeps both operations
visible:

```rust,ignore
let context = EvaluationContext::new(user)
    .with_flag("enabled", true)
    .with_counter("attempts", 1);

let enabled = Flag::new("enabled")
    .is_satisfied_by(&context);
```

Mutation and refresh of counters remain application responsibilities. This
avoids hidden global state and makes a test's inputs explicit.

## Async and errors

Swift's `throws` can carry any error conforming to `Error`. Rust associates one
concrete `Error` type with each async specification; composing rules therefore
requires a shared application error enum. Rust also requires the future to be
`Send` and the candidate to be `Sync`, making executor/thread assumptions
visible in the API.

The Rust async layer does not select Tokio or another runtime. The caller
awaits the future using the runtime already chosen by the application.

## What is intentionally not identical

Rust `0.1.0` does not port Swift property wrappers, Combine observation,
global providers, segments, or the full date-rule convenience surface. Those
features are platform or application concerns. Cross-language conformance
fixtures cover observable synchronous behavior—composition, first-match,
context defaults, and cooldown boundaries—rather than requiring identical
types or serialization.

For the historical source inventory and the full adopt/adapt/defer/exclude
matrix, see the maintainer-only [DocC Migration Map](swift-docc-map.md) and
[Swift Reference Audit](swift-reference-audit.md).
