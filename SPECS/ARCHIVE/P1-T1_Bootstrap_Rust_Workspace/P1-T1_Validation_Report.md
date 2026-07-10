# P1-T1 Validation Report — Bootstrap Rust Workspace

**Verdict:** PASS WITH REMOTE CI PENDING

**Date:** 2026-07-10

## Deliverable Checks

| Check | Result |
|---|---|
| Rust 2024 workspace and `specification-core` crate exist | PASS |
| Package declares MSRV 1.85, MIT metadata, README, and crate-local license | PASS |
| Core crate forbids unsafe code and inherits workspace lints | PASS |
| Makefile exposes format, lint, test, doctest, docs, release, package, coverage, and Miri paths | PASS |
| CI config covers Linux/macOS/Windows stable tests and Linux MSRV | PASS |
| CI config covers formatting, Clippy, release/package, docs, coverage, and Miri | PASS |
| mdBook Pages workflow builds/tests on PR and deploys only from `main` | PASS |
| GitHub Actions remote execution | PENDING: requires pushed PR |

## Commands Run

```text
make check
cargo package -p specification-core --allow-dirty
make coverage
make miri
cargo +stable fmt --all -- --check
cargo +stable clippy --workspace --all-targets --all-features -- -D warnings
cargo +stable test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo +stable doc --workspace --all-features --no-deps
cargo +stable build --workspace --release
cargo +stable package -p specification-core --allow-dirty
mdbook test docs
mdbook build docs
```

All listed local commands completed successfully. Both the MSRV path (Rust
1.85) and stable path compiled and tested the bootstrap crate. The crate has no
behavioral API yet, so unit and doctest suites correctly contain zero tests.

## Workflow Validation

Both GitHub Actions workflow files parsed successfully as YAML. `actionlint` is
not installed locally; GitHub Actions is the authoritative validation for
workflow execution and remains pending until this branch is pushed.

## Coverage Note

`cargo llvm-cov --summary-only` cannot produce a report for a crate with zero
instrumented behavior. The bootstrap path uses `--no-report` and passes. P1-T2
must replace it with the documented 90% core line-coverage gate once tests and
behavior exist.

## Follow-up

Push the branch and inspect GitHub Actions. P1-T2 can now implement the
synchronous Specification API against a verified workspace and CI baseline.
