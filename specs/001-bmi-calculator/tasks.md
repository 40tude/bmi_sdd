# Tasks: BMI Calculator

**Input**: Design documents from `/specs/001-bmi-calculator/`
**Prerequisites**: plan.md, spec.md, data-model.md, research.md, contracts/api-bmi.md, contracts/api-health.md

**Tests**: Included - spec constitution mandates TDD (SC-003, SC-004); unit tests for domain, integration tests via Reqwest.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: User story label (US1-US4) -- setup/foundational/polish phases have no story label
- Exact file paths included in every description

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Cargo project initialization and directory structure

- [ ] T001 Create Cargo.toml with all production deps (axum, tokio, serde+serde_json, clap, tracing, tracing-subscriber, thiserror, anyhow) and dev-deps (reqwest with json feature, tokio test runtime) in Cargo.toml
- [ ] T002 [P] Create src/lib.rs with module declarations (`mod domain; mod api; mod ui;`) and stub `pub fn build_router() -> axum::Router` returning empty Router in src/lib.rs
- [ ] T003 [P] Create tests/integration/api_test.rs with test module skeleton and shared `async fn spawn_server() -> String` helper that binds to 127.0.0.1:0 and returns base URL in tests/integration/api_test.rs

---

## Phase 2: Foundational - Domain Layer

**Purpose**: Pure domain types and logic that ALL user stories depend on

**CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Write unit tests for `calculate_bmi`: all 4 WHO boundary values, zero weight, negative height, NaN/Infinity result -- tests MUST fail before T006 in src/domain.rs
- [ ] T005 Define `BmiCategory` enum (Underweight/Normal/Overweight/Obese) with `Display`, `BmiResult` struct (bmi: f64, category: BmiCategory), and `DomainError` enum (InvalidWeight, InvalidHeight, NonFiniteResult) via thiserror in src/domain.rs
- [ ] T006 Implement `calculate_bmi(weight_kg: f64, height_m: f64) -> Result<BmiResult, DomainError>` and private `classify(bmi: f64) -> BmiCategory` in src/domain.rs -- make T004 tests pass

**Checkpoint**: `cargo test --lib` passes all domain unit tests

---

## Phase 3: User Story 1 - Calculate BMI via API (Priority: P1) -- MVP

**Goal**: POST /api/bmi accepts valid weight+height JSON and returns BMI value + WHO category

**Independent Test**: `curl -X POST http://localhost:3000/api/bmi -H 'Content-Type: application/json' -d '{"weight_kg":70.0,"height_m":1.75}'` returns `200 {"bmi":22.9,"category":"Normal"}`

### Tests for User Story 1

> Write FIRST -- MUST FAIL before implementation

- [ ] T007 [P] [US1] Write integration tests for POST /api/bmi success: 70kg/1.75m -> 22.9 Normal, 50kg/1.80m -> 15.4 Underweight, 90kg/1.70m -> 31.1 Obese in tests/integration/api_test.rs

### Implementation for User Story 1

- [ ] T008 [P] [US1] Define `BmiRequest` (Deserialize: weight_kg f64, height_m f64), `BmiResponse` (Serialize: bmi f64, category String), `ErrorResponse` (Serialize: error String) in src/api.rs
- [ ] T009 [US1] Implement `async fn bmi_handler(Json(req): Json<BmiRequest>) -> Result<Json<BmiResponse>, (StatusCode, Json<ErrorResponse>)>` calling `domain::calculate_bmi` and mapping result in src/api.rs
- [ ] T010 [US1] Register `POST /api/bmi` route with `bmi_handler` in `build_router()` in src/lib.rs
- [ ] T011 [P] [US1] Implement `#[derive(Parser)] struct Cli` with `--port u16` (default 3000) and `--log-level` args; add PORT env var override (env var wins over --port) in src/main.rs
- [ ] T012 [US1] Implement `#[tokio::main] async fn main()` with tracing-subscriber init (EnvFilter from log-level arg) and `axum::serve(TcpListener::bind(addr).await?, build_router()).await?` in src/main.rs

