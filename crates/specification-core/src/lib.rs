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

use std::{collections::BTreeMap, sync::Arc, time::Duration};

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
///
/// The `Send + Sync` bounds belong to the erased trait object and the
/// constructor, so sharing is an explicit API choice. The ordinary
/// [`Specification`] trait remains usable for single-threaded rules that do
/// not satisfy those bounds.
///
/// A captured non-thread-safe value is rejected at the shared boundary:
///
/// ```compile_fail
/// use std::rc::Rc;
/// use specification_core::SharedSpecification;
///
/// let state = Rc::new(1_u8);
/// let _rule = SharedSpecification::new(move |_: &u8| *state == 1);
/// ```
pub struct SharedSpecification<T: ?Sized> {
    inner: Arc<dyn Specification<T> + Send + Sync>,
}

impl<T: ?Sized> Clone for SharedSpecification<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
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

/// Immutable input data for specifications that need counters, flags, time,
/// and application-specific user data.
///
/// The context has no interior mutability or global registration mechanism. It
/// is `Send` and `Sync` when `UserData` and its standard-library fields are,
/// allowing callers to move or share a context according to the auto-trait
/// rules of the user data they provide.
pub struct EvaluationContext<UserData> {
    user_data: UserData,
    counters: BTreeMap<String, u64>,
    flags: BTreeMap<String, bool>,
    timestamps: BTreeMap<String, Duration>,
}

impl<UserData> EvaluationContext<UserData> {
    /// Creates an empty context containing typed `user_data`.
    pub fn new(user_data: UserData) -> Self {
        Self {
            user_data,
            counters: BTreeMap::new(),
            flags: BTreeMap::new(),
            timestamps: BTreeMap::new(),
        }
    }

    /// Adds or replaces a counter while constructing the context.
    pub fn with_counter(mut self, key: impl Into<String>, value: u64) -> Self {
        self.counters.insert(key.into(), value);
        self
    }

    /// Adds or replaces a flag while constructing the context.
    pub fn with_flag(mut self, key: impl Into<String>, value: bool) -> Self {
        self.flags.insert(key.into(), value);
        self
    }

    /// Adds or replaces a timestamp while constructing the context.
    pub fn with_timestamp(mut self, key: impl Into<String>, value: Duration) -> Self {
        self.timestamps.insert(key.into(), value);
        self
    }

    /// Returns the typed application data.
    pub fn user_data(&self) -> &UserData {
        &self.user_data
    }

    /// Returns a counter, treating a missing key as zero.
    pub fn counter(&self, key: &str) -> u64 {
        self.counters.get(key).copied().unwrap_or_default()
    }

    /// Returns a flag, treating a missing key as false.
    pub fn flag(&self, key: &str) -> bool {
        self.flags.get(key).copied().unwrap_or_default()
    }

    /// Returns the timestamp stored for `key`, if one exists.
    pub fn timestamp(&self, key: &str) -> Option<Duration> {
        self.timestamps.get(key).copied()
    }
}

/// A deterministic source of evaluation time supplied by the caller.
///
/// Implementations may use synchronization primitives when they need mutable
/// state, but the provider is owned by the specification that receives it.
/// There is no ambient or global clock.
pub trait Clock {
    /// Returns the current time in the clock's chosen monotonic epoch.
    fn now(&self) -> Duration;
}

/// A clock fixed at one instant, useful for deterministic evaluation and tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedClock(Duration);

impl FixedClock {
    /// Creates a clock that always returns `now`.
    pub const fn new(now: Duration) -> Self {
        Self(now)
    }
}

impl Clock for FixedClock {
    fn now(&self) -> Duration {
        self.0
    }
}

/// A specification satisfied while a named counter remains strictly below a maximum.
pub struct MaxCount {
    key: String,
    maximum_count: u64,
}

impl MaxCount {
    /// Creates a counter rule with the strict condition `counter < maximum_count`.
    pub fn new(key: impl Into<String>, maximum_count: u64) -> Self {
        Self {
            key: key.into(),
            maximum_count,
        }
    }
}

impl<UserData> Specification<EvaluationContext<UserData>> for MaxCount {
    fn is_satisfied_by(&self, context: &EvaluationContext<UserData>) -> bool {
        context.counter(&self.key) < self.maximum_count
    }
}

/// A specification satisfied when a named context flag is true.
pub struct Flag {
    key: String,
}

