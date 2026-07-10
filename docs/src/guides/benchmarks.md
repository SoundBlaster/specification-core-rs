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
