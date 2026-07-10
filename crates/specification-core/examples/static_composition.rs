//! Builds a reusable allocation-free rule from closure specifications.

use specification_core::Specification;

fn main() {
    let is_adult = |age: &u8| *age >= 18;
    let is_retired = |age: &u8| *age >= 65;
    let working_age = is_adult.and(is_retired.not());

    assert!(working_age.is_satisfied_by(&42));
    assert!(!working_age.is_satisfied_by(&17));
    assert!(!working_age.is_satisfied_by(&70));
}
