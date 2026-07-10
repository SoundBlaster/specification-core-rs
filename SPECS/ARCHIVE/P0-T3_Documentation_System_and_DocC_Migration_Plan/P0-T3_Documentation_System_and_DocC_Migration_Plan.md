# P0-T3 — Documentation System and DocC Migration Plan

**Status:** Planned

**Priority:** P0

**Dependencies:** P0-T1

## Objective

Establish the documentation system before public Rust API work begins. The
system must separate item-level rustdoc from mdBook guides, provide an
incremental mdBook skeleton, define enforceable documentation conventions, and
trace every current Swift DocC article or tutorial to a Rust destination or an
explicit exclusion.

## Scope and Deliverables

- Add `docs/book.toml` and mdBook source hierarchy with `SUMMARY.md`.
- Add `docs/contributing/documentation.md` as the detailed documentation style
  guide.
- Add a source-to-destination migration matrix for every Swift DocC article and
  tutorial, updating the existing inventory where necessary.
- Update agent and contributor documents to point to the completed style guide.
- Define doctest, `examples/`, rustdoc, and mdBook validation responsibilities.

## Acceptance Criteria

- mdBook has a valid `book.toml`, `src/`, and `SUMMARY.md` with no dangling
  referenced chapters.
- rustdoc and mdBook responsibilities are mutually exclusive and documented.
- The style guide requires English public docs, public-item documentation,
  executable examples, and accurate feature status.
- The migration matrix identifies a Rust destination or explicit exclusion for
  all 23 source articles and all 4 source tutorials.
- Lack of a Cargo workspace or installed mdBook binary is recorded truthfully.

## Test-First Verification Plan

1. Create the book and style-guide files before declaring the system complete.
2. Verify `SUMMARY.md` targets exist using a link/path check.
3. Verify inventory counts and mapping sections with `rg`.
4. Run `mdbook build` and `mdbook test` when available; otherwise record the
   missing tool and defer execution to P1-T1's quality-gate setup.

## Execution Plan

### Phase A — Information Architecture

Create the mdBook configuration and stable chapter layout. Use placeholders
only for genuinely future content and do not present them as implemented API.

### Phase B — Documentation Policy and Migration

Write the detailed style guide and complete the DocC traceability matrix. Adapt
Swift-specific property-wrapper and macro material rather than copying it.

### Phase C — Validation and Handoff

Verify book paths, policy statements, inventory coverage, and available tools.
Create a validation report, archive task artifacts, review the change, and
archive the review report when no follow-up is needed.

## Documentation Notes

All user-facing documentation is English. The book is a guide surface, while
rustdoc remains the API reference surface.
