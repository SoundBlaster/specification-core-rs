# Getting Started

`specification-core` models a business rule as a synchronous
`Specification<T>`. Candidates are borrowed, and closures can be used directly
as rules. The snippets below are mirrored by the crate-root rustdoc doctest;
mdBook's isolated test harness cannot link a workspace crate directly.

```rust,ignore
use specification_core::Specification;

let is_adult = |age: &u8| *age >= 18;
assert!(is_adult.is_satisfied_by(&18));
assert!(!is_adult.is_satisfied_by(&17));
```

Build larger rules with `and`, `or`, and `not`. The operands remain concrete
types, so composition needs neither a heap allocation nor dynamic dispatch.

```rust,ignore
use specification_core::Specification;

let is_adult = |age: &u8| *age >= 18;
let is_retired = |age: &u8| *age >= 65;
let working_age = is_adult.and(is_retired.not());

assert!(working_age.is_satisfied_by(&42));
assert!(!working_age.is_satisfied_by(&70));
```

`And` evaluates its left rule first and skips its right rule after a false
result. `Or` skips its right rule after a true result. `AllOf` and `AnyOf` add
the same short-circuit behavior for borrowed homogeneous slices; empty all-of
is true and empty any-of is false.

Type erasure, decisions, contexts, and built-in time rules are deliberately
later milestones. The [architecture baseline](../design/architecture.md)
explains those crate boundaries.
