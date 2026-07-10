# Serialization and macros

The core crate remains dependency-conscious. Optional integrations are separate
packages so applications that only need executable Rust specifications do not
pay for serialization or procedural macro dependencies.

## Versioned rule documents

`specification-core-serde` serializes a declarative `RuleDocument`. Its wire
format contains an explicit numeric `schema_version` and a tagged `RuleNode`
graph. Unknown versions are rejected during deserialization. The document can
describe constants, boolean composition, flags, maximum counts, and cooldown
durations.

The schema is deliberately not a serialization of `Specification<T>` itself.
Closures, captured state, trait objects, and application-specific user data
must be represented by an application-owned schema and bound to an evaluation
context explicitly.

## Predicate macro

`specification-core-macros` provides the opt-in
`#[specification(SpecName)]` attribute. Applied to a function shaped like
`fn(&Candidate) -> bool`, it generates a named unit struct implementing the
ordinary `Specification<Candidate>` trait. The generated name is explicit, and
the macro does not add context lookup, global state, or runtime behavior.

Both crates are optional. The core package can be built, tested, documented,
and published without enabling either integration.
