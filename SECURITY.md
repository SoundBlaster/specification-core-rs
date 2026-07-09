# Security Policy

## Reporting a Vulnerability

Please report suspected vulnerabilities privately through a [GitHub security advisory](https://github.com/SoundBlaster/specification-core-rs/security/advisories/new). Do not open a public issue, pull request, or discussion containing exploit details, credentials, private keys, access tokens, or sensitive user data.

Include a clear description, affected version or commit, reproduction steps, impact assessment, and any proposed mitigation. Reporters may use a minimal proof of concept only when it does not expose third-party systems or data.

## Supported Versions

The project has not published a stable crate release yet. Security fixes apply to the current default branch until a release support policy is adopted in a future ADR.

## Response Expectations

Maintainers will acknowledge a private report when practical, assess impact, and coordinate a fix before public disclosure. Timing depends on severity, reproduction quality, and release availability; no specific response SLA is promised at this stage.

## Security Boundaries

This library is intended to evaluate application rules. It does not by itself provide authentication, authorization, sandboxing, cryptographic key handling, or secure storage. Consumers remain responsible for validating untrusted input and enforcing their own security boundaries.
