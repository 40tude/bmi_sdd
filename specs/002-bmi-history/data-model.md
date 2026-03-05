# Data Model: BMI Calculation History

## Entities

### HistoryEntry (api.rs)

Represents one successful BMI calculation recorded in history.

| Field | Type | Constraints | Source |
|-------|------|-------------|--------|
| weight_kg | f64 | Positive (validated before entry creation) | Request payload |
| height_m | f64 | Positive (validated before entry creation) | Request payload |
| bmi | f64 | Finite, rounded to 1 decimal | `domain::calculate_bmi` result |
| category | String | One of: Underweight, Normal, Overweight, Obese | `BmiCategory::to_string()` |
| timestamp | String | ISO 8601 / RFC 3339 UTC | `chrono::Utc::now().to_rfc3339()` |

**Derives**: `Debug`, `Clone`, `Serialize`
**Location**: `src/api.rs` (needs Serde for JSON serialization)

### BoundedHistory\<T\> (domain.rs)

Generic bounded FIFO collection. Newest entries at front, evicts oldest when capacity exceeded.

| Field | Type | Constraints |
|-------|------|-------------|
| deque | VecDeque\<T\> | Preallocated with `with_capacity(max_size)` |
| max_size | usize | Immutable after construction. Set to 5 for BMI history. |

**Derives**: `Debug`
**Location**: `src/domain.rs` (no Serde, no I/O)

**Operations**:
- `new(max_size)` -- create empty collection with given cap
- `push(item)` -- insert at front; if `len > max_size`, evict oldest (pop back)
- `entries()` -- return `&[T]` or iterator, newest-first
- `len()` -- current entry count
- `is_empty()` -- true when empty

### AppState (api.rs)

Shared application state injected into Axum handlers via `State<AppState>`.

| Field | Type | Notes |
|-------|------|-------|
| history | Arc\<Mutex\<BoundedHistory\<HistoryEntry\>\>\> | `std::sync::Arc<std::sync::Mutex<...>>`. Arc enables Clone; lock held briefly (no .await inside). |

**Derives**: `Debug`, `Clone` (Arc is Clone; all clones share the same Mutex)
**Location**: `src/api.rs` (not lib.rs -- avoids circular import: lib.rs imports api::AppState)

## Relationships

```text
AppState (api.rs)
  +-- history: Arc<Mutex<BoundedHistory<HistoryEntry>>>
        |
        +-- BoundedHistory<T> (domain.rs) -- generic, framework-free
        |     +-- deque: VecDeque<T>
        |     +-- max_size: usize
        |
        +-- HistoryEntry (api.rs) -- concrete T, has Serialize
              +-- weight_kg, height_m, bmi, category, timestamp
```

## State Transitions

```text
Empty History (0 entries)
  -- push --> Partial History (1..4 entries)
  -- push --> Full History (5 entries)
  -- push --> Full + Evict (oldest removed, still 5 entries)
```

## Validation Rules

- History entries are created ONLY after successful `domain::calculate_bmi()` call
- Failed validations (zero weight, negative height, etc.) produce zero change in history
- No direct mutation of history entries after creation (immutable once pushed)
- Concurrent access serialized via `Mutex::lock()`
