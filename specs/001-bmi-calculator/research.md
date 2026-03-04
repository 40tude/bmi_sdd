# Research: BMI Calculator

**Feature**: 001-bmi-calculator | **Date**: 2026-03-04

No NEEDS CLARIFICATION items existed in Technical Context. Research focused on
best practices for chosen technologies.

## 1. Axum JSON API Patterns

**Decision**: Use Axum 0.8 with `Json<T>` extractor for request/response.

**Rationale**: Axum's extractor model provides clean, type-safe deserialization.
`Json<BmiInput>` auto-rejects malformed JSON with 400. For domain validation
errors, return `(StatusCode::UNPROCESSABLE_ENTITY, Json<ErrorBody>)` tuple.

**Key patterns**:
- Handler signature: `async fn handler(Json(input): Json<T>) -> Result<Json<R>, (StatusCode, Json<E>)>`
- Serve HTML via `Html<&'static str>` return type (auto-sets Content-Type)
- Router: `Router::new().route("/", get(index)).route("/api/bmi", post(calculate))`
- Server: `axum::serve(TcpListener::bind(addr).await?, app).await?`

**Alternatives considered**: Actix-web (heavier, actor model unnecessary),
Warp (filter composition less intuitive for simple REST). Constitution mandates
Axum.

## 2. Integration Testing with Random Port

**Decision**: Bind to `127.0.0.1:0`, extract port from `local_addr()`, spawn
server in background with `tokio::spawn`.

**Rationale**: OS-assigned port avoids conflicts in parallel test execution.
Extract `build_router() -> Router` as shared function for both `main` and tests.

**Key pattern**:
```rust
let listener = TcpListener::bind("127.0.0.1:0").await?;
let port = listener.local_addr()?.port();
tokio::spawn(axum::serve(listener, app));
// reqwest::Client hits http://127.0.0.1:{port}/...
```

**Alternatives considered**: `tower::ServiceExt` for in-process testing (no real
HTTP round-trip, less realistic). Reqwest chosen per constitution.

## 3. Clap CLI Configuration

**Decision**: Use Clap 4.x derive API with `#[derive(Parser)]`. PORT env var
override implemented manually post-parse.

**Rationale**: Clap's `env` attribute gives precedence to CLI flag over env var,
but spec requires env var to take precedence. Manual post-parse override is the
simplest solution.

**Key pattern**:
```rust
#[derive(Parser)]
struct Cli {
    #[arg(long, default_value_t = 3000)]
    port: u16,
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}
// After parse: if env PORT is set, override args.port
```

**Alternatives considered**: Skip Clap entirely and use `std::env::args()`
manually (too much boilerplate). Using Clap's `env` attribute alone (wrong
precedence order per FR-009).

## 4. Tracing Setup

**Decision**: `tracing` + `tracing-subscriber` with `EnvFilter` for configurable
log level from CLI.

**Rationale**: EnvFilter parses level strings ("debug", "info", etc.) directly.
Registry pattern allows composable layers.

**Key pattern**:
```rust
tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::try_new(&cli.log_level).unwrap_or_else(|_| EnvFilter::new("info")))
    .init();
```

**Alternatives considered**: `env_logger` (less structured, no span support).
Constitution mandates `tracing`.

## 5. Domain Purity Enforcement

**Decision**: `src/domain.rs` contains only pure functions. No `use axum::*`,
no `use serde::*`, no I/O. Accepts `f64` primitives, returns domain types.

**Rationale**: Constitution Principle I mandates zero-dep domain module.
Serde derives live on API-layer types that wrap domain types.

**Key pattern**:
- `domain::calculate_bmi(weight_kg: f64, height_m: f64) -> Result<BmiResult, DomainError>`
- `domain::BmiResult { bmi: f64, category: BmiCategory }`
- `domain::BmiCategory` enum with `Display` impl for string output
- `domain::DomainError` via `thiserror` (no Serde, no Axum)

**Alternatives considered**: Putting Serde derives on domain types (violates
Principle I). Single module for everything (violates Principle III).

## 6. Error Handling Strategy

**Decision**: Two-tier error handling: `thiserror` for domain errors, `anyhow`
for app-level errors. API layer maps `DomainError` to HTTP status codes.

**Rationale**: Domain errors are typed and predictable (invalid input). App-level
errors (bind failure, I/O) are unexpected. Constitution mandates both crates.

**Key mapping**:
- `DomainError::InvalidInput(msg)` -> 422 + `{"error": msg}`
- `DomainError::NonFiniteResult` -> 422 + `{"error": "computed BMI is not finite"}`
- Serde deserialization failure -> 422 (custom rejection handler) + `{"error": msg}`
