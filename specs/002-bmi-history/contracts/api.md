# API Contract: BMI History

## POST /api/bmi

Calculate BMI and record in history. Returns result with full history.

### Request

```json
{
  "weight_kg": 70.0,
  "height_m": 1.75
}
```

### Success Response (200 OK)

```json
{
  "bmi": 22.9,
  "category": "Normal",
  "history": [
    {
      "weight_kg": 70.0,
      "height_m": 1.75,
      "bmi": 22.9,
      "category": "Normal",
      "timestamp": "2026-03-05T14:30:45.123456+00:00"
    }
  ]
}
```

**Fields**:
- `bmi` (f64): calculated BMI, rounded to 1 decimal
- `category` (string): one of "Underweight", "Normal", "Overweight", "Obese"
- `history` (array): 0-5 entries, newest first. Each entry contains weight_kg, height_m, bmi, category, timestamp (RFC 3339)

**Invariants**:
- `history.length` is between 0 and 5 inclusive
- `history[0]` is always the entry just calculated (newest)
- History is shared across all clients (server-wide state)

### Error Response (422 Unprocessable Entity)

```json
{
  "error": "weight_kg must be positive"
}
```

No history entry created on validation failure. History unchanged.

## GET /health

Unchanged. Returns 200 OK with empty body.

## GET /

Serves HTML page. Updated to render history table from API response.

### History Table Rendering (UI)

After successful calculation, the UI displays a table below the result:

| # | Weight (kg) | Height (m) | BMI | Category | Time |
|---|-------------|------------|-----|----------|------|
| 1 | 70.0 | 1.75 | 22.9 | Normal | 2026-03-05T14:30:45... |

- Table visible only after first successful calculation
- Ordered newest-first (matches API response order)
- Max 5 rows

## Breaking Changes

- `BmiResponse` gains new `history` field (additive, backward-compatible for consumers ignoring unknown fields)
