# P0-T2 — Contribution and Agent Governance

**Status:** Planned

**Priority:** P0

**Dependencies:** P0-T1

## Objective

Create the operational contract for contributors and AI agents before Rust
implementation begins. The contract must make repository instructions,
contribution expectations, security reporting, license awareness, review scope,
and Flow ownership clear without duplicating the project charter.

## Scope and Deliverables

- Add root `AGENTS.md` with concise mandatory instructions for agents.
- Add `CONTRIBUTING.md` for human and automated contributors.
- Add `SECURITY.md` with private vulnerability-reporting guidance and supported
version posture.
- Add `docs/contributing/governance.md` with detailed review, decision, license,
and third-party contribution rules.
- Link the governance documents to the charter, ADR process, and Flow artifacts.

## Acceptance Criteria

- Agents receive explicit rules for scope, public documentation, testing,
  decisions, and Flow-managed files.
- Contributors can discover development workflow, commit/PR expectations, and
  how to propose design changes.
- Security issues have a non-public reporting path and do not request secrets in
  issues or pull requests.
- MIT licensing and third-party dependency review expectations are clear.
- Documents use valid repository-relative links and do not claim a Cargo
  workspace exists before P1-T1.

## Test-First Verification Plan

1. Create the governance files before treating governance as established.
2. Check required headings and policy statements with `rg`.
3. Check referenced files exist and `git diff --check` passes.
4. Record direct documentation checks; Cargo tooling gates remain deferred to
   P1-T1.

## Execution Plan

### Phase A — Agent Contract

Write a concise root `AGENTS.md` that preserves user-owned project rules and
points detailed contributor concerns to documentation rather than embedding a
large handbook.

### Phase B — Contributor and Security Policies

Write contributor, security, and governance documents with explicit boundaries
between accepted policy and future toolchain/documentation details.

### Phase C — Validation and Handoff

Verify required files, headings, cross-references, and whitespace. Produce the
validation report, archive the task artifacts, review the change, and archive
the review when no follow-up is necessary.

## Documentation Notes

Governance is public contributor documentation and is written in English. P0-T3
will add the detailed Rust documentation style guide referenced by these rules.
