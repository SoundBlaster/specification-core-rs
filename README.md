# specification-core-rs

An idiomatic Rust implementation of the Specification Pattern. The synchronous
core evaluates borrowed candidates and composes concrete rules without dynamic
dispatch or heap allocation.

```rust
use specification_core::Specification;

let is_adult = |age: &u8| *age >= 18;
let is_retired = |age: &u8| *age >= 65;
let working_age = is_adult.and(is_retired.not());

assert!(working_age.is_satisfied_by(&42));
assert!(!working_age.is_satisfied_by(&70));
```

## Development

The workspace requires Rust 1.85 or newer. Run the standard local quality gate:

```bash
make check
```

Install the documentation tool once before the first `make check`:

```bash
cargo +stable install mdbook --version 0.5.4 --locked
```

Additional quality paths:

```bash
make coverage
make miri
make package
```

`make coverage` requires `cargo-llvm-cov`; `make miri` requires the nightly
Miri component. CI provisions both tools automatically.

See the [contribution guide](CONTRIBUTING.md), the
[toolchain and quality policy](docs/engineering/toolchain-quality-release-policy.md),
the [release checklist](docs/engineering/release-checklist.md), and the
[mdBook source](docs/src/README.md).
