# Toolchain, Quality, and Release Policy

## Rust Baseline

The initial workspace uses Rust 2024 with `resolver = "3"` and declares
`rust-version = "1.85"` for every published package. Rust 1.85 is the minimum
supported Rust version (MSRV) because it is the release that stabilizes the
2024 edition. The pinned development toolchain may be newer, but it does not
change the published MSRV contract.

P1-T1 adds `rust-toolchain.toml`, workspace metadata, and CI jobs that check the
MSRV and current stable toolchain. Any MSRV increase requires an ADR, a
changelog note, and a release compatibility assessment.

## Supported Platforms

The core crate is platform-neutral Rust. The initial support tiers are:

| Tier | Target | CI expectation |
|---|---|---|
| Tested | `x86_64-unknown-linux-gnu` | Build, test, lint, docs |
| Tested | `aarch64-apple-darwin` | Build and test |
| Tested | `x86_64-pc-windows-msvc` | Build and test |
| Compatible | Other Rust tier-one targets | Best effort unless a platform dependency is added |

No platform-specific dependency may enter the core crate without an ADR and an
updated target matrix.

## Required Quality Gates

P1-T1 must make these commands available locally and in CI:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
mdbook build docs
mdbook test docs
```

Coverage uses `cargo llvm-cov`. P1-T1 validates the instrumented test path with
`--no-report`, because a bootstrap crate with no behavior produces no coverage
profile. P1-T2 replaces that bootstrap command with a core line-coverage gate
of at least 90% once executable behavior exists. A temporary exception needs a
tracked task with a reason and expiry.

## CI Parity with the Swift Reference

The original Swift package verifies multiple Apple toolchains, multiple Linux
Swift versions, formatting, release builds, Thread Sanitizer, and DocC Pages.
The Rust workflow preserves the same quality intent with Rust-native checks:

| Swift reference gate | Rust equivalent |
|---|---|
| macOS test matrix | stable Rust tests on macOS |
| Linux Swift-version matrix | Rust 1.85 MSRV and stable tests on Linux |
| Platform coverage | stable Rust tests on Linux, macOS, and Windows |
| SwiftFormat | rustfmt and Clippy with warnings denied |
| Thread Sanitizer | nightly Miri test job |
| Release build | Cargo release build and package verification |
| DocC build and Pages deployment | mdBook/rustdoc build, doctests, and Pages deployment |

The Rust matrix intentionally adds Windows and coverage instrumentation because
the core crate is platform-neutral and has an explicit coverage rollout policy.

## Dependencies and Features

- The synchronous core starts with zero required third-party dependencies unless
  a task and review demonstrate a clear benefit.
- Every dependency needs a compatible permissive license, active maintenance,
  a documented purpose, and MSRV compatibility.
- Use bounded SemVer requirements; never use wildcard requirements.
- Optional integration dependencies use explicit Cargo features and must not
  change the default core behavior.
- Features are additive and documented in rustdoc and the package manifest.
- GPL-only code or dependencies require an ADR that records the distribution
  consequences and maintainer approval.

## Unsafe-Code Policy

The core crate uses `#![forbid(unsafe_code)]`. A future exception requires an
ADR, a minimal isolated boundary, documented safety invariants, tests, and
review by a maintainer. Prefer a separate crate when an unsafe implementation
cannot remain small and auditable.

## Package and Release Policy

- The intended crates.io package name is `specification-core`, pending
  availability verification before publication.
- Releases use SemVer and a maintained `CHANGELOG.md`.
- Before `1.0`, breaking public API changes still require a documented rationale
  and a minor-version bump; avoid gratuitous churn within a `0.x` line.
- A release requires a clean checkout, passing quality gates, `cargo package
  --locked`, `cargo publish --dry-run`, documentation verification, license
  review, and maintainer approval.
- Tags use `vX.Y.Z`. Publishing ownership remains with SoundBlaster maintainers
  until a governance ADR changes it.

## P1-T1 Implementation Checklist

- Add workspace and package manifests with the selected edition, resolver, and
  MSRV.
- Add rustfmt, Clippy, rustdoc, test, mdBook, MSRV, and target-matrix CI jobs.
- Add a reproducible local command surface such as `make check`.
- Add the initial core crate with `forbid(unsafe_code)` and inherited lints.
- Add coverage tooling configuration without claiming a threshold has passed
  before P1-T2 introduces executable behavior.
