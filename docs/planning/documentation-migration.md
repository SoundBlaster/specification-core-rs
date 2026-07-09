# Swift DocC to Rust Documentation Migration

**Status:** Planning input

**Source repository:** `SoundBlaster/SpecificationCore`

**Source path:** `Sources/SpecificationCore/Documentation.docc`

## Source Inventory

The current Swift documentation corpus contains:

- 23 Markdown API and overview articles;
- 4 DocC tutorial files;
- 27 Swift tutorial code resources.

Generated API reference is not part of this count. `.DS_Store` is excluded.

## Destination Model

| Source material | Rust destination | Migration rule |
|---|---|---|
| `SpecificationCore.md` | crate-level rustdoc plus mdBook introduction | Split concise API entry point from conceptual overview |
| `GettingStartedCore.tutorial` | mdBook `getting-started/` chapters | Convert structure mechanically; rewrite all code in Rust |
| `PropertyWrappersGuide.tutorial` | mdBook migration and evaluation-pattern chapters | Explain explicit Rust evaluation instead of imitating wrappers |
| `MacrosAndAdvanced.tutorial` | advanced/planned-feature chapters | Publish only implemented behavior; retain future material as planning input |
| `Tutorials.tutorial` | mdBook `SUMMARY.md` | Convert chapter ordering and links |
| API Markdown articles | rustdoc plus selected concept chapters | Keep item reference in rustdoc; move only extended rationale to mdBook |
| Swift code resources | doctests or `examples/` targets | Rewrite idiomatically and make executable |

## API Article Groups

### Core traits and composition

- `Specification.md`
- `SpecificationOperators.md`
- `AnySpecification.md`
- `AsyncSpecification.md`
- `DecisionSpec.md`
- `ContextProviding.md`

### Context and built-ins

- `EvaluationContext.md`
- `DefaultContextProvider.md`
- `MockContextProvider.md`
- `PredicateSpec.md`
- `FirstMatchSpec.md`
- `MaxCountSpec.md`
- `CooldownIntervalSpec.md`
- `TimeSinceEventSpec.md`
- `DateRangeSpec.md`
- `DateComparisonSpec.md`

### Swift-specific declarative APIs

- `Satisfies.md`
- `AsyncSatisfies.md`
- `Decides.md`
- `Maybe.md`
- `SpecsMacro.md`
- `AutoContextMacro.md`

These articles require semantic adaptation. They must not imply that Rust has
Swift property wrappers or that procedural macros are available before they are
implemented and stabilized.

## Proposed mdBook Structure

```text
docs/
├── book.toml
└── src/
    ├── SUMMARY.md
    ├── README.md
    ├── getting-started/
    ├── concepts/
    ├── guides/
    ├── migration/
    └── design/
```

The exact chapter list is a deliverable of `P0-T3`.

## Migration Rules

1. Preserve conceptual intent, not Swift syntax or API shape.
2. Keep public API reference in rustdoc and avoid duplicating it in mdBook.
3. Write public documentation in English.
4. Use intra-doc links for Rust API items.
5. Prefer doctests for focused examples and `examples/` for complete scenarios.
6. Avoid ignored examples; use `no_run` only when execution is impractical.
7. Mark unavailable features as planned or omit them from published user docs.
8. Maintain traceability from every source tutorial/article to a destination or
   an explicit exclusion.

## Validation Targets

The final commands are set by `P0-T4`, but the documentation system must cover:

```bash
cargo test --workspace --doc
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
mdbook test docs
mdbook build docs
```

## Suggested Migration Passes

1. Convert DocC hierarchy and prose into valid mdBook Markdown.
2. Classify API articles between rustdoc, mdBook, and exclusion.
3. Rewrite Swift snippets as compiling Rust examples.
4. Reconcile chapters with the implemented API.
5. Run documentation gates and perform editorial review.
