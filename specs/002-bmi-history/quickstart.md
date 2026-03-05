# Quickstart: BMI History Feature

## Prerequisites

- Rust 2024 toolchain
- Existing codebase on branch `002-bmi-history`

## New Dependency

```toml
# Cargo.toml [dependencies]
chrono = { version = "0.4", features = ["serde"] }
```

## Build & Test

```powershell
cargo build
cargo test
cargo clippy
```

## Verify History Behavior

```powershell
# Start server
cargo run

# Submit calculations (separate terminal)
$body = '{"weight_kg":70,"height_m":1.75}'
Invoke-RestMethod -Uri http://localhost:3000/api/bmi -Method POST -Body $body -ContentType 'application/json'

# Response now includes history array
# { "bmi": 22.9, "category": "Normal", "history": [...] }
```

## Key Files to Modify

| File | Change |
|------|--------|
| `Cargo.toml` | Add `chrono` dependency |
| `src/domain.rs` | Add `BoundedHistory<T>` struct + unit tests |
| `src/api.rs` | Add `HistoryEntry`, `AppState`, update `bmi_handler` |
| `src/lib.rs` | Update `build_router()` to create/inject state |
| `src/ui.rs` | Add history table to HTML |
| `tests/api_test.rs` | Add history integration tests |
