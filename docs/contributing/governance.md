# Contribution Governance

## Decision Ownership

The project charter defines product direction. Workplan tasks define delivery scope. ADRs define durable architecture and release decisions. A pull request implements a task; it must not silently redefine product policy.

Use an ADR when a choice affects public API stability, crate boundaries, Rust baseline, supported platforms, feature flags, dependencies, unsafe code, serialization, async execution, FFI, or release policy.

## Review Expectations

Review verifies correctness, scope, documentation, safety, maintainability, and validation evidence. A review finding that cannot be safely addressed within the active task becomes a tracked Workplan follow-up. Review reports are archived with their task artifacts under Flow.

## Agent Expectations

Agents follow the concise contract in [`AGENTS.md`](../../AGENTS.md), then read the active PRD and repository instructions relevant to their task. They must make assumptions visible in task artifacts, preserve unrelated user changes, and keep external actions within explicit authorization.

## Third-Party Material

Every dependency, copied code fragment, documentation asset, or generated file must have known provenance and a license compatible with MIT distribution. Avoid GPL-only dependencies or content in the core crate unless an ADR records the reason, distribution implications, and maintainer approval. Prefer minimal dependencies and document optional features separately.

## Security and Disclosure

Security reports follow [`SECURITY.md`](../../SECURITY.md). Never place secrets, access tokens, vulnerability reproductions against real systems, or private customer data in issues, PRs, test fixtures, or documentation.

## Flow Ownership

Flow-managed command and role files are tooling distribution artifacts. Refresh them through the Flow update process rather than editing them for project-local rules. Project-specific policy belongs in this directory, `AGENTS.md`, `.flow/`, ADRs, and Workplan artifacts.
