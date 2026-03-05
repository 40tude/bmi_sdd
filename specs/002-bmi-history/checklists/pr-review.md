# PR Review Checklist: BMI Calculation History

**Purpose**: Validate requirement quality across API contract, UI/display, and concurrency/safety domains before merge
**Created**: 2026-03-05
**Feature**: [spec.md](../spec.md) | [contracts/api.md](../contracts/api.md)
**Depth**: Standard PR review
**Audience**: Reviewer

> **HARD BLOCKER items** (marked `[BLOCKER]`) must be resolved before merge.
> All other items are advisory for the reviewer.

---

## API Contract Requirements Quality

- [ ] CHK001 - Is the `history` field guaranteed present in ALL successful 200 responses, or only after the first calculation is made? [Clarity, Spec §FR-009]
- [ ] CHK002 - Does the spec define whether the 422 error response includes or explicitly omits the `history` field? [Gap, Spec §FR-005]
- [ ] CHK003 - Are the exact field names and types for `HistoryEntry` specified in the spec, or only in the contract doc? [Completeness, Spec §FR-001]
- [ ] CHK004 - Is the BMI rounding rule ("1 decimal") stated as a formal spec requirement, or only implicit in the contract example? [Gap, Contracts §POST-/api/bmi]
- [ ] CHK005 - Are backward-compatibility expectations for the new `history` field explicitly stated for existing API consumers? [Gap, Contracts §Breaking-Changes]
- [ ] CHK006 - Is the intentional absence of a separate `GET /history` endpoint documented as a requirement decision, or left undocumented? [Ambiguity, Spec §FR-009]

---

## UI/Display Requirements Quality

- [ ] CHK007 - Is the timestamp display format specified in the spec (human-readable locale string vs. raw RFC 3339)? [Gap, Spec §FR-008]
- [ ] CHK008 - Are the exact column names, column order, and the row-number (`#`) column specified in the spec, or only in the contract doc? [Gap, Spec §FR-007]
- [ ] CHK009 - Is the initial UI state before any calculation defined -- what is shown in place of the history table on first page load? [Gap, Spec §FR-007]
- [ ] CHK010 - Are accessibility requirements (WCAG level, keyboard navigation, screen-reader labels) defined for the history table? [Gap]
- [ ] CHK011 - Are responsive or mobile layout requirements defined for the history table? [Gap]
- [ ] CHK012 - Is "results page" in FR-007 defined -- is it the same page as the form, or a distinct view/route? [Ambiguity, Spec §FR-007]

---

## Concurrency & Safety Requirements

- [ ] CHK013 - [BLOCKER] Is thread-safety of shared history a formal Functional Requirement, or only an unvalidated Assumption? [Gap, Spec §Assumptions]
- [ ] CHK014 - [BLOCKER] Is "handled safely" quantified -- does it mean no data loss, no panic, no deadlock, linearizable writes, or all of the above? [Ambiguity, Spec §Assumptions]
- [ ] CHK015 - [BLOCKER] Is the behavior on mutex poisoning (a panicking lock holder) specified as a requirement? [Gap]
- [ ] CHK016 - Are ordering guarantees for near-simultaneous requests stated as a formal requirement, or only as an edge-case observation? [Ambiguity, Spec §Edge-Cases]
- [ ] CHK017 - Is the scope of "no data corruption" defined -- does it cover partial writes, torn reads, or only total entry loss? [Clarity, Spec §Assumptions]

---

## Data Model & Field Requirements

- [ ] CHK018 - Are data types and numeric precision for all `HistoryEntry` fields specified in the spec, not only in the contract doc? [Gap, Spec §FR-001]
- [ ] CHK019 - Is the timestamp format (RFC 3339 / ISO 8601) specified in the spec independently of the contract example? [Gap, Spec §FR-001, §FR-008]
- [ ] CHK020 - Is the timestamp source (server clock at request receipt vs. at calculation completion) explicitly specified? [Ambiguity, Spec §FR-001]
- [ ] CHK021 - Are the exact WHO category string values ("Underweight", "Normal", "Overweight", "Obese") normatively enumerated in the spec? [Completeness, Spec §Key-Entities]

---

## Acceptance Criteria Quality

- [ ] CHK022 - Does SC-003 use the term "user session" while the spec prohibits per-session isolation -- is this a terminological conflict? [Conflict, Spec §SC-003, §Assumptions]
- [ ] CHK023 - Is "all current tests pass" in SC-005 traceable to a specific, enumerated test set rather than an open-ended reference? [Clarity, Spec §SC-005]
- [ ] CHK024 - Do SC-001 through SC-005 collectively cover all nine functional requirements (FR-001 through FR-009), with no FR left unmapped? [Coverage, Spec §Success-Criteria]
- [ ] CHK025 - Is a success criterion defined for concurrent-access safety (no data loss under simultaneous requests)? [Gap, Spec §SC]

---

## Edge Case & Scenario Coverage

- [ ] CHK026 - Is the "server restart clears history" behavior documented as a formal requirement or only as an unvalidated assumption? [Gap, Spec §Assumptions]
- [ ] CHK027 - Are requirements defined for the scenario where the mutex lock cannot be acquired (contention, starvation, poisoned state)? [Gap]
- [ ] CHK028 - Is the behavior specified when two concurrent requests both trigger FIFO eviction simultaneously (capacity-boundary race)? [Gap, Coverage]

---

## Notes

- Check items off as completed: `[x]`
- For BLOCKER items: document the resolution (added as FR, accepted as assumption with justification, etc.) inline
- Items reference spec sections and gap markers for traceability
