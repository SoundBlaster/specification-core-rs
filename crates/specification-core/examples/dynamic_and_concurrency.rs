//! Shows the explicit boundaries for heterogeneous rules and thread sharing.

use specification_core::{BoxedSpecification, SharedSpecification, Specification};

fn main() {
    let local_rules: Vec<BoxedSpecification<u64>> = vec![
        BoxedSpecification::new(|value: &u64| *value > 0),
        BoxedSpecification::new(|value: &u64| *value % 2 == 0),
    ];

    assert!(local_rules[0].is_satisfied_by(&42));
    assert!(local_rules[1].is_satisfied_by(&42));

    let shared = SharedSpecification::new(|value: &u64| *value % 2 == 0);
    let worker_rule = shared.clone();

    std::thread::spawn(move || {
        assert!(worker_rule.is_satisfied_by(&42));
    })
    .join()
    .expect("worker thread should finish");
}
