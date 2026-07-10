# Getting Started

This chapter takes a small eligibility rule from a closure to a reusable,
typed decision. The complete static version is also available as
[`static_composition.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core/examples/static_composition.rs).

Add the core crate to an application:

```toml
[dependencies]
specification-core = "0.1"
```

`Specification<T>` evaluates a borrowed candidate. A closure with the shape
`Fn(&T) -> bool` implements the trait automatically, so a small rule needs no
boilerplate type.

```rust,ignore
use specification_core::Specification;

struct User {
    age: u8,
    verified: bool,
}

let is_adult = |user: &User| user.age >= 18;
let is_verified = |user: &User| user.verified;
let eligible = is_adult.and(is_verified);

let user = User { age: 21, verified: true };
assert!(eligible.is_satisfied_by(&user));
```

`eligible` is a concrete `And<...>` value. The static composition path does
not allocate or use dynamic dispatch, and `and`/`or` short-circuit from left to
right. Rust receives `&user` because evaluation borrows the candidate rather
than taking ownership of it.

```rust,ignore
use specification_core::Specification;

let is_adult = |age: &u8| *age >= 18;
let is_retired = |age: &u8| *age >= 65;
let working_age = is_adult.and(is_retired.not());

assert!(working_age.is_satisfied_by(&42));
assert!(!working_age.is_satisfied_by(&70));
```

`AllOf` and `AnyOf` apply the same behavior to a borrowed homogeneous slice;
their empty identities are `true` and `false`, respectively.

## Where to go next

- Need heterogeneous or runtime-selected rules? Read [Dynamic and shared
  specifications](../concepts/README.md#static-and-dynamic-rules).
- Need a typed outcome instead of `bool`? Read [Decisions and evaluation
  context](../guides/decisions-context.md).
- Need network or I/O-backed evaluation? Read [Async
  Specifications](../guides/async.md).
- Need to port an existing Swift rule? Read [Swift to
  Rust](../migration/swift-to-rust.md).

The `rust,ignore` blocks in this book are illustrative because mdBook does not
link standalone snippets to the workspace. Every complete workflow is backed
by a Cargo example, integration test, or compiled rustdoc example; links to
those sources are provided in the relevant chapter.
