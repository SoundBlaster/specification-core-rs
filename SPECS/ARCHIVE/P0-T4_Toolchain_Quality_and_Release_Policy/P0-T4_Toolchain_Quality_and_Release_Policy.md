# P0-T4 — Toolchain, Quality, and Release Policy

**Status:** Planned

**Priority:** P0

**Dependencies:** P0-T1

## Objective

Define a reproducible, enforceable Rust and release baseline for the Cargo
workspace created in P1-T1. The outcome must decide the edition, MSRV, supported
targets, quality gates, dependency and unsafe-code boundaries, and release
expectations rather than leaving P1-T1 to infer policy ad hoc.

## Scope and Deliverables

- Add an engineering policy covering Rust baseline, targets, local/CI commands,
  coverage, documentation gates, dependencies, Cargo features, unsafe code, and
  release process.
- Add ADRs for the Rust baseline and for core safety/quality policy.
- Update the ADR index and planning decisions to mark resolved choices.
- Define an implementation checklist P1-T1 can apply to manifests, CI, and
  contributor commands.

## Acceptance Criteria

- Rust edition, MSRV, and target support tiers are explicit.
- Tests, formatting, Clippy, rustdoc, mdBook, and coverage commands have clear
  thresholds or explicit rollout conditions.
- Core dependency and optional-feature admission rules are documented.
- Core unsafe-code policy is enforceable and exceptions require an ADR.
- SemVer, changelog, package, publication, and release ownership rules are
  documented.
- P1-T1 can implement the documented policy without choosing missing defaults.

## Test-First Verification Plan

1. Add policy and ADR files before declaring decisions resolved.
2. Verify required headings, Rust version, target triples, commands, and
   exception rules with `rg`.
3. Verify ADR index links and `git diff --check`.
4. Record that Cargo/CI execution is deferred until P1-T1 creates the workspace.

## Execution Plan

### Phase A — Baseline Decisions

Choose a conservative Rust edition/MSRV pair and documented platform tiers that
are compatible with a public library.

### Phase B — Enforcement Policy

Specify command-level gates and rules for dependencies, feature flags, unsafe
code, documentation, coverage, packaging, and releases.

### Phase C — Validation and Handoff

Verify policy completeness and ADR traceability. Archive the PRD and validation
report, review the delta, skip FOLLOW-UP only when no actionable issue remains,
and archive the review.

## Documentation Notes

Policy text is normative contributor documentation in English. It must describe
only tooling that P1-T1 can realistically provision.
