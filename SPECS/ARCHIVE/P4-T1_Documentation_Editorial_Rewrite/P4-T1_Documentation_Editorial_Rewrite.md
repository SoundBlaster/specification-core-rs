# P4-T1: Documentation Editorial Rewrite

**Status:** INPROGRESS  
**Priority:** P1  
**Phase:** Documentation Editorial Quality  
**Dependencies:** P3-T3 — Publish 0.1.0  

## Objective

Turn the mechanically complete documentation set into a reader-first guide for
the released `0.1.0` workspace. The book must explain how the Rust API works,
when each ownership and dispatch boundary is appropriate, and how the design
maps to the Swift reference implementation.

## Deliverables

1. Replace stale pre-release language in the landing page, Getting Started,
   and migration metadata.
2. Add a coherent learning path covering static composition, dynamic and
   shared specifications, decisions, immutable context, injected clocks,
   async evaluation, serialization, and macros.
3. Add complete runnable Cargo examples for the public workflows that are not
   currently represented by examples.
4. Add a side-by-side Swift-to-Rust migration guide and move maintainer audit
   material out of the primary reader path.
5. Publish the generated rustdoc below the Pages artifact and link it from the
   book and repository README; fix generated edit links.
6. Validate documentation with the configured Rust, docs, package, coverage,
   and integration gates, and record the evidence in a validation report.

## Acceptance criteria

- The first page accurately describes the shipped `0.1.0` release and offers
  links to Getting Started, API reference, crates, source, and examples.
- A new reader can follow one `User`-style scenario from a closure rule to
  composition, `FirstMatch`, context/time rules, and explicit evaluation.
- Every optional integration has either a complete Cargo example or an
  explicit link to its executable test/example source.
- Swift comparison covers borrowing, type erasure, concurrency, context and
  clock injection, async errors, property wrappers, and macros.
- The Pages artifact contains `api/` rustdoc and the book has working links to
  it and to docs.rs.
- No user-facing page describes shipped capabilities as future work.
- `make check`, `make test`, `make lint`, `make fmt-check`, `make docs`,
  `make coverage`, and `make package` pass (with documented environment
  requirements for optional tools).
- A rendered Pages smoke check confirms navigation, API links, edit links, and
  the absence of broken internal links.

## Non-goals

- No changes to the public Rust API or runtime behavior.
- No attempt to make Swift and Rust source-level APIs identical.
- No serialization of executable closures or trait objects.
- No new date/segment rules beyond the released Rust `0.1.0` scope.

## Implementation order

1. Fix navigation, landing content, stale status text, and external/API links.
2. Add examples and user-facing chapters, then add the Swift migration guide.
3. Update the Pages workflow and README links.
4. Run all gates, build the book and API artifact, and perform a link/content
   audit before archiving.

---
**Archived:** 2026-07-10
**Verdict:** PASS
