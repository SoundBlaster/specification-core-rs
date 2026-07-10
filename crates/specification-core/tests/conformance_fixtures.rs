//! Executes the versioned, language-neutral conformance fixture corpus.

use std::{cell::RefCell, rc::Rc, time::Duration};

use serde_json::Value;
use specification_core::{
    AllOf, AnyOf, BoxedSpecification, Cooldown, DecisionSpecification, EvaluationContext,
    FirstMatch, FixedClock, Flag, MaxCount, Specification,
};

type Trace = Rc<RefCell<Vec<String>>>;

#[test]
fn v1_conformance_manifest_passes() {
    let manifest = read_json(include_str!(
        "../../../fixtures/conformance/v1/manifest.json"
    ));
    assert_eq!(integer(&manifest, "schema_version"), 1);

    for fixture_name in array(&manifest, "fixtures") {
        let fixture = read_json(fixture_source(
            fixture_name
                .as_str()
                .expect("fixture path must be a string"),
        ));
        assert_eq!(integer(&fixture, "schema_version"), 1);

        match string(&fixture, "group") {
            "composition" => run_composition_cases(&fixture),
            "first_match" => run_first_match_cases(&fixture),
            "context" => run_context_cases(&fixture),
            group => panic!("unsupported conformance fixture group: {group}"),
        }
    }
}

fn run_composition_cases(fixture: &Value) {
    for case in array(fixture, "cases") {
        let trace = Rc::new(RefCell::new(Vec::new()));
        let candidate = integer(case, "candidate") as i64;
        let result = evaluate_expression(value(case, "expression"), &trace, candidate);
        assert_expected(case, result, &trace);
    }
}

fn evaluate_expression(expression: &Value, trace: &Trace, candidate: i64) -> bool {
    match string(expression, "op") {
        "and" => {
            let left = compile_leaf(value(expression, "left"), trace);
            let right = compile_leaf(value(expression, "right"), trace);
            left.and(right).is_satisfied_by(&candidate)
        }
        "or" => {
            let left = compile_leaf(value(expression, "left"), trace);
            let right = compile_leaf(value(expression, "right"), trace);
            left.or(right).is_satisfied_by(&candidate)
        }
        "not" => compile_leaf(value(expression, "inner"), trace)
            .not()
            .is_satisfied_by(&candidate),
        "all_of" => {
            let specifications = array(expression, "items")
                .iter()
                .map(|item| compile_leaf(item, trace))
                .collect::<Vec<_>>();
            AllOf::new(&specifications).is_satisfied_by(&candidate)
        }
        "any_of" => {
            let specifications = array(expression, "items")
                .iter()
                .map(|item| compile_leaf(item, trace))
                .collect::<Vec<_>>();
            AnyOf::new(&specifications).is_satisfied_by(&candidate)
        }
        operation => panic!("unsupported composition operation: {operation}"),
    }
}

fn compile_leaf(expression: &Value, trace: &Trace) -> BoxedSpecification<i64> {
    match string(expression, "op") {
        "probe" => {
            let name = string(expression, "name").to_owned();
            let result = boolean(expression, "result");
            let trace = Rc::clone(trace);
            BoxedSpecification::new(move |_: &i64| {
                trace.borrow_mut().push(name.clone());
                result
            })
        }
        operation => panic!("unsupported leaf operation: {operation}"),
    }
}

fn run_first_match_cases(fixture: &Value) {
    for case in array(fixture, "cases") {
        let trace = Rc::new(RefCell::new(Vec::new()));
        let mut rules = FirstMatch::<i64, String>::new();

        for rule in array(case, "rules") {
            let name = string(rule, "name").to_owned();
            let result = boolean(rule, "result");
            let decision = string(rule, "decision").to_owned();
            let trace = Rc::clone(&trace);
            rules.push(
                move |_: &i64| {
                    trace.borrow_mut().push(name.clone());
                    result
                },
                decision,
            );
        }

        let expected = value(case, "expected");
        let candidate = integer(case, "candidate") as i64;
        match value(expected, "decision") {
            Value::Null => assert!(rules.decide(&candidate).is_none(), "{}", string(case, "id")),
            expected_decision => {
                let expected_decision = expected_decision
                    .as_str()
                    .expect("expected decision must be a string");
                if let Some(fallback) = case.get("fallback") {
                    let fallback = fallback
                        .as_str()
                        .expect("fallback must be a string")
                        .to_owned();
                    assert_eq!(
                        rules.decide_or(&candidate, &fallback),
                        expected_decision,
                        "{}",
                        string(case, "id")
                    );
                } else {
                    assert_eq!(
                        rules.decide(&candidate).map(String::as_str),
                        Some(expected_decision),
                        "{}",
                        string(case, "id")
                    );
                }
            }
        }
        assert_trace(case, &trace);
    }
}

