#![forbid(unsafe_code)]
//! Composable, strongly typed business rules.
//!
//! A [`Specification`] evaluates a borrowed candidate. Specifications compose
//! through [`Specification::and`], [`Specification::or`], and
//! [`Specification::not`] without dynamic dispatch or heap allocation.
//!
//! ```
//! use specification_core::Specification;
//!
//! let is_adult = |age: &u8| *age >= 18;
//! let is_senior = |age: &u8| *age >= 65;
//! let eligible = is_adult.and(is_senior.not());
//!
//! assert!(eligible.is_satisfied_by(&42));
//! assert!(!eligible.is_satisfied_by(&70));
//! ```

use std::sync::Arc;

/// A deterministic boolean rule over a borrowed candidate.
///
/// Implement this trait for a focused business rule, then compose it with the
/// provided boolean combinators. Evaluation borrows the candidate, so rules do
/// not take ownership of the value they inspect.
pub trait Specification<T: ?Sized> {
    /// Returns whether `candidate` satisfies this rule.
    fn is_satisfied_by(&self, candidate: &T) -> bool;

    /// Combines this rule with `other` using short-circuiting logical AND.
    ///
    /// The resulting [`And`] evaluates this rule first. It evaluates `other`
    /// only when this rule returns `true`.
    fn and<Other>(self, other: Other) -> And<Self, Other>
    where
        Self: Sized,
        Other: Specification<T>,
    {
        And {
            left: self,
            right: other,
        }
    }

    /// Combines this rule with `other` using short-circuiting logical OR.
    ///
    /// The resulting [`Or`] evaluates this rule first. It evaluates `other`
    /// only when this rule returns `false`.
    fn or<Other>(self, other: Other) -> Or<Self, Other>
    where
        Self: Sized,
        Other: Specification<T>,
    {
        Or {
            left: self,
            right: other,
        }
    }

    /// Creates a rule that negates this rule's result.
    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not { inner: self }
    }
}

impl<T: ?Sized, Predicate> Specification<T> for Predicate
where
    Predicate: Fn(&T) -> bool,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self(candidate)
    }
}

/// A specification that requires both of two specifications to be satisfied.
///
/// Construct this type with [`Specification::and`]. Its operands remain
/// concrete types, so the composition does not allocate or use dynamic
/// dispatch.
pub struct And<Left, Right> {
    left: Left,
    right: Right,
}

impl<T: ?Sized, Left, Right> Specification<T> for And<Left, Right>
where
    Left: Specification<T>,
    Right: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.left.is_satisfied_by(candidate) && self.right.is_satisfied_by(candidate)
    }
}

/// A specification that requires either of two specifications to be satisfied.
///
/// Construct this type with [`Specification::or`]. Its operands remain
/// concrete types, so the composition does not allocate or use dynamic
/// dispatch.
pub struct Or<Left, Right> {
    left: Left,
    right: Right,
}

impl<T: ?Sized, Left, Right> Specification<T> for Or<Left, Right>
where
    Left: Specification<T>,
    Right: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.left.is_satisfied_by(candidate) || self.right.is_satisfied_by(candidate)
    }
}

/// A specification that negates another specification's result.
///
/// Construct this type with [`Specification::not`].
pub struct Not<Inner> {
    inner: Inner,
}

impl<T: ?Sized, Inner> Specification<T> for Not<Inner>
where
    Inner: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        !self.inner.is_satisfied_by(candidate)
    }
}

/// A specification that always returns `true`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Always;

impl<T: ?Sized> Specification<T> for Always {
    fn is_satisfied_by(&self, _: &T) -> bool {
        true
    }
}

/// A specification that always returns `false`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Never;

impl<T: ?Sized> Specification<T> for Never {
    fn is_satisfied_by(&self, _: &T) -> bool {
        false
    }
}

/// A borrowed collection evaluated with logical AND.
///
/// An empty collection is satisfied, the identity for logical AND. Evaluation
/// stops at the first unsatisfied specification.
pub struct AllOf<'specifications, SpecificationType> {
    specifications: &'specifications [SpecificationType],
}

impl<'specifications, SpecificationType> AllOf<'specifications, SpecificationType> {
    /// Creates an all-of specification that borrows `specifications`.
    pub const fn new(specifications: &'specifications [SpecificationType]) -> Self {
        Self { specifications }
    }
}

impl<T: ?Sized, SpecificationType> Specification<T> for AllOf<'_, SpecificationType>
where
    SpecificationType: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.specifications
            .iter()
            .all(|specification| specification.is_satisfied_by(candidate))
    }
}

/// A borrowed collection evaluated with logical OR.
///
/// An empty collection is not satisfied, the identity for logical OR.
/// Evaluation stops at the first satisfied specification.
pub struct AnyOf<'specifications, SpecificationType> {
    specifications: &'specifications [SpecificationType],
}

