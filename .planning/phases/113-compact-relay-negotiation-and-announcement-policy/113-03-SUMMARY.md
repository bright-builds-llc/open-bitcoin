---
phase: 113-compact-relay-negotiation-and-announcement-policy
plan: 03
subsystem: network-peer-policy
tags: [rust, bip152, compact-relay, announcement-policy, guardrails, parity]

requires:
  - phase: 113-compact-relay-negotiation-and-announcement-policy
    provides: Plan 113-01 compact relay negotiation state and Plan 113-02 compact announcement decision policy
provides:
  - Explicit guardrail tests for compact announcement fallback, suppression, and scope isolation
  - Node-shell regression proving compact getdata remains suppressed after compact announcement negotiation policy
  - Repo-native verification evidence for deterministic local Phase 113 completion
affects: [phase-114-compact-reconstruction, phase-115-missing-transaction-fallback, phase-116-operator-evidence]

tech-stack:
  added: []
  patterns:
    - Phase-scoped compact relay guardrail tests use stored PeerManager state instead of synthetic policy-only fixtures
    - Compact getdata suppression remains node-shell test coverage separate from compact announcement policy
    - Wrapper-owned final git mutation with task commits recorded as pending-final-commit

key-files:
  created:
    - .planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-03-SUMMARY.md
    - packages/open-bitcoin-network/src/peer/compact_relay/tests.rs
  modified:
    - packages/open-bitcoin-network/src/peer/compact_relay.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Low-bandwidth sendcmpct remains compact relay capability evidence but never authorizes direct compact block announcements."
  - "Adjacent relay, permission, inbound protection, and block-serving state cannot activate compact announcement policy by implication."
  - "Compact getdata stays in the suppressed/missing path; compact reconstruction and missing-transaction behavior remain deferred to later phases."
  - "No git commits were created because the parent wrapper reserves final git mutation for verification-first orchestration."

patterns-established:
  - "Phase 113 guardrail tests assert fixed low-cardinality reason labels rather than dynamic peer, block, transaction, or permission values."
  - "File-length guard remediation moves tests into sibling test modules while preserving production breadcrumb formatting."

