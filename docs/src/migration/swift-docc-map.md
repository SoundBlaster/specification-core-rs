# Swift DocC Migration Map

The Swift `SpecificationCore` DocC corpus is conceptual source material. This
map records an explicit Rust destination or exclusion for every authored DocC
article and tutorial. It does not promise source-level API compatibility.

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
| `AsyncSpecification.md` | deferred async guide | Do not publish as implemented before async ADR and code |
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
| `CooldownIntervalSpec.md` | time-rule guide | Defer until clock policy is implemented |
| `TimeSinceEventSpec.md` | time-rule guide | Defer until clock policy is implemented |
| `DateRangeSpec.md` | time-rule guide | Defer until time representation is chosen |
| `DateComparisonSpec.md` | time-rule guide | Defer until time representation is chosen |

## Swift-specific Declarative Articles

| Swift DocC source | Rust destination | Treatment |
|---|---|---|
| `Satisfies.md` | migration guide | Explain explicit evaluation; no property-wrapper port |
| `AsyncSatisfies.md` | migration and deferred async guide | Do not promise runtime support |
| `Decides.md` | decision guide | Adapt to ordinary Rust values/functions |
| `Maybe.md` | decision guide | Adapt optional decisions to `Option` |
| `SpecsMacro.md` | macro roadmap | Defer until procedural macro crate is justified |
| `AutoContextMacro.md` | macro roadmap | Defer; avoid implicit global context |

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
