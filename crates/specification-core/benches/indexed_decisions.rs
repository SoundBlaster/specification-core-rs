#![allow(missing_docs, reason = "benchmark entry points are private")]

use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use specification_core::{DecisionSpecification, FirstMatch, IndexedFirstMatch};

struct Candidate {
    key: String,
    marker: u8,
}

fn catalog(
    rule_count: usize,
) -> (
    FirstMatch<Candidate, usize>,
    IndexedFirstMatch<Candidate, usize>,
) {
    let mut linear = FirstMatch::new();
    let mut indexed = IndexedFirstMatch::new(|candidate: &Candidate| candidate.key.as_str());
    for index in 0..rule_count {
        // The 214-rule fixture places 200 unique, unrelated keys before the
        // 14 built-in targets, mirroring H3's large-catalog miss cost.
        let key = if rule_count == 214 && index < 200 {
            format!("unrelated-{index:03}")
        } else {
            format!("target-{:02}", index % 14)
        };
        let marker = (index % 7) as u8;
        let linear_key = key.clone();
        let indexed_key = key.clone();
        linear.push(
            move |candidate: &Candidate| candidate.key == linear_key && candidate.marker == marker,
            index,
        );
        indexed.push_keyed(
            key,
            move |candidate: &Candidate| candidate.key == indexed_key && candidate.marker == marker,
            index,
        );
    }
    (linear, indexed)
}

fn candidates() -> Vec<Candidate> {
    (0..20_000)
        .map(|index| Candidate {
            key: if index % 4 == 0 {
                "unregistered".to_owned()
            } else {
                format!("target-{:02}", index % 14)
            },
            marker: (index % 11) as u8,
        })
        .collect()
}

fn checksum<E>(candidates: &[Candidate], evaluator: &E) -> u64
where
    E: DecisionSpecification<Candidate, Decision = usize>,
{
    candidates.iter().fold(0_u64, |sum, candidate| {
        sum.wrapping_add(evaluator.decide(candidate).copied().unwrap_or(usize::MAX) as u64)
    })
}

fn median(samples: &mut [Duration]) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn sample<E>(
    rule_count: usize,
    name: &str,
    sample_number: usize,
    candidates: &[Candidate],
    expected: u64,
    evaluator: &E,
) -> Duration
where
    E: DecisionSpecification<Candidate, Decision = usize>,
{
    let start = Instant::now();
    let mut result = 0_u64;
    for candidate in candidates {
        result = result.wrapping_add(
            evaluator
                .decide(black_box(candidate))
                .copied()
                .unwrap_or(usize::MAX) as u64,
        );
    }
    let elapsed = start.elapsed();
    assert_eq!(result, expected, "timed checksum changed for {name}");
    let ns_per_candidate = elapsed.as_nanos() as f64 / candidates.len() as f64;
    println!("{rule_count},{name},{sample_number},{ns_per_candidate:.3},{result}");
    elapsed
}

fn warm_up<E>(candidates: &[Candidate], evaluator: &E)
where
    E: DecisionSpecification<Candidate, Decision = usize>,
{
    black_box(checksum(candidates, evaluator));
}

fn main() {
    let candidates = candidates();
    println!("rules,implementation,sample,ns_per_candidate,checksum");
    for rule_count in [14, 214] {
        let (linear, indexed) = catalog(rule_count);
        let expected = checksum(&candidates, &linear);
        let actual = checksum(&candidates, &indexed);
        assert_eq!(
            actual, expected,
            "parity check failed for {rule_count} rules"
        );
        for candidate in &candidates {
            assert_eq!(
                linear.decide(candidate),
                indexed.decide(candidate),
                "per-candidate parity failed for {rule_count} rules"
            );
        }

        warm_up(&candidates, &linear);
        warm_up(&candidates, &indexed);
        let mut linear_samples = Vec::with_capacity(31);
        let mut indexed_samples = Vec::with_capacity(31);
        for sample_number in 0..31 {
            if sample_number % 2 == 0 {
                linear_samples.push(sample(
                    rule_count,
                    "linear",
                    sample_number,
                    &candidates,
                    expected,
                    &linear,
                ));
                indexed_samples.push(sample(
                    rule_count,
                    "indexed",
                    sample_number,
                    &candidates,
                    expected,
                    &indexed,
                ));
            } else {
                indexed_samples.push(sample(
                    rule_count,
                    "indexed",
                    sample_number,
                    &candidates,
                    expected,
                    &indexed,
                ));
                linear_samples.push(sample(
                    rule_count,
                    "linear",
                    sample_number,
                    &candidates,
                    expected,
                    &linear,
                ));
            }
        }
        let linear_median = median(&mut linear_samples);
        let indexed_median = median(&mut indexed_samples);
        eprintln!(
            "{rule_count} rules: linear median {:.3} ns/candidate; indexed median {:.3} ns/candidate",
            linear_median.as_nanos() as f64 / candidates.len() as f64,
            indexed_median.as_nanos() as f64 / candidates.len() as f64,
        );
    }
}
