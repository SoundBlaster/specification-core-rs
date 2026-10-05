# Benchmarks

The Criterion baseline compares an equivalent static closure specification with
an explicit `BoxedSpecification` dynamic boundary:

```sh
cargo bench -p specification-core --bench dispatch
```

It measures repeated single-predicate evaluation with `black_box`. It does not
measure allocation, construction, context lookup, composition depth, or
application throughput. Record the Rust version, target, CPU, power mode,
command, and Criterion report when comparing runs. Results are not a
performance guarantee or cross-machine comparison.

The indexed decision benchmark compares `FirstMatch` and
`IndexedFirstMatch` with a 14-rule catalog and a 214-rule catalog. The larger
catalog places 200 unique unrelated keys before the same 14 target rules, so
candidate keys miss those first 200 rules. The harness checks every generated
candidate and the aggregate checksum, including misses, before timing. It
warms both paths, alternates their order across 31 rounds, and prints all raw
samples as CSV on stdout with median summaries on stderr:

```sh
mkdir -p target/benchmark-artifacts
cargo bench -p specification-core --bench indexed_decisions \
  > target/benchmark-artifacts/indexed-decisions.csv \
  2> target/benchmark-artifacts/indexed-decisions-median.txt
```

Retain both files with the Rust version, target, CPU, power mode, and command.
The run describes this workload only; it does not establish a general speedup.

The `static_decisions` benchmark uses the actual macro crate and core
specifications. It compares a handwritten dispatcher with a 14-key catalog and
a catalog containing 214 keyed rules: 200 distinct always-false unrelated
rules before the same 14 target rules. It checks every result and checksum
outside timing, warms both paths, alternates their order across 25 rounds, and
prints each raw sample as CSV:

```sh
mkdir -p /Volumes/FlashCard/specification-core-rs-static/benchmark-artifacts
CARGO_TARGET_DIR=/Volumes/FlashCard/specification-core-rs-static \
  cargo bench -p specification-core-macros --bench static_decisions \
  > /Volumes/FlashCard/specification-core-rs-static/benchmark-artifacts/static-decisions.csv \
  2> /Volumes/FlashCard/specification-core-rs-static/benchmark-artifacts/static-decisions-median.txt
```

Record the compiler, target, machine, power mode, and command beside both
files. The handwritten dispatcher is a purpose-built comparator; these
synthetic measurements do not establish behavior for an application's real
catalog.

CI runs this benchmark on Linux and macOS and validates its samples with
`python3 scripts/check_indexed_performance.py target/benchmark-artifacts/indexed-decisions.csv`.
The indexed 214-rule median must fit `2 × indexed 14-rule median + 20 ns +
6 × combined MAD`. This is a coarse same-host regression budget, not a
significance test. Raw CSV, environment and JSON summaries are job artifacts;
deterministic invocation-count tests establish which buckets are skipped.
