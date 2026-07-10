# Contributing to specification-core-rs

Thank you for contributing. This repository is building an idiomatic Rust implementation of the Specification Pattern; see the [project charter](docs/project-charter.md) before proposing an API or architecture change.

## Before You Start

- Check `SPECS/Workplan.md` and open pull requests for existing work.
- Discuss a new public API, dependency, feature flag, async model, FFI boundary, or compatibility commitment before implementation.
- Use an ADR for decisions with lasting public, security, or maintenance impact.
- Keep pull requests focused on one Workplan task or a narrowly scoped fix.

## Development Workflow

Flow tasks follow `BRANCH → SELECT → PLAN → EXECUTE → ARCHIVE → REVIEW → FOLLOW-UP → ARCHIVE-REVIEW`. Normal task branches use the Flow branch pattern; an explicitly approved stacked series may use the preceding task branch as its base.

Run `make check` before submitting a Rust implementation change. Use
`make coverage`, `make miri`, and `make package` when the active task requires
their corresponding release or safety evidence. CI verifies the complete matrix
described in the [toolchain and quality policy](docs/engineering/toolchain-quality-release-policy.md).

Install mdBook with `cargo +stable install mdbook --version 0.5.4 --locked`
before the first documentation check. CI provisions mdBook, cargo-llvm-cov, and
the nightly Miri component independently of the pinned package MSRV.

## Pull Requests

- Describe the task, behavior, and validation actually performed.
- Update documentation and examples together with public behavior changes.
- Follow the [documentation guide](docs/contributing/documentation.md) for rustdoc, mdBook, and example requirements.
- Do not combine unrelated refactors, formatting sweeps, or dependency updates.
- Respond to review findings with a follow-up task when they exceed the current scope.

## Licensing and Third-Party Work

Contributions are submitted under the repository's [MIT License](LICENSE). Only add third-party code, assets, or dependencies with a compatible license, clear provenance, and a documented maintenance reason. Do not copy proprietary material or include credentials, keys, or private data.

## Security Issues

Do not disclose security vulnerabilities in public issues or pull requests. Follow [SECURITY.md](SECURITY.md) for private reporting instructions.
