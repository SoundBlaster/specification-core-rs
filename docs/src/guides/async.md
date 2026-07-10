# Async specifications

`AsyncSpecification<T>` is runtime-neutral. It returns a `Send` future with a
typed `Result<bool, Error>` and does not start, own, or select an executor.
Applications can await the future from Tokio, async-std, smol, or a custom
executor without changing the specification type.

The trait uses return-position `impl Future`, so static implementations and
combinators retain concrete future types. It is intentionally not directly
dyn-compatible. Use `BoxedAsyncSpecification<T, Error>` when a heterogeneous
or runtime-selected collection is needed; that boundary requires
`Send + Sync + 'static` and boxes each returned future.

The following is a conceptual usage shape. The mdBook runner does not link
standalone snippets against the workspace crate; the crate's rustdoc and unit
tests provide the executable verification.

```rust,ignore
use specification_core::AsyncSpecification;

// `rule` is an application-defined AsyncSpecification<u64>.
let result = executor.run(rule.is_satisfied_by(&42));
```

`and` and `or` evaluate operands sequentially and short-circuit according to
their boolean identities. An error from the evaluated operand is returned
immediately; a skipped operand is never polled. `not` maps `Ok(true)` to
`Ok(false)` and vice versa while preserving errors.

The `Send` future contract means a borrowed candidate held across an await
point must be `Sync`. Cancellation remains an executor concern: dropping the
future stops observing it, while any external side effects are governed by the
application's future implementation.
