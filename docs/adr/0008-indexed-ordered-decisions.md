# ADR 0008: Index ordered decisions by necessary keys

Status: Accepted for implementation under BuildHunter issue 25

## Context

`FirstMatch` evaluates every preceding rule. A runtime catalog with hundreds of
unrelated exact-name rules therefore has linear evaluation cost. BuildHunter's
measurements motivate an additive indexed evaluator, without changing the
existing evaluator or assuming predicates are pure.

## Decision

Add `IndexedFirstMatch` with caller-declared necessary keys and unkeyed rules.
Compile key buckets once. For each candidate, evaluate only its bucket and the
unkeyed rules, merged in original insertion order. Stop at the first match.
The caller promises a keyed predicate cannot match any other key; this is an
explicit contract, not an inferred property of an arbitrary closure.

Support borrowed key lookup, no per-evaluation allocation, and an explicitly
thread-safe shared variant consistent with `SharedSpecification`. Use the
standard randomized hash implementation initially. `may_match` is conservative
when unkeyed rules exist. Preserve duplicate keys, no-match, fallback and
short-circuit behavior. Do not reorder or memoize arbitrary predicates.

## Validation

Seeded randomized parity tests compare the index with linear first-match.
Invocation-count tests verify priority and skipped unrelated buckets. Release
benchmarks compare 14 and 214 rules, with equal results checked outside timing
and raw samples recorded. Rust 1.85 compatibility and existing quality gates
remain required.

## Consequences

Exact-key lookup avoids scanning unrelated keyed rules. Unkeyed rules and large
individual buckets still have linear cost; no universal O(1) claim is made.
Key-aware compile-time macros, declarative rule compilation, tracing and purity
opt-ins are separate follow-up stages, not dependencies of this additive API.
