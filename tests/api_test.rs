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

// --- T010: US1 - history appears in response after single calculation ---

#[tokio::test]
async fn history_appears_after_calculation() {
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
    let history = body["history"]
        .as_array()
        .expect("history must be an array");
    assert_eq!(
        history.len(),
        1,
        "one calculation must produce one history entry"
    );
    assert_eq!(history[0]["weight_kg"].as_f64().unwrap(), 70.0);
    assert_eq!(history[0]["height_m"].as_f64().unwrap(), 1.75);
    assert!(history[0]["bmi"].is_number(), "bmi must be a number");
    assert!(
        history[0]["category"]
            .as_str()
            .is_some_and(|s| !s.is_empty()),
        "category must be a non-empty string"
    );
    assert!(
        history[0]["timestamp"]
            .as_str()
            .is_some_and(|s| !s.is_empty()),
        "timestamp must be a non-empty string"
    );
}

// --- T011: US2 - FIFO eviction at capacity ---

#[tokio::test]
async fn history_evicts_oldest_at_capacity() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    // Post 6 requests; weights 60.0..=65.0 (1 kg apart)
    let mut last_body = serde_json::Value::Null;
    for i in 0u8..6 {
        let weight = 60.0 + f64::from(i);
        let resp = client
            .post(format!("{base}/api/bmi"))
            .json(&serde_json::json!({"weight_kg": weight, "height_m": 1.75}))
            .send()
            .await
            .expect("request failed");
        assert_eq!(resp.status(), 200);
        last_body = resp.json().await.expect("body parse failed");
    }
    let history = last_body["history"]
        .as_array()
        .expect("history must be an array");
    assert_eq!(history.len(), 5, "history must never exceed 5 entries");
    // The first weight (60.0) must have been evicted
    let weights: Vec<f64> = history
        .iter()
        .map(|e| e["weight_kg"].as_f64().unwrap())
        .collect();
    assert!(
        !weights.contains(&60.0),
        "oldest entry (60.0 kg) must have been evicted"
    );
    assert_eq!(
        history[0]["weight_kg"].as_f64().unwrap(),
        65.0,
        "newest entry must be at index 0"
    );
}

// --- T012: US2 - ordering preserved after eviction ---

#[tokio::test]
async fn history_ordering_preserved_after_eviction() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    // Fill history to capacity (5 entries)
    for i in 0u8..5 {
        let weight = 70.0 + f64::from(i);
        client
            .post(format!("{base}/api/bmi"))
            .json(&serde_json::json!({"weight_kg": weight, "height_m": 1.75}))
            .send()
            .await
            .expect("request failed");
    }
    // 6th request triggers eviction; the 5th entry (74.0 kg) becomes index 1
    let resp = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 80.0, "height_m": 1.75}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    let history = body["history"].as_array().expect("history must be array");
    assert_eq!(history.len(), 5);
    assert_eq!(
        history[0]["weight_kg"].as_f64().unwrap(),
        80.0,
        "newest entry (80.0 kg) must be at index 0 after eviction"
    );
    assert_eq!(
        history[1]["weight_kg"].as_f64().unwrap(),
        74.0,
        "previously-newest entry (74.0 kg) must be at index 1"
    );
}

// --- T013: US3 - history shared across clients ---

#[tokio::test]
async fn history_shared_across_clients() {
    let base = spawn_server().await;
    let client_a = reqwest::Client::new();
    let client_b = reqwest::Client::new();
    // First client sends a calculation
    client_a
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 70.0, "height_m": 1.75}))
        .send()
        .await
        .expect("client_a request failed");
    // Second client sends a different calculation
    let resp = client_b
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 80.0, "height_m": 1.80}))
        .send()
        .await
        .expect("client_b request failed");
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    let history = body["history"].as_array().expect("history must be array");
    assert!(
        history.len() >= 2,
        "shared history must contain both calculations"
    );
    let weights: Vec<f64> = history
        .iter()
        .map(|e| e["weight_kg"].as_f64().unwrap())
        .collect();
    assert!(
        weights.contains(&70.0),
        "client_a entry (70.0 kg) must be in shared history"
    );
    assert!(
        weights.contains(&80.0),
        "client_b entry (80.0 kg) must be in shared history"
    );
}

// --- T014: Polish - failed validation does not pollute history ---

#[tokio::test]
async fn failed_validation_does_not_add_history() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();
    // Invalid request -- must not add to history
    let invalid = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": -1.0, "height_m": 1.75}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(invalid.status(), 422);
    // Valid request -- history must have exactly one entry
    let resp = client
        .post(format!("{base}/api/bmi"))
        .json(&serde_json::json!({"weight_kg": 70.0, "height_m": 1.75}))
        .send()
        .await
        .expect("request failed");
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("body parse failed");
    let history = body["history"].as_array().expect("history must be array");
    assert_eq!(
        history.len(),
        1,
        "failed request must not have added an entry to history"
    );
}
