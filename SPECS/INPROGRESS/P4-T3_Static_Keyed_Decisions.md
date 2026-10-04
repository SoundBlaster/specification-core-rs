# P4-T3: Static Keyed Decisions

Authorized by the BuildHunter issue 25 implementation request. Depends on
P4-T2 / ADR-0008. Architecture: ADR-0009.

Acceptance: additive optional `first_match!`, original rule priority across
keyed/unkeyed rules, literal aliases and duplicates, necessary-key contract,
one-time construction, one key projection, no per-evaluation heap allocation,
actual-library benchmark and tests. Existing APIs remain compatible. Versions
and releases are deferred until application integration.
