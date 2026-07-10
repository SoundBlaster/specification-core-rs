# P2-T3 — Validation Report

**Date:** 2026-07-10

**Verdict:** PASS

## Acceptance Criteria

| Criterion | Evidence | Result |
|---|---|---|
| Explicitly versioned serialization schema | `RuleDocument` emits `schema_version: 1`; deterministic JSON round-trip and unknown-version rejection tests | PASS |
| Separate proc-macro crate | `specification-core-macros` is an independent `proc-macro` workspace package | PASS |
| Core usable without optional integrations | `cargo test -p specification-core --no-default-features`; `cargo tree -p specification-core --edges normal` lists only core | PASS |
| Demonstrated ergonomic value | Macro integration test generates and evaluates `AdultSpec`; serde integration round-trips a declarative rule graph | PASS |

## Quality Gates

| Gate | Command | Result |
|---|---|---|
| Formatting, lint, tests, docs, release | `make check` | PASS — all workspace crates |
| Coverage | `make coverage` | PASS — 93.39% total lines, threshold 90% |
| Miri | `make miri` | PASS — core, serde, and macro integration tests; proc-macro unit tests run outside Miri by toolchain design |
| Package verification | `cargo package --workspace --allow-dirty --locked` | PASS — all three packages |
| Core isolation | `cargo test -p specification-core --no-default-features` | PASS — 24 tests and 3 doctests |

## Dependency and License Evidence

The lockfile contains serde, syn, quote, and proc-macro2. Their installed crate
manifests report `MIT OR Apache-2.0`; all are isolated from the core package's
normal dependency tree.

## Evaluation Outcome

Both prototypes demonstrate a bounded use case and are retained as optional
crates. The schema remains declarative and cannot serialize arbitrary closures
or trait objects. The macro remains intentionally narrow; broader DSLs,
implicit context, and schema-to-executable compilation are deferred.
