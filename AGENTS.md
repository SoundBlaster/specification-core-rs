# Repository Instructions for Agents

## Scope and Workflow

- Read the active task in `SPECS/INPROGRESS/` and its archived context before making changes.
- Follow `SPECS/COMMANDS/FLOW.md` for tasks executed through Flow.
- Treat `SPECS/Workplan.md` as the task source of truth.
- Keep changes scoped to one task and stage explicit paths only.
- Record durable architecture choices in `docs/adr/`; do not leave them only in chat, commit messages, or review comments.

## Project Direction

- Follow the product and compatibility policy in [`docs/project-charter.md`](docs/project-charter.md).
- Prefer idiomatic Rust over source-level Swift API parity.
- Do not add async runtimes, FFI, procedural macros, serialization, global state, or platform integrations without an approved task and ADR.

## Documentation and Validation

- Write public API documentation, examples, and contributor-facing material in English.
- Update relevant rustdoc, guides, and executable examples with every public behavior change.
- Run the quality gates configured in `.flow/params.yaml` when they are available; record unavailable gates and their reason in validation reports.
- Do not claim tests, coverage, platform support, or performance results that were not verified in the current task.

## Managed Files

- Do not hand-edit Flow-managed `SPECS/COMMANDS/`, `SPECS/ROLES/`, `.agents/skills/flow-*`, or `plugins/flow/` unless the task explicitly updates Flow itself.
- Keep project-specific configuration in `.flow/`, project documentation in `docs/`, and task artifacts in the user-owned Flow directories.

## Security and Licensing

- Never add secrets, tokens, credentials, or private keys to the repository.
- Follow [`SECURITY.md`](SECURITY.md) for vulnerability handling and [`docs/contributing/governance.md`](docs/contributing/governance.md) for third-party and licensing rules.
