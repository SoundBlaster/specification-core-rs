# Swift Reference Audit and Rust Porting Matrix (Maintainer Reference)

> Historical planning artifact from Phase 0. For the current public migration
> walkthrough, use [Swift to Rust](swift-to-rust.md).

**Audited source:** `SoundBlaster/SpecificationCore`

**Revision:** `7909e62da7ca0416dc24c8165bfd0e6d4cf16e57`

**Audit date:** 2026-07-10

## Evidence Scope

The audited Swift package has a library target, a macro target, and one test
target. Its manifest declares Swift tools 5.10, Apple platform minimums, and
dependencies on SwiftSyntax, MacroTesting, and the Swift DocC plugin.

The implementation contains 23 non-DocC Swift files in `SpecificationCore`,
three macro implementation files, and 12 XCTest source files. The test suite
includes decision, first-match, date, async, wrapper, macro, smoke, and
performance test groups. This inventory is source evidence; it does not confirm
the README's coverage, latency, or concurrency claims.

## Build Baseline

Command:

```text
swift test
```

Toolchain:

```text
Apple Swift version 6.4 (swiftlang-6.4.0.23.5 clang-2100.3.23.5)
Target: arm64-apple-macosx26.0
```

Result: **FAIL before tests run**. `FirstMatchSpec.Builder.build()` at
`Sources/SpecificationCore/Specs/FirstMatchSpec.swift:206` has an ambiguous
call between the erased-pair initializer and generic initializer. The build
also warns that `Documentation.docc` is unhandled by the Swift target.

This failure prevents a current pass/fail statement about the Swift test suite.
It does not invalidate source-level behavior that is independently covered by
specific XCTest files; it must not be ported as a Rust behavior.

## Source Evidence

| Area | Source paths | Test evidence |
|---|---|---|
| Core composition | `Core/Specification.swift`, `Core/SpecificationOperators.swift` | `SpecificationCoreTests.swift` |
| Type erasure and collection rules | `Core/AnySpecification.swift` | `AnySpecificationPerformanceTests.swift` |
| Decisions | `Core/DecisionSpec.swift`, `Specs/FirstMatchSpec.swift` | `DecisionSpecTests.swift`, `FirstMatchSpecTests.swift` |
| Context and providers | `Context/EvaluationContext.swift`, `Context/DefaultContextProvider.swift`, `Core/ContextProviding.swift` | smoke and wrapper tests |
| Built-in rules | `Specs/PredicateSpec.swift`, `MaxCountSpec.swift`, time/date specs | date and wrapper tests |
| Async | `Core/AsyncSpecification.swift`, `Wrappers/AsyncSatisfies.swift` | `AsyncFeaturesTests.swift`, `AsyncSatisfiesWrapperTests.swift` |
| Declarative/macro APIs | `Wrappers/*.swift`, `Definitions/AutoContextSpecification.swift`, `SpecificationCoreMacros/*.swift` | wrapper and macro tests |

## Capability Matrix

| Swift capability | Classification | Rust destination | Rationale |
|---|---|---|---|
| `Specification` boolean contract | Adopt | P1-T2 | Core domain abstraction |
| Static `and`, `or`, `not` composition | Adopt | P1-T2 | Preserve boolean and short-circuit semantics |
| Swift `&&`, `||`, `!` overloads | Adapt | P1-T2 | Prefer named combinators initially |
| `PredicateSpec` closure wrapper | Adopt | P1-T2 | Implement through a closure-compatible trait or adapter |
| Constants and collection all/any | Adopt | P1-T2 | Preserve logical identities |
| `AnySpecification` | Adapt | P1-T3 | Use explicit Rust trait objects and ownership bounds |
| `DecisionSpec` and boolean-result adapter | Adopt | P1-T3 | Preserve typed optional decision model |
| `FirstMatchSpec` and fallback | Adopt | P1-T3 | Preserve priority and short-circuit behavior |
| `FirstMatchSpec.Builder` | Adapt | P1-T3 | Avoid Swift overload ambiguity |
| `EvaluationContext` counters, flags, events, segments | Adapt | P1-T4 | Preserve concepts with typed/generic data |
| `[String: Any]` user data | Exclude | P1-T4 | Replace with generic user data or deliberate typed values |
| Generic/static context providers | Adapt | P1-T4/P2-T1 | Keep dependency injection with Rust sharing bounds |
| Default provider, locks, Combine updates | Exclude | P2-T1 | Avoid global state and Apple-specific observation |
| Counter and flag rules | Adapt | P1-T4 | Retain semantics after context design is fixed |
| Cooldown, event, and date rules | Defer | P1-T4 | Require clock/time representation policy |
| Property wrappers | Exclude | Migration guide | Rust uses explicit evaluation and `Option` |
| `AsyncSpecification` and async wrappers | Defer | P2-T2 | Requires runtime-neutral future/object-safety design |
| Swift macros and auto-context | Defer | P2-T3 | Separate procedural macro decision and crate boundary |
| Domain composite examples | Exclude | Examples/docs | Demonstrations, not reusable core API |
| Combine observation | Exclude | None | Platform-specific integration |
| README performance figures | Exclude | P3-T2 | Re-establish with Rust benchmarks |

## Behavioral Contract for Rust Tests

| Behavior | Swift evidence | Rust requirement |
|---|---|---|
| AND evaluates left first and skips right after false | `Specification.swift` | Preserve short-circuiting |
| OR evaluates left first and skips right after true | `Specification.swift` | Preserve short-circuiting |
| NOT negates its wrapped result | `Specification.swift` | Preserve boolean complement |
| Empty all-of is true; empty any-of is false | `AnySpecification.swift` | Preserve logical identities |
| First match wins and later pairs are not evaluated | `DecisionSpecTests.swift` | Preserve ordered short-circuiting |
| No match produces no result; fallback produces a result | `FirstMatchSpecTests.swift` | Model as `Option` and explicit fallback |
| Missing counter is zero; missing flag is false | `EvaluationContext.swift` | Document and test if this context model is adopted |
| `MaxCountSpec` uses strict `< maximum_count` | `MaxCountSpec.swift` | Preserve strict bound |
| Missing cooldown/event timestamp permits evaluation | time specs | Preserve only after clock API exists |
| Cooldown threshold uses `>=` | time specs | Preserve boundary inclusivity |
| Date range is inclusive; missing comparison event is false | date specs | Preserve only after time policy is decided |
| Async evaluation can fail | `AsyncSpecification.swift` | Preserve error propagation in P2-T2 |

Rust specifications receive candidates by reference unless a later API decision
requires ownership. This is an intentional adaptation of Swift's by-value
protocol method and is not a behavioral incompatibility.

## P1-T2 Boundary

P1-T2 implements only the synchronous boolean core: trait contract, closure
predicates, static composition, constants, collection identities, tests, and
documentation. It excludes dynamic dispatch, decisions, context, time,
providers, async, wrappers, macros, and domain examples.

## Provenance and License

The audited Swift repository declares the MIT License, copyright 2025 Egor
Merkushev. This Rust repository also uses MIT. The Rust implementation is an
independent rewrite; copied substantial code, prose, or assets must retain the
applicable copyright and license notice. Swift package dependencies are not
inherited by the Rust core merely because they appear in the Swift manifest.

## Relationship to Documentation Migration

The [DocC migration map](swift-docc-map.md) tracks authored
documentation destinations. This audit tracks executable behavior and source
classification. Both documents must be consulted before a Rust feature is
declared compatible with Swift.
