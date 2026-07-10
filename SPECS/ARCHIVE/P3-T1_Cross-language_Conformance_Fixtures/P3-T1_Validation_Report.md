# P3-T1 Validation Report — Cross-language Conformance Fixtures

**Date:** 2026-07-10
**Verdict:** PASS

## Deliverables

- Added versioned JSON fixtures and manifest under `fixtures/conformance/v1/`.
- Added a Rust integration adapter that embeds the listed fixtures at compile
  time, verifies schema version, outcomes, and observable evaluation traces.
- Added the cross-language adapter contract, v1 limits, and ADR-0007.
- Kept the corpus separate from the optional `specification-core-serde` rule
  document format.

## Quality Gates

| Gate | Command | Result |
|---|---|---|
| Format, Clippy, tests, doctests, rustdoc, mdBook, release build | `make check` | PASS |
| Line coverage | `make coverage` | PASS — 93.39% total lines (threshold: 90%) |
| Undefined-behavior interpreter | `make miri` | PASS |
| Core crate package verification | `make package` | PASS |
| Focused fixture adapter | `cargo test -p specification-core --test conformance_fixtures` | PASS |

## Notes

The adapter uses `include_str!` rather than runtime filesystem reads. The JSON
files remain independently consumable by Swift or another adapter, while the
Rust test remains compatible with Miri's default isolation mode.

Async scheduling and error transport are intentionally outside fixture schema
v1 and remain covered by the runtime-neutral Rust async contract and tests.
