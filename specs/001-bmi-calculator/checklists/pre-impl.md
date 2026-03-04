# Pre-Implementation Checklist: BMI Calculator

**Purpose**: Author self-review -- validate requirement quality across all domains before writing code
**Created**: 2026-03-04
**Feature**: [spec.md](../spec.md)

## Domain Calculation Requirements

- [ ] CHK001 - Is the rounding rule (half-up, half-even, truncate) specified for 1-decimal BMI rounding? [Clarity, Spec §FR-002]
- [ ] CHK002 - Does the WHO boundary notation (18.5-24.9, 25.0-29.9) unambiguously cover all f64 values -- is there a defined category for raw values between 24.9 and 25.0? [Ambiguity, Spec §FR-003]
- [ ] CHK003 - Is classification applied to the pre-rounded or post-rounded BMI value? [Ambiguity, Spec §FR-002, §FR-003]
- [ ] CHK004 - Is the NaN/Infinity domain error specified with its own error message, or does it reuse an existing error string? [Completeness, Spec §Edge Cases]

## API Contract Requirements

- [ ] CHK005 - Are Content-Type requirements specified for POST /api/bmi requests and responses? [Completeness, Gap]
- [ ] CHK006 - Is the exact `category` string value (e.g., "Normal" vs "normal" vs "NORMAL") specified for each WHO category in JSON responses? [Clarity, Spec §FR-004]
- [ ] CHK007 - Are HTTP status codes defined for non-422/200 failure modes (e.g., 405 Method Not Allowed, 415 Unsupported Media Type)? [Coverage, Gap]
- [ ] CHK008 - Is the exact error message string format specified for missing JSON fields (FR-005 names the field but not the message template)? [Clarity, Spec §FR-005]
- [ ] CHK009 - Is the numeric precision of `bmi` in the JSON response (1 decimal) specified as an API contract requirement or only as an internal formula detail? [Clarity, Spec §FR-004]

## Input Validation Requirements

- [ ] CHK010 - Are requirements defined for non-numeric input values (e.g., `"weight_kg": "abc"`)? [Coverage, Gap]
- [ ] CHK011 - Is validation priority specified when multiple fields are simultaneously invalid? [Clarity, Gap]
- [ ] CHK012 - Are requirements defined for extra/unexpected JSON fields in the request body? [Coverage, Gap]
- [ ] CHK013 - Is the distinction between a missing field (omitted key) and a null field (`"weight_kg": null`) specified in validation requirements? [Clarity, Gap]

## Web UI Requirements

- [ ] CHK014 - Is the exact result display format specified (e.g., "BMI: 22.9 - Normal" vs separate value and category fields)? [Clarity, Spec §FR-007]
- [ ] CHK015 - Are requirements defined for UI behavior when the fetch call to /api/bmi fails due to a network error (not a validation error)? [Coverage, Gap]
- [ ] CHK016 - Is the UI behavior specified for rapid successive form submissions (disable button, debounce)? [Coverage, Gap]
- [ ] CHK017 - Are accessibility requirements defined for the Bootstrap form (label associations, ARIA attributes, keyboard navigation)? [Coverage, Gap]
- [ ] CHK018 - Is the state of the result/error display area between consecutive submissions specified (cleared before each request, persisted until replaced)? [Clarity, Gap]

## Acceptance Criteria Quality

- [ ] CHK019 - Does each FR (FR-001 through FR-010) map to at least one acceptance scenario or success criterion? [Traceability, Gap]
- [ ] CHK020 - Is SC-003 "both invalid-input variants" defined -- which two variants are intended? [Clarity, Spec §SC-003]
- [ ] CHK021 - Are acceptance scenarios defined for BMI values at exact category boundaries (18.5, 25.0, 30.0)? [Completeness, Spec §SC-003]
- [ ] CHK022 - Is FR-010 (server-side error logging) covered by any acceptance scenario or measurable success criterion? [Traceability, Gap]

## Non-Functional & Cross-Cutting Requirements

- [ ] CHK023 - Is the <1s performance threshold in SC-001 defined as a single-request ceiling or a percentile latency target? [Clarity, Spec §SC-001]
- [ ] CHK024 - Are logging requirements specified beyond configurable log level (structured format, destination, which event types are logged)? [Clarity, Spec §FR-010]
- [ ] CHK025 - Is the health check response body format specified (empty body, plain text, JSON)? [Clarity, Spec §FR-008]
- [ ] CHK026 - Are deployment requirements for the Heroku target specified beyond PORT env var support (e.g., build command, release phase)? [Coverage, Gap]
