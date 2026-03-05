# Tasks: BMI Calculation History

**Input**: Design documents from `/specs/002-bmi-history/`
**Prerequisites**: plan.md, spec.md, data-model.md, contracts/api.md, research.md, quickstart.md

**Tests**: Included -- plan.md constitution check confirms TDD (unit tests for BoundedHistory + integration tests via reqwest).

**Organization**: Tasks grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Exact file paths included in all descriptions

## Path Conventions

Single project: `src/`, `tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add new dependency required by all subsequent phases

- [ ] T001 Add `chrono = { version = "0.4", features = ["serde"] }` to `[dependencies]` in `Cargo.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures and shared state that all user stories depend on

**CRITICAL**: No user story work can begin until this phase is complete

- [ ] T002 Write `#[cfg(test)]` unit test module for `BoundedHistory<T>` in `src/domain.rs`: cover `new()` creates empty collection, `push()` adds entry at front, `entries()` returns newest-first, `len()`, `is_empty()`, and FIFO eviction when push exceeds max_size -- tests MUST fail (compile error) before T003
- [ ] T003 Implement `BoundedHistory<T>` struct in `src/domain.rs`: fields `deque: VecDeque<T>` and `max_size: usize`; impl `new(max_size: usize) -> Self` (with_capacity), `push(&mut self, item: T)` (push_front + pop_back when len > max_size), `entries(&self) -> impl Iterator<Item = &T>` (via deque.iter(), newest-first), `len(&self) -> usize`, `is_empty(&self) -> bool`; derive `Debug`
- [ ] T004 [P] Add `HistoryEntry` struct in `src/api.rs`: fields `weight_kg: f64`, `height_m: f64`, `bmi: f64`, `category: String`, `timestamp: String`; derive `Debug, Clone, Serialize`
- [ ] T005 Add `AppState` struct in `src/api.rs`: field `history: std::sync::Arc<std::sync::Mutex<BoundedHistory<HistoryEntry>>>`; derive `Debug, Clone`; impl `AppState { pub fn new() -> Self { Self { history: std::sync::Arc::new(std::sync::Mutex::new(BoundedHistory::new(5))) } } }`
- [ ] T006 Update `build_router()` in `src/lib.rs`: create `let state = AppState::new()`, attach via `.with_state(state)` -- return type stays `Router` (Router<()> after with_state); import `crate::api::AppState`

**Checkpoint**: Foundation ready -- BoundedHistory unit tests pass, AppState injectable

---

## Phase 3: User Story 1 - View History After Calculation (Priority: P1) -- MVP

**Goal**: POST /api/bmi returns `history` array in response; HTML page renders history table below result

**Independent Test**: POST `{"weight_kg":70,"height_m":1.75}`, assert `response["history"]` is an array of length 1 with correct fields

### Implementation for User Story 1

