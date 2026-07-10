# P3-T3 Validation Report — Publish 0.1.0

**Date:** 2026-07-10
**Verdict:** PASS — RELEASE PREPARATION COMPLETE

## Release evidence

- MSRV is Rust 1.85; support tiers and release sequence are documented in the
  release checklist.
- `CHANGELOG.md` describes public 0.1.0 content, pre-1.0 SemVer policy, and
  known limitations.
- The dependency/license review records package licenses, direct dependency
  purpose, and permissive compatibility.
- `cargo search specification-core --limit 5` returned no package with the
  intended name at validation time. Owner access must still be verified by the
  maintainer immediately before publication.

## Quality gates

| Gate | Result |
|---|---|
| `make check` | PASS |
| `make coverage` | PASS — 93.39% total line coverage |
| `make miri` | PASS |
| `cargo package --workspace --allow-dirty --locked` | PASS — all 3 crates verified from package staging |
| `cargo publish -p specification-core --dry-run --allow-dirty --locked` | PASS |
| `cargo publish -p specification-core-serde --dry-run --allow-dirty --locked` | PASS |
| `cargo publish -p specification-core-macros --dry-run --allow-dirty --locked` | PASS |

## Publication hold

This task intentionally did not run `cargo publish`, create `v0.1.0`, or create
a GitHub Release. Those irreversible actions require explicit maintainer
confirmation after this release-preparation PR is merged.
