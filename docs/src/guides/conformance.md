# Cross-language conformance

The Rust crate is an idiomatic implementation, not a source-level translation
of the Swift package. Cross-language compatibility is therefore expressed as
observable behavior in the versioned JSON corpus at
`fixtures/conformance/v1/`.

## Running the corpus

The Rust adapter is an integration test and is included in the normal test
gate:

```sh
make test
```

It reads the manifest, executes every listed fixture, and verifies both the
outcome and the named evaluation trace. A missing fixture, invalid JSON, or an
unknown schema version fails the test.

## Adapter contract

An adapter for another implementation must:

1. Read `manifest.json`, then each listed JSON fixture.
2. Require the document's `schema_version` to be supported.
3. Evaluate each case with the supplied candidate and context input.
4. Compare `expected.result` or `expected.decision`, plus the ordered
   `expected.trace`.
5. Report unsupported fixture groups explicitly; it must not silently skip
   them.

The trace consists of probe names, not internal method calls. It specifies the
observable ordering and short-circuit behavior of composition and first-match
rules.

## Semantics in v1

| Group | Contract |
|---|---|
| `composition` | AND and OR short-circuit left-to-right; NOT negates; empty all-of is true and empty any-of is false. |
| `first_match` | Rules are ordered; the first satisfied rule wins; no match is `None`; an explicit fallback is returned only by fallback evaluation. |
| `context` | Missing counters are zero, missing flags are false, `MaxCount` is strict `<`, and cooldown uses `>=` at the boundary. Future timestamps do not satisfy cooldown, including zero duration. |

All context timestamps and `now_ms` values are non-negative integer
milliseconds in an abstract monotonic epoch. Adapters must not substitute wall
clock time.

## Intentional differences and limits

Rust evaluates a borrowed candidate (`&T`); Swift's protocol input convention
does not need to be copied to satisfy a fixture. Rust also has no global default
context provider, property wrappers, Combine observation, or Apple-platform
date integration. Context and clocks are injected explicitly.

The v1 corpus does not cover async scheduling or typed async errors because
those would require a shared executor and error-transport contract. Rust async
behavior remains specified by the `AsyncSpecification` API and its tests.

`specification-core-serde` uses a separate versioned `RuleDocument` schema.
Conformance fixtures are test data, not a promise that every executable rule
can be serialized or reconstructed.
