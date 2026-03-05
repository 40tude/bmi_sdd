# Research: BMI Calculation History

## R1: Axum 0.8 Shared Mutable State

**Decision**: Use `axum::extract::State<AppState>` with `Router::with_state()`.

**Rationale**: Axum 0.8's built-in state extraction is the canonical approach. State must implement `Clone` (cheap via inner `Arc` or by wrapping mutex directly). Handlers receive state via `State(state): State<AppState>` extractor.

**Alternatives considered**:
- Extension layer middleware: more complex, no benefit for simple shared state
- Global static: not testable, violates clean layering

## R2: Mutex Selection

**Decision**: `std::sync::Mutex` (standard library).

**Rationale**: Lock holds only for `VecDeque::push_front` + conditional `pop_back` -- microseconds, zero `.await` inside lock. Tokio docs recommend `std::sync::Mutex` for short synchronous critical sections. Avoids async mutex overhead and `.await` on lock acquisition.

**Alternatives considered**:
- `tokio::sync::Mutex`: needed only when holding lock across `.await` points. Overkill here.
- `tokio::sync::RwLock`: benefits many-reader scenarios. Our reads and writes are equally cheap and brief; added complexity not justified.
- `parking_lot::Mutex`: external dep for marginal perf gain on a 5-element collection. YAGNI.

## R3: Timestamp Approach

**Decision**: `chrono` crate with `Utc::now().to_rfc3339()`.

**Rationale**: One-liner ISO 8601 output. Native serde support. De facto standard in Rust web ecosystem. ~500KB binary overhead is negligible vs Axum+tokio baseline.

**Alternatives considered**:
- `time` crate: smaller community, less ecosystem support. No compelling advantage.
- `std::time::SystemTime`: no built-in ISO 8601 formatting -- requires ~20 lines of manual date math. Violates simplicity principle for zero-dependency purity.

## R4: Bounded FIFO Collection

**Decision**: `std::collections::VecDeque<T>` with `push_front` + conditional `pop_back`, wrapped in generic `BoundedHistory<T>` struct in `domain.rs`.

**Rationale**: `VecDeque::with_capacity(n)` does NOT enforce a hard cap (just preallocates). Wrapper struct encapsulates the max-size invariant and is independently unit-testable (TDD requirement). `push_front` places newest at index 0; natural iteration is newest-first (satisfies FR-004).

**Alternatives considered**:
- No wrapper (manage VecDeque directly in handler): loses unit-testable FIFO eviction logic. FIFO eviction is P1 user story -- deserves isolated tests.
- `push_front` + `truncate`: equivalent for single insertions but `pop_back` is more explicit about intent.
- Circular buffer crate: external dependency for a trivial 2-line operation. YAGNI.

## R5: Domain Purity for History Types

**Decision**: `BoundedHistory<T>` generic struct in `domain.rs` (no Serde). `HistoryEntry` with Serde derives in `api.rs`.

**Rationale**: Constitution mandates domain has no Serde dependency. `BoundedHistory<T>` is generic and framework-free. The concrete `HistoryEntry` type needs `Serialize` + `Clone` for JSON responses, so it belongs in the API layer.

**Alternatives considered**:
- `HistoryEntry` in domain without Serde, separate response type in API: extra mapping boilerplate for no gain (entry has same fields either way).
- New `history.rs` module: unnecessary file for two small types. Keeps module count minimal.
