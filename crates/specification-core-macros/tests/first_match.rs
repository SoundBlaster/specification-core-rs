//! Integration tests for the generated static keyed evaluator.

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

use specification_core::{DecisionSpecification, FirstMatch, Specification};
use specification_core_macros::first_match;

#[derive(Debug)]
struct Facts {
    name: String,
    directory: bool,
    cargo_parent: bool,
    own_environment: bool,
}

struct OwnEnvironment;

impl Specification<Facts> for OwnEnvironment {
    fn is_satisfied_by(&self, candidate: &Facts) -> bool {
        candidate.own_environment
    }
}

fn assert_send_sync<T: Send + Sync>(_: &T) {}

#[test]
fn mixed_keyed_unkeyed_alias_and_duplicate_rules_match_linear_order() {
    let generated = first_match! {
        context: Facts,
        decision: u16,
        key: |facts: &Facts| facts.name.as_str(),
        rules: [
            keyed([".build"], |facts: &Facts| facts.name == ".build" && facts.directory, 1),
            unkeyed(OwnEnvironment, 2),
            keyed(
                ["target", "target-alias"],
                |facts: &Facts| facts.name == "target-alias" && facts.directory && facts.cargo_parent,
                3,
            ),
            keyed(
                ["target"],
                |facts: &Facts| facts.name == "target" && facts.directory && facts.cargo_parent,
                4,
            ),
            unkeyed(|_: &Facts| false, 9),
        ]
    };

    let mut linear = FirstMatch::<Facts, u16>::new();
    linear.push(|facts: &Facts| facts.name == ".build" && facts.directory, 1);
    linear.push(OwnEnvironment, 2);
    linear.push(
        |facts: &Facts| facts.name == "target-alias" && facts.directory && facts.cargo_parent,
        3,
    );
    linear.push(
        |facts: &Facts| facts.name == "target" && facts.directory && facts.cargo_parent,
        4,
    );
    linear.push(|_: &Facts| false, 9);

    let names = [
        ".build",
        "target",
        "target-alias",
        "__pycache__",
        "unknown",
        "",
    ];
    for name in names {
        for flags in 0..8 {
            let candidate = Facts {
                name: name.to_owned(),
                directory: flags & 1 != 0,
                cargo_parent: flags & 2 != 0,
                own_environment: flags & 4 != 0,
            };
            assert_eq!(
                generated.decide(&candidate),
                linear.decide(&candidate),
                "parity mismatch for name={name:?}, flags={flags:03b}"
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NoCloneDecision(&'static str);

#[test]
fn construction_is_projection_then_interleaved_source_order_and_decisions_are_borrowed() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let projection_events = Arc::clone(&events);
    let first_spec_events = Arc::clone(&events);
    let first_decision_events = Arc::clone(&events);
    let second_spec_events = Arc::clone(&events);
    let second_decision_events = Arc::clone(&events);
    let generated = first_match! {
        context: Facts,
        decision: NoCloneDecision,
        key: {
            projection_events.lock().unwrap().push("projection");
            |facts: &Facts| facts.name.as_str()
        },
        rules: [
            keyed(["target"], {
                first_spec_events.lock().unwrap().push("spec-1");
                |facts: &Facts| facts.name == "target" && facts.directory
            }, {
                first_decision_events.lock().unwrap().push("decision-1");
                NoCloneDecision("directory")
            }),
            unkeyed({
                second_spec_events.lock().unwrap().push("spec-2");
                |facts: &Facts| facts.own_environment
            }, {
                second_decision_events.lock().unwrap().push("decision-2");
                NoCloneDecision("environment")
            }),
        ]
    };
    assert_eq!(
        *events.lock().unwrap(),
        ["projection", "spec-1", "decision-1", "spec-2", "decision-2"]
    );

    let candidate = Facts {
        name: "target".to_owned(),
        directory: true,
        cargo_parent: false,
        own_environment: true,
    };
    let first = generated
        .decide(&candidate)
        .expect("first rule should match");
    let second = generated
        .decide(&candidate)
        .expect("first rule should match again");
    assert_eq!(first, &NoCloneDecision("directory"));
    assert!(
        std::ptr::eq(first, second),
        "decision is borrowed from stored evaluator state"
    );
}

#[test]
fn empty_rules_return_none_and_unkeyed_rules_cover_unknown_keys() {
    let empty = first_match! {
        context: Facts,
        decision: u16,
        key: |facts: &Facts| facts.name.as_str(),
        rules: []
    };
    let unkeyed = first_match! {
        context: Facts,
        decision: u16,
        key: |facts: &Facts| facts.name.as_str(),
        rules: [unkeyed(OwnEnvironment, 7)]
    };
    let candidate = Facts {
        name: "unknown-key".to_owned(),
        directory: false,
        cargo_parent: false,
        own_environment: true,
    };
    assert_eq!(empty.decide(&candidate), None);
    assert_eq!(unkeyed.decide(&candidate), Some(&7));
}

#[test]
fn unrelated_key_skips_predicate_and_non_copy_decision_is_borrowed() {
    let predicate_calls = Arc::new(AtomicUsize::new(0));
    let calls = Arc::clone(&predicate_calls);
    let generated = first_match! {
        context: Facts,
        decision: NoCloneDecision,
        key: |facts: &Facts| facts.name.as_str(),
        rules: [
            keyed(["target", "target-alias"], {
                move |facts: &Facts| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    (facts.name == "target" || facts.name == "target-alias") && facts.directory
                }
            }, NoCloneDecision("target")),
        ]
    };
    let candidate = Facts {
        name: "unrelated".to_owned(),
        directory: true,
        cargo_parent: true,
        own_environment: false,
    };
    assert_eq!(generated.decide(&candidate), None);
    assert_eq!(predicate_calls.load(Ordering::SeqCst), 0);

    let target = Facts {
        name: "target".to_owned(),
        ..candidate
    };
    let first = generated
        .decide(&target)
        .expect("registered key should match");
    let second = generated
        .decide(&target)
        .expect("registered key should match again");
    assert_eq!(first, &NoCloneDecision("target"));
    assert!(std::ptr::eq(first, second));
    assert_eq!(predicate_calls.load(Ordering::SeqCst), 2);
}

fn projection_factory(
    constructions: Arc<AtomicUsize>,
    calls: Arc<AtomicUsize>,
) -> impl for<'a> Fn(&'a Facts) -> &'a str {
    constructions.fetch_add(1, Ordering::SeqCst);
    move |facts| {
        calls.fetch_add(1, Ordering::SeqCst);
        facts.name.as_str()
    }
}

fn predicate_factory(
    constructions: Arc<AtomicUsize>,
    evaluations: Arc<AtomicUsize>,
) -> impl Fn(&Facts) -> bool + Send + Sync {
    constructions.fetch_add(1, Ordering::SeqCst);
    move |facts| {
        evaluations.fetch_add(1, Ordering::SeqCst);
        facts.name == "target"
    }
}

fn decision_factory(constructions: Arc<AtomicUsize>, value: u16) -> u16 {
    constructions.fetch_add(1, Ordering::SeqCst);
    value
}

#[test]
fn expressions_construct_once_projection_runs_once_and_evaluator_is_shareable() {
    let projection_calls = Arc::new(AtomicUsize::new(0));
    let projection_constructions = Arc::new(AtomicUsize::new(0));
    let spec_constructions = Arc::new(AtomicUsize::new(0));
    let spec_evaluations = Arc::new(AtomicUsize::new(0));
    let decision_constructions = Arc::new(AtomicUsize::new(0));
    let generated = first_match! {
        context: Facts,
        decision: u16,
        key: projection_factory(
            Arc::clone(&projection_constructions),
            Arc::clone(&projection_calls),
        ),
        rules: [
            unkeyed(predicate_factory(
                Arc::clone(&spec_constructions),
                Arc::clone(&spec_evaluations),
            ), decision_factory(Arc::clone(&decision_constructions), 7)),
            keyed(
                ["target"],
                predicate_factory(
                    Arc::clone(&spec_constructions),
                    Arc::clone(&spec_evaluations),
                ),
                decision_factory(Arc::clone(&decision_constructions), 8),
            ),
        ]
    };

    assert_eq!(spec_constructions.load(Ordering::SeqCst), 2);
    assert_eq!(decision_constructions.load(Ordering::SeqCst), 2);
    assert_eq!(projection_constructions.load(Ordering::SeqCst), 1);
    assert_eq!(projection_calls.load(Ordering::SeqCst), 0);
    assert_send_sync(&generated);

    let candidate = Facts {
        name: "target".to_owned(),
        directory: true,
        cargo_parent: true,
        own_environment: false,
    };
    assert_eq!(generated.decide(&candidate), Some(&7));
    assert_eq!(projection_calls.load(Ordering::SeqCst), 1);
    assert_eq!(spec_evaluations.load(Ordering::SeqCst), 1);
    assert_eq!(spec_constructions.load(Ordering::SeqCst), 2);
    assert_eq!(decision_constructions.load(Ordering::SeqCst), 2);

    let shared = Arc::new(generated);
    let worker_evaluator = Arc::clone(&shared);
    let result = std::thread::spawn(move || {
        let worker_candidate = Facts {
            name: "target".to_owned(),
            directory: true,
            cargo_parent: true,
            own_environment: false,
        };
        worker_evaluator.decide(&worker_candidate).copied()
    })
    .join()
    .expect("worker should finish");
    assert_eq!(result, Some(7));
}

#[test]
fn duplicate_literal_key_keeps_first_matching_rule() {
    let generated = first_match! {
        context: Facts,
        decision: u16,
        key: |facts: &Facts| facts.name.as_str(),
        rules: [
            keyed(["target"], |facts: &Facts| facts.name == "target", 11),
            keyed(["target"], |facts: &Facts| facts.name == "target", 12),
        ]
    };
    let candidate = Facts {
        name: "target".to_owned(),
        directory: true,
        cargo_parent: false,
        own_environment: false,
    };
    assert_eq!(generated.decide(&candidate), Some(&11));
}

#[test]
fn keyed_match_short_circuits_later_unkeyed_specification() {
    let fallback_calls = Arc::new(AtomicUsize::new(0));
    let generated = first_match! {
        context: Facts,
        decision: u16,
        key: |facts: &Facts| facts.name.as_str(),
        rules: [
            keyed(["target"], |facts: &Facts| facts.name == "target", 1),
            unkeyed({
                let calls = Arc::clone(&fallback_calls);
                move |_: &Facts| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    true
                }
            }, 2),
        ]
    };
    let candidate = Facts {
        name: "target".to_owned(),
        directory: true,
        cargo_parent: false,
        own_environment: false,
    };

    assert_eq!(generated.decide(&candidate), Some(&1));
    assert_eq!(fallback_calls.load(Ordering::SeqCst), 0);
}
