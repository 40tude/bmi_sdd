// Rust guideline compliant 2026-02-16

//! Embedded HTML page served at the root URL.

use axum::response::Html;

/// Bootstrap-styled BMI calculator page with inline fetch-based result display.
///
/// Served as a static `&str` to avoid any filesystem dependency at runtime.
pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>BMI Calculator</title>
  <link rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"
    integrity="sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH"
    crossorigin="anonymous">
</head>
<body>
<div class="container mt-5" style="max-width:480px">
  <h1 class="mb-4">BMI Calculator</h1>
  <form id="bmi-form">
    <div class="mb-3">
      <label for="weight" class="form-label">Weight (kg)</label>
      <input type="number" step="0.1" min="0.1" class="form-control"
             id="weight" placeholder="e.g. 70.0" required>
    </div>
    <div class="mb-3">
      <label for="height" class="form-label">Height (m)</label>
      <input type="number" step="0.01" min="0.01" class="form-control"
             id="height" placeholder="e.g. 1.75" required>
    </div>
    <button type="submit" class="btn btn-primary w-100">Calculate</button>
  </form>
  <div id="result" class="mt-3"></div>
  <div id="history" class="mt-4"></div>
</div>
<script>
document.getElementById('bmi-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const weight_kg = parseFloat(document.getElementById('weight').value);
  const height_m  = parseFloat(document.getElementById('height').value);
  const resultDiv  = document.getElementById('result');
  const historyDiv = document.getElementById('history');
  try {
    const response = await fetch('/api/bmi', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ weight_kg, height_m }),
    });
    const data = await response.json();
    if (response.ok) {
      resultDiv.innerHTML =
        `<div class="alert alert-success">BMI: ${data.bmi} &mdash; ${data.category}</div>`;
      if (data.history && data.history.length > 0) {
        const catBadge = (cat) => {
          const cls = cat === 'Normal' ? 'success' :
                      cat === 'Obese'  ? 'danger'  : 'warning';
          return `<span class="badge text-bg-${cls}">${cat}</span>`;
        };
        const rows = data.history.map((entry, i) =>
          `<tr><td>${i + 1}</td>` +
          `<td class="text-end">${entry.weight_kg}</td>` +
          `<td class="text-end">${entry.height_m}</td>` +
          `<td class="text-end">${entry.bmi}</td>` +
          `<td>${catBadge(entry.category)}</td>` +
          `<td>${new Date(entry.timestamp).toLocaleString()}</td></tr>`
        ).join('');
        historyDiv.innerHTML =
          `<div class="card">` +
          `<div class="card-header fw-bold">Calculation History</div>` +
          `<div class="card-body p-0">` +
          `<table class="table table-striped table-hover table-bordered table-sm mb-0">` +
          `<thead class="table-dark"><tr><th>#</th>` +
          `<th class="text-end">Weight kg</th><th class="text-end">Height m</th>` +
          `<th class="text-end">BMI</th><th>Category</th><th>Time</th></tr></thead>` +
          `<tbody>${rows}</tbody></table>` +
          `</div></div>`;
      }
    } else {
      resultDiv.innerHTML =
        `<div class="alert alert-danger">Error: ${data.error}</div>`;
    }
  } catch (err) {
    resultDiv.innerHTML =
      `<div class="alert alert-danger">Error: ${err.message}</div>`;
  }
});
</script>
</body>
</html>
"#;

// --- T018: Root handler ---

/// Handles `GET /`: serves the embedded BMI calculator HTML page.
pub(crate) async fn root_handler() -> Html<&'static str> {
    Html(INDEX_HTML)
}
