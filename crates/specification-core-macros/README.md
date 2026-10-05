# specification-core-macros

Opt-in procedural macros for reducing boilerplate around named predicate
specifications. The crate is separate from `specification-core`; macro use is
never required by the core API.

## `first_match!`

`first_match!` builds a concrete type that implements
`specification_core::DecisionSpecification<Context>`. It stores each rule's
specification and decision in typed fields. Construction expressions run once
when the macro expression is evaluated; `decide` borrows the stored decision.

This excerpt illustrates the DSL shape; the integration tests compile the same
macro against both workspace crates.

```rust,ignore
use specification_core::{DecisionSpecification, Specification};
use specification_core_macros::first_match;

struct Facts { name: String, directory: bool, cargo_parent: bool }
struct ParentCargo;
impl Specification<Facts> for ParentCargo {
    fn is_satisfied_by(&self, facts: &Facts) -> bool { facts.cargo_parent }
}

let route = first_match! {
    context: Facts,
    decision: u16,
    key: |facts: &Facts| facts.name.as_str(),
    rules: [
        keyed([".build"], |facts: &Facts| facts.name == ".build" && facts.directory, 1),
        unkeyed(ParentCargo, 2),
        keyed(["target", "target-alias"], |facts: &Facts| {
            (facts.name == "target" || facts.name == "target-alias")
                && facts.directory && facts.cargo_parent
        }, 3),
    ]
};
assert_eq!(route.decide(&Facts {
    name: "target".into(), directory: true, cargo_parent: true,
}), Some(&2));
```

The `keyed` form accepts one or more string literals as aliases. Its
specification must never match an input whose projected key is outside that
alias set. This is a necessary-condition contract because the macro cannot
inspect arbitrary specifications. Rules without a sound key use `unkeyed`.
For each input, generated dispatch checks only the matching keyed rules and all
unkeyed rules, in their original declaration order. Unknown keys check only
unkeyed rules. Duplicate keys are allowed and retain priority; matching stops
at the first true specification. The projection returns a borrowed `&str`, is
called once per decision, and generated dispatch performs no per-decision
allocation.

The complete expansion is exercised against the actual core library in the
[`first_match` integration tests](tests/first_match.rs).
