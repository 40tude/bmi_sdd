# Feature Specification: BMI Calculation History

**Feature Branch**: `002-bmi-history`
**Created**: 2026-03-05
**Status**: Draft
**Input**: User description: "BMI calculation history showing the last 5 entries using an in-memory VecDeque, shared across all users."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View History After Calculation (Priority: P1)

A user submits a BMI calculation and immediately sees a history table below the result showing all recent calculations, including the one just submitted.

**Why this priority**: Core feature -- without visible history, the feature has no user-facing value.

**Independent Test**: Can be fully tested by submitting one BMI calculation and verifying a history table appears with that entry.

**Acceptance Scenarios**:

1. **Given** no calculations have been made, **When** a user submits weight=70 kg and height=1.75 m, **Then** the result page shows the BMI value and category, plus a history table with exactly 1 row containing weight, height, BMI, category, and a timestamp.
2. **Given** 3 calculations already exist in history, **When** a user submits a new calculation, **Then** the history table shows 4 rows, with the newest entry appearing first (most recent at top).

---

### User Story 2 - FIFO Eviction at Capacity (Priority: P1)

When a 6th calculation is submitted, the oldest entry is automatically removed so the history never exceeds 5 entries.

**Why this priority**: Equally critical as Story 1 -- unbounded history would be a defect. The 5-entry cap is a core constraint.

**Independent Test**: Can be tested by submitting 6 calculations and verifying the first one is no longer visible.

**Acceptance Scenarios**:

1. **Given** 5 entries exist in history, **When** a user submits a 6th calculation, **Then** the history table still shows exactly 5 rows, and the oldest entry (the 1st calculation) is no longer present.
2. **Given** 5 entries exist in history, **When** a user submits a 6th calculation, **Then** the newest entry appears first in the table and the previously newest entries shift down by one position.

---

### User Story 3 - Shared History Across Users (Priority: P2)

All users see the same history. A calculation submitted by one user is visible to any other user who subsequently views the page or submits a calculation.

**Why this priority**: Important for the server-wide shared design, but lower than basic functionality since the app currently has no user isolation anyway.

**Independent Test**: Can be tested by submitting a calculation from one client, then calling the history endpoint from a different client and verifying the entry appears.

**Acceptance Scenarios**:

1. **Given** User A submits a calculation, **When** User B submits a different calculation, **Then** User B's history table shows both entries.

---

### Edge Cases

- What happens when the very first calculation is submitted (empty history)? History table appears with 1 row.
- What happens if two users submit calculations at nearly the same time? Both entries are recorded; ordering is determined by the order the server processes them. No data loss occurs.
- What happens if a calculation fails validation (e.g., negative weight)? No entry is added to history; the error response is returned and history remains unchanged.
- What happens when the server restarts? History is lost (in-memory only). This is expected and acceptable.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST store each successful BMI calculation as a history entry containing: weight (kg), height (m), BMI value, WHO category, and timestamp.
- **FR-002**: System MUST retain at most 5 history entries at any time.
- **FR-003**: When a 6th entry is added, the system MUST remove the oldest entry (FIFO eviction).
- **FR-004**: System MUST return history entries ordered from newest to oldest.
- **FR-005**: System MUST NOT add a history entry when a BMI calculation fails validation.
- **FR-006**: History MUST be shared across all users (server-wide, not per-session).
- **FR-007**: The history table MUST be visible on the results page after each successful calculation.
- **FR-008**: Each history entry MUST display the timestamp of when the calculation was performed.
- **FR-009**: History MUST be available via the API response so both the web UI and API consumers can access it.

### Key Entities

- **History Entry**: Represents one successful BMI calculation. Attributes: weight (kg), height (m), BMI value, WHO category (Underweight/Normal/Overweight/Obese), timestamp of calculation.
- **History List**: Server-wide ordered collection of History Entries, capped at 5, newest first.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After any successful BMI calculation, the history table is visible and contains at least 1 entry and at most 5 entries.
- **SC-002**: The 6th calculation causes the oldest entry to disappear, maintaining exactly 5 entries.
- **SC-003**: History entries from one user session are visible to all other users.
- **SC-004**: Failed calculations (invalid input) produce zero change in the history.
- **SC-005**: Existing BMI calculation functionality continues to work without regression (all current tests pass).

## Assumptions

- History is ephemeral: server restart clears all entries. Persistent storage is explicitly out of scope.
- Per-user session isolation is out of scope; all users share one global history.
- Timestamp precision to the second is sufficient for display purposes.
- No authentication or user identification is required.
- Concurrent access is handled safely; no data corruption under simultaneous requests.
