# specification-core Guide

`specification-core` is an idiomatic Rust implementation of the Specification
Pattern. Release `0.1.0` provides a synchronous core, explicit dynamic and
thread-sharing boundaries, typed decisions, immutable evaluation context,
deterministic time rules, a runtime-neutral async layer, and opt-in serde and
macro crates.

This book is the reader-facing guide. The generated [API
reference](https://soundblaster.github.io/specification-core-rs/api/specification_core/)
contains complete signatures and item-level rustdoc. The same crates are also
available on [docs.rs](https://docs.rs/specification-core/0.1.0/specification_core/).

## Start here

- [Getting Started](getting-started/) — build and evaluate a first
  rule in a few minutes.
- [Core Concepts](concepts/) — understand static composition,
  allocation boundaries, and typed outcomes.
- [Decisions and Evaluation Context](guides/decisions-context.md) — route
  candidates and evaluate counters, flags, and cooldowns.
- [Async Specifications](guides/async.md) — use typed errors without choosing
  an executor in the library.
- [Migrating from Swift](migration/swift-to-rust.md) — map the Swift API to
  the Rust ownership and concurrency model.

## What is in 0.1.0

The released API includes boolean composition, `AllOf`/`AnyOf`,
`BoxedSpecification`, `SharedSpecification`, `FirstMatch`,
`EvaluationContext`, `MaxCount`, `Flag`, `Cooldown`, `AsyncSpecification`,
`RuleDocument`, and the `#[specification(Name)]` predicate macro.

Date-range rules, segment providers, property wrappers, Combine observation,
and global context providers are intentionally outside the Rust `0.1.0`
scope. Use ordinary closures or application-owned adapters for domain-specific
rules.

## Documentation surfaces

- **This book** explains concepts, workflows, design trade-offs, and Swift
  migration.
- **rustdoc/docs.rs** is the authoritative API reference and compiles focused
  examples as doctests.
- **Cargo examples and integration tests** are the executable end-to-end
  scenarios linked from the relevant chapters.
