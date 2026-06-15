---
phase: 77-corruption-and-lock-recovery-hardening
plan: 03
subsystem: recovery
tags: [rust, status, recovery, lock-probe, operator-cli]

requires:
  - phase: 77-corruption-and-lock-recovery-hardening
    provides: Plan 77-01 recovery evidence DTOs and Plan 77-02 probe-only Fjall lock evidence
provides:
  - Probe-only CLI status recovery evidence collector
  - Top-level status JSON recovery evidence projection
  - Human status Recovery evidence rendering after Sync recovery
  - Forbidden status store-open regression coverage
affects:
  - status
  - support
  - service-status
  - parity-breadcrumbs

tech-stack:
  added: []
  patterns:
    - Status collection uses probe-only lock evidence and live RPC/service signals instead of opening Fjall stores.
    - Human rendering consumes shared recovery evidence labels from the status snapshot contract.

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
    - packages/open-bitcoin-cli/src/operator/status/service_status.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Status recovery evidence is collected through `probe_fjall_lock`, service same-datadir evidence, and live RPC availability without opening Fjall stores."
  - "Status no longer uses durable store fallback for sync, metrics, service restart metadata, or wallet selection during inspection-only status collection."
  - "Human status renders the shared recovery evidence field immediately after `Sync recovery:` while JSON remains the shared `serde_json` snapshot."

patterns-established:
  - "Probe-only status paths preserve unavailable reasons instead of attempting durable store inspection."
  - "Concurrent datadir use is inferred from bounded lock, service same-datadir, and live RPC evidence."

requirements-completed: [REC-05, REC-06, REC-07, REC-08]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-15T22:56:40Z

duration: 21 min
completed: 2026-06-15
---

# Phase 77 Plan 03: Status Recovery Evidence Projection Summary

**Operator status now exposes typed recovery evidence from probe-only lock, service, and RPC signals without opening Fjall stores during inspection.**

## Performance

- **Duration:** 21 min
- **Started:** 2026-06-15T22:35:24Z
- **Completed:** 2026-06-15T22:56:40Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Added `collect_status_recovery_evidence` for CLI status using `probe_fjall_lock` and the shared pure `classify_recovery` contract.
- Removed status collection's durable store opens for sync fallback, metrics history, service restart metadata, and wallet RPC selection.
- Added JSON coverage for unavailable, stale-lock, and concurrent-datadir recovery evidence.
- Added human rendering for `Recovery evidence:` immediately after `Sync recovery:`.
- Updated the operator binary stopped-node JSON test to require top-level `recovery_evidence`.

## Task Commits

1. **Task 1 RED: status recovery evidence tests** - `7004b04` (test)
2. **Task 1 GREEN: probe-only status recovery evidence** - `a5e31ac` (feat)
3. **Task 2 RED: recovery evidence render tests** - `8096f48` (test)
4. **Task 2 GREEN: human recovery evidence rendering** - `a51cd16` (feat)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs` - Probe-only status recovery evidence collector.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Wires recovery evidence into live, stopped, and unreachable snapshots without store opens.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - Keeps status sync projection RPC/unavailable-only.
- `packages/open-bitcoin-cli/src/operator/status/service_status.rs` - Preserves same-datadir service evidence without loading runtime metadata.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Adds human `Recovery evidence:` rendering.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Adds status recovery evidence and rendering coverage.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Verifies binary stopped-node JSON includes top-level recovery evidence.
- `docs/parity/source-breadcrumbs.json` - Adds the new CLI status recovery evidence source file.

## Decisions Made

- Status uses live RPC data when reachable and explicit unavailable sync status when stopped/unreachable; it no longer falls back to durable sync state through Fjall.
- Metrics history is unavailable in probe-only status with the explicit reason `metrics history unavailable: probe-only status does not open Fjall stores`.
- Wallet RPC access stays at root scope during status setup; wallet-specific unavailable state comes from live RPC wallet errors instead of store inspection.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Removed indirect service status store open**
- **Found during:** Task 1 (Add status recovery evidence collector)
- **Issue:** The plan's negative scan covered `status.rs`, `sync_state.rs`, and the new collector, but `collect_service_status` still reached `sync_state::durable_runtime_metadata`, which opened Fjall during `open-bitcoin status`.
- **Fix:** Changed service restart/resume status to preserve service-manager same-datadir evidence while marking runtime metadata-derived details unavailable with the probe-only reason.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status/service_status.rs`, `packages/open-bitcoin-cli/src/operator/status/tests.rs`
- **Verification:** Strict forbidden scan passed across `status.rs`, `sync_state.rs`, `recovery_evidence.rs`, and `service_status.rs`.
- **Committed in:** `a5e31ac`

---

**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** The adjustment was required to satisfy the probe-only status inspection rule. It narrowed status behavior and did not add feature scope.

## Issues Encountered

None.

## Known Stubs

None.

## Verification

- `cargo fmt --all --manifest-path packages/Cargo.toml -- --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib status_recovery_evidence_ --all-features`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib status_recovery_evidence_render_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_status_json_succeeds_for_stopped_node --all-features`
- Strict forbidden API scan for `FjallNodeStore::open`, `WalletRegistry::load`, `Database::builder`, recovery marker mutation, and file deletion APIs in status inspection files.

## Diff Review

- Production file line counts stayed below the repo trigger: `status.rs` 508, `recovery_evidence.rs` 49, `sync_state.rs` 111, `service_status.rs` 212, `render.rs` 606.
- Stub scan found no TODO, FIXME, placeholder, coming-soon text, or hardcoded empty UI data stubs in touched files.
- `git status --short` was clean before writing this summary.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 77-04 to project recovery evidence into support/status-adjacent surfaces while preserving the probe-only inspection boundary.

## Self-Check: PASSED

- Summary file exists.
- Created recovery evidence collector exists.
- Task commits found: `7004b04`, `a5e31ac`, `8096f48`, `a51cd16`.
- No failed self-check marker remains.

---
*Phase: 77-corruption-and-lock-recovery-hardening*
*Completed: 2026-06-15*
