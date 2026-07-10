# specification-core Guide

This book explains the concepts, design choices, and migration path for
`specification-core`. It complements the crate's rustdoc API reference rather
than duplicating it.

The synchronous Rust core is implemented and the guide documents its accepted
contracts, usage patterns, and planned learning paths. Features not yet
implemented remain marked as future work in the relevant chapters.

## Documentation Surfaces

- rustdoc documents public Rust items, signatures, invariants, and focused
  executable examples.
- This book documents concepts, end-to-end guides, architecture, and migration
  from Swift `SpecificationCore`.
- `examples/` will hold complete runnable scenarios after the Cargo workspace
  exists.
