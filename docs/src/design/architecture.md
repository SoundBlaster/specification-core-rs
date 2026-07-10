# Architecture Baseline

The initial implementation is one synchronous core library crate. Optional
procedural macros, serialization support, and FFI facades are separate future
crates that may depend on the core but never reverse that dependency direction.

See the normative [architecture baseline](https://github.com/SoundBlaster/specification-core-rs/blob/main/docs/architecture/README.md)
and [project charter](https://github.com/SoundBlaster/specification-core-rs/blob/main/docs/project-charter.md)
for the current design contract.
