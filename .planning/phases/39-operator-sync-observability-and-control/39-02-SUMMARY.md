---
phase: 39-operator-sync-observability-and-control
plan: "02"
subsystem: rpc-cli-sync-control
tags: [rust, json-rpc, operator-cli, daemon-sync, fjall]
requires:
  - phase: 39-01
    provides: daemon-owned durable sync runtime state and operator sync command surface
provides:
  - Open Bitcoin sync-control RPC methods for status, pause, and resume
  - daemon-owned sync-control port that keeps Fjall store ownership in open-bitcoind
  - RPC-first operator sync commands with offline direct-store fallback
  - locked-store and auth-failure regressions for live daemon control
affects: [phase-39, operator-sync, rpc, cli, live-mainnet-uat]
tech-stack:
  added: []
  patterns:
    - daemon-owned store-backed control port for live sync mutation
    - terminal fallback only for unreachable or unconfigured local RPC
key-files:
  created: []
  modified:
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/node.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - docs/parity/catalog/rpc-cli-config.md
key-decisions:
  - "Live open-bitcoin sync status/pause/resume now use authenticated local RPC before direct store access."
  - "Reachable daemon authentication, HTTP, and JSON-RPC failures are terminal instead of falling back to direct Fjall mutation."
  - "Offline direct-store sync control remains available only when local RPC is unavailable or not configured."
patterns-established:
  - "Daemon-owned mutable state should expose local RPC/control ports rather than requiring second-process Fjall opens."
  - "Operator fallbacks must not downgrade reachable authenticated daemon failures into unauthenticated local mutations."
requirements-completed: [PEERMAIN-04, CHAINMAIN-05, RESUME-04, OBSMAIN-01, OBSMAIN-02, OBSMAIN-03, OBSMAIN-04, VERMAIN-01, VERMAIN-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: "39-2026-05-02T11-46-08"
generated_at: "2026-05-23T01:37:12.684Z"
duration: "about 45 min"
completed: 2026-05-23
---

# Phase 39 Plan 02 Summary

**Live operator sync status, pause, and resume now route through authenticated daemon RPC instead of opening the daemon-owned Fjall store from a second process.**

## Performance

- **Duration:** about 45 min
- **Started:** 2026-05-23T00:52:00Z
- **Completed:** 2026-05-23T01:37:12Z
- **Tasks:** 3
- **Files modified:** 17 source/doc files plus planning artifacts

## Accomplishments

- Added node-scoped Open Bitcoin extension RPC methods: `openbitcoinsyncstatus`, `openbitcoinsyncpause`, and `openbitcoinsyncresume`.
- Added daemon sync-control plumbing so `open-bitcoind` remains the sole process owner of the durable sync runtime and Fjall store while local RPC can read or mutate sync control.
- Reworked the live daemon control handle to answer RPC status/pause/resume from a clone of the daemon process's already-open Fjall handle, so control does not wait for the sync worker to exit a live network round.
- Changed `open-bitcoin sync status|pause|resume` to try authenticated local RPC first and preserve offline direct-store behavior when no daemon is reachable.
- Added hermetic regressions for the reported Fjall `Locked` failure and for auth failures that must not fall back to direct-store mutation.
- Updated the parity catalog to list the new Open Bitcoin extension RPC methods.

## Task Commits

No task commits were created during inline execution. Changes remain in the working tree for review.

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/method.rs` - Registers and normalizes the new sync-control RPC methods.
- `packages/open-bitcoin-rpc/src/method/node.rs` - Defines sync-control request and response contracts.
- `packages/open-bitcoin-rpc/src/context.rs` - Adds daemon sync-control backends and context accessors.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Routes sync-control RPC calls to the daemon-owned control port.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Starts the sync worker with a cloneable store-backed control handle without releasing store ownership.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Allows cloning the in-process Fjall store handle so daemon RPC control can share the daemon-opened database without a second process open.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Makes sync commands RPC-first with direct-store fallback only for unreachable or unconfigured RPC.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Covers locked-store live RPC behavior and terminal auth failure behavior.
- `docs/parity/catalog/rpc-cli-config.md` and `docs/parity/index.json` - Record the new Open Bitcoin-only RPC extensions.
- `docs/metrics/lines-of-code.md` - Refreshed by the repo-native LOC generator required by verification.

## Decisions Made

- The live control path uses local JSON-RPC over the existing authenticated daemon server instead of a sidecar store.
- Direct Fjall access remains a stopped-daemon/offline compatibility path, not a live-daemon control path.
- HTTP 401/403, non-200 daemon responses, JSON-RPC errors, and malformed live RPC responses fail the command rather than falling back to direct-store mutation.

## Deviations from Plan

- `packages/open-bitcoin-cli/src/client.rs` also needed an exhaustive `MethodCall` serialization update for the new RPC variants.
- The parity catalog and tracked LOC report were updated because repo guidance requires extension-surface documentation and generated artifact freshness.
- The stale May 2 verification artifact was archived to `39-VERIFICATION.before-gap-plan.md` before execution so the GSD pre-execution lifecycle gate could validate the new gap plan.

## Issues Encountered

- The first full `bash scripts/verify.sh` run stopped because `docs/metrics/lines-of-code.md` was stale. Regenerating it with `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` resolved the issue, and the final verify run passed.
- The first live retest reached daemon RPC but failed with `daemon sync control timed out`. The root cause was waiting on the busy sync worker thread; switching the daemon RPC control handle to the daemon process's already-open store handle resolved the timeout in a local live-shape daemon check.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The deterministic lock regression is fixed and repo verification passes. The original live mainnet UAT should be rerun against a fresh `open-bitcoind` process to confirm the same behavior through the real daemon instead of the hermetic fake RPC server.

---
*Phase: 39-operator-sync-observability-and-control*
*Completed: 2026-05-23*
