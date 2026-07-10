# Documentation Guide

## Scope and Language

Write public API documentation, user guides, examples, release notes, and
contributor-facing documentation in technical English. Keep internal planning
artifacts understandable to maintainers, but do not make a public user depend
on untranslated chat history.

## Documentation Surfaces

rustdoc is the authoritative API reference. Use `//!` for crate and module
documentation and `///` for public items. mdBook explains concepts, tutorials,
architecture, and migration. Do not duplicate every item-level API description
in mdBook; link readers to rustdoc instead.

Every public module, trait, struct, enum, public function, public method,
public field, associated type, and Cargo feature must be documented once the
core crate exists. Begin each item with a concise summary sentence. Use
`# Examples`, `# Errors`, `# Panics`, and `# Safety` sections whenever they
apply.

## Examples

Use a doctest for a focused API example and a Cargo `examples/` target for a
complete scenario. Examples must be executable whenever practical. Prefer
ordinary runnable code; use `no_run` only when execution would be impractical
and explain why. Do not use `ignore` without a written justification.

Use intra-doc links such as [`crate::Specification`] for public Rust items.
Public behavior changes update rustdoc, relevant book chapters, and executable
examples in the same task.

## mdBook

Keep every path named in `docs/src/SUMMARY.md` present. Run `mdbook build docs`
and `mdbook test docs` after mdBook is installed. The book must state the status
of unreleased APIs honestly and must not present a planned feature as available.

## Swift DocC Migration

The Swift corpus is source material, not a copy target. Preserve conceptual
intent, but rewrite Swift property wrappers as explicit Rust evaluation,
protocol existentials as traits or explicit trait objects, `Any` context storage
as typed/generic data, and Swift concurrency examples as runtime-neutral Rust
design when applicable.

Maintain the mapping in [the DocC migration map](../src/migration/swift-docc-map.md).
Every source article and tutorial must have a destination or explicit exclusion.

## Validation

When the toolchain exists, documentation changes run:

```text
cargo test --workspace --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
mdbook test docs
mdbook build docs
```

Until P1-T1 installs these tools and projects, validation reports must record
which checks were structurally verified and which checks were unavailable.