requirements-completed: [CMP-05, CMP-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 113-2026-07-04T22-53-48
generated_at: 2026-07-05T00:00:49Z

duration: 36m 49s
completed: 2026-07-05
---

# Phase 113 Plan 03: Fallback and Scope Guardrails Summary

**Compact relay announcement behavior is pinned with deterministic fallback, suppression, and scope-isolation tests, while compact getdata serving remains suppressed and separate from direct announcements.**

## Performance

- **Duration:** 36m 49s
- **Started:** 2026-07-04T23:24:00Z
- **Completed:** 2026-07-05T00:00:49Z
- **Tasks:** 3 completed
- **Files modified:** 8 including this summary

## Accomplishments

- Added Phase 113 peer-manager tests for low-bandwidth fallback, high/low toggle refresh, unsupported-version evidence, already-known header fallback, missing header fallback, unavailable block suppression, and resource-limited suppression.
- Added scope-isolation tests proving `WtxidRelay`, transaction relay setup, block serving alone, download permission, protected inbound permission, and default activation policy do not grant compact announcements.
- Added the node-shell regression `phase113_compact_getdata_remains_suppressed_after_negotiation_policy`, proving `InventoryType::CompactBlock` getdata does not emit `WireNetworkMessage::CompactBlock` and remains in the missing/notfound path.
- Moved compact relay unit tests into `packages/open-bitcoin-network/src/peer/compact_relay/tests.rs` so production source stays under the repo file-length guard.
- Refreshed parity breadcrumbs and the tracked LOC report after the file move and verification-driven line-count changes.

## Task Changes

No commits were created. The parent wrapper owns verification-first final git mutation, so all commit fields are recorded as `pending-final-commit`.

1. **Task 1: Pin headers, inventory, and suppress fallback decisions** - `pending-final-commit`
   - Added Phase 113 fallback tests in `packages/open-bitcoin-network/src/peer/tests.rs`.
   - Covered low-bandwidth `sendcmpct(false, 2)`, high -> low -> high preference toggles, unsupported-only and unsupported-after-supported versions, current-header fallback, missing-header fallback, unavailable block suppression, and resource pressure suppression.
2. **Task 2: Prove adjacent relay and permission surfaces cannot activate compact announcements** - `pending-final-commit`
   - Added scope-isolation tests for transaction relay, `wtxidrelay`, block serving without compact relay, download permission, protected inbound permission, and default activation policy.
   - Preserved the no-claim guardrails for package relay, bloom/filter serving, compact filters, public defaults, public-network CI, and production readiness.
3. **Task 3: Preserve compact getdata suppression and run phase verification** - `pending-final-commit`
   - Added the managed-network compact getdata suppression regression in `packages/open-bitcoin-node/src/network/tests.rs`.
   - Regenerated `docs/metrics/lines-of-code.md` only after verification required LOC freshness.

**Plan metadata:** `pending-final-commit`

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/tests.rs` - Phase 113 fallback, toggle, unsupported-version, and scope-isolation peer tests.
- `packages/open-bitcoin-node/src/network/tests.rs` - Managed-network regression for suppressed compact getdata behavior.
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` - Production module now delegates tests to a sibling test module.
- `packages/open-bitcoin-network/src/peer/compact_relay/tests.rs` - Moved compact relay unit tests plus fallback reason coverage.
- `packages/open-bitcoin-network/src/peer.rs` - Minimal formatting-only line-count trim to satisfy the production file-length guard.
- `docs/parity/source-breadcrumbs.json` - Added the new compact relay test module breadcrumb entry.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated LOC report after verification requested it.
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-03-SUMMARY.md` - Execution summary and self-check record.

## Decisions Made

- Low-bandwidth compact relay preference remains stored capability state only; direct compact announcements still require high-bandwidth preference and positive gate results.
- Unsupported `sendcmpct` versions remain evidence-only when a supported version 2 preference is already recorded.
- Compact block getdata serving remains suppressed in the node shell and does not start compact reconstruction, missing-transaction, mempool lookup, or validation/connect handoff behavior.
- Commit recording is deferred to the parent wrapper as `pending-final-commit` to honor the strict verification-first git gate.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split compact relay inline tests to satisfy file-length verification**
- **Found during:** Task 3 verification
- **Issue:** `bash scripts/verify.sh` reached the production Rust file-length guard and reported `packages/open-bitcoin-network/src/peer/compact_relay.rs` over the 628-line limit after Phase 113 growth.
- **Fix:** Moved the compact relay unit tests into `packages/open-bitcoin-network/src/peer/compact_relay/tests.rs` and kept `compact_relay.rs` as the production module with `#[cfg(test)] mod tests;`.
- **Files modified:** `packages/open-bitcoin-network/src/peer/compact_relay.rs`, `packages/open-bitcoin-network/src/peer/compact_relay/tests.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_ -- --nocapture`, `bun run scripts/check-parity-breadcrumbs.ts --check`, `bash scripts/check-file-lengths.sh`, and final `bash scripts/verify.sh`.
- **Committed in:** `pending-final-commit`

**2. [Rule 3 - Blocking] Trimmed `peer.rs` to satisfy the same file-length guard**
- **Found during:** Task 3 verification
- **Issue:** `packages/open-bitcoin-network/src/peer.rs` remained at the strict line-count threshold after formatting and breadcrumb regeneration.
- **Fix:** Applied a minimal formatting-only blank-line reduction without changing peer behavior.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`
- **Verification:** `bash scripts/check-file-lengths.sh` and final `bash scripts/verify.sh`.
- **Committed in:** `pending-final-commit`

**3. [Rule 3 - Blocking] Covered fallback reason labels after coverage verification**
- **Found during:** Task 3 verification
- **Issue:** Full verification coverage reported uncovered lines for compact fallback reason labels and fallback eligibility mapping.
- **Fix:** Extended `compact_announcement_gate_reasons_map_to_ineligible_reasons` to include `CompactHeadersFallback` and `CompactInventoryFallback` labels and eligibility.
- **Files modified:** `packages/open-bitcoin-network/src/peer/compact_relay/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compact_announcement_gate_reasons_map_to_ineligible_reasons -- --nocapture` and final `bash scripts/verify.sh`.
- **Committed in:** `pending-final-commit`

**Total deviations:** 3 auto-fixed blocking verification issues.
**Impact on plan:** All fixes were required to pass repo-native verification and kept behavior within the planned compact relay guardrail scope.

## Issues Encountered

- `docs/metrics/lines-of-code.md` became stale after the Phase 113 source/test changes. It was regenerated with `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`.
- No authentication gates or external setup were encountered.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_ -- --nocapture` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase113_compact_getdata_remains_suppressed_after_negotiation_policy -- --nocapture` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compact_announcement_gate_reasons_map_to_ineligible_reasons -- --nocapture` - passed
- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `bash scripts/check-file-lengths.sh` - passed
- `bash scripts/verify.sh` - passed in 8m 4.722s after summary and state updates

## Known Stubs

None found in files created or modified for this plan.

## Threat Flags

None. The plan added tests and generated evidence only; it did not introduce new network endpoints, auth paths, file access patterns, schema changes, public defaults, package relay, filter serving, public-network CI, or production-readiness surfaces.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 114 can build compact reconstruction behavior on top of a bounded announcement policy: direct announcements are gated and tested, while compact getdata and reconstruction remain explicitly deferred.

## Self-Check: PASSED

- Found `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-03-SUMMARY.md`.
- Verified all task and metadata commit fields are `pending-final-commit`.
- Verified final `bash scripts/verify.sh` output recorded `verify.sh completed in 8m 4.722s`.

---
*Phase: 113-compact-relay-negotiation-and-announcement-policy*
*Completed: 2026-07-05*
