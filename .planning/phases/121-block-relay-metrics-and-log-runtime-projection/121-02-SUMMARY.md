---
phase: 121-block-relay-metrics-and-log-runtime-projection
plan: 02
subsystem: observability
tags: [block-relay, open-bitcoind, ManagedRpcContext, Phase-121-checker, OBS-03, verify.sh]

requires:
  - phase: 121-01
    provides: set_block_relay_metric_status_provider + Available-gated persist/log projection
provides:
  - open-bitcoind ManagedRpcContext provider wiring with activation outer gate
  - Phase 121 Bun checker + dual verify.sh inclusion
  - OBS-03 Complete with operator-observability runtime-projection note
affects:
  - Phase 121 verification / milestone closeout

tech-stack:
  added: []
  patterns:
    - Arc::clone shared_context before inbound move for second provider closure
    - Activation-gated FieldAvailability outer wrapper over BlockRelayEvidenceStatus

key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/sync_seed.rs
    - scripts/check-phase121-block-relay-metrics-log-runtime.ts
    - scripts/check-phase121-block-relay-metrics-log-runtime.test.ts
  modified:
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - scripts/verify.sh
    - docs/architecture/operator-observability.md
    - docs/parity/source-breadcrumbs.json
    - .planning/REQUIREMENTS.md

key-decisions:
  - "Production source is ManagedRpcContext::block_relay_evidence_status (not sync network)"
  - "Outer Available only when block_serving.activation is Available; reuse BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON"
  - "No sync-disabled twin block-relay metrics worker in Phase 121"

patterns-established:
  - "Pattern: dual-region verify.sh Phase checker wiring after Phase 116 adjacency"
  - "Pattern: checker requires write_block_relay_log_omits_sensitive_markers (D-10)"

requirements-completed: [OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 121-2026-07-14T04-25-57
generated_at: 2026-07-14T07:57:30Z

duration: 37min
completed: 2026-07-14
---

# Phase 121 Plan 02: Daemon Wiring And OBS-03 Closeout Summary

**open-bitcoind now feeds activation-gated ManagedRpcContext block-relay evidence into DurableSyncRuntime, with a Phase 121 Bun checker + dual verify.sh wiring that marks OBS-03 Complete.**

## Performance

- **Duration:** 37 min
- **Started:** 2026-07-14T07:20:38Z
- **Completed:** 2026-07-14T07:57:06Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Wired `set_block_relay_metric_status_provider` beside inbound in `start_daemon_sync_worker`, cloning `shared_context` so both closures own an Arc.
- Outer Available gated on `block_serving.activation`; lock failure / activation-Unavailable use `BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON`.
- Added `check-phase121-block-relay-metrics-log-runtime.ts` (+ tests) including D-10 `write_block_relay_log_omits_sensitive_markers`, dual verify.sh regions, operator-observability Phase 121 note, and OBS-03 Complete.

## Task Commits

Each task was committed atomically:

1. **Task 1: open-bitcoind ManagedRpcContext provider wiring** - (combined feat commit with Task 2)
2. **Task 2: Phase 121 Bun checker + verify.sh + OBS-03 closeout** - (combined feat commit with Task 1)

**Combined feat:** `9d50ae43` — feat(121-02): wire block-relay provider, Phase 121 checker, and OBS-03 closeout

**Plan metadata:** pending docs commit with SUMMARY/STATE/ROADMAP

_Note: Batched Task 1+2 into one feat commit to avoid duplicate full verify hook runs (same posture as 121-01)._

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — provider wiring + sync_seed import
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/sync_seed.rs` — extracted `seed_initial_sync_state` (file-length limit)
- `scripts/check-phase121-block-relay-metrics-log-runtime.ts` — corpus checker
- `scripts/check-phase121-block-relay-metrics-log-runtime.test.ts` — pass/fail fixtures
- `scripts/verify.sh` — visible + run_step Phase 121 entries after Phase 116
- `docs/architecture/operator-observability.md` — Phase 121 runtime projection subsection
- `docs/parity/source-breadcrumbs.json` — sync_seed.rs breadcrumb
- `.planning/REQUIREMENTS.md` — OBS-03 Complete

## Decisions Made

- Production evidence from ManagedRpcContext only (D-01 / Pitfall 2).
- Activation outer gate for Available (D-04 / research A1).
- No twin sync-disabled worker (research Open Question 2).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extracted sync_seed to satisfy 628-line production file limit**
- **Found during:** Task 1 commit verify (`open-bitcoind.rs` 630 lines)
- **Issue:** Provider wiring pushed the binary past the production Rust line limit
- **Fix:** Moved `seed_initial_sync_state` to `open_bitcoind/sync_seed.rs` and registered parity breadcrumbs
- **Files modified:** `open-bitcoind.rs`, `sync_seed.rs`, `source-breadcrumbs.json`
- **Commit:** `9d50ae43`

**2. [Rule 1 - Bug] Provider-setter mutation test needed both sync.rs and metrics.rs edits**
- **Found during:** Task 2 TDD (RED fixture)
- **Issue:** Checker concatenates sync.rs + metrics.rs for provider needles; mutating only metrics.rs left the needle in sync.rs
- **Fix:** Mutation removes the setter string from both fixture files
- **Files modified:** `check-phase121-block-relay-metrics-log-runtime.test.ts`
- **Commit:** `9d50ae43`

## Verification Results

```text
cargo check -p open-bitcoin-rpc --bin open-bitcoind → ok
bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts → 8 pass
bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts → passed
bash scripts/verify.sh (pre-commit) → passed with feat commit 9d50ae43
```

## Known Stubs

None.

## Threat Flags

None — daemon provider surface matches plan threat model (T-121-06/07 mitigated by Unavailable gate + no-claim checker).

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs (`set_block_relay_metric_status_provider`)
- FOUND: scripts/check-phase121-block-relay-metrics-log-runtime.ts (`write_block_relay_log_omits_sensitive_markers`)
- FOUND: scripts/verify.sh (4 Phase 121 matches)
- FOUND: commit 9d50ae43
