# ADR-0001: Adopt Rust 2024 and MSRV 1.85

**Status:** Accepted

**Date:** 2026-07-10

**Deciders:** SoundBlaster maintainers

**Related Workplan Tasks:** P0-T4, P1-T1

## Context

The first workspace needs a stable edition and explicit MSRV before dependencies,
CI, and public package metadata are chosen. Rust 2024 requires Rust 1.85 or
newer, and Cargo's `rust-version` field communicates the supported compiler
contract to users and tooling.

## Decision

All initial published packages use `edition = "2024"`, workspace
`resolver = "3"`, and `rust-version = "1.85"`. CI verifies both Rust 1.85 and
current stable. Raising the MSRV requires a new ADR and release assessment.

## Consequences

The project can use Rust 2024 syntax and resolver behavior while retaining a
clear, testable lower compiler boundary. Dependency updates must remain
compatible with Rust 1.85 or require an approved MSRV change.

## Alternatives Considered

- Rust 2021 with a lower MSRV: rejected because a new library benefits from the
  current stable edition and has no legacy compatibility requirement.
- Latest stable only: rejected because it makes downstream support expectations
  unpredictable.

## Follow-up

P1-T1 creates manifests, a toolchain file, and CI verification.
