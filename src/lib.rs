// Rust guideline compliant 2026-02-16

pub mod api;
pub mod domain;
pub mod ui;

/// Builds the application router with all registered routes.
pub fn build_router() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(ui::root_handler))
        .route("/api/bmi", axum::routing::post(api::bmi_handler))
        .route("/health", axum::routing::get(api::health_handler))
}
