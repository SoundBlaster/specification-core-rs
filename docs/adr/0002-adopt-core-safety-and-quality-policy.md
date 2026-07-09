# ADR-0002: Adopt Core Safety and Quality Policy

**Status:** Accepted

**Date:** 2026-07-10

**Deciders:** SoundBlaster maintainers

**Related Workplan Tasks:** P0-T4, P1-T1, P1-T2

## Context

The core library is intended to be small, reusable, and safe across application
domains. It needs enforceable defaults for unsafe code, linting, tests,
documentation, coverage, dependencies, and optional features before code is
introduced.

## Decision

The core crate forbids unsafe code. Required quality gates are formatting,
Clippy with warnings denied, workspace tests, doctests, rustdoc warnings denied,
and mdBook build/test. The core starts with no required third-party dependencies.
Coverage measurement is provisioned in P1-T1 and reaches a 90% core line gate
when P1-T2 introduces executable behavior.

## Consequences

Code and CI remain simple to audit. An unsafe optimization, mandatory runtime,
or dependency with non-trivial licensing cost requires an ADR and review. The
coverage rollout avoids a misleading percentage before meaningful tests exist.

## Alternatives Considered

- Allow unsafe code by default: rejected because no current core requirement
  justifies it.
- Defer all lint and documentation gates: rejected because public API quality
  is a first-release requirement.
- Add convenience dependencies immediately: rejected to preserve a minimal core
  and postpone ecosystem commitments.

## Follow-up

P1-T1 implements the lint, CI, and tooling surface; P1-T2 enables the coverage
threshold with real behavior and tests.
