// Rust guideline compliant 2026-02-16

//! Axum handlers, JSON request/response types, shared state, and input validation.

use axum::{
    Json,
    extract::{FromRequest, Request, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::domain;

// --- T004: History entry and shared application state ---

/// One successful BMI calculation recorded in server-wide history.
#[derive(Debug, Clone, Serialize)]
pub struct HistoryEntry {
    /// Weight submitted by the user (kg).
    pub weight_kg: f64,
    /// Height submitted by the user (m).
    pub height_m: f64,
    /// Calculated BMI rounded to 1 decimal place.
    pub bmi: f64,
    /// WHO category string.
    pub category: String,
    /// RFC 3339 UTC timestamp of the calculation.
    pub timestamp: String,
}

/// Maximum entries retained in server-wide BMI history (FIFO eviction).
///
/// Ephemeral -- cleared on server restart. Increasing this value raises
/// peak memory proportionally (one `HistoryEntry` per slot).
const MAX_HISTORY_ENTRIES: usize = 5;

// --- T005: Shared application state ---

/// Shared server state injected into Axum handlers via `State<AppState>`.
///
/// Wraps a `Mutex`-guarded `BoundedHistory` behind an `Arc` so all clones
/// share the same underlying collection. Cloning is cheap (ref-count increment).
#[derive(Debug)]
pub struct AppState {
    /// Thread-safe, bounded BMI calculation history.
    pub history: std::sync::Arc<std::sync::Mutex<domain::BoundedHistory<HistoryEntry>>>,
}

impl AppState {
    /// Creates a fresh `AppState` with an empty history bounded to `MAX_HISTORY_ENTRIES`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            history: std::sync::Arc::new(std::sync::Mutex::new(domain::BoundedHistory::new(
                MAX_HISTORY_ENTRIES,
            ))),
        }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            history: std::sync::Arc::clone(&self.history),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// --- JSON request/response types ---

/// JSON request body for `POST /api/bmi`.
#[derive(Debug, Deserialize)]
pub struct BmiRequest {
    /// Weight in kilograms (must be positive).
    pub weight_kg: f64,
    /// Height in meters (must be positive).
    pub height_m: f64,
}

/// JSON success response for `POST /api/bmi`.
#[derive(Debug, Serialize)]
pub struct BmiResponse {
    /// Calculated BMI rounded to 1 decimal place.
    pub bmi: f64,
    /// WHO category string.
    pub category: String,
    /// Server-wide history: last `MAX_HISTORY_ENTRIES` calculations, newest first.
    pub history: Vec<HistoryEntry>,
}

/// JSON error response body.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Human-readable error description.
    pub error: String,
}

// --- T014: Custom extractor remapping JSON rejection 400 -> 422 ---

/// Wraps [`Json<T>`] and maps deserialization failures to 422 Unprocessable Entity.
pub(crate) struct ValidJson<T>(T);

impl<T, S> FromRequest<S> for ValidJson<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) => Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse {
                    error: rejection.body_text(),
                }),
            )),
        }
    }
}

// --- T008: BMI handler (with shared history state) ---

/// Handles `POST /api/bmi`: calculates BMI, records it in history, returns both.
///
/// # Errors
///
/// Returns 422 when JSON deserialization fails or domain validation rejects the input.
/// Failed requests do not modify the history.
pub(crate) async fn bmi_handler(
    State(state): State<AppState>,
    ValidJson(req): ValidJson<BmiRequest>,
) -> Result<Json<BmiResponse>, (StatusCode, Json<ErrorResponse>)> {
    tracing::debug!(weight = %req.weight_kg, height = %req.height_m, "bmi request received");
    domain::calculate_bmi(req.weight_kg, req.height_m)
        .map(|result| {
            let entry = HistoryEntry {
                weight_kg: req.weight_kg,
                height_m: req.height_m,
                bmi: result.bmi,
                category: result.category.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            };
            let history: Vec<HistoryEntry> = {
                let mut guard = state.history.lock().unwrap();
                // T015: log before push so eviction check uses pre-push length
                if guard.len() >= MAX_HISTORY_ENTRIES {
                    tracing::debug!("history eviction triggered");
                }
                guard.push(entry);
                tracing::debug!(history.len = guard.len(), "history entry added");
                guard.entries().cloned().collect()
            };
            Json(BmiResponse {
                bmi: result.bmi,
                category: result.category.to_string(),
                history,
            })
        })
        .map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })
}

// --- T021: Health check handler ---

/// Handles `GET /health`: returns 200 OK for load balancers and monitoring.
pub(crate) async fn health_handler() -> StatusCode {
    StatusCode::OK
}
