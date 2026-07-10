# P1-T2 Validation Report — Core Specification API

**Verdict:** PASS WITH REMOTE CI PENDING

**Date:** 2026-07-10

## Deliverable Checks

| Check | Result |
|---|---|
| `Specification<T>` evaluates borrowed candidates | PASS |
| Compatible closures implement `Specification<T>` | PASS |
| Concrete `And`, `Or`, and `Not` preserve boolean semantics | PASS |
| AND and OR short-circuit in the required direction | PASS |
| `Always`, `Never`, `AllOf`, and `AnyOf` have documented identities | PASS |
| Public API has rustdoc examples and narrative mdBook coverage | PASS |
| Local and CI coverage commands enforce at least 90% line coverage | PASS |
| GitHub Actions remote execution | PENDING: requires pushed PR |

## Commands Run

```text
make check
make coverage
make miri
make package
cargo +stable fmt --all -- --check
cargo +stable clippy --workspace --all-targets --all-features -- -D warnings
cargo +stable test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo +stable doc --workspace --all-features --no-deps
```

All commands passed. The MSRV path and stable path each compiled and tested the
crate. The unit suite contains nine tests; the crate-root doctest passed.

## Coverage Evidence

`cargo llvm-cov --workspace --all-features --fail-under-lines 90` passed with
100.00% total line coverage (159 of 159 lines) for the core crate. The command
now enforces the 90% requirement locally and in GitHub Actions, replacing the
P1-T1 bootstrap `--no-report` path.

## Documentation and Workflow Validation

`mdbook test docs`, `mdbook build docs`, rustdoc, and crate doctests passed.
The mdBook snippets are explicitly marked `ignore` with a written reason: its
isolated rustdoc harness cannot link the workspace crate; their API behavior is
verified by the executable crate-root doctest. Both workflow files parsed as
YAML. `actionlint` is not installed locally.

## Follow-up

Open the stacked P1-T2 pull request, inspect remote GitHub Actions, then begin
P1-T3 from this branch so its dynamic-dispatch and decision API remains a
separate reviewable delta.