**Checkpoint**: `cargo test` passes T007 integration tests; `cargo run` serves POST /api/bmi returning correct BMI and category

---

## Phase 4: User Story 2 - Reject Invalid Input (Priority: P1)

**Goal**: POST /api/bmi returns 422 with descriptive error for zero/negative/missing/empty inputs -- no 500s

**Independent Test**: `curl -X POST http://localhost:3000/api/bmi -d '{"weight_kg":0.0,"height_m":1.75}'` returns `422 {"error":"weight_kg must be positive"}`

### Tests for User Story 2

> Write FIRST -- MUST FAIL before implementation

- [ ] T013 [P] [US2] Write integration tests for 422 error paths: zero weight, negative height, missing field, empty body -- all expect 422 with `{"error": "..."}` in tests/integration/api_test.rs

### Implementation for User Story 2

- [ ] T014 [US2] Implement custom Axum rejection handler `async fn handle_json_rejection(err: JsonRejection) -> (StatusCode, Json<ErrorResponse>)` remapping 400 to 422 for deserialization errors; register it via `Router::layer(DefaultBodyLimit::disable()).route_layer(...)` or `axum::extract::rejection` handling in src/api.rs
- [ ] T015 [US2] Verify `DomainError::InvalidWeight`, `DomainError::InvalidHeight`, and `DomainError::NonFiniteResult` all map to `StatusCode::UNPROCESSABLE_ENTITY` with correct error strings in `bmi_handler` in src/api.rs

**Checkpoint**: `cargo test` passes T013 integration tests; all error cases return 422 with correct JSON

---

## Phase 5: User Story 3 - Calculate BMI via Web UI (Priority: P2)

**Goal**: Root URL serves Bootstrap-styled HTML form; form submits via fetch to /api/bmi and displays result inline without page reload

**Independent Test**: Open `http://localhost:3000/` in browser, enter weight 70 and height 1.75, click submit, see "BMI: 22.9 - Normal" appear without page reload

### Tests for User Story 3

> Write FIRST -- MUST FAIL before implementation

- [ ] T016 [P] [US3] Write integration test for GET / returning 200 with `Content-Type: text/html` and body containing `<form` in tests/integration/api_test.rs

### Implementation for User Story 3

- [ ] T017 [P] [US3] Implement embedded HTML page as `pub const INDEX_HTML: &str` -- Bootstrap 5 CDN, form with weight_kg and height_m inputs, fetch POST to /api/bmi on submit, display result/error div inline in src/ui.rs
- [ ] T018 [US3] Implement `async fn root_handler() -> axum::response::Html<&'static str>` returning `Html(INDEX_HTML)` in src/ui.rs
- [ ] T019 [US3] Register `GET /` route with `root_handler` in `build_router()` in src/lib.rs

**Checkpoint**: `cargo test` passes T016; browser shows form and displays BMI result inline on submit

---

## Phase 6: User Story 4 - Health Check (Priority: P2)

**Goal**: GET /health returns 200 OK for monitoring systems and load balancers

**Independent Test**: `curl http://localhost:3000/health` returns `200 OK`

### Tests for User Story 4

> Write FIRST -- MUST FAIL before implementation

- [ ] T020 [P] [US4] Write integration test for GET /health returning 200 status code in tests/integration/api_test.rs

### Implementation for User Story 4

- [ ] T021 [P] [US4] Implement `async fn health_handler() -> StatusCode` returning `StatusCode::OK` in src/api.rs
- [ ] T022 [US4] Register `GET /health` route with `health_handler` in `build_router()` in src/lib.rs

**Checkpoint**: `cargo test` passes T020; `curl /health` returns 200

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and end-to-end verification across all stories