fn run_context_cases(fixture: &Value) {
    for case in array(fixture, "cases") {
        let context_input = case.get("context").and_then(Value::as_object);
        let mut context = EvaluationContext::new(());

        if let Some(counters) = context_input.and_then(|input| input.get("counters")) {
            for (key, count) in counters.as_object().expect("counters must be an object") {
                context = context.with_counter(key, count.as_u64().expect("counter must be u64"));
            }
        }
        if let Some(flags) = context_input.and_then(|input| input.get("flags")) {
            for (key, enabled) in flags.as_object().expect("flags must be an object") {
                context = context.with_flag(key, enabled.as_bool().expect("flag must be bool"));
            }
        }
        if let Some(timestamps) = context_input.and_then(|input| input.get("timestamps_ms")) {
            for (key, timestamp) in timestamps
                .as_object()
                .expect("timestamps must be an object")
            {
                context = context.with_timestamp(
                    key,
                    Duration::from_millis(timestamp.as_u64().expect("timestamp must be u64")),
                );
            }
        }

        let rule = value(case, "rule");
        let result = match string(rule, "op") {
            "max_count" => MaxCount::new(string(rule, "key"), integer(rule, "maximum_count"))
                .is_satisfied_by(&context),
            "flag" => Flag::new(string(rule, "key")).is_satisfied_by(&context),
            "cooldown" => Cooldown::new(
                string(rule, "key"),
                Duration::from_millis(integer(rule, "duration_ms")),
                FixedClock::new(Duration::from_millis(
                    context_input
                        .and_then(|input| input.get("now_ms"))
                        .and_then(Value::as_u64)
                        .unwrap_or_default(),
                )),
            )
            .is_satisfied_by(&context),
            operation => panic!("unsupported context operation: {operation}"),
        };
        assert_expected(case, result, &Rc::new(RefCell::new(Vec::new())));
    }
}

fn assert_expected(case: &Value, result: bool, trace: &Trace) {
    assert_eq!(
        result,
        boolean(value(case, "expected"), "result"),
        "{}",
        string(case, "id")
    );
    assert_trace(case, trace);
}

fn assert_trace(case: &Value, trace: &Trace) {
    let expected_trace = array(value(case, "expected"), "trace")
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .expect("trace entry must be a string")
                .to_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(*trace.borrow(), expected_trace, "{}", string(case, "id"));
}

fn fixture_source(fixture_name: &str) -> &'static str {
    match fixture_name {
        "composition.json" => include_str!("../../../fixtures/conformance/v1/composition.json"),
        "first-match.json" => include_str!("../../../fixtures/conformance/v1/first-match.json"),
        "context.json" => include_str!("../../../fixtures/conformance/v1/context.json"),
        unsupported => panic!("fixture is not embedded by the Rust adapter: {unsupported}"),
    }
}

fn read_json(source: &str) -> Value {
    serde_json::from_str(source).expect("fixture must be valid JSON")
}

fn value<'a>(document: &'a Value, key: &str) -> &'a Value {
    document
        .get(key)
        .unwrap_or_else(|| panic!("missing `{key}`"))
}

fn array<'a>(document: &'a Value, key: &str) -> &'a Vec<Value> {
    value(document, key)
        .as_array()
        .unwrap_or_else(|| panic!("`{key}` must be an array"))
}

fn string<'a>(document: &'a Value, key: &str) -> &'a str {
    value(document, key)
        .as_str()
        .unwrap_or_else(|| panic!("`{key}` must be a string"))
}

fn integer(document: &Value, key: &str) -> u64 {
    value(document, key)
        .as_u64()
        .unwrap_or_else(|| panic!("`{key}` must be an unsigned integer"))
}

fn boolean(document: &Value, key: &str) -> bool {
    value(document, key)
        .as_bool()
        .unwrap_or_else(|| panic!("`{key}` must be a boolean"))
}
