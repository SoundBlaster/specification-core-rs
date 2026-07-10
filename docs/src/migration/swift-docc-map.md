# Swift DocC Migration Map (Maintainer Reference)

The Swift `SpecificationCore` DocC corpus is conceptual source material. This
historical map records an explicit Rust destination or exclusion for every
authored DocC article and tutorial. The reader-facing comparison is
[Swift to Rust](swift-to-rust.md); this page is traceability data and does not
promise source-level API compatibility.

For executable-source evidence and adopt/adapt/defer/exclude decisions, see the
[Swift reference audit](swift-reference-audit.md).

## Overview and Core API Articles

| Swift DocC source | Rust destination | Treatment |
|---|---|---|
| `SpecificationCore.md` | crate rustdoc and this book introduction | Split API overview from conceptual guide |
| `Specification.md` | `Specification` rustdoc and concepts | Rewrite trait semantics |
| `SpecificationOperators.md` | combinator rustdoc and getting-started | Adapt operators to Rust ergonomics |
| `AnySpecification.md` | dynamic-dispatch rustdoc and concepts | Adapt type erasure to explicit trait objects |
| `DecisionSpec.md` | decision rustdoc and concepts | Preserve ordered typed decision rationale |
| `AsyncSpecification.md` | async specification guide and rustdoc | Implemented as runtime-neutral RPITIT futures with explicit Send bounds |
| `ContextProviding.md` | context rustdoc and concepts | Adapt provider model to dependency injection |

## Context and Built-in Specification Articles

| Swift DocC source | Rust destination | Treatment |
|---|---|---|
| `EvaluationContext.md` | context rustdoc and concepts | Replace string-to-`Any` storage with typed/generic design |
| `DefaultContextProvider.md` | context guide | Exclude global singleton semantics |
| `MockContextProvider.md` | testing guide and test helpers | Adapt to Rust test fixtures |
| `PredicateSpec.md` | core rustdoc | Use closure implementation when appropriate |
| `FirstMatchSpec.md` | decision rustdoc and example | Preserve deterministic priority behavior |
| `MaxCountSpec.md` | built-in rules guide | Rewrite against Rust context API |
| `CooldownIntervalSpec.md` | decisions/context guide | Implemented with an injected `Clock` and `Duration` |
| `TimeSinceEventSpec.md` | no Rust 0.1.0 equivalent | Not ported; application-owned rule |
| `DateRangeSpec.md` | no Rust 0.1.0 equivalent | Not ported; application-owned rule |
| `DateComparisonSpec.md` | no Rust 0.1.0 equivalent | Not ported; application-owned rule |

## Swift-specific Declarative Articles

| Swift DocC source | Rust destination | Treatment |
|---|---|---|
| `Satisfies.md` | migration guide | Explain explicit evaluation; no property-wrapper port |
| `AsyncSatisfies.md` | async specification guide | Rewrite as explicit future evaluation; executor remains caller-owned |
| `Decides.md` | decision guide | Adapt to ordinary Rust values/functions |
| `Maybe.md` | decision guide | Adapt optional decisions to `Option` |
| `SpecsMacro.md` | serialization and macros guide | Implemented as an explicit predicate macro in a separate crate |
| `AutoContextMacro.md` | serialization and macros guide | Exclude implicit context; retain explicit dependency injection |

## Tutorials

| Swift DocC source | Rust destination | Treatment |
|---|---|---|
| `Tutorials.tutorial` | `SUMMARY.md` | Convert tutorial hierarchy to book navigation |
| `GettingStartedCore.tutorial` | `getting-started/` | Convert prose; rewrite each Swift code resource as Rust doctest or example |
| `PropertyWrappersGuide.tutorial` | `migration/` | Explain idiomatic explicit evaluation instead of wrappers |
| `MacrosAndAdvanced.tutorial` | `guides/` and macro roadmap | Retain concepts; gate all macro claims on implementation |

## Source-Code Resource Rule

All 27 Swift tutorial code resources are mapped individually during the
corresponding chapter implementation. Each resource becomes one of: a Rust
doctest, a complete Cargo example, an adapted conceptual snippet, or an
explicit exclusion with a reason. The migration issue must preserve this
resource-level traceability rather than only the tutorial-level mapping above.
