# specification-core

`specification-core` is the synchronous, dependency-conscious Rust foundation
for composable business rules based on the Specification Pattern. A
`Specification<T>` evaluates a borrowed `T`; closures work directly as
specifications and the `and`, `or`, and `not` methods construct statically
dispatched, allocation-free rules.

```rust
use specification_core::Specification;

let is_positive = |value: &i32| *value > 0;
assert!(is_positive.is_satisfied_by(&1));
```
