# specification-core Guide

This book explains the concepts, design choices, and migration path for
`specification-core`. It complements the crate's rustdoc API reference rather
than duplicating it.

The synchronous Rust core has not been implemented yet. The guide therefore
documents accepted project direction and planned learning paths without claiming
that unreleased APIs are available.

## Documentation Surfaces

- rustdoc documents public Rust items, signatures, invariants, and focused
  executable examples.
- This book documents concepts, end-to-end guides, architecture, and migration
  from Swift `SpecificationCore`.
- `examples/` will hold complete runnable scenarios after the Cargo workspace
  exists.
