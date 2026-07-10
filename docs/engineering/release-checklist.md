# 0.x Release Checklist

## Compatibility contract

- MSRV is Rust 1.85; the workspace uses edition 2024.
- Tested targets are `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`, and
  `x86_64-pc-windows-msvc`. Other Rust tier-one targets are best effort.
- Publish in dependency order: `specification-core`,
  `specification-core-serde`, then `specification-core-macros`.

## Release gate

From a clean checkout of the intended commit:

```sh
make check
make coverage
make miri
cargo package --workspace --locked
cargo publish -p specification-core --dry-run --locked
cargo publish -p specification-core-serde --dry-run --locked
cargo publish -p specification-core-macros --dry-run --locked
```

Review `CHANGELOG.md`, manifests, package contents, and the dependency/license
record before approval. Verify crates.io name availability and maintainer owner
access immediately before publishing.

## Irreversible release actions

After explicit maintainer approval, publish in the listed order, wait for each
crate to become available on crates.io, then create annotated tag `vX.Y.Z` and
the GitHub Release from that tag. Do not replace a published crate version.
