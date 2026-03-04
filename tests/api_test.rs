// Rust guideline compliant 2026-02-16

//! Integration tests for the BMI calculator HTTP API.
//!
//! Each test spawns a real server on a random OS-assigned port and sends
//! HTTP requests via reqwest, validating full round-trip behavior.

/// Spawns the application server bound to a random port and returns the base URL.
async fn spawn_server() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("failed to bind listener");
    let port = listener
        .local_addr()
        .expect("failed to get local addr")
        .port();
    tokio::spawn(async move {
        axum::serve(listener, bmi_sdd::build_router())
            .await
            .expect("server failed");
    });
    format!("http://127.0.0.1:{port}")
}

// --- US1: Calculate BMI via API (T007) ---

#[tokio::test]
async fn test_bmi_normal() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 70.0, "height_m": 1.75}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    assert_eq!(body["bmi"].as_f64().unwrap(), 22.9);
    assert_eq!(body["category"], "Normal");
}

#[tokio::test]
async fn test_bmi_underweight() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 50.0, "height_m": 1.80}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    assert_eq!(body["bmi"].as_f64().unwrap(), 15.4);
    assert_eq!(body["category"], "Underweight");
}

#[tokio::test]
async fn test_bmi_obese() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 90.0, "height_m": 1.70}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    assert_eq!(body["bmi"].as_f64().unwrap(), 31.1);
    assert_eq!(body["category"], "Obese");
}

// --- US2: Reject invalid input (T013) ---

#[tokio::test]
async fn test_bmi_zero_weight_returns_422() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 0.0, "height_m": 1.75}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 422);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    assert_eq!(body["error"], "weight_kg must be positive");
}

#[tokio::test]
async fn test_bmi_negative_height_returns_422() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 70.0, "height_m": -1.75}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 422);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    assert_eq!(body["error"], "height_m must be positive");
}

#[tokio::test]
async fn test_bmi_missing_field_returns_422() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base}/api/bmi"))
        .header("content-type", "application/json")
        .body(r#"{"weight_kg": 70.0}"#)
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 422);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    let error_msg = body["error"].as_str().expect("error field missing");
    assert!(
        error_msg.contains("height_m"),
        "expected error about height_m, got: {error_msg}"
    );
}

#[tokio::test]
async fn test_bmi_empty_body_returns_422() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base}/api/bmi"))
        .header("content-type", "application/json")
        .body("")
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 422);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    assert!(
        !body["error"].as_str().unwrap_or("").is_empty(),
        "expected non-empty error message"
    );
}

// --- US3: Web UI (T016) ---

#[tokio::test]
async fn test_root_returns_html_with_form() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(&base).send().await.expect("request failed");
    assert_eq!(resp.status(), 200);
    let content_type = resp
        .headers()
        .get("content-type")
        .expect("missing content-type")
        .to_str()
        .expect("invalid header value");
    assert!(
        content_type.contains("text/html"),
        "expected text/html, got: {content_type}"
    );
    let body = resp.text().await.expect("body read failed");
    assert!(body.contains("<form"), "expected <form in body");
}

// --- US4: Health check (T020) ---

#[tokio::test]
async fn test_health_returns_200() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{base}/health"))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 200);
}
