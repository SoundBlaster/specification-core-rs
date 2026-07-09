# P0-T5 Validation Report — Swift Reference Audit and Porting Matrix

**Verdict:** PASS WITH REFERENCE BASELINE FAILURE RECORDED

**Date:** 2026-07-10

## Deliverable Checks

| Check | Result |
|---|---|
| Swift package revision, manifest, license, sources, and tests were inspected | PASS |
| Current `swift test` baseline was run and exact failure recorded | PASS |
| Major public API families have adopt/adapt/defer/exclude classifications | PASS |
| Core composition, decision, context, time, and async semantics are recorded | PASS |
| P1-T2 scope and exclusions are explicit | PASS |
| DocC migration map links to the executable-source audit | PASS |
| `git diff --check` passes | PASS |

## Commands Run

```text
swift --version
swift test
git rev-parse HEAD
git log -1 --format='%H %s'
find Sources/SpecificationCore and Tests/SpecificationCoreTests
rg public declarations and focused XCTest behavior
git diff --check
```

The Swift baseline used Apple Swift 6.4 on arm64 macOS 26 and failed before
tests at `FirstMatchSpec.Builder.build()` because its initializer call is
ambiguous. The audit records this as a reference defect, not a Rust requirement.

## Quality Gates Not Applicable

The Rust Cargo workspace and mdBook executable are still owned by P1-T1. This
task is documentation and reference-audit work; it does not claim Rust build,
Clippy, coverage, rustdoc, or mdBook execution.

## Follow-up

No new Phase 0 task is needed. P1-T2 uses the matrix to implement only the
synchronous boolean core and to create its initial behavioral test suite.
