# Architecture Decision Records

An ADR records a durable decision that affects the public API, architecture,
release process, compatibility, security posture, or long-term maintenance
cost. ADRs are concise and immutable after acceptance; a later ADR supersedes
an earlier one instead of rewriting history.

## Lifecycle

1. Copy [`0000-template.md`](0000-template.md) to a numbered filename such as
   `0001-adopt-rust-2024-edition.md`.
2. Set the status to `Proposed` while the decision is under review.
3. Set it to `Accepted`, `Rejected`, `Superseded`, or `Deprecated` when the
   outcome is known.
4. Link related Workplan tasks, PRs, implementation changes, and superseding
   ADRs.

## When to Write an ADR

Write an ADR for choices such as the MSRV, unsafe-code policy, crate boundaries,
public dynamic-dispatch API, async model, feature policy, serialization format,
or release process. Do not write one for a reversible local refactor.

## Index

- [ADR-0001: Adopt Rust 2024 and MSRV 1.85](0001-adopt-rust-2024-and-msrv-1-85.md)
- [ADR-0002: Adopt Core Safety and Quality Policy](0002-adopt-core-safety-and-quality-policy.md)
- [ADR-0003: Establish Swift Behavioral Reference](0003-establish-swift-behavioral-reference.md)
