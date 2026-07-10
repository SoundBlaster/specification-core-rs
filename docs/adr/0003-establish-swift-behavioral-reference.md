# ADR-0003: Establish Swift Behavioral Reference

**Status:** Accepted

**Date:** 2026-07-10

**Deciders:** SoundBlaster maintainers

**Related Workplan Tasks:** P0-T5, P1-T2, P3-T1

## Context

The Rust implementation uses Swift `SpecificationCore` as behavioral source
material, but the Swift surface contains language-specific wrappers, macros,
global-provider behavior, and a current compile defect. Future Rust tasks need
a durable compatibility boundary that is stronger than planning notes but does
not require source-level API parity.

## Decision

The audited Swift revision `7909e62da7ca0416dc24c8165bfd0e6d4cf16e57` is the
initial behavioral reference. The rendered [Swift Reference Audit](../src/migration/swift-reference-audit.md)
and DocC migration map are the compatibility inputs. P1-T2 implements only the
synchronous boolean core; dynamic dispatch, decisions, context, time, async,
wrappers, macros, and platform integrations remain in their assigned Workplan
tasks.

The known Swift `FirstMatchSpec.Builder.build()` compile ambiguity is a source
defect, not a Rust behavior requirement.

## Consequences

Rust tests preserve only the audited semantic contract and document deliberate
adaptations such as borrowed candidates and explicit dynamic dispatch. README
performance and coverage claims are not imported. A later compatibility change
updates this ADR or records a superseding ADR.

## Alternatives Considered

- Treat the Swift README as the compatibility specification: rejected because
  its claims are not a verified executable contract.
- Replicate all Swift public symbols: rejected because property wrappers,
  Combine, and Swift macros do not define idiomatic Rust API.
- Postpone all compatibility work: rejected because P1-T2 needs testable scope.

## Follow-up

P1-T2 adds synchronous core tests. P3-T1 converts audited behavior into
portable conformance fixtures where both implementations can execute them.
