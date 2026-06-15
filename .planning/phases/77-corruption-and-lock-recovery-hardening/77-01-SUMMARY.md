---
phase: 77-corruption-and-lock-recovery-hardening
plan: 01
subsystem: recovery
tags: [rust, status, recovery, storage, serde]

requires:
  - phase: 76-disk-and-resource-bound-enforcement
    provides: resource-bound status contracts used by recovery classifier precedence
provides:
  - Shared `RecoveryEvidenceSnapshot` status contract
  - Pure recovery classifier for storage, lock, marker, and resource signals
  - Backward-compatible top-level `recovery_evidence` status field
affects:
  - status
  - support
  - dashboard
  - soak
  - parity-breadcrumbs

tech-stack:
  added: []
  patterns:
    - Pure classifier returns typed recovery evidence without filesystem or datadir mutation.
    - Stable `SyncRecoveryCategory` labels are preserved beside richer cause and action-class fields.

key-files:
  created:
    - packages/open-bitcoin-node/src/recovery.rs
  modified:
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Added recovery evidence as a top-level status field so stopped-node and store-open failures can be represented before sync state is available."
  - "Kept existing `sync.recovery_category` compatibility labels unchanged and added typed cause/action evidence beside them."
  - "Implemented classification as a pure function with no automatic repair, deletion, lock cleanup, reindex, or datadir relocation behavior."

patterns-established:
  - "Recovery action classes: `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, and `stop_and_escalate` are durable safety guidance."
  - "Lock evidence DTOs use the shared `{ kind, lock_path, detail }` shape required by Plan 77-02."

requirements-completed: [REC-06, REC-07, REC-08]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-15T21:46:58Z

duration: 16 min
completed: 2026-06-15
---

# Phase 77 Plan 01: Recovery Evidence Contract and Classifier Summary

**Typed recovery evidence and a pure classifier now map storage, lock, marker, and resource signals into stable status JSON without hidden datadir mutation.**

## Performance

- **Duration:** 16 min
- **Started:** 2026-06-15T21:30:52Z
- **Completed:** 2026-06-15T21:46:58Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Added `RecoveryEvidenceSnapshot`, `RecoveryActionClass`, `RecoveryCause`, `RecoveryEvidenceBasis`, `LockEvidence`, and `LockEvidenceKind` in `packages/open-bitcoin-node/src/recovery.rs`.
- Added top-level `OpenBitcoinStatusSnapshot.recovery_evidence` with legacy JSON defaulting to `Unavailable { reason: "recovery evidence unavailable" }`.
- Implemented `classify_recovery` precedence for resource stops, concurrent datadir use, active/stale locks, schema mismatch, corruption, partial writes, unreadable namespaces, and backend failures.
- Added parity breadcrumb coverage for the new Rust recovery module.

## Task Commits

1. **Task 1 RED: recovery evidence contract tests** - `214b587` (test)
2. **Task 1 GREEN: recovery evidence status contract** - `57ee214` (feat)
3. **Task 2 RED: recovery classifier matrix tests** - `538bde2` (test)
4. **Task 2 GREEN: recovery classifier matrix** - `b492266` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/recovery.rs` - Pure recovery evidence contracts, DTOs, classifier, and classifier tests.
- `packages/open-bitcoin-node/src/status.rs` - Added top-level recovery evidence field with serde defaulting.
- `packages/open-bitcoin-node/src/lib.rs` - Public recovery module and contract exports.
- `packages/open-bitcoin-node/src/status/tests.rs` - Contract tests for JSON labels, lock evidence shape, and legacy status defaulting.
- `packages/open-bitcoin-cli/src/operator/status.rs` and CLI test fixtures - Populate default recovery evidence in existing status snapshots.
- `docs/parity/source-breadcrumbs.json` - Added breadcrumb coverage for the new recovery module.

## Decisions Made

- Recovery evidence is top-level on `OpenBitcoinStatusSnapshot`, not nested only under sync, so stopped-node and store-open failures can be represented.
- Existing stable `SyncRecoveryCategory` labels were not renamed; richer typed causes and action classes sit beside them.
- Classifier output provides guidance only. It does not perform or imply automatic repair, lock cleanup, deletion, reindex, or datadir relocation.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Known Stubs

None.

## Verification

- `cargo fmt --all --manifest-path packages/Cargo.toml -- --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib recovery_evidence_contract_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib status_recovery_evidence_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib recovery_classifier_ --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features -- -D warnings`
- `bun run scripts/check-parity-breadcrumbs.ts --check`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 77-02 to populate `LockEvidence` from probe-only lock collection without changing the shared DTO shape.

## Self-Check: PASSED

- Summary file exists.
- Created recovery module exists.
- Task commits found: `214b587`, `57ee214`, `538bde2`, `b492266`.
- No `## Self-Check: FAILED` marker remains.

---
*Phase: 77-corruption-and-lock-recovery-hardening*
*Completed: 2026-06-15*
