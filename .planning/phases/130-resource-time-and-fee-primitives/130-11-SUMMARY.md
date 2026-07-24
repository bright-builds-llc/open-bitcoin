---
phase: 130-resource-time-and-fee-primitives
plan: "11"
subsystem: rpc-admission
tags: [rust, mempool, rpc, explicit-time, relay-intent, privacy]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Explicit-time managed local admission and node caller migration from Plans 130-05 through 130-07
provides:
  - Checked RPC-shell Unix-seconds sampling for local sendrawtransaction admission
  - Deterministic submit_local_transaction_with_relay_evidence_at seam with activation-resolved relay intent
  - Removed submit_local_transaction_outcome no-time compatibility methods from bridge and authority
  - Direct authenticated response detail with identity-free shared network status
affects: [phase-136, rpc-admission, local-relay, FEEP-03, FEEP-04, FEEP-05]
tech-stack:
  added: []
  patterns:
    - Effectful clock sampling stays in the RPC dispatch shell; managed context remains deterministic via `_at`
    - Local relay intent is Requested only when authoritative relay activation is enabled
key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  modified:
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Sample SystemTime only in dispatch/node.rs with checked conversion; never unwrap_or(0)."
  - "Resolve RelayIntent::Requested from relay activation enabled; otherwise NotRequested."
  - "Migrate the final RPC caller and delete both no-time outcome adapters in one commit so the workspace stays compilable."
  - "Extract mempool recovery helpers into a sibling module to preserve the production file-length contract."
patterns-established:
  - "Local RPC admission acquires wall-clock time only at the sendrawtransaction shell boundary."
  - "Deterministic RPC tests inject exact Unix seconds through submit_local_transaction_with_relay_evidence_at."
requirements-completed: []
requirements-addressed: [FEEP-03, FEEP-04, FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-24T04:00:07Z
duration: 25 min
completed: 2026-07-24
---

# Phase 130 Plan 11: Local RPC Admission Timing and Privacy Summary

**Local `sendrawtransaction` now samples checked Unix seconds in the RPC shell, stores exact local/requested metadata through the managed authority, and no longer depends on no-time compatibility adapters**

## Performance

- **Duration:** 25 min
- **Started:** 2026-07-24T03:35:11Z
- **Completed:** 2026-07-24T04:00:07Z
- **Tasks:** 1
- **Files modified:** 9

## Accomplishments

- Added `submit_local_transaction_with_relay_evidence_at` and shell `current_timestamp_unix_seconds` with fail-closed pre-epoch and overflow mapping to internal RPC errors.
- Resolved typed relay intent from authoritative relay activation and preserved detailed txid/replacement/eviction data only on the authenticated direct response.
- Removed `submit_local_transaction_outcome` from both the admission bridge and runtime authority after migrating the final RPC caller in the same commit.
- Added focused RPC tests for exact metadata at timestamp 60, typed clock failure, and identity-free shared status serialization.
- Extracted mempool recovery helpers so `context/network.rs` stays under the production line-limit gate.
- Passed the list gate, focused sendrawtransaction suite, no-time source scan, timed workspace `--all-targets` check, and full normal-hook verification.

## TDD Execution

- **Task 1 RED:** Existing relay evidence expectations still assumed fail-closed no-time metadata and zero fanout; new exact-time and clock-failure tests were authored first.
- **Task 1 GREEN:** Shell sampling, `_at` forwarding, compatibility removal, and updated privacy/relay assertions landed together so the workspace remained compilable at the commit boundary.

## Task Commits

1. **Task 1: Sample local acceptance time in the RPC shell** - `ea59457f` (feat)

**Plan metadata:** `63080d3f` (docs: complete plan)

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Checked shell clock sampling and `_at` local submission.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Deterministic `_at` seam, metadata accessor, activation-resolved relay intent.
- `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs` - Extracted recovery helpers for the file-length contract.
- `packages/open-bitcoin-rpc/src/context.rs` - Declared the recovery module.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Exact metadata, clock failure, and privacy regressions.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Removed no-time outcome adapter.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Removed no-time outcome adapter; added metadata and activation helpers.
- `docs/parity/source-breadcrumbs.json` - Registered the new recovery module.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked LOC freshness.

## Decisions Made

- Clock acquisition stays exclusively in `dispatch/node.rs`; the managed RPC context never reads wall-clock time.
- Relay intent follows activation rather than always requesting relay, matching Plan 130-05's fail-closed rule when activation is disabled.
- The wallet `submit_local_transaction` AdmissionResult path remains deprecated but present; only the outcome no-time shim owned by this plan was removed.

## ASVS Mitigations

- **ASVS-130-V1/V13:** Inventoried old and `_at` callers across node and RPC targets; removed both compatibility definitions; passed timed `cargo check --workspace --all-targets`.
- **ASVS-130-V2 preservation / V4 / V8:** Kept txid and metadata out of shared status while authenticated responses retain detailed outcomes.
- **ASVS-130-V7/V11:** Clock-before-epoch and conversion failure map to typed internal RPC errors with no zero-time fallback.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split network.rs after production line-limit failure**
- **Found during:** Task 1 commit hook
- **Issue:** `packages/open-bitcoin-rpc/src/context/network.rs` exceeded the 628-line production limit after the `_at` seam and metadata helpers landed.
- **Fix:** Moved mempool snapshot recovery helpers into `context/mempool_recovery.rs` and registered parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs`, `packages/open-bitcoin-rpc/src/context.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Line-limit checker and full normal-hook verify passed on the successful task commit.
- **Committed in:** `ea59457f` (part of task commit)

---

**Total deviations:** 1 auto-fixed (Rule 3)
**Impact on plan:** Necessary for the repository file-length contract; no behavior change beyond module extraction.

## Issues Encountered

- The first commit attempt failed only on the production line-limit gate after the rest of verify had already passed; the recovery-module extraction cleared it.

## Authentication Gates

None

## Known Stubs

None

## Threat Flags

None — no new network endpoint, auth path, or trust-boundary schema was introduced beyond the plan threat model.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-rpc/src/context/network.rs`
- FOUND: `packages/open-bitcoin-rpc/src/dispatch/node.rs`
- FOUND: `packages/open-bitcoin-node/src/network/runtime_authority.rs`
- FOUND: `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs`
- FOUND: commit `ea59457f`
- FOUND: no `submit_local_transaction_outcome(` in node/RPC production sources
