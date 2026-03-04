# Contract: POST /api/bmi

## Endpoint

`POST /api/bmi`

## Request

**Content-Type**: `application/json`

```json
{
  "weight_kg": 70.0,
  "height_m": 1.75
}
```

| Field | Type | Required | Constraints |
|-------|------|----------|-------------|
| weight_kg | f64 | yes | Must be positive (> 0) |
| height_m | f64 | yes | Must be positive (> 0) |

## Success Response

**Status**: `200 OK`
**Content-Type**: `application/json`

```json
{
  "bmi": 22.9,
  "category": "Normal"
}
```

| Field | Type | Description |
|-------|------|-------------|
| bmi | f64 | BMI rounded to 1 decimal place |
| category | string | One of: "Underweight", "Normal", "Overweight", "Obese" |

### Category Boundaries (WHO)

| Category | Range |
|----------|-------|
| Underweight | BMI < 18.5 |
| Normal | 18.5 <= BMI <= 24.9 |
| Overweight | 25.0 <= BMI <= 29.9 |
| Obese | BMI >= 30.0 |

## Error Responses

### 422 Unprocessable Entity -- Domain Validation

Returned when fields are present but values are invalid.

```json
{
  "error": "weight_kg must be positive"
}
```

| Trigger | Error Message |
|---------|---------------|
| weight_kg <= 0 | "weight_kg must be positive" |
| height_m <= 0 | "height_m must be positive" |
| Computed BMI is Infinity/NaN | "computed BMI is not finite" |

### 422 Unprocessable Entity -- Deserialization Failure

Returned when JSON is malformed, fields are missing, or types are wrong.

```json
{
  "error": "Failed to deserialize the JSON body: missing field `height_m`"
}
```

### 400 Bad Request

Returned when request body is not valid JSON at all (empty body, broken syntax).
This is Axum's default behavior for `Json<T>` extractor failure.

Note: Per spec, empty body and missing fields should return 422. A custom
rejection handler must be implemented to remap Axum's default 400 to 422 for
deserialization errors.

## Examples

### Normal BMI

```
POST /api/bmi
{"weight_kg": 70.0, "height_m": 1.75}
-> 200 {"bmi": 22.9, "category": "Normal"}
```

### Underweight

```
POST /api/bmi
{"weight_kg": 50.0, "height_m": 1.80}
-> 200 {"bmi": 15.4, "category": "Underweight"}
```

### Obese

```
POST /api/bmi
{"weight_kg": 90.0, "height_m": 1.70}
-> 200 {"bmi": 31.1, "category": "Obese"}
```

### Invalid weight

```
POST /api/bmi
{"weight_kg": 0.0, "height_m": 1.75}
-> 422 {"error": "weight_kg must be positive"}
```
