# Core Concepts

The library applies the Specification Pattern: small, deterministic rules can
be evaluated against a candidate and composed into larger rules. Rust's trait
system makes static composition the normal path; dynamic dispatch remains an
explicit opt-in boundary.

The project prioritizes typed evaluation data, explicit ownership, visible
concurrency contracts, and runtime-neutral design. Detailed API semantics are
published in rustdoc as implementation tasks complete.
