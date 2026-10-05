# P4-T2 validation report

Date: 2026-10-04. Status: local implementation validated; remote CI pending.

## Behavioral evidence

The additive index preserves the original insertion priority between the matching
key bucket and unkeyed rules. Eight focused tests cover three seeded mixed
catalogs, unknown keys, empty/no-match, duplicate keys, both priority directions,
short-circuit invocation counts, one borrowed projection and Arc/worker use.
Keyed predicates in all examples honor the documented necessary-key contract.

## Quality gates

- Workspace all-feature tests, format, all-target Clippy and rustdoc: passed.
- Doctests, mdBook build and mdBook test: passed.
- `cargo llvm-cov --workspace --all-features --fail-under-lines 90`: passed,
  93.76% total lines, 98.58% core lines.
- Python sample/gate validation: 4 tests passed; recorded benchmark passes the
  coarse catalog scaling budget.
- Rendered link validation unavailable: `lychee` is not installed locally.
- CI adds Linux/macOS Release samples and a scaling gate. Remote jobs have not
  been observed yet; platform matrix and Miri remain remote validation.

## Release benchmark

Actual core crate, Rust 1.85.0, Apple M4 Pro/arm64 macOS. 20,000 candidates,
31 samples per implementation/catalog, warm-up and alternating order. Per-item
parity and checksum checks include missing keys. Power mode was not recorded.

| Catalog | Linear median | Indexed median |
|---|---:|---:|
| 14 keyed targets | 54.408 ns/candidate | 19.983 ns/candidate |
| 200 unique unrelated keys + 14 targets | 369.800 ns/candidate | 23.494 ns/candidate |

Raw CSV and environment logs are generated in `target/benchmark-artifacts/`
and uploaded by CI. These synthetic figures establish skipped unrelated rules,
not filesystem scan speed. Unkeyed lists and large individual buckets still
have linear cost. No crate version was changed.
