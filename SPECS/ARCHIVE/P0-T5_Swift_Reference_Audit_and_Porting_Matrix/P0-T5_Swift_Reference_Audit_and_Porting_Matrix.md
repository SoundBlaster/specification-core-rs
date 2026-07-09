# P0-T5 — Swift Reference Audit and Porting Matrix

**Status:** Planned

**Priority:** P0

**Dependencies:** P0-T1, P0-T3

## Objective

Audit the actual Swift `SpecificationCore` source, tests, documentation, and
current build behavior. Convert that evidence into a Rust porting matrix that
classifies each capability as adopt, adapt, defer, or exclude. P1-T2 must be
able to implement the synchronous core without relying on README claims or
assuming source-level API parity.

## Scope and Deliverables

- Record the audited Swift revision, package topology, public API groups, and
  available test groups.
- Record the current Swift build/test baseline with exact toolchain context and
  any reproducible failure.
- Add a capability matrix for composition, type erasure, decisions, context,
  time rules, wrappers, macros, and async behavior.
- Add semantic notes for short-circuiting, collection identities, first-match
  priority, defaults, time semantics, and error behavior.
- Verify source provenance and MIT license compatibility.

## Acceptance Criteria

- The audit distinguishes observed source behavior from README/documentation
  claims.
- Every major public API family has an adoption classification and Rust target.
- The known `FirstMatchSpec.Builder.build()` compilation ambiguity is recorded
  with command and toolchain evidence if it remains reproducible.
- P1-T2's scope and exclusions are explicit.
- The matrix has links to source files, relevant tests, and the documentation
  migration map.

## Test-First Verification Plan

1. Inspect the Swift package manifest, source declarations, tests, license, and
   Git revision before writing conclusions.
2. Run the Swift test/build baseline and preserve the actual result.
3. Cross-check every matrix group against source or test evidence.
4. Verify audit links and headings with `rg` and `git diff --check`.

## Execution Plan

### Phase A — Evidence Collection

Collect manifest, revision, public declaration, test, documentation, and build
evidence from the Swift repository without modifying it.

### Phase B — Porting Classification

Write a conservative adopt/adapt/defer/exclude matrix and identify executable
behavioral fixtures needed later. Do not reproduce Swift defects as Rust API
requirements.

### Phase C — Validation and Handoff

Verify traceability, record validation results, archive task artifacts, review
the delta, and archive the review when no actionable defect remains.

## Documentation Notes

The resulting audit is contributor-facing English documentation. It is a
behavioral reference for P1-T2 and complements, rather than replaces, the DocC
migration map.
