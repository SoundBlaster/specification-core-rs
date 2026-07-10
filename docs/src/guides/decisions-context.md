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
