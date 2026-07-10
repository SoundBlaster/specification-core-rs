//! Evaluates context rules and selects a typed first-match decision.

use std::time::Duration;

use specification_core::{
    Cooldown, DecisionSpecification, EvaluationContext, FirstMatch, FixedClock, Flag, MaxCount,
    Specification,
};

fn main() {
    let context = EvaluationContext::new("account-42")
        .with_counter("attempts", 1)
        .with_flag("enabled", true)
        .with_timestamp("last_action", Duration::from_secs(10));
    let clock = FixedClock::new(Duration::from_secs(15));

    assert!(MaxCount::new("attempts", 3).is_satisfied_by(&context));
    assert!(Flag::new("enabled").is_satisfied_by(&context));
    assert!(Cooldown::new("last_action", Duration::from_secs(5), clock).is_satisfied_by(&context));

    let mut route = FirstMatch::<EvaluationContext<&str>, &'static str>::new();
    route.push(Flag::new("enabled"), "enabled-route");
    assert_eq!(route.decide(&context), Some(&"enabled-route"));
}
