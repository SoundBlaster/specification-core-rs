# specification-core-rs

An idiomatic Rust implementation of the Specification Pattern. The project is
currently establishing its synchronous core and public quality contract.

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
and the [mdBook source](docs/src/README.md).
