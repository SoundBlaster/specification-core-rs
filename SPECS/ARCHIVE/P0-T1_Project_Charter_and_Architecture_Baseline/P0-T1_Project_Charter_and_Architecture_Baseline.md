# P0-T1 — Project Charter and Architecture Baseline

**Status:** Planned

**Priority:** P0

**Dependencies:** None

## Objective

Promote the initial product direction from planning notes into a normative
project charter and architecture baseline. The result must give contributors
and agents a stable answer to what this repository is building, what it is not
building, and how future decisions become durable ADRs.

## Scope and Deliverables

- Create `docs/project-charter.md` with audience, goals, non-goals, release
  posture, and compatibility policy.
- Create `docs/architecture/README.md` defining the intended workspace and
  crate boundaries without pre-creating implementation crates.
- Create `docs/adr/README.md` and `docs/adr/0000-template.md` for lightweight
  Architecture Decision Records.
- Link the charter to the existing planning inputs and distinguish accepted
  policy from open decisions.

## Acceptance Criteria

- The charter makes idiomatic Rust precedence and behavioral compatibility
  policy explicit.
- Non-goals exclude premature async, FFI, macros, and platform integrations
  from the synchronous-core milestone.
- Architecture documentation defines a one-crate initial workspace and rules
  for later optional crates.
- ADR guidance specifies status, context, decision, consequences, and
  supersession handling.
- All Markdown links resolve to repository files and no normative claim depends
  only on chat history.

## Test-First Verification Plan

1. Add the target documents before treating the decision set as complete.
2. Verify each required heading and policy phrase with `rg`.
3. Verify Markdown links and whitespace with shell checks and `git diff --check`.
4. Record outcomes in the validation report. No Cargo quality gate applies
   because the workspace is intentionally created by P1-T1.

## Execution Plan

### Phase A — Charter

Use `docs/planning/project-decisions.md` as input and write the normative
charter. Preserve unresolved questions as links to later Phase 0 tasks rather
than silently deciding them.

### Phase B — Architecture and ADRs

Describe public package, workspace, extension, and documentation boundaries.
Add an ADR index and reusable template; do not create speculative implementation
modules.

### Phase C — Verification and Handoff

Check scope, links, and repository status. Capture the validation evidence,
then archive the PRD and validation report through Flow.

## Documentation Notes

The charter and architecture baseline are public contributor documentation.
They must be written in English and remain consistent with the Workplan and
planning inputs.
