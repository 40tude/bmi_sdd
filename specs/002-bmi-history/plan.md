# Implementation Plan: BMI Calculation History

**Branch**: `002-bmi-history` | **Date**: 2026-03-05 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-bmi-history/spec.md`

## Summary

Add server-wide BMI calculation history: last 5 entries stored in an in-memory `VecDeque` wrapped in `std::sync::Mutex`, shared via Axum `State<AppState>`. History returned in API response and rendered as HTML table. FIFO eviction at capacity. Ephemeral (lost on restart).

## Technical Context

**Language/Version**: Rust 2024 (edition 2024)
**Primary Dependencies**: Axum 0.8, tokio, serde, chrono 0.4 (new)
**Storage**: In-memory `VecDeque` (no persistence)
**Testing**: `cargo test` -- unit tests for bounded FIFO, integration tests via reqwest
**Target Platform**: Windows 11 dev, Linux deploy (Heroku)
**Project Type**: web-service
**Performance Goals**: N/A (5-entry collection, trivial ops)
**Constraints**: Max 5 history entries, ephemeral on restart
**Scale/Scope**: Single server, shared across all users, no auth

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Pre-Design | Post-Design | Notes |
|-----------|-----------|-------------|-------|
| I. Domain Purity | PASS | PASS | `BoundedHistory<T>` is generic, no I/O/Serde. Timestamp created at API layer. |
| II. TDD | PASS | PASS | Unit tests for bounded FIFO + integration tests for history API |
| III. Clean Layering | PASS | PASS | Domain: `BoundedHistory`. API: `HistoryEntry`, state, handlers. UI: table rendering. |
| IV. Observability | PASS | PASS | `tracing::debug` on history push/eviction |
| V. Simplicity | VIOLATION | VIOLATION | "Stateless" becomes stateful. See Complexity Tracking. |

## Project Structure

### Documentation (this feature)

```text
specs/002-bmi-history/
+-- plan.md
+-- research.md
+-- data-model.md
+-- quickstart.md
+-- contracts/
|   +-- api.md
+-- tasks.md             # Created by /speckit.tasks (not /speckit.plan)
```

### Source Code (repository root)

```text
src/
+-- domain.rs      # Add BoundedHistory<T> (generic bounded FIFO)
+-- api.rs         # Add HistoryEntry, AppState, update bmi_handler, add history
+-- lib.rs         # Update build_router() to create and inject AppState
+-- main.rs        # Unchanged
+-- ui.rs          # Update HTML with history table rendering

tests/
+-- api_test.rs    # Add history integration tests (stories 1-3 + edge cases)
```

**Structure Decision**: Single-crate flat module layout (unchanged from 001). No new modules -- `BoundedHistory<T>` in `domain.rs`, state management in `api.rs` + `lib.rs`. Keeps existing architecture intact with minimal surface area change.

## Key Design Decisions

### 1. Mutex Choice: `std::sync::Mutex` (not `tokio::sync::Mutex`)

Lock holds only for a `VecDeque::push_front` + conditional `pop_back` (microseconds, no `.await` inside lock). Per tokio docs: "it is ok and often preferred to use the ordinary Mutex from the standard library" for short, synchronous critical sections.

### 2. State Injection: `AppState` created inside `build_router()`

Each `build_router()` call creates a fresh `AppState`. Tests get isolated state per server instance. No signature change needed for `build_router()` -- keeps test compatibility simple.

### 3. Domain Purity: `HistoryEntry` in API layer

`HistoryEntry` needs `Serialize` + `Clone` (Serde dependency). Constitution forbids Serde in domain. So `HistoryEntry` lives in `api.rs`. `BoundedHistory<T>` in domain is generic and Serde-free.

### 4. Timestamp: `chrono::Utc::now().to_rfc3339()`

One-liner ISO 8601. ~500KB binary overhead is negligible vs Axum+tokio. Simpler than manual `SystemTime` formatting. Chrono is the ecosystem standard for Rust web services.

### 5. API Response: Add `history` field to `BmiResponse`

Backward-compatible addition (new field). Existing tests parse via `serde_json::Value` and check specific keys -- won't break. Satisfies FR-009 (history available via API).

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| In-memory state (violates "stateless" in Principle V) | Feature spec explicitly requires server-wide shared history via VecDeque | No alternative -- history IS state. Ephemeral on restart, no persistence added. |
