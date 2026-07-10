# P3-T3 — Publish 0.1.0

**Status:** INPROGRESS
**Priority:** P1
**Dependencies:** P2-T2, P3-T2
**Date:** 2026-07-10

## Goal

Prepare and validate the first public 0.1.0 release for every workspace crate.
This task produces a mergeable release-preparation PR. Actual crates.io
publication, the `v0.1.0` tag, and GitHub Release require explicit maintainer
confirmation after this PR is merged.

## Deliverables

- `CHANGELOG.md` with 0.1.0 stability guarantees and known limitations.
- Release documentation for MSRV, platforms, crate relationships, and a
  maintainer release checklist.
- Dependency and license review covering runtime and development dependencies.
- Clean-checkout package and `cargo publish --dry-run` verification for
  `specification-core`, `specification-core-serde`, and
  `specification-core-macros` in publish order.

## Acceptance Criteria

- MSRV 1.85 and support tiers are documented.
- All publishable packages verify cleanly.
- License/dependency review is recorded.
- Release notes accurately describe 0.x guarantees and limitations.

## Non-goals

Do not run `cargo publish`, create a tag, or create a GitHub Release without
an explicit confirmation after release preparation has merged.
