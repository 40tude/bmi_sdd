# Implementation Plan: BMI Calculator

**Branch**: `001-bmi-calculator` | **Date**: 2026-03-04 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-bmi-calculator/spec.md`

## Summary

Stateless BMI calculator web service in Rust/Axum. Pure domain module computes
BMI and WHO classification. JSON API at `/api/bmi` accepts weight/height, returns
BMI + category. Bootstrap web form at root URL submits via fetch. Health endpoint
at `/health`. CLI configures port and log level. TDD with unit + integration tests.

## Technical Context

**Language/Version**: Rust edition 2024
**Primary Dependencies**: Axum, Tokio, Serde + serde_json, Clap, tracing + tracing-subscriber, thiserror, anyhow
**Storage**: N/A (stateless, no persistence)
**Testing**: `cargo test` (unit), Reqwest + Tokio test runtime (integration)
**Target Platform**: Windows 11 (dev), Linux (Heroku deploy)
**Project Type**: web-service
**Performance Goals**: <1s response for BMI calculation (SC-001)
**Constraints**: Stateless, no database, no auth, no API versioning
**Scale/Scope**: 1 API endpoint, 1 health endpoint, 1 HTML page, ~500 LOC

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Domain Purity | PASS | Domain module (`src/domain.rs`) has zero I/O, zero framework deps. Accepts primitives, returns domain types. |
| II. TDD | PASS | Unit tests cover all 4 WHO boundaries + invalid inputs. Integration tests exercise all API paths via Reqwest. `cargo test` with no manual setup. |
| III. Clean Layering | PASS | Three layers: Domain (pure functions), API (Axum handlers + Serde types), UI (embedded HTML). One-way deps: API -> Domain. Domain imports nothing from API/UI. |
| IV. Observability | PASS | tracing + tracing-subscriber initialized at startup. Log level configurable via CLI. All errors logged server-side. |
| V. Simplicity | PASS | No database, no auth, no API versioning, no persistence. Bootstrap from CDN. Only what spec requires. |

**GATE RESULT: ALL PASS** -- proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-bmi-calculator/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── api-bmi.md       # POST /api/bmi contract
│   └── api-health.md    # GET /health contract
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Entry point: CLI parsing, tracing init, server start
├── domain.rs            # Pure BMI calculation + classification (no deps)
├── api.rs               # Axum handlers, JSON request/response types, validation
├── ui.rs                # Embedded HTML string, handler to serve root page
└── lib.rs               # Module declarations, app builder (Router)

tests/
└── integration/
    └── api_test.rs      # Reqwest-based HTTP round-trip tests
```

**Structure Decision**: Single crate, flat module layout. Constitution requires
3 layers (Domain/API/UI) but project scope is small enough for flat files in
`src/`. Integration tests in `tests/` directory use Cargo's built-in test
discovery. No need for multi-crate workspace.

## Complexity Tracking

> No constitution violations. Table left empty.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (none) | -- | -- |

## Post-Design Constitution Re-Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Domain Purity | PASS | `data-model.md` confirms domain types have no Serde/Axum deps. API layer wraps domain types with Serde. |
| II. TDD | PASS | Test strategy confirmed: unit tests for domain boundaries, integration tests via Reqwest with random port. |
| III. Clean Layering | PASS | 4 source files enforce separation: domain.rs (pure), api.rs (Axum+Serde), ui.rs (HTML), lib.rs (router). One-way deps confirmed. |
| IV. Observability | PASS | Research confirms tracing+EnvFilter pattern. CLI log-level flag feeds into subscriber init. |
| V. Simplicity | PASS | No extras introduced. Flat module layout, single crate, no workspace. |

**POST-DESIGN GATE: ALL PASS**

## Known Issues

- `update-agent-context.ps1` script fails: `New-TemporaryFile` cmdlet not found in PowerShell. CLAUDE.md not auto-generated. Non-blocking; can be created manually or script fixed.
