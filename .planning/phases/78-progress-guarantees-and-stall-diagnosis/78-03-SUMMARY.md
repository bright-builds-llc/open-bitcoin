---
phase: 78-progress-guarantees-and-stall-diagnosis
plan: 03
subsystem: soak-reports
tags: [rust, soak, progress-guarantees, stall-diagnosis, operator-reports]

requires:
  - phase: 78
    plan: 02
    provides: "Runtime progress-credit and stall diagnosis evidence on shared SyncStatus"
  - phase: 78
    plan: 07
    provides: "Downstream CLI/RPC constructors tolerate Phase 78 SyncStatus fields"
provides:
  - "Soak checkpoints persist shared progress-credit, threshold, peer contribution, and stall diagnosis evidence"
  - "Soak Markdown and JSON reports project Phase 78 evidence from ledger checkpoints"
  - "Focused coverage for available and unavailable progress-guarantee checkpoint states"
affects: [soak-ledger, soak-report, support-redaction, phase-78]

tech-stack:
  added: []
  patterns:
    - "Soak reports remain projections over datadir-owned ledger events"
    - "Checkpoint events box large status payloads to keep the ledger enum compact"
    - "Redaction checkers should not require forbidden marker strings in production report modules"

key-files:
  created:
    - .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-03-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/soak/ledger.rs
    - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
    - packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs
    - packages/open-bitcoin-cli/src/operator/soak/report.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - scripts/check-phase75-soak-runner.ts
    - scripts/check-phase75-soak-runner.test.ts
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Projected only the shared Phase 78 SyncStatus evidence fields into soak checkpoints; the report layer does not reclassify raw counters."
  - "Rendered Markdown with compact key=value evidence lines while omitting absent optional fields."
  - "Boxed SoakLedgerEvent::Checkpoint status payload after schema expansion triggered clippy::large_enum_variant."

patterns-established:
  - "Rejected progress activity labels use `kind=<label> observed_count=<n> reason=<reason>`."
  - "Peer contribution labels use `peer=<peer> kind=<label> messages=<n> headers=<n> blocks=<n> failure=<label-or-unavailable>`."
  - "Phase redaction checkers anchor safe boundary language in production files and keep forbidden marker strings in tests/docs."

requirements-completed: [PROG-01, PROG-02, PROG-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-17T08:34:49Z

duration: 1h 32m
completed: 2026-06-17
---

# Phase 78-03: Soak Checkpoint and Report Progress Evidence Summary

**Soak checkpoints and reports now carry shared progress-credit and stall diagnosis evidence without making reports a separate source of truth.**

## Performance

- **Duration:** 1h 32m
- **Completed:** 2026-06-17T08:34:49Z
- **Tasks:** 3
- **Files modified:** 10

## Accomplishments

- Expanded `SoakCheckpointStatus` with Phase 78 progress credit, rejected activity, expected window, threshold, last useful work, last peer contribution, and stall diagnosis fields.
- Projected those fields directly from `snapshot.sync.progress_credit`, `expected_progress_window`, `no_progress_threshold`, `last_useful_work`, `last_peer_contribution`, and `stall_diagnosis`.
- Added JSON field-name coverage, Markdown rendering coverage, and unavailable-state coverage for the new checkpoint fields.

## Task Commits

This plan will be committed as one atomic implementation commit with this summary artifact.

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` - Added Phase 78 checkpoint fields and boxed checkpoint ledger events.
- `packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs` - Projects shared SyncStatus evidence into checkpoint fields.
- `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` - Boxes checkpoint events when appending to the ledger.
- `packages/open-bitcoin-cli/src/operator/soak/report.rs` - Renders progress guarantee and stall diagnosis evidence in Markdown reports.
- `packages/open-bitcoin-cli/src/operator/soak/tests.rs` - Covers JSON field names, Markdown labels, redaction, and boxed checkpoint fixtures.
- `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` - Covers available and unavailable runtime checkpoint projection.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Updates support fixture checkpoint construction for the expanded schema.
- `scripts/check-phase75-soak-runner.ts` - Updates production report redaction anchors to safe wording.
- `scripts/check-phase75-soak-runner.test.ts` - Updates Phase 75 checker fixture anchors.
- `docs/metrics/lines-of-code.md` - Refreshed tracked LOC report.

## Decisions Made

- Kept progress/stall classification out of the report renderer; reports only serialize and format what the shared status snapshot already computed.
- Used boxed checkpoint payloads rather than suppressing clippy because the expanded schema made the ledger event enum materially larger.
- Updated the Phase 75 checker instead of reintroducing forbidden marker strings into `soak/report.rs`, preserving the newer Phase 78 negative source-scan requirement.

## Deviations from Plan

### Auto-fixed Issues

**1. Large enum variant after checkpoint schema expansion**
- **Found during:** `cargo clippy --all-targets --all-features -- -D warnings`
- **Issue:** `SoakLedgerEvent::Checkpoint` became much larger than the other enum variants after adding Phase 78 fields.
- **Fix:** Changed `Checkpoint { status }` to store `Box<SoakCheckpointStatus>` and updated construction/projection sites.
- **Verification:** Full clippy, build, tests, and `scripts/verify.sh` passed.

**2. Stale Phase 75 checker required forbidden strings in production report source**
- **Found during:** `bash scripts/verify.sh`
- **Issue:** The old checker required `wallet material` and `unbounded peer tables` in `soak/report.rs`, conflicting with the Phase 78 negative source scan.
- **Fix:** Changed the checker to anchor safe redaction-boundary wording in the production report module while retaining forbidden-marker assertions in tests/support surfaces.
- **Verification:** `bun test scripts/check-phase75-soak-runner.test.ts`, `bun run scripts/check-phase75-soak-runner.ts`, and `scripts/verify.sh` passed.

## Issues Encountered

- `scripts/verify.sh` required regenerating `docs/metrics/lines-of-code.md` after source changes.
- The production Rust file-length guard required tightening `soak/runtime/helpers.rs` back to 627 lines.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_progress_guarantee_report_json_preserves_checkpoint_field_names --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_progress_guarantee_checkpoint_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_progress_guarantee_report_ --all-features`
- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features`
- `bun test scripts/check-phase75-soak-runner.test.ts`
- `bun run scripts/check-phase75-soak-runner.ts`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`

## User Setup Required

None.

## Next Phase Readiness

Plan 78-04 can render the same shared progress-guarantee evidence through the operator status/dashboard surfaces, and later plans can rely on soak reports carrying the ledger-backed Phase 78 evidence.

---
*Phase: 78-progress-guarantees-and-stall-diagnosis*
*Completed: 2026-06-17*
