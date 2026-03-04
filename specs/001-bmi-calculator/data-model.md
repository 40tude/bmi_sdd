# Data Model: BMI Calculator

**Feature**: 001-bmi-calculator | **Date**: 2026-03-04

## Domain Layer (`src/domain.rs`)

No Serde, no Axum, no I/O dependencies.

### BmiCategory (enum)

| Variant | Condition | Display String |
|---------|-----------|----------------|
| Underweight | BMI < 18.5 | "Underweight" |
| Normal | 18.5 <= BMI <= 24.9 | "Normal" |
| Overweight | 25.0 <= BMI <= 29.9 | "Overweight" |
| Obese | BMI >= 30.0 | "Obese" |

Derives: `Debug`, `Clone`, `Copy`, `PartialEq`.
Implements: `std::fmt::Display` (returns category string).

### BmiResult (struct)

| Field | Type | Description |
|-------|------|-------------|
| bmi | f64 | Calculated BMI, rounded to 1 decimal |
| category | BmiCategory | WHO classification |

Derives: `Debug`, `Clone`, `PartialEq`.

### DomainError (enum, thiserror)

| Variant | Message | Trigger |
|---------|---------|---------|
| InvalidWeight(String) | "weight_kg must be positive" | weight <= 0 |
| InvalidHeight(String) | "height_m must be positive" | height <= 0 |
| NonFiniteResult | "computed BMI is not finite" | BMI is Infinity or NaN |

Derives: `Debug`.
Implements: `std::fmt::Display` via `#[error(...)]`, `std::error::Error`.

### Functions

```
calculate_bmi(weight_kg: f64, height_m: f64) -> Result<BmiResult, DomainError>
classify(bmi: f64) -> BmiCategory
```

`calculate_bmi` validates inputs, computes `weight_kg / height_m^2`, rounds to
1 decimal, classifies, and returns `BmiResult`. `classify` is a pure helper
called internally.

## API Layer (`src/api.rs`)

### BmiRequest (struct, Serde)

| Field | Type | JSON Key | Validation |
|-------|------|----------|------------|
| weight_kg | f64 | "weight_kg" | Delegated to domain |
| height_m | f64 | "height_m" | Delegated to domain |

Derives: `Deserialize`.

### BmiResponse (struct, Serde)

| Field | Type | JSON Key |
|-------|------|----------|
| bmi | f64 | "bmi" |
| category | String | "category" |

Derives: `Serialize`.
Built from `domain::BmiResult` via conversion (category uses `Display`).

### ErrorResponse (struct, Serde)

| Field | Type | JSON Key |
|-------|------|----------|
| error | String | "error" |

Derives: `Serialize`.

## Relationships

```
BmiRequest --(extract values)--> domain::calculate_bmi()
domain::BmiResult --(convert)--> BmiResponse
domain::DomainError --(map to 422)--> ErrorResponse
Serde rejection --(map to 422)--> ErrorResponse
```

## State Transitions

None. Application is stateless. Each request is independent.
