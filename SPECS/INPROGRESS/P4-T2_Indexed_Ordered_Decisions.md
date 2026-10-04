# P4-T2: Indexed Ordered Decisions

Source: https://github.com/SoundBlaster/BuildHunter/issues/25 (H2/H3).
Status: INPROGRESS. No release/version changes in this task.

Implement ADR 0008 as an additive core API. Preserve insertion-order semantics,
short-circuiting and borrowed result lifetimes; provide a shared Send + Sync path
for worker threads. Lookup must not allocate per candidate. Existing FirstMatch
remains compatible.

Acceptance: deterministic randomized parity against the linear reference;
interleaved keyed/unkeyed and duplicate-key tests; conservative may_match;
invocation-count test for 200 unrelated rules; documented key soundness contract;
Release benchmark for 14/214 rules with parity checks and raw samples; repository
format, Clippy, test, documentation and available coverage gates.

The main agent owns integration, CI, commits and publication. The implementation
agent owns core source/tests/benchmarks/examples and relevant guides only.