impl<'specifications, SpecificationType> AnyOf<'specifications, SpecificationType> {
    /// Creates an any-of specification that borrows `specifications`.
    pub const fn new(specifications: &'specifications [SpecificationType]) -> Self {
        Self { specifications }
    }
}

impl<T: ?Sized, SpecificationType> Specification<T> for AnyOf<'_, SpecificationType>
where
    SpecificationType: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.specifications
            .iter()
            .any(|specification| specification.is_satisfied_by(candidate))
    }
}

/// An explicitly type-erased, owned specification.
///
/// Use this wrapper only when a runtime-selected or heterogeneous collection of
/// rules is needed. Static composition remains the allocation-free default.
pub struct BoxedSpecification<T: ?Sized> {
    inner: Box<dyn Specification<T>>,
}

impl<T: ?Sized> BoxedSpecification<T> {
    /// Erases the concrete type of `specification` and takes ownership of it.
    pub fn new<Concrete>(specification: Concrete) -> Self
    where
        Concrete: Specification<T> + 'static,
    {
        Self {
            inner: Box::new(specification),
        }
    }
}

impl<T: ?Sized> Specification<T> for BoxedSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.inner.is_satisfied_by(candidate)
    }
}

/// An explicitly type-erased specification that can be shared across threads.
#[derive(Clone)]
pub struct SharedSpecification<T: ?Sized> {
    inner: Arc<dyn Specification<T> + Send + Sync>,
}

impl<T: ?Sized> SharedSpecification<T> {
    /// Erases and shares a `Send + Sync` specification.
    pub fn new<Concrete>(specification: Concrete) -> Self
    where
        Concrete: Specification<T> + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(specification),
        }
    }
}

impl<T: ?Sized> Specification<T> for SharedSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.inner.is_satisfied_by(candidate)
    }
}

/// A rule that selects a typed decision for a candidate.
pub trait DecisionSpecification<T: ?Sized> {
    /// The decision selected by this rule.
    type Decision;

    /// Returns the selected decision, or `None` when no rule matches.
    fn decide(&self, candidate: &T) -> Option<&Self::Decision>;
}

/// Ordered, typed decisions selected by the first matching specification.
///
/// Rules are evaluated in insertion order. The first satisfied rule wins and
/// later rules are not evaluated.
pub struct FirstMatch<T: ?Sized, Decision> {
    rules: Vec<(BoxedSpecification<T>, Decision)>,
}

impl<T: ?Sized, Decision> FirstMatch<T, Decision> {
    /// Creates a first-match decision set with no rules.
    pub const fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Appends a rule and its decision, preserving evaluation order.
    pub fn push<Concrete>(&mut self, specification: Concrete, decision: Decision)
    where
        Concrete: Specification<T> + 'static,
    {
        self.rules
            .push((BoxedSpecification::new(specification), decision));
    }

    /// Returns the first matching decision or `fallback` when no rule matches.
    pub fn decide_or<'decision>(
        &'decision self,
        candidate: &T,
        fallback: &'decision Decision,
    ) -> &'decision Decision {
        self.decide(candidate).unwrap_or(fallback)
    }
}

impl<T: ?Sized, Decision> Default for FirstMatch<T, Decision> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized, Decision> DecisionSpecification<T> for FirstMatch<T, Decision> {
    type Decision = Decision;

