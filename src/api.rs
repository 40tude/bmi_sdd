// Rust guideline compliant 2026-02-16

//! Axum handlers, JSON request/response types, and input validation.

use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::domain;

// --- T008: JSON request/response types ---

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

// --- T009: BMI handler ---

/// Handles `POST /api/bmi`: calculates BMI and returns WHO category.
///
/// # Errors
///
/// Returns 422 when JSON deserialization fails or domain validation rejects the input.
pub(crate) async fn bmi_handler(
    ValidJson(req): ValidJson<BmiRequest>,
) -> Result<Json<BmiResponse>, (StatusCode, Json<ErrorResponse>)> {
    // T015: InvalidWeight, InvalidHeight, NonFiniteResult all map to 422
    domain::calculate_bmi(req.weight_kg, req.height_m)
        .map(|result| {
            Json(BmiResponse {
                bmi: result.bmi,
                category: result.category.to_string(),
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