- [ ] T007 [US1] Update `BmiResponse` struct in `src/api.rs` to add `history: Vec<HistoryEntry>` field (Serialize already derived)
- [ ] T008 [US1] Update `bmi_handler` in `src/api.rs`: add `State(state): State<AppState>` parameter; after successful `calculate_bmi`, create `HistoryEntry { weight_kg, height_m, bmi: rounded_bmi, category: category.to_string(), timestamp: chrono::Utc::now().to_rfc3339() }`; lock mutex, push entry, collect `state.history.lock().unwrap().entries().cloned().collect()` into `BmiResponse.history`; `state` is `State<AppState>` (AppState clones cheaply via Arc); add `use chrono::Utc`
- [ ] T009 [US1] Update HTML template in `src/ui.rs`: add Bootstrap history table (columns: #, Weight kg, Height m, BMI, Category, Time) populated from `data.history` JS array; hide table when `history.length === 0`, show after successful calculation; insert below existing result display
- [ ] T010 [US1] Add integration test `history_appears_after_calculation` in `tests/api_test.rs`: POST `{"weight_kg":70.0,"height_m":1.75}`, parse response as `serde_json::Value`, assert `response["history"].as_array().unwrap().len() == 1`, assert `history[0]["weight_kg"] == 70.0`, `history[0]["height_m"] == 1.75`, `history[0]["bmi"]` is a number, `history[0]["category"]` is a string, `history[0]["timestamp"]` is non-empty

**Checkpoint**: Story 1 complete -- single calculation shows history in both API response and HTML page

---

## Phase 4: User Story 2 - FIFO Eviction at Capacity (Priority: P1)

**Goal**: History never exceeds 5 entries; the 6th calculation evicts the oldest

**Independent Test**: POST 6 BMIs with distinct weights, assert response.history has exactly 5 entries and the 1st entry's weight is absent

### Implementation for User Story 2

- [ ] T011 [US2] Add integration test `history_evicts_oldest_at_capacity` in `tests/api_test.rs`: POST 6 requests with `weight_kg` values 60.0 through 65.0; after 6th POST assert `response["history"].as_array().unwrap().len() == 5`; assert no entry in history has `weight_kg == 60.0`; assert `history[0]["weight_kg"] == 65.0`
- [ ] T012 [US2] Add integration test `history_ordering_preserved_after_eviction` in `tests/api_test.rs`: POST 5 BMIs, assert history len == 5; POST 6th BMI, assert newest is at `history[0]` and the previously-newest weight is at `history[1]`

**Checkpoint**: Stories 1+2 complete -- bounded FIFO behavior verified end-to-end

---

## Phase 5: User Story 3 - Shared History Across Users (Priority: P2)

**Goal**: Calculations from any client appear in history for all subsequent clients

**Independent Test**: Two sequential POST requests with different weights; second response's history contains both entries

### Implementation for User Story 3

- [ ] T013 [US3] Add integration test `history_shared_across_clients` in `tests/api_test.rs`: issue two sequential POST requests with distinct `weight_kg` values (e.g., 70.0 and 80.0); assert second response's `history` array has length >= 2; assert both weight values are present in the history entries

**Checkpoint**: All 3 user stories complete and independently verified

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Edge case coverage, observability, and final validation

- [ ] T014 [P] Add integration test `failed_validation_does_not_add_history` in `tests/api_test.rs`: POST `{"weight_kg":-1.0,"height_m":1.75}`, assert status 422; then POST valid body and assert `response["history"].as_array().unwrap().len() == 1` (failed request added nothing)
- [ ] T015 [P] Add `tracing::debug!` calls in `bmi_handler` in `src/api.rs`: one log after push (e.g., `"history entry added, len={}"`) and one when eviction occurs (len exceeded max before push)
- [ ] T016 Run `cargo fmt -- --check` and fix any formatting issues; then run `cargo clippy -- -D warnings` and fix all reported warnings in `src/domain.rs`, `src/api.rs`, `src/lib.rs`, `src/ui.rs`
- [ ] T017 Run `cargo test` and confirm all existing tests plus new history tests pass with zero failures in `tests/api_test.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies -- start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 -- BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2 -- most implementation work
- **US2 (Phase 4)**: Depends on Phase 2 + T008 (handler must push to history before eviction can be tested)
- **US3 (Phase 5)**: Depends on Phase 2 + T008 (shared state wired up in handler)
- **Polish (Phase 6)**: After all story phases complete

### User Story Dependencies

- **US1 (P1)**: Requires Foundational -- wires handler + response + UI (primary implementation)
- **US2 (P1)**: Requires Foundational + US1 T008 -- FIFO logic is in BoundedHistory (foundational) but exercised via handler
- **US3 (P2)**: Requires Foundational + US1 T008 -- AppState shared via build_router() (already wired)

### Within US1 (Phase 3)

- T007 (BmiResponse field): can start as soon as Foundational completes
- T008 (handler update): depends on T007 (needs history field in BmiResponse)
- T009 (UI update): depends on T008 (server must return history for UI to render it)
- T010 (integration test): depends on T008 (handler must return history for test to pass)

### Parallel Opportunities

- T004 (HistoryEntry in api.rs) is parallel with T002+T003 (BoundedHistory in domain.rs) -- different files
- T014 (edge case test) is parallel with T015 (tracing) -- different files
- T011 and T012 (US2 tests) can be written in any order -- both in api_test.rs but independent test functions

---

## Parallel Example: Foundational Phase

```text
# Start these simultaneously (different files):
Stream A: T002 -> T003  (unit tests then BoundedHistory impl in src/domain.rs)
Stream B: T004          (HistoryEntry struct in src/api.rs)

# Sequential after both streams complete:
T005 (AppState in src/api.rs) -> T006 (build_router update in src/lib.rs)
```

## Parallel Example: Polish Phase

```text
# Start simultaneously:
Task T014: edge case test in tests/api_test.rs
Task T015: tracing logs in src/api.rs

# Sequential after both:
T016 (cargo clippy) -> T017 (cargo test)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001)
2. Complete Phase 2: Foundational (T002-T006) -- CRITICAL, blocks all stories
3. Complete Phase 3: User Story 1 (T007-T010)
4. **STOP and VALIDATE**: `POST /api/bmi` returns `history` array; HTML page shows history table
5. US1 demo ready -- deploy/share if appropriate

### Incremental Delivery

1. Setup + Foundational -- BoundedHistory unit tested and AppState injected
2. US1 -- API + HTML history visible (MVP)
3. US2 -- FIFO eviction cap verified end-to-end
4. US3 -- Shared state verified across clients
5. Polish -- edge cases, tracing, clippy, full test run

---

## Notes

- T002 unit tests MUST NOT compile until T003 implements the struct (true TDD gate)
- `entries()` returns `impl Iterator<Item = &T>` via `self.deque.iter()` (read-only, no `make_contiguous` needed); handler collects into `Vec<HistoryEntry>` inside the mutex lock scope
- `build_router()` return type stays `Router` after `.with_state()` -- Axum consumes `Router<AppState>` and returns `Router<()>`; existing tests remain compatible
- `AppState` placed in `src/api.rs` (not lib.rs) per plan.md to avoid circular module imports (`lib.rs` imports `api::AppState`; `api.rs` imports `domain::BoundedHistory`)
- Existing tests in `tests/api_test.rs` parse responses via `serde_json::Value` -- the new `history` field in `BmiResponse` is additive and backward-compatible