    fn decide(&self, candidate: &T) -> Option<&Decision> {
        self.rules.iter().find_map(|(specification, decision)| {
            specification.is_satisfied_by(candidate).then_some(decision)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::{
        AllOf, Always, AnyOf, BoxedSpecification, DecisionSpecification, FirstMatch, Never,
        SharedSpecification, Specification,
    };

    struct CountingSpecification {
        result: bool,
        evaluations: Rc<Cell<u8>>,
    }

    impl CountingSpecification {
        fn new(result: bool) -> Self {
            Self {
                result,
                evaluations: Rc::new(Cell::new(0)),
            }
        }

        fn counter(&self) -> Rc<Cell<u8>> {
            Rc::clone(&self.evaluations)
        }
    }

    impl Specification<i32> for CountingSpecification {
        fn is_satisfied_by(&self, _: &i32) -> bool {
            self.evaluations.set(self.evaluations.get() + 1);
            self.result
        }
    }

    #[test]
    fn closures_are_specifications() {
        let is_positive = |number: &i32| *number > 0;

        assert!(is_positive.is_satisfied_by(&1));
        assert!(!is_positive.is_satisfied_by(&0));
    }

    #[test]
    fn and_requires_both_rules_to_succeed() {
        let between = (|number: &i32| *number > 5).and(|number: &i32| *number < 10);

        assert!(between.is_satisfied_by(&7));
        assert!(!between.is_satisfied_by(&5));
        assert!(!between.is_satisfied_by(&10));
    }

    #[test]
    fn and_short_circuits_after_a_false_left_side() {
        let left = CountingSpecification::new(false);
        let right = CountingSpecification::new(true);
        let left_evaluations = left.counter();
        let right_evaluations = right.counter();
        let specification = left.and(right);

        assert!(!specification.is_satisfied_by(&0));
        assert_eq!(left_evaluations.get(), 1);
        assert_eq!(right_evaluations.get(), 0);
    }

    #[test]
    fn or_accepts_either_rule() {
        let outside = (|number: &i32| *number < 0).or(|number: &i32| *number > 10);

        assert!(outside.is_satisfied_by(&-1));
        assert!(outside.is_satisfied_by(&11));
        assert!(!outside.is_satisfied_by(&5));
    }

    #[test]
    fn or_short_circuits_after_a_true_left_side() {
        let left = CountingSpecification::new(true);
        let right = CountingSpecification::new(false);
        let left_evaluations = left.counter();
        let right_evaluations = right.counter();
        let specification = left.or(right);

        assert!(specification.is_satisfied_by(&0));
        assert_eq!(left_evaluations.get(), 1);
        assert_eq!(right_evaluations.get(), 0);
    }

    #[test]
    fn not_negates_a_rule() {
        let is_even = |number: &i32| *number % 2 == 0;
        let is_odd = is_even.not();

        assert!(is_odd.is_satisfied_by(&3));
        assert!(!is_odd.is_satisfied_by(&4));
    }

    #[test]
    fn constants_have_expected_results() {
        assert!(Always.is_satisfied_by(&"candidate"));
        assert!(!Never.is_satisfied_by(&"candidate"));
    }

    #[test]
    fn all_of_uses_the_and_identity_and_short_circuits() {
        let empty: [CountingSpecification; 0] = [];
        let failing = CountingSpecification::new(false);
        let skipped = CountingSpecification::new(true);
        let failing_evaluations = failing.counter();
        let skipped_evaluations = skipped.counter();
        let specifications = [failing, skipped];

        assert!(AllOf::new(&empty).is_satisfied_by(&0));
        assert!(!AllOf::new(&specifications).is_satisfied_by(&0));
        assert_eq!(failing_evaluations.get(), 1);
        assert_eq!(skipped_evaluations.get(), 0);
    }

    #[test]
    fn any_of_uses_the_or_identity_and_short_circuits() {
        let empty: [CountingSpecification; 0] = [];
        let succeeding = CountingSpecification::new(true);
        let skipped = CountingSpecification::new(false);
        let succeeding_evaluations = succeeding.counter();
        let skipped_evaluations = skipped.counter();
        let specifications = [succeeding, skipped];

        assert!(!AnyOf::new(&empty).is_satisfied_by(&0));
        assert!(AnyOf::new(&specifications).is_satisfied_by(&0));
        assert_eq!(succeeding_evaluations.get(), 1);
        assert_eq!(skipped_evaluations.get(), 0);
    }

    #[test]
    fn boxed_specifications_store_heterogeneous_rules() {
        let specifications: Vec<BoxedSpecification<i32>> = vec![
            BoxedSpecification::new(|number: &i32| *number > 0),
            BoxedSpecification::new(Always),
        ];

        assert!(specifications[0].is_satisfied_by(&1));
        assert!(specifications[1].is_satisfied_by(&-1));
    }

    #[test]
    fn shared_specifications_are_cloneable() {
        let specification = SharedSpecification::new(|number: &i32| *number % 2 == 0);
        let shared = specification.clone();

        assert!(shared.is_satisfied_by(&2));
        assert!(!specification.is_satisfied_by(&3));
    }

    #[test]
    fn first_match_returns_the_first_typed_decision_and_short_circuits() {
        let first = CountingSpecification::new(true);
        let later = CountingSpecification::new(true);
        let first_evaluations = first.counter();
        let later_evaluations = later.counter();
        let mut decisions = FirstMatch::new();
        decisions.push(first, "first");
        decisions.push(later, "later");

        assert_eq!(decisions.decide(&0), Some(&"first"));
        assert_eq!(first_evaluations.get(), 1);
        assert_eq!(later_evaluations.get(), 0);
    }

    #[test]
    fn first_match_exposes_no_match_and_explicit_fallback() {
        let mut decisions = FirstMatch::new();
        decisions.push(|number: &i32| *number > 10, "large");
        let fallback = "default";

        assert_eq!(decisions.decide(&5), None);
        assert_eq!(decisions.decide_or(&5, &fallback), &fallback);
    }
}
