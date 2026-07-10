# Async Specifications

`AsyncSpecification<T>` is runtime-neutral. It returns a `Send` future with a
typed `Result<bool, Error>` and does not start or own an executor. Tokio,
async-std, smol, or a custom executor can await the same rule.

The runnable standard-library example is
[`async_composition.rs`](https://github.com/SoundBlaster/specification-core-rs/blob/main/crates/specification-core/examples/async_composition.rs).

```rust,ignore
use std::{convert::Infallible, future::{ready, Future}};
use specification_core::AsyncSpecification;

struct AtLeast(u8);

impl AsyncSpecification<u8> for AtLeast {
    type Error = Infallible;

    fn is_satisfied_by<'a>(
        &'a self,
        age: &'a u8,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send + 'a {
        ready(Ok(*age >= self.0))
    }
}

let working_age = AtLeast(18).and(AtLeast(65).not());
let result = working_age.is_satisfied_by(&42).await?;
```

The future borrows both the specification and candidate. Because it is `Send`,
the executor may move the future between worker threads; `T` must therefore be
`Sync`. A composed async rule requires both operands to use the same error
type, usually an application-owned error enum.

`and` and `or` evaluate sequentially and short-circuit. An error from the
operand that was evaluated is returned immediately; a skipped operand is never
polled. `not` flips only the boolean and preserves the error.

## Dynamic async rules

The trait uses return-position `impl Future`, so it is not directly object-safe.
Use `BoxedAsyncSpecification<T, Error>` when a heterogeneous or
runtime-selected collection is required. This is an explicit `Send + Sync +
'static` boundary and allocates one boxed future per evaluation.

Cancellation remains an executor concern: dropping the future stops observing
it, while any external side effects belong to the application implementation.
