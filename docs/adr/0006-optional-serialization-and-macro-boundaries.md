# ADR-0006: Optional Serialization and Macro Boundaries

**Status:** Accepted

**Date:** 2026-07-10

**Deciders:** SoundBlaster maintainers

**Related Workplan Tasks:** P2-T3

## Context

Applications may need to persist or transport declarative rule graphs, and a
small amount of repetitive predicate boilerplate may be suitable for a proc
macro. The executable `Specification<T>` trait also supports arbitrary
closures, captured state, and generic context, none of which has a universal
serialization representation. Both integrations would add dependencies and
compiler surface if placed in the core crate.

## Decision

Adopt two opt-in extension crates:

- `specification-core-serde` defines a declarative `RuleDocument` with an
  explicit numeric `schema_version` (currently `1`) and a tagged `RuleNode`
  graph for constants, boolean composition, flags, counters, and cooldown
  durations. Deserialization rejects unknown versions. The document is data,
  not an executable trait object; applications bind it to context and clocks
  explicitly.
- `specification-core-macros` provides `#[specification(SpecName)]` for the
  narrow `fn(&Candidate) -> bool` predicate shape. It generates a named unit
  struct implementing the ordinary core trait. The macro is opt-in, requires
  an explicit generated name, and introduces no hidden context or runtime.

Both crates remain separate workspace packages. `specification-core` retains
zero required integration dependencies and remains fully usable by itself.

## Consequences

The wire schema can evolve independently from crate SemVer and can reject
unknown formats safely. It intentionally cannot round-trip arbitrary Rust
code. The macro removes boilerplate for named predicates but adds compile-time
dependencies and is not required for any API surface. Dynamic schema-to-rule
compilation and broader macro DSLs remain future design work.

## Alternatives Considered

- Derive serialization directly on executable core specifications: rejected
  because closures, trait objects, and generic user data have no stable schema.
- Put serde or proc-macro dependencies in `specification-core`: rejected to
  preserve the minimal default dependency graph and compile-time cost.
- Introduce an implicit context macro: rejected because it would hide
  dependency injection and weaken concurrency/auditability guarantees.

## Dependency and License Review

The adopted extension dependencies are serde (MIT/Apache-2.0), syn
(MIT/Apache-2.0), quote (MIT/Apache-2.0), and proc-macro2 (MIT/Apache-2.0).
They are isolated from the core package and are covered by workspace package
checks.

## Follow-up

Before a public release, define schema migration policy and add compatibility
fixtures for every accepted version. Expand or deprecate the macro only after
downstream ergonomics demonstrate value beyond the predicate prototype.
