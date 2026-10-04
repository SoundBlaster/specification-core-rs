# Decisions and Evaluation Context

Use a boolean `Specification<T>` when the result is only yes/no. Use
`FirstMatch<T, Decision>` when the rule set must select a typed route, tier, or
policy.

## A complete context scenario

The executable version of this scenario is
[`context_and_decisions.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core/examples/context_and_decisions.rs).

```rust,ignore
use std::time::Duration;

use specification_core::{
    Cooldown, DecisionSpecification, EvaluationContext, FirstMatch,
    FixedClock, Flag, MaxCount, Specification,
};

let context = EvaluationContext::new("account-42")
    .with_counter("attempts", 1)
    .with_flag("enabled", true)
    .with_timestamp("last_action", Duration::from_secs(10));

let clock = FixedClock::new(Duration::from_secs(15));

assert!(MaxCount::new("attempts", 3).is_satisfied_by(&context));
assert!(Flag::new("enabled").is_satisfied_by(&context));
assert!(Cooldown::new(
    "last_action",
    Duration::from_secs(5),
    clock,
).is_satisfied_by(&context));

let mut route = FirstMatch::<EvaluationContext<&str>, &'static str>::new();
route.push(Flag::new("enabled"), "enabled-route");

assert_eq!(route.decide(&context), Some(&"enabled-route"));
```

`EvaluationContext<UserData>` is immutable after construction. It keeps
application data typed, while counters, flags, and timestamps are named
inputs. A missing counter reads as zero, a missing flag as false, and a missing
timestamp as `None`.

## Boundary rules

- `MaxCount` is satisfied only while `counter < maximum`.
- `Flag` is satisfied only when the named flag is `true`.
- `Cooldown` permits a missing timestamp.
- `Cooldown` accepts the exact `elapsed >= duration` boundary.
- A future timestamp never satisfies cooldown, including zero duration.

`Clock` is injected instead of read from global state. `FixedClock` is useful in
tests; an application can inject a monotonic production clock with the same
trait.

## Ordered decisions and fallback

`FirstMatch` evaluates rules in insertion order and stops at the first match.
With no match, `decide` returns `None`. `decide_or` makes a fallback explicit
without adding a hidden `Always` rule:

```rust,ignore
let fallback = "default-route";
let selected = route.decide_or(&context, &fallback);
```

The decision is borrowed from the `FirstMatch` value. This avoids cloning a
large decision payload and makes its lifetime visible in the type system.

## Indexed decisions for keyed catalogs

For a large catalog with exact string keys, `IndexedFirstMatch` can skip rules
whose declared key does not match the candidate. Its key projection runs once
per `decide` call, and lookup borrows the candidate's string without allocating.
The index merges the matching bucket with unkeyed rules in insertion order, so
earlier fallbacks retain priority. This excerpt is illustrative because mdBook
runs guide code blocks without the crate dependency; the complete version is a
runnable Cargo example linked below:

```rust,ignore
use specification_core::{DecisionSpecification, IndexedFirstMatch};

struct Artifact<'a> { name: &'a str, is_directory: bool }
let mut classifier = IndexedFirstMatch::new(|item: &Artifact<'_>| item.name);
classifier.push_keyed("target", |item: &Artifact<'_>| item.name == "target" && item.is_directory, "rust-build");
classifier.push_unkeyed(|item: &Artifact<'_>| item.name.ends_with(".egg-info"), "python-metadata");

let item = Artifact { name: "target", is_directory: true };
assert_eq!(classifier.decide(&item), Some(&"rust-build"));
assert!(classifier.may_match("target"));
```

Register a rule as keyed only when that key is a necessary condition: the
predicate must never match a candidate with another projected key. Arbitrary
closures cannot be inspected to prove this. Put rules without a sound key in
the unkeyed list. Those rules are checked for every candidate and make
`may_match` return `true` for any key. Duplicate keys are allowed; large
buckets and unkeyed rules are scanned in insertion order. The index requires
`Send + Sync` specifications and can be shared across worker threads when the
decisions are `Sync`. See the runnable
[`indexed_decisions.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core/examples/indexed_decisions.rs).
