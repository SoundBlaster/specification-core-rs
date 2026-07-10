# Serialization and Macros

The optional integration crates keep the core dependency-free. Add only the
boundary your application needs:

```toml
[dependencies]
specification-core = "0.1"
specification-core-serde = "0.1"
specification-core-macros = "0.1"
```

## Versioned rule documents

`specification-core-serde` stores a declarative rule graph. It does not and
cannot serialize arbitrary closures, captured state, or trait objects. The
schema is versioned independently from crate SemVer and unknown versions are
rejected.

The complete JSON round-trip is covered by the serde crate tests and the
[`versioned_document.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core-serde/examples/versioned_document.rs)
example:

```rust,ignore
use specification_core_serde::{RuleDocument, RuleNode};

let document = RuleDocument::new(RuleNode::AllOf {
    rules: vec![
        RuleNode::Flag { key: "premium".into() },
        RuleNode::Not {
            rule: Box::new(RuleNode::MaxCount {
                key: "attempts".into(),
                maximum: 3,
            }),
        },
    ],
});
```

The wire shape is tagged and deterministic:

```json
{"schema_version":1,"rule":{"kind":"flag","key":"premium"}}
```

`RuleDocument` is data, not an evaluator. Applications must bind the decoded
graph to their own typed context and clock policy.

## Predicate macro

`#[specification(Name)]` turns one free function into a named stateless
`Specification` type:

```rust,ignore
use specification_core::Specification;
use specification_core_macros::specification;

struct User { age: u8 }

#[specification(Adult)]
fn is_adult(user: &User) -> bool {
    user.age >= 18
}

assert!(Adult.is_satisfied_by(&User { age: 21 }));
```

The macro validates the immutable `&Candidate` argument and `bool` return type.
It does not add context lookup, allocation, async behavior, or generated
composition. The compiled integration example is
[`tests/usage.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core-macros/tests/usage.rs).
