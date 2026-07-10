#![allow(missing_docs)]

use specification_core::Specification;
use specification_core_macros::specification;

struct User {
    age: u8,
}

#[specification(AdultSpec)]
fn is_adult(candidate: &User) -> bool {
    candidate.age >= 18
}

#[test]
fn attribute_macro_generates_a_named_specification() {
    let rule = AdultSpec;

    assert!(rule.is_satisfied_by(&User { age: 21 }));
    assert!(!rule.is_satisfied_by(&User { age: 17 }));
    assert!(is_adult(&User { age: 21 }));
}