- [ ] T023 Extract `pub fn resolve_port(cli_port: u16) -> u16` from main() that reads PORT env var and falls back to cli_port; add #[cfg(test)] unit test asserting PORT overrides cli_port in src/main.rs
- [ ] T024 [P] Run `cargo test` and confirm all unit + integration tests pass with zero warnings
- [ ] T025 [P] Execute all manual verification steps from quickstart.md against running server (BMI calculation, invalid input, health check, web UI); time POST /api/bmi with `curl -w "%{time_total}\n"` and confirm < 1s (SC-001)
- [ ] T026 [P] Run `cargo fmt --check` and `cargo clippy -- -D warnings` with zero errors/warnings in project root
- [ ] T027 [P] Create `Procfile` in project root with `web: ./target/release/<package-name>` (replace package-name with Cargo.toml `name` field)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies -- start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 -- blocks all user stories
- **User Stories (Phases 3-6)**: All depend on Phase 2 completion
  - US1 (Phase 3): No dependency on other user stories
  - US2 (Phase 4): Depends on US1 handler (T009) -- same endpoint, adds rejection handling
  - US3 (Phase 5): Depends on Phase 2 only -- independent of US1/US2
  - US4 (Phase 6): Depends on Phase 2 only -- fully independent
- **Polish (Phase 7)**: Depends on all user stories complete

### User Story Dependencies

- **US1 (P1)**: Starts after Phase 2 -- no story dependencies
- **US2 (P1)**: Starts after US1 handler (T009) -- extends the same handler
- **US3 (P2)**: Starts after Phase 2 -- fully independent; can run in parallel with US1
- **US4 (P2)**: Starts after Phase 2 -- fully independent; can run in parallel with US1/US3

### Within Each User Story

- Test task(s) MUST be written first and FAIL before implementation
- Types/structs before handler implementation
- Handler before route registration
- Route registration before integration test can pass

### Parallel Opportunities

- Phase 1: T002 and T003 can run in parallel
- Phase 3: T007 (tests), T008 (types), T011 (CLI) can all run in parallel
- Phase 4: T013 (tests) can run in parallel with T014 (rejection handler)
- Phase 5: T016 (test) and T017 (HTML) can run in parallel
- Phase 6: T020 (test) and T021 (health handler) can run in parallel
- Phase 7: T024, T025, T026, and T027 can all run in parallel

---

## Parallel Example: User Story 1

```
# Launch in parallel (different files, no mutual dependencies):
T007: Write integration tests for /api/bmi success in tests/integration/api_test.rs
T008: Define BmiRequest/BmiResponse/ErrorResponse types in src/api.rs
T011: Implement Clap CLI struct + PORT env var override in src/main.rs

# Sequential after T008:
T009: Implement bmi_handler in src/api.rs

# Sequential after T009:
T010: Register POST /api/bmi in build_router() in src/lib.rs

# Sequential after T011:
T012: Implement main() with tracing + axum::serve in src/main.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Domain Layer (CRITICAL -- blocks all stories)
3. Complete Phase 3: User Story 1 (API success path)
4. **STOP and VALIDATE**: `cargo test`, `curl /api/bmi` with valid inputs
5. Deploy/demo if ready

### Incremental Delivery

1. Phase 1 + Phase 2 -> Domain layer ready
2. Phase 3 (US1) -> POST /api/bmi success path -- MVP
3. Phase 4 (US2) -> Input validation and error handling
4. Phase 5 (US3) -> Web UI
5. Phase 6 (US4) -> Health check + deployment readiness
6. Phase 7 -> Polish and final validation

### Parallel Team Strategy

With multiple developers, once Phase 2 is done:

- Developer A: US1 (Phase 3) then US2 (Phase 4)
- Developer B: US3 (Phase 5) -- fully independent
- Developer C: US4 (Phase 6) -- fully independent

---

## Notes

- [P] tasks operate on different files with no mutual dependencies -- safe to parallelize
- [USn] label maps each task to its user story for traceability
- Domain unit tests live in `#[cfg(test)] mod tests` at bottom of src/domain.rs
- Integration tests use OS-assigned port (127.0.0.1:0) to avoid conflicts
- Each checkpoint validates the story independently before moving to next priority
- Commit after each checkpoint or logical group
