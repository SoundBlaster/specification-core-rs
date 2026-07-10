# Dependency and License Review — 0.1.0

**Date:** 2026-07-10

## Package licenses

All workspace packages declare `MIT` through the workspace manifest. The root
MIT license is included in package verification; no copied third-party source
or GPL-only dependency is introduced by this release.

## Runtime dependencies

| Package | Dependency | Purpose | License |
|---|---|---|
| `specification-core` | None | Core remains dependency-free at runtime | — |
| `specification-core-serde` | `serde` | Versioned document serialization | MIT OR Apache-2.0 |
| `specification-core-macros` | `proc-macro2`, `quote`, `syn` | Procedural macro parsing and expansion | MIT OR Apache-2.0 |

## Development-only dependencies

`serde_json` validates serialized documents and fixtures. `criterion` provides
the optional benchmark harness. Their resolved dependency graph is used only
for tests or benchmarks and is not a required dependency of the core package.
All reviewed direct dependencies use permissive MIT/Apache-2.0 compatible
licenses. `cargo package` and publish dry-runs verify the packaged manifests.

## Result

Approved for release preparation. Re-run the package and dry-run commands from
the release checklist on the final tagged commit before publication.
