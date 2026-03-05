// Rust guideline compliant 2026-02-16

pub mod api;
pub mod domain;
pub mod ui;

/// Builds the application router with all registered routes and shared state.
///
/// Each call creates a fresh `AppState`, so integration tests get isolated
/// history per server instance. Return type is `Router<()>` -- Axum converts
/// `Router<AppState>` to `Router<()>` via `.with_state()`.
pub fn build_router() -> axum::Router {
    let state = api::AppState::new();
    axum::Router::new()
        .route("/", axum::routing::get(ui::root_handler))
        .route("/api/bmi", axum::routing::post(api::bmi_handler))
        .route("/health", axum::routing::get(api::health_handler))
        .with_state(state)
}
