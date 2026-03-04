# Feature Specification: BMI Calculator

**Feature Branch**: `001-bmi-calculator`
**Created**: 2026-03-04
**Status**: Draft
**Input**: User description: "BMI Calculator web application with API, domain logic, and Bootstrap UI"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Calculate BMI via API (Priority: P1)

A client application sends weight (kg) and height (m) as JSON to the
BMI endpoint. The system calculates BMI, classifies it into a WHO
category, and returns the result as JSON.

**Why this priority**: Core value proposition. Every other feature
depends on the calculation endpoint working correctly.

**Independent Test**: Send a POST request to `/api/bmi` with valid
weight and height; verify the response contains correct BMI value and
category string.

**Acceptance Scenarios**:

1. **Given** a running server, **When** POST `/api/bmi` with
   `{"weight_kg": 70.0, "height_m": 1.75}`, **Then** response is 200
   with `{"bmi": 22.9, "category": "Normal"}`
2. **Given** a running server, **When** POST `/api/bmi` with
   `{"weight_kg": 50.0, "height_m": 1.80}`, **Then** response is 200
   with `{"bmi": 15.4, "category": "Underweight"}`
3. **Given** a running server, **When** POST `/api/bmi` with
   `{"weight_kg": 90.0, "height_m": 1.70}`, **Then** response is 200
   with `{"bmi": 31.1, "category": "Obese"}`

---

### User Story 2 - Reject Invalid Input (Priority: P1)

When a client submits invalid data (zero weight, negative height,
missing fields, empty body), the system returns a 422 response with a
meaningful error message. No server crash or 500 error.

**Why this priority**: Input validation is essential for reliability and
shares P1 with the calculation endpoint.

**Independent Test**: Send malformed requests and verify 422 responses
with descriptive error strings.

**Acceptance Scenarios**:

1. **Given** a running server, **When** POST `/api/bmi` with
   `{"weight_kg": 0.0, "height_m": 1.75}`, **Then** response is 422
   with `{"error": "weight_kg must be positive"}`
2. **Given** a running server, **When** POST `/api/bmi` with
   `{"weight_kg": 70.0, "height_m": -1.0}`, **Then** response is 422
   with `{"error": "height_m must be positive"}`
3. **Given** a running server, **When** POST `/api/bmi` with missing
   `height_m` field, **Then** response is 422 with error message
4. **Given** a running server, **When** POST `/api/bmi` with empty
   body, **Then** response is 422 with error message

---

### User Story 3 - Calculate BMI via Web UI (Priority: P2)

A user opens the application in a browser, sees a form with weight and
height fields, submits values, and sees the calculated BMI and category
displayed on the same page without a full reload.

**Why this priority**: Provides a user-friendly interface on top of the
API. Depends on US1 and US2 being complete.

**Independent Test**: Open the root URL in a browser, fill in weight
and height, click submit, and verify the result appears on screen.

**Acceptance Scenarios**:

1. **Given** a user visits the root URL, **When** the page loads,
   **Then** a form with weight (kg) and height (m) fields and a submit
   button is displayed
2. **Given** the form is displayed, **When** the user enters valid
   weight and height and submits, **Then** the BMI value and category
   are shown on the page
3. **Given** the form is displayed, **When** the user enters invalid
   data and submits, **Then** an error message is displayed

---

### User Story 4 - Health Check (Priority: P2)

An external monitoring system or load balancer sends GET `/health` to
verify the server is running.

**Why this priority**: Required for deployment readiness but does not
deliver direct end-user value.

**Independent Test**: Send GET `/health` and verify 200 OK response.

**Acceptance Scenarios**:

1. **Given** a running server, **When** GET `/health`, **Then**
   response is 200 OK

---

### Edge Cases

- Weight is zero: MUST return 422 with "weight_kg must be positive"
- Height is zero: MUST return 422 with "height_m must be positive"
- Weight is negative: MUST return 422 with "weight_kg must be positive"
- Height is negative: MUST return 422 with "height_m must be positive"
- Missing JSON field: MUST return 422 with descriptive error
- Empty request body: MUST return 422 with descriptive error
- Extremely large weight/height: accepted (no upper bound) but MUST
  return 422 if computed BMI is Infinity or NaN

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept weight in kilograms and height in
  meters via JSON POST to `/api/bmi`
- **FR-002**: System MUST calculate BMI as weight / height^2, rounded
  to 1 decimal place
- **FR-003**: System MUST classify BMI into WHO categories:
  Underweight (< 18.5), Normal (18.5-24.9), Overweight (25.0-29.9),
  Obese (>= 30.0)
- **FR-004**: System MUST return 200 with `{"bmi": f64, "category":
  "string"}` on valid input
- **FR-005**: System MUST return 422 with `{"error": "string"}` on
  invalid input, with messages identifying which field is invalid
- **FR-006**: System MUST serve a web page at the root URL with a
  Bootstrap-styled form for BMI calculation
- **FR-007**: Web form MUST submit to `/api/bmi` via fetch (no page
  reload) and display results inline
- **FR-008**: System MUST respond to GET `/health` with 200 OK
- **FR-009**: System MUST accept `--port` CLI flag and PORT env var
  for port configuration; env var takes precedence; default port is 3000
- **FR-010**: System MUST log all errors server-side with configurable
  log level

### Key Entities

- **BmiInput**: weight in kg (f64) and height in meters (f64);
  represents the raw user input before validation
- **BmiResult**: calculated BMI value (f64, 1 decimal) and WHO
  category classification
- **BmiCategory**: one of Underweight, Normal, Overweight, Obese;
  maps to display strings for JSON output

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can calculate BMI and receive a categorized result
  in under 1 second via both API and web UI
- **SC-002**: All invalid inputs (zero, negative, missing fields)
  produce clear error messages without server errors
- **SC-003**: Domain calculation tests cover all 4 WHO category
  boundaries and both invalid-input variants
- **SC-004**: Integration tests verify all API success and error
  paths via HTTP round-trips
- **SC-005**: Application starts with configurable port and log level
  from command line

### Assumptions

- BMI formula: weight_kg / (height_m^2), standard WHO formula
- Rounding: 1 decimal place (e.g., 22.857 becomes 22.9)
- Category boundaries follow WHO standard exactly (inclusive/exclusive
  as specified in FR-003)
- No upper-bound validation on weight or height (per non-goals)
- Application is stateless; each request is independent
- Bootstrap served from CDN, not bundled locally
- Web framework: Axum (Tokio-based, tower ecosystem)

## Clarifications

### Session 2026-03-04

- Q: Which Rust web framework should be used? → A: Axum (Tokio-based, tower ecosystem)
- Q: Default port when neither --port nor PORT provided? → A: 3000
- Q: How to handle extreme values producing Infinity/NaN BMI? → A: Reject with 422
