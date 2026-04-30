---
phase: 20-wallet-runtime-expansion
plan: 02
subsystem: wallet
tags: [wallet, fjall, rescan, descriptors, persistence]
requires:
  - phase: 20-01
    provides: ranged descriptor cursor state, send-intent contracts, wallet rescan progress math
provides:
  - durable named-wallet registry records in the node wallet namespace
  - selected-wallet metadata and per-wallet rescan checkpoint persistence
  - restart-time bounded wallet rescan recovery against durable chainstate
affects: [20-03, 20-04, 20-05, rpcwallet, wallet-status]
tech-stack:
  added: []
  patterns: [fjall wallet namespace records, bounded restart-safe wallet rescans, typed node-shell wallet selection errors]
key-files:
  created: [packages/open-bitcoin-node/src/wallet_registry.rs]
  modified:
    - packages/open-bitcoin-node/src/wallet.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Persist wallet registry membership, selected-wallet metadata, and rescan checkpoints as separate records in the existing Fjall wallet namespace instead of inventing a second store."
  - "Resume wallet rescans by replaying bounded height windows from durable chainstate snapshots and checkpoint after each chunk."
  - "Normalize stored `#ob::` ranged-descriptor metadata during node snapshot decode so Plan 20-01 snapshots remain reloadable without touching wallet-core files outside this plan."
patterns-established:
  - "Wallet selection and duplicate-name validation live in open-bitcoin-node wallet_registry shell types, not in open-bitcoin-wallet."
  - "Restart-safe wallet rescans reload persisted jobs on runtime open and advance one bounded chunk per resume cycle."
requirements-completed: [WAL-05, WAL-06, WAL-07]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-04-27T11:10:10Z
duration: 17 min
completed: 2026-04-27
---

# Phase 20 Plan 02: Wallet Runtime Expansion Summary

**Durable named-wallet registry records, selected-wallet metadata, and restart-safe bounded wallet rescans in the existing Fjall node store**

## Performance

- **Duration:** 17 min
- **Started:** 2026-04-27T10:53:00Z
- **Completed:** 2026-04-27T11:10:10Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- Added a new `wallet_registry` shell module with typed duplicate-wallet, unknown-wallet, and stale-selection errors plus persisted per-wallet rescan job records.
- Extended Fjall wallet persistence and snapshot codecs to round-trip named wallet snapshots, selected-wallet metadata, ranged descriptor cursor state, and rescan checkpoints across reopen.
- Added a restart-time wallet rescan runtime in `sync.rs` that reloads persisted jobs, advances them in bounded chunks, checkpoints progress, and preserves completion state for downstream consumers.

## Task Commits

The plan's TDD tasks were delivered in one passing code commit because repo-local Rust commit rules require full green verification before every commit.

1. **Task 1 + Task 2: durable wallet registry, checkpoint persistence, and restart recovery** - `0d289bf` (feat)

**Plan metadata:** pending docs commit

## Files Created/Modified
- `packages/open-bitcoin-node/src/wallet_registry.rs` - named-wallet registry contracts, typed errors, and registry persistence orchestration
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - named wallet, selected-wallet, and rescan-job record persistence in Fjall
- `packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs` - schema-versioned wallet registry and rescan checkpoint DTOs
- `packages/open-bitcoin-node/src/sync.rs` - restart-time wallet rescan runtime and bounded chainstate replay
- `packages/open-bitcoin-node/src/wallet.rs` - managed wallet cursor persistence helpers for ranged address allocation
- `docs/parity/source-breadcrumbs.json` - parity breadcrumb coverage for the new node wallet registry file

## Decisions Made

- Kept all new wallet runtime durability inside the existing Fjall wallet namespace so WAL-05 through WAL-07 stay on the project storage ADR.
- Modeled restart recovery around height-windowed replays of durable chainstate because the current runtime persists UTXO/tip state, not historical wallet event logs.
- Repaired the node-side wallet snapshot decoder to accept stored `#ob::` range metadata emitted by Plan 20-01, preserving restart compatibility without expanding this plan outside its write set.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Repaired ranged descriptor snapshot reload compatibility**
- **Found during:** Task 2 (durable named-wallet registry and restart-safe rescan checkpoint persistence)
- **Issue:** Reloading a ranged wallet snapshot failed because stored descriptor text used `#ob::...` metadata while the parser accepted `#ob:...`.
- **Fix:** Normalized the stored range metadata in the node snapshot codec before reparsing the descriptor while preserving the original persisted text.
- **Files modified:** `packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage:: -- --nocapture`
- **Committed in:** `0d289bf`

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** The auto-fix was required for restart-safe ranged descriptor persistence and kept the plan inside the node-owned write set.

## Issues Encountered

- Repo-local Rust commit rules conflict with a committed RED-state TDD step, so the failing-test phase stayed local only and the deliverable code landed as one green feature commit.
- `bash scripts/verify.sh` remains blocked by the pre-existing stale LOC report at `docs/metrics/lines-of-code.md`, which is outside Plan 20-02's owned write set and is tracked in `deferred-items.md`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 20 now has durable wallet selection and restart-safe rescan state ready for `-rpcwallet` routing and wallet-scoped RPC work in Plan 20-03.
- The repo-native verify script still needs the LOC report refreshed by a plan that owns `docs/metrics/lines-of-code.md`.

## Self-Check: PASSED

- Found summary artifact: `.planning/phases/20-wallet-runtime-expansion/20-02-SUMMARY.md`
- Found code commit: `0d289bf`
