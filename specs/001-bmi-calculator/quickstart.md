# Quickstart: BMI Calculator

**Feature**: 001-bmi-calculator | **Date**: 2026-03-04

## Prerequisites

- Rust toolchain (edition 2024)
- Cargo

## Build and Run

```bash
cargo build
cargo run -- --port 3000 --log-level info
```

Or with env var (overrides --port):
```bash
PORT=8080 cargo run
```

## Test

```bash
cargo test
```

Runs both unit tests (domain logic) and integration tests (HTTP round-trips).

## Manual Verification

### Calculate BMI (valid input)

```bash
curl -X POST http://localhost:3000/api/bmi \
  -H "Content-Type: application/json" \
  -d '{"weight_kg": 70.0, "height_m": 1.75}'
```

Expected: `200 OK` with `{"bmi": 22.9, "category": "Normal"}`

### Invalid input

```bash
curl -X POST http://localhost:3000/api/bmi \
  -H "Content-Type: application/json" \
  -d '{"weight_kg": 0.0, "height_m": 1.75}'
```

Expected: `422` with `{"error": "weight_kg must be positive"}`

### Health check

```bash
curl http://localhost:3000/health
```

Expected: `200 OK`

### Web UI

Open `http://localhost:3000/` in a browser. Fill in weight and height, submit.
Result appears inline without page reload.

## Project Structure

```
src/
  main.rs      -- CLI, tracing init, server startup
  lib.rs       -- Module declarations, router builder
  domain.rs    -- Pure BMI calculation (no deps)
  api.rs       -- Axum handlers, JSON types, validation
  ui.rs        -- Embedded HTML page handler
tests/
  integration/
    api_test.rs -- Reqwest-based HTTP tests
```
