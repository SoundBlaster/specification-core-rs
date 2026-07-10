# Concurrency contracts

The synchronous API keeps thread sharing explicit. A regular
`Specification<T>` does not require `Send` or `Sync`, so a local rule may use
single-threaded state when that is the correct domain model.

Use `SharedSpecification<T>` when a rule must cross a thread boundary. Its
constructor requires the concrete rule to be `Send + Sync + 'static`, and its
cloneable wrapper uses an `Arc` internally:

The complete runnable version is
[`dynamic_and_concurrency.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core/examples/dynamic_and_concurrency.rs).

The following snippet is illustrative in mdBook (the book test runner does not
link standalone blocks against the workspace crate); the equivalent rustdoc
example is compiled by Cargo's doctest gate.

```rust,ignore
use specification_core::{SharedSpecification, Specification};

let rule = SharedSpecification::new(|value: &u64| *value % 2 == 0);
let worker_rule = rule.clone();

std::thread::spawn(move || assert!(worker_rule.is_satisfied_by(&2)))
    .join()
    .expect("worker thread should finish");
```

`EvaluationContext<UserData>` is immutable after construction. It can be moved
or shared whenever `UserData` satisfies the corresponding Rust auto-traits.
There is no global context registry.

Time-dependent rules receive their `Clock` explicitly. A mutable or shared
clock is therefore an application-owned dependency: use an appropriate
standard-library synchronization primitive and inject it into the rule. This
keeps concurrent behavior visible in the type graph and makes deterministic
tests possible with `FixedClock`.
