# Decisions and evaluation context

Use `FirstMatch` when evaluation must select a typed outcome rather than only a
boolean. Rules are checked in insertion order and the first satisfied rule
wins. `decide` returns `None` when no rule matches; `decide_or` makes a
fallback explicit.

`EvaluationContext<UserData>` is immutable input for counters, flags,
timestamps, and application data. Missing counters read as zero and missing
flags as false. Time rules take an injected `Clock`; use `FixedClock` for
deterministic tests. `MaxCount` uses strict `<`, while `Cooldown` accepts the
exact duration boundary and rejects future timestamps.

Run the complete scenario:

```sh
cargo run -p specification-core --example context_and_decisions
```
