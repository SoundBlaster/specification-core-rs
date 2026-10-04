#![allow(missing_docs, reason = "benchmark entry points are private")]

use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use specification_core::{DecisionSpecification, Never, Specification};
use specification_core_macros::first_match;

struct Facts {
    name: String,
    directory: bool,
    parent_cargo: bool,
    primary: bool,
    own_environment: bool,
}

struct IsDirectory;
impl Specification<Facts> for IsDirectory {
    fn is_satisfied_by(&self, candidate: &Facts) -> bool {
        candidate.directory
    }
}

struct ParentCargo;
impl Specification<Facts> for ParentCargo {
    fn is_satisfied_by(&self, candidate: &Facts) -> bool {
        candidate.parent_cargo
    }
}

struct OwnEnvironment;
impl Specification<Facts> for OwnEnvironment {
    fn is_satisfied_by(&self, candidate: &Facts) -> bool {
        candidate.own_environment
    }
}

struct TargetCargo;
impl Specification<Facts> for TargetCargo {
    fn is_satisfied_by(&self, candidate: &Facts) -> bool {
        (candidate.name == "target" || candidate.name == "target-alias")
            && candidate.directory
            && candidate.parent_cargo
            && candidate.primary
    }
}

struct PythonBytecode;
impl Specification<Facts> for PythonBytecode {
    fn is_satisfied_by(&self, candidate: &Facts) -> bool {
        candidate.name == "module.pyc" && !candidate.directory
    }
}

