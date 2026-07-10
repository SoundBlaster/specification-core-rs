# Architecture Baseline

## Initial Workspace

The repository will become a Cargo workspace in `P1-T1`. Its initial public
package is a single library crate:

```text
specification-core-rs/
└── crates/
    └── specification-core/
```

The root owns repository-wide metadata, CI, contributor documentation, and
release policy. The core crate owns the synchronous Specification Pattern API.
No implementation crate is created by this document; `P1-T1` implements this
boundary after the toolchain policy is decided.

## Core Responsibilities

The first crate may contain:

- the `Specification` trait and static boolean combinators;
- explicit dynamic specification wrappers;
- typed decision specifications and deterministic first-match evaluation;
- immutable evaluation-context abstractions and built-in rules;
- rustdoc examples and testable public contracts.

It must not require an async runtime, FFI toolchain, UI framework, or optional
serialization dependency for its normal synchronous use.

## Extension Boundaries

Future capabilities belong in separate crates when they introduce a distinct
dependency surface, compiler integration, or platform boundary:

```text
specification-core              ← synchronous public core
specification-core-macros       ← optional procedural macros
specification-core-serde        ← optional serialization integration
specification-core-ffi          ← optional foreign-language facade
```

Extensions depend on the core crate; the core crate must not depend on an
extension. An extension is added only after an ADR defines its user value,
feature boundary, ownership model, and release implications.

## Documentation Boundaries

- rustdoc and docs.rs provide item-level API reference.
- mdBook provides conceptual guides, tutorials, and migration material.
- `docs/architecture/` records stable architecture explanations.
- `docs/adr/` records individual irreversible or costly decisions.
- `docs/planning/` preserves pre-ADR inputs and source inventories.

## Dependency Direction

```text
applications
    │
optional extension crates ───► specification-core
    │
documentation and conformance fixtures
```

No optional extension may make the core crate depend on it transitively.
