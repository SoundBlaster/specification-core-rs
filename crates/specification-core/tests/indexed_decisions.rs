//! Behavioral and concurrency checks for indexed ordered decisions.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use specification_core::{
    BoxedSpecification, DecisionSpecification, IndexedFirstMatch, Specification,
};

#[derive(Debug)]
struct Candidate {
    key: String,
    value: u32,
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn indexed_decisions_are_send_and_sync_for_shared_worker_use() {
    assert_send_sync::<IndexedFirstMatch<Candidate, usize>>();

    let mut indexed = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());
    indexed.push_keyed(
        "rust",
        |candidate: &Candidate| candidate.key == "rust" && candidate.value == 7,
        42,
    );
    let shared = Arc::new(indexed);
    let worker_index = Arc::clone(&shared);
    let decision = std::thread::spawn(move || {
        let candidate = Candidate {
            key: "rust".to_owned(),
            value: 7,
        };
        worker_index.decide(&candidate).copied()
    })
    .join()
    .expect("worker should finish");
    assert_eq!(decision, Some(42));
}

#[test]
fn seeded_randomized_rules_match_linear_first_match() {
    let keys = ["swift", "rust", "python", "node", "gradle", "other"];
    for initial_seed in [0x6d2b_79f5_u32, 0x1234_5678, 0xfeed_beef] {
        let mut seed = initial_seed;
        let mut next = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            seed
        };

        let mut indexed = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());
        let mut linear = Vec::new();
        let mut keyed_count = 0;
        let mut unkeyed_count = 0;
        for index in 0..257 {
            let random = next();
            if (random >> 16) & 3 == 0 {
                unkeyed_count += 1;
                let threshold = random % 8;
                let specification = move |candidate: &Candidate| candidate.value % 8 == threshold;
                indexed.push_unkeyed(specification, index);
                linear.push((BoxedSpecification::new(specification), index));
            } else {
                keyed_count += 1;
                let key = keys[((random >> 8) as usize) % keys.len()].to_owned();
                let threshold = next() % 8;
                let indexed_key = key.clone();
                let linear_key = key.clone();
                let specification = move |candidate: &Candidate| {
                    candidate.key == indexed_key && candidate.value % 8 == threshold
                };
                indexed.push_keyed(key, specification, index);
                linear.push((
                    BoxedSpecification::new(move |candidate: &Candidate| {
                        candidate.key == linear_key && candidate.value % 8 == threshold
                    }),
                    index,
                ));
            }
        }
        assert!(keyed_count > 0, "seed {initial_seed} needs keyed rules");
        assert!(unkeyed_count > 0, "seed {initial_seed} needs unkeyed rules");

        for key in keys.into_iter().chain(["missing"]) {
            for value in 0..64 {
                let candidate = Candidate {
                    key: key.to_owned(),
                    value,
                };
                let expected = linear.iter().find_map(|(specification, decision)| {
                    specification
                        .is_satisfied_by(&candidate)
                        .then_some(decision)
                });
                assert_eq!(
                    indexed.decide(&candidate),
                    expected,
                    "seed {initial_seed} {key} {value}"
                );
            }
        }
    }
}

#[test]
fn interleaved_fallback_and_duplicate_keys_keep_priority_and_short_circuit() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut indexed = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());

    let first = Arc::clone(&calls);
    indexed.push_keyed(
        "rust",
        move |_: &Candidate| {
            first.fetch_add(1, Ordering::SeqCst);
            false
        },
        "first duplicate",
    );
    let fallback = Arc::clone(&calls);
    indexed.push_unkeyed(
        move |_: &Candidate| {
            fallback.fetch_add(1, Ordering::SeqCst);
            true
        },
        "fallback",
    );
    let later_duplicate = Arc::clone(&calls);
    indexed.push_keyed(
        "rust",
        move |candidate: &Candidate| {
            later_duplicate.fetch_add(1, Ordering::SeqCst);
            candidate.key == "rust"
        },
        "later duplicate",
    );

    let candidate = Candidate {
        key: "rust".to_owned(),
        value: 0,
    };
    assert_eq!(indexed.decide(&candidate), Some(&"fallback"));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn unmatched_key_skips_two_hundred_unrelated_rules() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut indexed = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());
    for index in 0..200 {
        let calls = Arc::clone(&calls);
        let key = format!("key-{index}");
        let predicate_key = key.clone();
        indexed.push_keyed(
            key,
            move |candidate: &Candidate| {
                calls.fetch_add(1, Ordering::SeqCst);
                candidate.key == predicate_key
            },
            index,
        );
    }

    let candidate = Candidate {
        key: "not-registered".to_owned(),
        value: 0,
    };
    assert_eq!(indexed.decide(&candidate), None);
    assert!(!indexed.may_match("not-registered"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[test]
fn may_match_includes_unkeyed_fallbacks_even_for_missing_key() {
    let mut indexed = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());
    indexed.push_unkeyed(|_: &Candidate| false, "fallback");
    assert!(indexed.may_match("unknown"));
}

#[test]
fn empty_and_nonmatching_catalogs_return_no_decision() {
    let empty =
        IndexedFirstMatch::<Candidate, usize>::new(|candidate: &Candidate| candidate.key.as_str());
    let candidate = Candidate {
        key: "absent".to_owned(),
        value: 0,
    };
    assert_eq!(empty.decide(&candidate), None);
    assert!(!empty.may_match("absent"));

    let mut nonmatching = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());
    nonmatching.push_keyed("registered", |_: &Candidate| false, 1);
    nonmatching.push_unkeyed(|_: &Candidate| false, 2);
    assert_eq!(nonmatching.decide(&candidate), None);
    assert!(nonmatching.may_match("absent"));
}

#[test]
fn decision_projects_the_candidate_key_once() {
    let projections = Arc::new(AtomicUsize::new(0));
    let projection_count = Arc::clone(&projections);
    let mut indexed = IndexedFirstMatch::new(move |candidate: &Candidate| {
        projection_count.fetch_add(1, Ordering::SeqCst);
        candidate.key.as_str()
    });
    indexed.push_keyed(
        "rust",
        |candidate: &Candidate| candidate.key == "rust",
        "rust",
    );

    let candidate = Candidate {
        key: "rust".to_owned(),
        value: 0,
    };
    assert_eq!(indexed.decide(&candidate), Some(&"rust"));
    assert_eq!(projections.load(Ordering::SeqCst), 1);
}

#[test]
fn keyed_success_short_circuits_a_later_unkeyed_rule() {
    let fallback_calls = Arc::new(AtomicUsize::new(0));
    let mut indexed = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());
    indexed.push_keyed(
        "rust",
        |candidate: &Candidate| candidate.key == "rust" && candidate.value == 7,
        "keyed",
    );
    let calls = Arc::clone(&fallback_calls);
    indexed.push_unkeyed(
        move |_: &Candidate| {
            calls.fetch_add(1, Ordering::SeqCst);
            true
        },
        "fallback",
    );

    let candidate = Candidate {
        key: "rust".to_owned(),
        value: 7,
    };
    assert_eq!(indexed.decide(&candidate), Some(&"keyed"));
    assert_eq!(fallback_calls.load(Ordering::SeqCst), 0);
}
