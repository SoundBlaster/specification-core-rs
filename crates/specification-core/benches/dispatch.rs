//! Baseline comparison of static and explicit boxed dispatch.
#![allow(
    missing_docs,
    reason = "Criterion generates private benchmark entry points"
)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use specification_core::{BoxedSpecification, Specification};

fn dispatch(c: &mut Criterion) {
    let candidate = 42_u64;
    let static_rule = |value: &u64| *value >= 18;
    let dynamic_rule = BoxedSpecification::new(|value: &u64| *value >= 18);

    let mut group = c.benchmark_group("single_predicate");
    group.bench_function("static", |bencher| {
        bencher.iter(|| static_rule.is_satisfied_by(black_box(&candidate)));
    });
    group.bench_function("boxed_dynamic", |bencher| {
        bencher.iter(|| dynamic_rule.is_satisfied_by(black_box(&candidate)));
    });
    group.finish();
}

criterion_group!(benches, dispatch);
criterion_main!(benches);