impl Flag {
    /// Creates a rule for a named boolean flag.
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

impl<UserData> Specification<EvaluationContext<UserData>> for Flag {
    fn is_satisfied_by(&self, context: &EvaluationContext<UserData>) -> bool {
        context.flag(&self.key)
    }
}

/// A specification satisfied after a named timestamp is at least one duration old.
///
/// `clock` is dependency-injected. A `Cooldown` can therefore be shared only
/// when its clock implementation is also `Send + Sync`.
pub struct Cooldown<ClockType> {
    key: String,
    duration: Duration,
    clock: ClockType,
}

impl<ClockType> Cooldown<ClockType> {
    /// Creates a cooldown rule using an explicitly provided clock.
    pub fn new(key: impl Into<String>, duration: Duration, clock: ClockType) -> Self {
        Self {
            key: key.into(),
            duration,
            clock,
        }
    }
}

impl<UserData, ClockType> Specification<EvaluationContext<UserData>> for Cooldown<ClockType>
where
    ClockType: Clock,
{
    fn is_satisfied_by(&self, context: &EvaluationContext<UserData>) -> bool {
        context.timestamp(&self.key).is_none_or(|timestamp| {
            let now = self.clock.now();
            now >= timestamp && now - timestamp >= self.duration
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use std::thread;
    use std::time::Duration;

    use super::{
        AllOf, Always, AnyOf, BoxedSpecification, Clock, Cooldown, DecisionSpecification,
        EvaluationContext, FirstMatch, FixedClock, Flag, MaxCount, Never, SharedSpecification,
        Specification,
    };

    fn assert_send_sync<T: Send + Sync>() {}

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
    fn shared_api_and_immutable_context_expose_expected_auto_traits() {
        assert_send_sync::<SharedSpecification<i32>>();
        assert_send_sync::<EvaluationContext<String>>();
        assert_send_sync::<Cooldown<FixedClock>>();
    }

    #[test]
    fn shared_specification_can_be_evaluated_concurrently() {
        let specification = SharedSpecification::new(|number: &u64| *number % 2 == 0);
        let handles = (0..8)
            .map(|_| {
                let specification = specification.clone();
                thread::spawn(move || {
                    (0..1_000_u64)
                        .filter(|number| specification.is_satisfied_by(number))
                        .count()
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            assert_eq!(handle.join().expect("worker thread should finish"), 500);
        }
    }

    #[derive(Clone)]
    struct CountingClock {
        now: Duration,
        reads: Arc<AtomicUsize>,
    }

    impl Clock for CountingClock {
        fn now(&self) -> Duration {
            self.reads.fetch_add(1, Ordering::Relaxed);
            self.now
        }
    }

    #[test]
    fn injected_thread_safe_clock_can_back_a_shared_cooldown() {
        let reads = Arc::new(AtomicUsize::new(0));
        let clock = CountingClock {
            now: Duration::from_secs(100),
            reads: Arc::clone(&reads),
        };
        let specification =
            SharedSpecification::new(Cooldown::new("last_login", Duration::from_secs(60), clock));
        let handles = (0..4)
            .map(|_| {
                let specification = specification.clone();
                let context = EvaluationContext::new(())
                    .with_timestamp("last_login", Duration::from_secs(40));
                thread::spawn(move || specification.is_satisfied_by(&context))
            })
            .collect::<Vec<_>>();

        for handle in handles {
            assert!(handle.join().expect("worker thread should finish"));
        }
        assert_eq!(reads.load(Ordering::Relaxed), 4);
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

    #[test]
    fn context_preserves_typed_user_data_and_default_values() {
        let context = EvaluationContext::new("user").with_counter("visits", 3);

        assert_eq!(context.user_data(), &"user");
        assert_eq!(context.counter("visits"), 3);
        assert_eq!(context.counter("missing"), 0);
        assert!(!context.flag("missing"));
        assert_eq!(context.timestamp("missing"), None);
    }

    #[test]
    fn max_count_uses_a_strict_boundary() {
        let specification = MaxCount::new("attempts", 3);

        assert!(
            specification.is_satisfied_by(&EvaluationContext::new(()).with_counter("attempts", 2))
        );
        assert!(
            !specification.is_satisfied_by(&EvaluationContext::new(()).with_counter("attempts", 3))
        );
        assert!(specification.is_satisfied_by(&EvaluationContext::new(())));
    }

    #[test]
    fn flag_requires_a_true_context_value() {
        let specification = Flag::new("premium");

        assert!(
            specification.is_satisfied_by(&EvaluationContext::new(()).with_flag("premium", true))
        );
        assert!(
            !specification.is_satisfied_by(&EvaluationContext::new(()).with_flag("premium", false))
        );
        assert!(!specification.is_satisfied_by(&EvaluationContext::new(())));
    }

    #[test]
    fn cooldown_uses_an_injected_clock_and_inclusive_boundary() {
        let specification = Cooldown::new(
            "last_login",
            Duration::from_secs(60),
            FixedClock::new(Duration::from_secs(100)),
        );

        assert!(specification.is_satisfied_by(&EvaluationContext::new(())));
        assert!(specification.is_satisfied_by(
            &EvaluationContext::new(()).with_timestamp("last_login", Duration::from_secs(40))
        ));
        assert!(!specification.is_satisfied_by(
            &EvaluationContext::new(()).with_timestamp("last_login", Duration::from_secs(41))
        ));
    }

    #[test]
    fn cooldown_rejects_future_timestamps_even_at_zero_duration() {
        let specification = Cooldown::new(
            "last_login",
            Duration::ZERO,
            FixedClock::new(Duration::from_secs(100)),
        );

        assert!(!specification.is_satisfied_by(
            &EvaluationContext::new(()).with_timestamp("last_login", Duration::from_secs(101))
        ));
    }
}
