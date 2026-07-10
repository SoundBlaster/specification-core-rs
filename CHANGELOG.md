# Changelog

All notable changes to this project are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project follows Semantic Versioning.

## [0.1.0] - 2026-07-10

### Added

- `specification-core`: synchronous and runtime-neutral asynchronous
  specification traits, allocation-free static boolean composition, explicit
  boxed/shared boundaries, typed first-match decisions, immutable evaluation
  context, and deterministic built-in counter, flag, and cooldown rules.
- `specification-core-serde`: versioned declarative rule documents as an
  opt-in integration crate.
- `specification-core-macros`: opt-in named predicate specification macro.
- Cross-language conformance fixtures, runnable examples, mdBook guides, and
  static-versus-dynamic dispatch benchmark baseline.

### Stability and limitations

- This is the first `0.x` release. Public APIs are documented and follow
  SemVer; breaking changes before 1.0 will use a minor-version bump with a
  documented rationale.
- The core is runtime-neutral and does not select an async executor, provide
  FFI, global mutable context, or platform-specific integrations.
- Serialization is declarative only and cannot represent arbitrary closures,
  trait objects, or generic application data.
- The macro and serde crates are optional; `specification-core` has no required
  runtime dependencies.

[0.1.0]: https://github.com/SoundBlaster/specification-core-rs/releases/tag/v0.1.0
