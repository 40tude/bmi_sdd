<!--
Sync Impact Report
===================
Version change: N/A (initial) -> 1.0.0
Added principles:
  - I. Domain Purity
  - II. Test-Driven Development
  - III. Clean Layering
  - IV. Observability
  - V. Simplicity
Added sections:
  - Tech Stack & Constraints
  - Development Workflow
  - Governance
Removed sections: none
Templates requiring updates:
  - .specify/templates/plan-template.md: no update needed (Constitution Check is dynamic)
  - .specify/templates/spec-template.md: no update needed (generic template)
  - .specify/templates/tasks-template.md: no update needed (generic template)
  - .specify/templates/checklist-template.md: no update needed (generic template)
Follow-up TODOs: none
-->

# BMI Calculator Constitution

## Core Principles

### I. Domain Purity

All BMI calculation and classification logic MUST reside in a pure domain
module with zero I/O and zero framework dependencies. Domain functions
accept primitives and return domain types. This ensures testability and
portability independent of the web framework.

### II. Test-Driven Development

TDD is mandatory. Red-Green-Refactor cycle strictly enforced:
- Unit tests for domain logic MUST cover calculation accuracy, WHO
  category boundaries, and rejection of invalid inputs (zero, negative).
- Integration tests MUST exercise API endpoints via HTTP using Reqwest:
  valid requests, invalid inputs, missing fields.
- All tests MUST pass via `cargo test` with no manual setup.

### III. Clean Layering

The application MUST maintain three distinct layers:
- **Domain**: pure functions, no dependencies on Axum or Serde.
- **API**: Axum handlers, JSON request/response types (Serde), input
  validation, error mapping to HTTP status codes.
- **UI**: embedded HTML page served by Axum, Bootstrap via CDN,
  fetch-based form submission to `/api/bmi`.

Cross-layer imports follow one direction: API depends on Domain; UI is
independent static content. Domain MUST NOT import API or UI types.

### IV. Observability

All errors MUST be logged server-side using `tracing`. The application
MUST initialize `tracing-subscriber` at startup with configurable log
level (via Clap CLI flag). Structured logging is preferred over ad-hoc
print statements.

### V. Simplicity

YAGNI strictly enforced. The application is stateless -- no database, no
persistence, no authentication, no API versioning. Only build what the
spec requires. Reject complexity that serves hypothetical future needs.

## Tech Stack & Constraints

- **Language**: Rust (edition 2024)
- **Async runtime**: Tokio
- **Web framework**: Axum
- **Serialization**: Serde + serde_json
- **Error handling**: thiserror (domain errors), anyhow (app-level)
- **Logging**: tracing + tracing-subscriber
- **CLI**: Clap (--port flag, log level flag)
- **Test HTTP client**: Reqwest (integration tests)
- **UI**: Bootstrap CDN, embedded HTML
- **Deployment**: Heroku via Rust buildpack; PORT env var overrides
  --port CLI flag; Procfile required
- **Platform**: Windows 11 dev environment; deploy target is Linux
  (Heroku)

## Development Workflow

- Single `main` branch; feature work on short-lived branches.
- Commit messages: `<action>: <what>` format, max 50 chars, US English.
- `cargo fmt` and `cargo clippy` MUST pass before commit.
- `cargo test` MUST pass before merge.
- No files with names starting with `nul` or `null` (Windows reserved).

## Governance

This constitution is the authoritative reference for project decisions.
All code reviews and PRs MUST verify compliance with these principles.

**Amendment procedure**: Any principle change requires documentation of
rationale, version bump per semver, and update of this file.

**Version**: 1.0.0 | **Ratified**: 2026-03-04 | **Last Amended**: 2026-03-04