struct NameIs(&'static str);
impl Specification<Facts> for NameIs {
    fn is_satisfied_by(&self, candidate: &Facts) -> bool {
        candidate.name == self.0
    }
}

fn static_14() -> impl DecisionSpecification<Facts, Decision = u16> {
    first_match! {
        context: Facts,
        decision: u16,
        key: |facts: &Facts| facts.name.as_str(),
        rules: [
            keyed([".build"], NameIs(".build").and(IsDirectory), 1),
            unkeyed(OwnEnvironment, 2),
            keyed(["target", "target-alias"], TargetCargo, 3),
            keyed(["target"], NameIs("target").and(IsDirectory).and(ParentCargo), 4),
            keyed(["__pycache__"], NameIs("__pycache__").and(IsDirectory), 5),
            keyed([".pytest_cache"], NameIs(".pytest_cache").and(IsDirectory), 6),
            keyed([".mypy_cache"], NameIs(".mypy_cache").and(IsDirectory), 7),
            keyed([".ruff_cache"], NameIs(".ruff_cache").and(IsDirectory), 8),
            keyed([".pytype"], NameIs(".pytype").and(IsDirectory), 9),
            keyed([".tox"], NameIs(".tox").and(IsDirectory), 10),
            keyed([".nox"], NameIs(".nox").and(IsDirectory), 11),
            keyed(["build"], NameIs("build").and(IsDirectory), 12),
            keyed(["dist"], NameIs("dist").and(IsDirectory), 13),
            keyed(["pkg.egg-info"], NameIs("pkg.egg-info"), 14),
            keyed(["module.pyc"], PythonBytecode, 15),
        ]
    }
}

fn static_214() -> impl DecisionSpecification<Facts, Decision = u16> {
    first_match! {
        context: Facts,
        decision: u16,
        key: |facts: &Facts| facts.name.as_str(),
        rules: [
            keyed(["unrelated-000"], Never, 0),
            keyed(["unrelated-001"], Never, 0),
            keyed(["unrelated-002"], Never, 0),
            keyed(["unrelated-003"], Never, 0),
            keyed(["unrelated-004"], Never, 0),
            keyed(["unrelated-005"], Never, 0),
            keyed(["unrelated-006"], Never, 0),
            keyed(["unrelated-007"], Never, 0),
            keyed(["unrelated-008"], Never, 0),
            keyed(["unrelated-009"], Never, 0),
            keyed(["unrelated-010"], Never, 0),
            keyed(["unrelated-011"], Never, 0),
            keyed(["unrelated-012"], Never, 0),
            keyed(["unrelated-013"], Never, 0),
            keyed(["unrelated-014"], Never, 0),
            keyed(["unrelated-015"], Never, 0),
            keyed(["unrelated-016"], Never, 0),
            keyed(["unrelated-017"], Never, 0),
            keyed(["unrelated-018"], Never, 0),
            keyed(["unrelated-019"], Never, 0),
            keyed(["unrelated-020"], Never, 0),
            keyed(["unrelated-021"], Never, 0),
            keyed(["unrelated-022"], Never, 0),
            keyed(["unrelated-023"], Never, 0),
            keyed(["unrelated-024"], Never, 0),
            keyed(["unrelated-025"], Never, 0),
            keyed(["unrelated-026"], Never, 0),
            keyed(["unrelated-027"], Never, 0),
            keyed(["unrelated-028"], Never, 0),
            keyed(["unrelated-029"], Never, 0),
            keyed(["unrelated-030"], Never, 0),
            keyed(["unrelated-031"], Never, 0),
            keyed(["unrelated-032"], Never, 0),
            keyed(["unrelated-033"], Never, 0),
            keyed(["unrelated-034"], Never, 0),
            keyed(["unrelated-035"], Never, 0),
            keyed(["unrelated-036"], Never, 0),
            keyed(["unrelated-037"], Never, 0),
            keyed(["unrelated-038"], Never, 0),
            keyed(["unrelated-039"], Never, 0),
            keyed(["unrelated-040"], Never, 0),
            keyed(["unrelated-041"], Never, 0),
            keyed(["unrelated-042"], Never, 0),
            keyed(["unrelated-043"], Never, 0),
            keyed(["unrelated-044"], Never, 0),
            keyed(["unrelated-045"], Never, 0),
            keyed(["unrelated-046"], Never, 0),
            keyed(["unrelated-047"], Never, 0),
            keyed(["unrelated-048"], Never, 0),
            keyed(["unrelated-049"], Never, 0),
            keyed(["unrelated-050"], Never, 0),
            keyed(["unrelated-051"], Never, 0),
            keyed(["unrelated-052"], Never, 0),
            keyed(["unrelated-053"], Never, 0),
            keyed(["unrelated-054"], Never, 0),
            keyed(["unrelated-055"], Never, 0),
            keyed(["unrelated-056"], Never, 0),
            keyed(["unrelated-057"], Never, 0),
            keyed(["unrelated-058"], Never, 0),
            keyed(["unrelated-059"], Never, 0),
            keyed(["unrelated-060"], Never, 0),
            keyed(["unrelated-061"], Never, 0),
            keyed(["unrelated-062"], Never, 0),
            keyed(["unrelated-063"], Never, 0),
            keyed(["unrelated-064"], Never, 0),
            keyed(["unrelated-065"], Never, 0),
            keyed(["unrelated-066"], Never, 0),
            keyed(["unrelated-067"], Never, 0),
            keyed(["unrelated-068"], Never, 0),
            keyed(["unrelated-069"], Never, 0),
            keyed(["unrelated-070"], Never, 0),
            keyed(["unrelated-071"], Never, 0),
            keyed(["unrelated-072"], Never, 0),
            keyed(["unrelated-073"], Never, 0),
            keyed(["unrelated-074"], Never, 0),
            keyed(["unrelated-075"], Never, 0),
            keyed(["unrelated-076"], Never, 0),
            keyed(["unrelated-077"], Never, 0),
            keyed(["unrelated-078"], Never, 0),
            keyed(["unrelated-079"], Never, 0),
            keyed(["unrelated-080"], Never, 0),
            keyed(["unrelated-081"], Never, 0),
            keyed(["unrelated-082"], Never, 0),
            keyed(["unrelated-083"], Never, 0),
            keyed(["unrelated-084"], Never, 0),
            keyed(["unrelated-085"], Never, 0),
            keyed(["unrelated-086"], Never, 0),
            keyed(["unrelated-087"], Never, 0),
            keyed(["unrelated-088"], Never, 0),
            keyed(["unrelated-089"], Never, 0),
            keyed(["unrelated-090"], Never, 0),
            keyed(["unrelated-091"], Never, 0),
            keyed(["unrelated-092"], Never, 0),
            keyed(["unrelated-093"], Never, 0),
            keyed(["unrelated-094"], Never, 0),
            keyed(["unrelated-095"], Never, 0),
            keyed(["unrelated-096"], Never, 0),
            keyed(["unrelated-097"], Never, 0),
            keyed(["unrelated-098"], Never, 0),
            keyed(["unrelated-099"], Never, 0),
            keyed(["unrelated-100"], Never, 0),
            keyed(["unrelated-101"], Never, 0),
            keyed(["unrelated-102"], Never, 0),
            keyed(["unrelated-103"], Never, 0),
            keyed(["unrelated-104"], Never, 0),
            keyed(["unrelated-105"], Never, 0),
            keyed(["unrelated-106"], Never, 0),
            keyed(["unrelated-107"], Never, 0),
            keyed(["unrelated-108"], Never, 0),
            keyed(["unrelated-109"], Never, 0),
            keyed(["unrelated-110"], Never, 0),
            keyed(["unrelated-111"], Never, 0),
            keyed(["unrelated-112"], Never, 0),
            keyed(["unrelated-113"], Never, 0),
            keyed(["unrelated-114"], Never, 0),
            keyed(["unrelated-115"], Never, 0),
            keyed(["unrelated-116"], Never, 0),
            keyed(["unrelated-117"], Never, 0),
            keyed(["unrelated-118"], Never, 0),
            keyed(["unrelated-119"], Never, 0),
            keyed(["unrelated-120"], Never, 0),
            keyed(["unrelated-121"], Never, 0),
            keyed(["unrelated-122"], Never, 0),
            keyed(["unrelated-123"], Never, 0),
            keyed(["unrelated-124"], Never, 0),
            keyed(["unrelated-125"], Never, 0),
            keyed(["unrelated-126"], Never, 0),
            keyed(["unrelated-127"], Never, 0),
            keyed(["unrelated-128"], Never, 0),
            keyed(["unrelated-129"], Never, 0),
            keyed(["unrelated-130"], Never, 0),
            keyed(["unrelated-131"], Never, 0),
            keyed(["unrelated-132"], Never, 0),
            keyed(["unrelated-133"], Never, 0),
            keyed(["unrelated-134"], Never, 0),
            keyed(["unrelated-135"], Never, 0),
            keyed(["unrelated-136"], Never, 0),
            keyed(["unrelated-137"], Never, 0),
            keyed(["unrelated-138"], Never, 0),
            keyed(["unrelated-139"], Never, 0),
            keyed(["unrelated-140"], Never, 0),
            keyed(["unrelated-141"], Never, 0),
            keyed(["unrelated-142"], Never, 0),
            keyed(["unrelated-143"], Never, 0),
            keyed(["unrelated-144"], Never, 0),
            keyed(["unrelated-145"], Never, 0),
            keyed(["unrelated-146"], Never, 0),
            keyed(["unrelated-147"], Never, 0),
            keyed(["unrelated-148"], Never, 0),
            keyed(["unrelated-149"], Never, 0),
            keyed(["unrelated-150"], Never, 0),
            keyed(["unrelated-151"], Never, 0),
            keyed(["unrelated-152"], Never, 0),
            keyed(["unrelated-153"], Never, 0),
            keyed(["unrelated-154"], Never, 0),
            keyed(["unrelated-155"], Never, 0),
            keyed(["unrelated-156"], Never, 0),
            keyed(["unrelated-157"], Never, 0),
            keyed(["unrelated-158"], Never, 0),
            keyed(["unrelated-159"], Never, 0),
            keyed(["unrelated-160"], Never, 0),
            keyed(["unrelated-161"], Never, 0),
            keyed(["unrelated-162"], Never, 0),
            keyed(["unrelated-163"], Never, 0),
            keyed(["unrelated-164"], Never, 0),
            keyed(["unrelated-165"], Never, 0),
            keyed(["unrelated-166"], Never, 0),
            keyed(["unrelated-167"], Never, 0),
            keyed(["unrelated-168"], Never, 0),
            keyed(["unrelated-169"], Never, 0),
            keyed(["unrelated-170"], Never, 0),
            keyed(["unrelated-171"], Never, 0),
            keyed(["unrelated-172"], Never, 0),
            keyed(["unrelated-173"], Never, 0),
            keyed(["unrelated-174"], Never, 0),
            keyed(["unrelated-175"], Never, 0),
            keyed(["unrelated-176"], Never, 0),
            keyed(["unrelated-177"], Never, 0),
            keyed(["unrelated-178"], Never, 0),
            keyed(["unrelated-179"], Never, 0),
            keyed(["unrelated-180"], Never, 0),
            keyed(["unrelated-181"], Never, 0),
            keyed(["unrelated-182"], Never, 0),
            keyed(["unrelated-183"], Never, 0),
            keyed(["unrelated-184"], Never, 0),
            keyed(["unrelated-185"], Never, 0),
            keyed(["unrelated-186"], Never, 0),
            keyed(["unrelated-187"], Never, 0),
            keyed(["unrelated-188"], Never, 0),
            keyed(["unrelated-189"], Never, 0),
            keyed(["unrelated-190"], Never, 0),
            keyed(["unrelated-191"], Never, 0),
            keyed(["unrelated-192"], Never, 0),
            keyed(["unrelated-193"], Never, 0),
            keyed(["unrelated-194"], Never, 0),
            keyed(["unrelated-195"], Never, 0),
            keyed(["unrelated-196"], Never, 0),
            keyed(["unrelated-197"], Never, 0),
            keyed(["unrelated-198"], Never, 0),
            keyed(["unrelated-199"], Never, 0),
            keyed([".build"], NameIs(".build").and(IsDirectory), 1),
            unkeyed(OwnEnvironment, 2),
            keyed(["target", "target-alias"], TargetCargo, 3),
            keyed(["target"], NameIs("target").and(IsDirectory).and(ParentCargo), 4),
            keyed(["__pycache__"], NameIs("__pycache__").and(IsDirectory), 5),
            keyed([".pytest_cache"], NameIs(".pytest_cache").and(IsDirectory), 6),
            keyed([".mypy_cache"], NameIs(".mypy_cache").and(IsDirectory), 7),
            keyed([".ruff_cache"], NameIs(".ruff_cache").and(IsDirectory), 8),
            keyed([".pytype"], NameIs(".pytype").and(IsDirectory), 9),
            keyed([".tox"], NameIs(".tox").and(IsDirectory), 10),
            keyed([".nox"], NameIs(".nox").and(IsDirectory), 11),
            keyed(["build"], NameIs("build").and(IsDirectory), 12),
            keyed(["dist"], NameIs("dist").and(IsDirectory), 13),
            keyed(["pkg.egg-info"], NameIs("pkg.egg-info"), 14),
            keyed(["module.pyc"], PythonBytecode, 15),
        ]
    }
}

fn handwritten(candidate: &Facts) -> Option<u16> {
    if candidate.name == ".build" && candidate.directory {
        return Some(1);
    }
    if candidate.own_environment {
        return Some(2);
    }
    match candidate.name.as_str() {
        "target-alias" if candidate.directory && candidate.parent_cargo && candidate.primary => {
            Some(3)
        }
        "target" if candidate.directory && candidate.parent_cargo && candidate.primary => Some(3),
        "target" if candidate.directory && candidate.parent_cargo => Some(4),
        "__pycache__" if candidate.directory => Some(5),
        ".pytest_cache" if candidate.directory => Some(6),
        ".mypy_cache" if candidate.directory => Some(7),
        ".ruff_cache" if candidate.directory => Some(8),
        ".pytype" if candidate.directory => Some(9),
        ".tox" if candidate.directory => Some(10),
        ".nox" if candidate.directory => Some(11),
        "build" if candidate.directory => Some(12),
        "dist" if candidate.directory => Some(13),
        "pkg.egg-info" => Some(14),
        "module.pyc" if !candidate.directory => Some(15),
        _ => None,
    }
}

fn candidates() -> Vec<Facts> {
    const TARGETS: [&str; 14] = [
        ".build",
        "target",
        "target-alias",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".ruff_cache",
        ".pytype",
        ".tox",
        ".nox",
        "build",
        "dist",
        "pkg.egg-info",
        "module.pyc",
    ];
    (0..20_000)
        .map(|index| {
            let name = if index % 4 == 0 {
                format!("unrelated-{:03}", index % 200)
            } else {
                TARGETS[index % TARGETS.len()].to_owned()
            };
            Facts {
                directory: name != "module.pyc",
                parent_cargo: index % 3 == 0,
                primary: index % 2 == 0,
                own_environment: index % 13 == 0,
                name,
            }
        })
        .collect()
}

fn checksum<E>(candidates: &[Facts], evaluator: &E) -> u64
where
    E: DecisionSpecification<Facts, Decision = u16>,
{
    candidates.iter().fold(0_u64, |sum, candidate| {
        sum.wrapping_add(u64::from(
            evaluator.decide(candidate).copied().unwrap_or(u16::MAX),
        ))
    })
}

fn median(samples: &mut [Duration]) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn measure<E>(
    catalog: usize,
    implementation: &str,
    round: usize,
    candidates: &[Facts],
    expected: u64,
    evaluator: &E,
) -> Duration
where
    E: DecisionSpecification<Facts, Decision = u16>,
{
    let start = Instant::now();
    let mut result = 0_u64;
    for candidate in candidates {
        result = result.wrapping_add(u64::from(
            evaluator
                .decide(black_box(candidate))
                .copied()
                .unwrap_or(u16::MAX),
        ));
    }
    let elapsed = start.elapsed();
    assert_eq!(
        result, expected,
        "timed result changed for {implementation}"
    );
    let ns = elapsed.as_nanos() as f64 / candidates.len() as f64;
    println!("{catalog},{implementation},{round},{ns:.3},{result}");
    elapsed
}

fn warm_up<E>(candidates: &[Facts], evaluator: &E)
where
    E: DecisionSpecification<Facts, Decision = u16>,
{
    black_box(checksum(candidates, evaluator));
}

fn bench_pair<S, H>(
    catalog: usize,
    candidates: &[Facts],
    static_evaluator: &S,
    handwritten_evaluator: &H,
    expected: u64,
) -> (Vec<Duration>, Vec<Duration>)
where
    S: DecisionSpecification<Facts, Decision = u16>,
    H: DecisionSpecification<Facts, Decision = u16>,
{
    warm_up(candidates, static_evaluator);
    warm_up(candidates, handwritten_evaluator);
    let mut static_samples = Vec::with_capacity(25);
    let mut handwritten_samples = Vec::with_capacity(25);
    for round in 0..25 {
        if round % 2 == 0 {
            static_samples.push(measure(
                catalog,
                "static",
                round,
                candidates,
                expected,
                static_evaluator,
            ));
            handwritten_samples.push(measure(
                catalog,
                "handwritten",
                round,
                candidates,
                expected,
                handwritten_evaluator,
            ));
        } else {
            handwritten_samples.push(measure(
                catalog,
                "handwritten",
                round,
                candidates,
                expected,
                handwritten_evaluator,
            ));
            static_samples.push(measure(
                catalog,
                "static",
                round,
                candidates,
                expected,
                static_evaluator,
            ));
        }
    }
    (static_samples, handwritten_samples)
}

fn main() {
    let candidates = candidates();
    let small = static_14();
    let large = static_214();
    let expected = candidates.iter().map(handwritten).collect::<Vec<_>>();
    assert_eq!(
        candidates
            .iter()
            .map(|item| small.decide(item).copied())
            .collect::<Vec<_>>(),
        expected.clone(),
        "14-key parity failed"
    );
    assert_eq!(
        candidates
            .iter()
            .map(|item| large.decide(item).copied())
            .collect::<Vec<_>>(),
        expected.clone(),
        "214-rule parity failed"
    );
    let small_checksum = checksum(&candidates, &small);
    let large_checksum = checksum(&candidates, &large);
    let expected_checksum = expected
        .iter()
        .map(|value| u64::from(value.unwrap_or(u16::MAX)))
        .sum::<u64>();
    assert_eq!(small_checksum, expected_checksum);
    assert_eq!(large_checksum, expected_checksum);

    println!("catalog,implementation,round,ns_per_candidate,checksum");
    let (mut small_static, mut small_handwritten) =
        bench_pair(14, &candidates, &small, &Handwritten, small_checksum);
    let (mut large_static, mut large_handwritten) =
        bench_pair(214, &candidates, &large, &Handwritten, large_checksum);
    eprintln!(
        "14 key static median: {:.3} ns/candidate",
        median(&mut small_static).as_nanos() as f64 / candidates.len() as f64
    );
    eprintln!(
        "14 key handwritten median: {:.3} ns/candidate",
        median(&mut small_handwritten).as_nanos() as f64 / candidates.len() as f64
    );
    eprintln!(
        "214 rule static median: {:.3} ns/candidate",
        median(&mut large_static).as_nanos() as f64 / candidates.len() as f64
    );
    eprintln!(
        "214 rule handwritten median: {:.3} ns/candidate",
        median(&mut large_handwritten).as_nanos() as f64 / candidates.len() as f64
    );
}

struct Handwritten;
impl DecisionSpecification<Facts> for Handwritten {
    type Decision = u16;

    fn decide(&self, candidate: &Facts) -> Option<&u16> {
        // A thread-local value is unnecessary: this helper is only a benchmark
        // comparator. Store the result in a stable static table instead below.
        static DECISIONS: [u16; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        handwritten(candidate).map(|index| &DECISIONS[usize::from(index)])
    }
}
