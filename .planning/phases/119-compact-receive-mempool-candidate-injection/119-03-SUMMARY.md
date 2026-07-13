---
phase: 119-compact-receive-mempool-candidate-injection
plan: 03
subsystem: network
tags: [compact-block, mempool, bip152, reconstruction, lifecycle]

requires:
  - phase: 119-01
    provides: PeerManager::on_mempool_transaction_removed forwarder
  - phase: 119-02
    provides: ManagedPeerNetwork CompactBlock intercept with mempool+extra facts
provides:
  - Mempool lifecycle wtxid hooks clearing volatile partial compact slots
  - Runtime injected-path proofs for reconstruction, collision, duplicate, missing, lifecycle, GOV-04
  - Knots-aligned parity breadcrumbs for Phase 119 receive/lifecycle seams
affects:
  - Phase 120 compact-download timeout scheduling
  - Phase 121 DurableSyncRuntime block-relay metrics

tech-stack:
  added: []
  patterns:
    - Connected-block and admission exits forward removal wtxid before serving demotion
    - Live receive_message CompactBlock suite proves RCN-02/RCN-03/GOV-04 without empty facts

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Forward removal.wtxid before TxServing demotion on connected-block lifecycle (D-07)"
  - "Evicted/Expired use outcome.maybe_wtxid(); replaced/evicted victims resolve via relay_serving before demotion; never hook admitted Replaced wtxid"
  - "Explicit phase119_live_receive_duplicate_short_ids_are_typed_not_silent proves D-09.2 duplicate on injected path (not Phase 115 blocktxn DuplicateResponse)"

patterns-established:
  - "Pattern: shell mempool exits call PeerManager::on_mempool_transaction_removed with wtxid only"
  - "Pattern: compact_receive_cases exercise ManagedPeerNetwork::receive_message CompactBlock, not empty-facts handle_message"

requirements-completed: [RCN-02, RCN-03, GOV-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 119-2026-07-13T16-08-52
generated_at: 2026-07-13T19:01:30Z

duration: 10min
completed: 2026-07-13
---

# Phase 119 Plan 03: Lifecycle Hook and Runtime Proofs Summary

**Mempool-removal wtxid hooks clear in-flight compact partial slots, with live injected-path tests covering reconstruction, collision, duplicate, missing, lifecycle cleanup, and untouched package/filter defaults.**

## Performance

- **Duration:** 10 min
- **Started:** 2026-07-13T18:50:21Z
- **Completed:** 2026-07-13T19:01:19Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Hooked `apply_connected_block_mempool_lifecycle`, Evicted/Expired admission exits, and replaced/evicted victims into `PeerManager::on_mempool_transaction_removed`.
- Expanded `compact_receive_cases` with Phase 119 runtime proofs including an explicit duplicate short-id typed-failure test.
- Aligned parity breadcrumbs to Knots InitData / compact receive anchors; confirmed no Phase 120 timeout or Phase 121 DurableSyncRuntime metric wiring.

## Task Commits

Each task was committed atomically:

1. **Task 1: Hook mempool lifecycle removals into PeerManager forwarder** - `5c7b0662` (test) → `f77f99ec` (feat)
2. **Task 2: Runtime injected-path reconstruction and RCN-03 outcomes** - `68f3bfd9` (test)
3. **Task 3: Parity breadcrumbs and focused verification** - `5078b7c8` (chore)

**Plan metadata:** `7b513bd9` (docs: complete plan)

_Note: Task 1 used TDD RED → GREEN. Task 2 tests landed against the Plan 02 inject path plus Task 1 hooks._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — connected-block and reorg Evicted/Expired wtxid forwarder
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` — Evicted/Expired and replaced/evicted-victim wtxid forwarder
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` — connected-block slot-clear proof
- `packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs` — injected-path RCN-02/RCN-03/GOV-04 suite
- `docs/parity/source-breadcrumbs.json` — compact-receive + lifecycle Knots anchors

## Decisions Made

- Forward connected-block `removal.wtxid` before TxServing cleanup so compact slots clear even when serving records are demoted.
- Resolve replaced/evicted victim wtxids from relay serving (or local tx cache) before demotion; never treat the admitted Replaced wtxid as a removal.
- Make duplicate proof explicit via `phase119_live_receive_duplicate_short_ids_are_typed_not_silent` on the live receive path.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None beyond arranging the lifecycle fixture so the announced compact partial stays in-flight (matched + missing short IDs) while a conflict block removes the matched mempool tx.

## User Setup Required

None

## Known Stubs

None

## Threat Flags

None — lifecycle hook clears only matching wtxid slots; no new network endpoints or package/filter activation (T-119-09..T-119-12 mitigated as planned).

## Next Phase Readiness

Phase 119 complete. Ready for Phase 120 (compact-download timeout scheduling / misbehavior) or milestone verification; DurableSyncRuntime block-relay metrics remain Phase 121.

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib mempool_lifecycle` — pass (7)
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib compact_receive` — pass (14)
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib on_mempool_transaction_removed` — pass
- `bun scripts/check-parity-breadcrumbs.ts` — pass
- No `expire_compact_download_timeouts` calls under `packages/open-bitcoin-node/src/network`
- No DurableSyncRuntime block-relay metric projection added

## Self-Check: PASSED

- Modified key files present on disk
- Commits `5c7b0662`, `f77f99ec`, `68f3bfd9`, `5078b7c8`, `7b513bd9` present in git log
- Acceptance must_haves verified (lifecycle wtxid hook, runtime suite, breadcrumbs, deferred surfaces untouched)
