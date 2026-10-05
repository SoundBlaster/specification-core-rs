# ADR-0009: Generate Static Keyed Decisions

Status: Accepted for the feature branch; release pending validation.

## Context

BuildHunter issue 25 (H5/H19) requests a static backend alongside the runtime
index in ADR-0008. Its user explicitly authorized library implementation.
Sequential static chains still compare the same name repeatedly. The existing
optional macro crate is the boundary for compile-time code generation.

## Decision

Add `first_match!` to the optional macro crate. Explicit context/result types,
a borrowed string key projection, literal key groups and unkeyed rules produce
a concrete decision evaluator. Specifications and decisions are constructed
once; evaluation projects the key once and uses generated `match` dispatch.
Each arm checks only its keyed rules plus unkeyed rules, in declaration order.
Duplicate keys are valid and retain priority. As with the runtime index, a key
is a necessary condition, not an optimization inferred from arbitrary code.

No rule reordering, inferred purity, runtime registry, heap allocation per
evaluation, global state, or core dependency on the macro crate is introduced.
MSRV remains 1.85. The dynamic index remains the backend for runtime catalogs.

## Validation

Require ordered linear parity, keyed/unkeyed priority, duplicates, short-circuit
and constructor/projection invocation tests; invalid declarations need useful
compile errors. Benchmark real library composition against handwritten dispatch
with parity outside timing and raw repeated samples. No performance claim or
application adoption precedes these checks.
