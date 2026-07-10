# Architecture Baseline

The workspace contains a synchronous and runtime-neutral async core library,
plus optional procedural-macro and versioned-serialization crates. Each
integration may depend on the core, but the dependency direction is never
reversed.

See the normative [architecture baseline](https://github.com/SoundBlaster/specification-core-rs/blob/main/docs/architecture/README.md)
and [project charter](https://github.com/SoundBlaster/specification-core-rs/blob/main/docs/project-charter.md)
for the current design contract.
