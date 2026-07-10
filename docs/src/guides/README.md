# Documentation and Testing

Public behavior is documented and tested together. Focused snippets belong in
rustdoc doctests; complete scenarios belong in Cargo `examples/` or integration
tests; this book holds narrative material that connects several APIs or
explains a design.

The book's standalone snippets are marked `rust,ignore` when they need the
workspace crate. Every such workflow links to a compiled example or test. Run
the complete local documentation gate with:

```sh
make docs
```

The detailed conventions are defined in the contributor
[documentation guide](https://github.com/SoundBlaster/specification-core-rs/blob/main/docs/contributing/documentation.md).
