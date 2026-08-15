---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "10"
subsystem: mempool-snapshot-recovery
tags: [rust, mempool, snapshot, recovery, resource-bounds]
requires:
  - phase: 135-09
    provides: policy-accounted recovery bounds and the 50,001-record durability regression
provides:
  - finite policy-independent persisted-input decode and topology ceilings
  - current-policy replay and final-membership classification after bounded loading
  - exact encoded, transaction-byte, and input-edge boundary proofs
affects: [phase-135-verification, MPDUR-02, snapshot-recovery]
tech-stack:
  added: []
  patterns:
    - one checked format-owned factory for persisted decode and topology limits
    - current policy retained only for live capture, replay, trimming, and final membership
key-files:
  created:
    - .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-10-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/load_limits.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/gap_closure.rs
    - packages/open-bitcoin-node/src/network/recovery/topology.rs
    - packages/open-bitcoin-node/src/network/recovery/staging.rs
    - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs
key-decisions:
  - "Persisted input uses one finite format-owned contract independent of current live mempool capacity."
  - "The four globally coexisting encoded dimensions remain distinct from per-transaction and input-edge allocation ceilings embedded within the transaction-byte budgets."
  - "The managed network handle remains authoritative for current-policy replay after bounded RPC loading."
patterns-established:
  - "Checked format arithmetic derives all persisted-input counters once and maps them into decode and topology adapters."
  - "Persisted candidates reach fresh current-policy replay before final survivor and DroppedEvicted classification."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-10T03:14:39Z
duration: 15m
completed: 2026-08-09
---

# Phase 135 Plan 10: Persisted-Input Boundary Repair Summary

A checked, policy-independent snapshot input contract now bounds streaming decode and recovery topology before current-policy replay decides final mempool membership.

## Performance

- **Duration:** 15m
- **Started:** 2026-08-10T03:00:16Z
- **Completed:** 2026-08-10T03:14:39Z
- **Tasks:** 3
- **Files modified:** 11, comprising ten Rust implementation/test paths and this summary

## Accomplishments

- Replaced current-policy-derived persisted decode and topology limits with one checked format factory while preserving `MempoolCapacityBounds::from_capacity` on live capture.
- Proved the exact four-dimensional encoded tuple: 268,435,456 encoded bytes, 220,096 records, 5,000 unbroadcast members, and 67,108,864 aggregate transaction bytes. Record 220,097 produces 268,435,968 bytes and fails the encoded budget.
- Kept the 4,194,304-byte per-transaction ceiling and the 1,636,801 aggregate / 102,300 per-record input-edge ceilings as individual allocation bounds within transaction-byte budgets, not as extra dimensions of a false seven-field simultaneous maximum.
- Repaired zero-current-capacity and two-to-one trim recovery so candidates reach replay and final non-survivors are classified `DroppedEvicted`.
- Preserved the 50,001-record Sync/reopen regression and changed the near-capacity Fjall fixture to price its exact canonical entry through `accounted_memory_for_entry`.
- Migrated all node and RPC callers away from `from_policy`; RPC tests prove default, zero, one-byte, and `usize::MAX` current capacities all receive the same finite persisted-input decode contract.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: Add persisted-input boundary proofs** - `e46c799e` (`test`)
2. **Task 1 GREEN: Separate persisted input from live policy** - `703ede93` (`fix`)
3. **Task 2: Migrate RPC persisted-input callers** - `2fb22879` (`fix`)

Task 3 publishes this immutable summary in a separate documentation commit after the self-check.

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` and `snapshot_codec/mempool.rs` - Export and derive the checked seven-field internal contract from format byte constants and the 41-byte minimum serialized input.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs` - Prove exact/one-over encoded, aggregate-edge, per-record-edge, per-transaction, and aggregate transaction-byte behavior.
- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` - Map format-owned limits into the existing bounded streaming decoder.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/load_limits.rs` - Derive live fixture capacity from exact entry accounting.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/gap_closure.rs` - Keep 50,001 records below the persisted encoded, record, and aggregate transaction-byte budgets across Sync/reopen.
- `packages/open-bitcoin-node/src/network/recovery/topology.rs` and `network/recovery/staging.rs` - Bound vertices and checked input-edge accumulation before fresh current-policy replay.
- `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs` - Construct the format contract before bounded loading and preserve handle-owned current-policy staging and consuming install.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs` - Load the clean-shutdown checkpoint through the same format-owned limits.
- `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-10-SUMMARY.md` - Record focused execution evidence without claiming independent phase verification.

## Decisions Made

- The globally coexisting encoded tuple contains only encoded bytes, records, unbroadcast members, and aggregate transaction bytes. Per-transaction and edge limits are individually enforced ceilings derived from those transaction-byte budgets.
- A minimum non-witness input is exactly 41 bytes: a 36-byte outpoint, one-byte empty-script CompactSize, and four-byte sequence. Checked division therefore yields `floor(67_108_864 / 41) = 1_636_801` aggregate edges and `floor(4_194_304 / 41) = 102_300` per-record edges.
- Persisted snapshots carry no historical policy authority. The managed network handle supplies its fresh current policy to staged admission, expiry, trimming, and final membership after the RPC shell completes bounded loading.
- Caller-owned `MempoolSnapshotDecodeLimits::new` remains only in the hostile narrow-limit retention test.

## Verification Evidence

All Cargo commands ran sequentially through `scripts/command-timings.ts` against the checkout-scoped cooperative target lock:

- `phase135-10-node-compile` - passed in 5.207s; the complete `open-bitcoin-node` all-feature test target compiled with `--no-run`.
- `phase135-10-fmt` - passed in 1.028s.
- `phase135-10-clippy` - passed in 13.004s for `open-bitcoin-node` and `open-bitcoin-rpc`, all targets/features, with warnings denied.
- `phase135-10-build` - passed in 10.732s for both affected packages, all targets/features.
- `phase135-10-zero-capacity-replay` - 1 passed in 0.281s.
- `phase135-10-capacity-trim-replay` - 1 passed in 0.137s.
- `phase135-10-accounted-capacity` - 1 passed in 0.692s.
- `phase135-10-persisted-input-limits` - 8 passed in 0.133s, covering the exact formulas and streaming/topology one-over cases.
- `phase135-10-former-50k` - 1 passed in 5.676s with 50,001 records across Sync/reopen.
- `phase135-10-rpc-contract` - 1 passed in 0.247s across all four current-capacity cases.
- `phase135-10-rpc-checkpoint` - 10 passed in 2.324s.
- `phase135-10-rpc-retention` - the hostile narrow-limit snapshot-retention regression passed in 26.552s.
- `git diff --check` passed, and the final search found no node/RPC `MempoolSnapshotDecodeLimits::from_policy` or `RecoveryTopologyLimits::from_policy` call or compatibility constructor.

Per plan, this executor did not run `bash scripts/verify.sh`, did not run the Plan 11 checker/parity/LOC transaction, and did not create canonical `135-VERIFICATION.md`. No Phase 135 pass is claimed here.

## Scope and Protected-Artifact Audit

The union of files in commits `e46c799e`, `703ede93`, and `2fb22879` is exactly the ten Plan 10 implementation/test paths listed in frontmatter. No new Rust file, dependency, checker, parity breadcrumb, LOC artifact, schema version, network endpoint, authentication path, or public operator surface was added.

The protected baseline hashes recorded before implementation remained identical after all source commits:

| Artifact | SHA-256 |
| --- | --- |
| `.planning/ROADMAP.md` | `9e98936c5f6851c60208dedebc142b5a313bd6e245c5938f3ab5c74ec5db2dcb` |
| `.planning/STATE.md` | `005c2964939a1a0eb1807a0d5863e011f6aa8c6ae38b039bc0fa2693c1026d2e` |
| `.planning/config.json` | `8b73424e3ac7e407c58c7197a4fe1d6a86c0692881b28794aa413928cc1fd59e` |
| `.planning/REQUIREMENTS.md` | `bbaac4e3fb9fa65845cccf46a0beda41a20bc25e8e2b0a77e50961c94211dd99` |
| `135-CONTEXT.md` | `386e2b94ca28a6798b10c6406137e2e7c0827b233440eee69744a851216eec1a` |
| `135-RESEARCH.md` | `773fe737d5cd74ea9cfae6857e56b898091427bb4b5a259c7a4a0dd0cc02988b` |
| `135-REVIEW.md` | `1e06ac9121fe0d2f554a9bc6e7b4a95ed8119dd66e845e613a99b9f72dbed5c5` |

Plans and summaries 135-01 through 135-09 were absent from every Plan 10 commit and retained their audited content hashes. Canonical `135-VERIFICATION.md` was absent at start and remains absent. The already-dirty shared planning paths and untracked Plans 08-11 belong to the parent orchestration transaction; Plan 10 neither staged nor committed them.

## Simplification Review

- One `persisted_mempool_input_limits` checked factory owns all persisted decode and topology candidate ceilings.
- One existing `MempoolCapacityBounds::from_capacity` call continues to own live capture bounds.
- No duplicated limit arithmetic, fixed 50,000/1,600,000 cap, saturation, compatibility wrapper, historical-policy field, or inferred persisted policy remains.
- The Fjall and RPC shells only map the format contract into bounded I/O; replay decisions stay in the existing mempool core.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Compatibility] Preserved existing missing-parent classification at the per-record topology boundary**

- **Found during:** Task 1 focused topology regression
- **Issue:** Returning `ResourceBoundExceeded` directly for every per-record input-edge overage changed an existing recovery fixture that intentionally classifies its oversized dependency corpus as `DroppedMissingParent`.
- **Fix:** Kept the checked per-record pre-allocation guard while preserving the existing missing-parent outcome for that recovery path; dedicated exact/one-over topology tests still prove both edge ceilings fail closed.
- **Files modified:** `packages/open-bitcoin-node/src/network/recovery/topology.rs`
- **Verification:** The existing topology regression and all eight `persisted_input_limits` tests pass.
- **Committed in:** `703ede93`

**Total deviations:** 1 auto-fixed bug compatibility issue. The fix preserved existing behavior without adding scope or weakening the planned allocation bound.

## Issues Encountered

- The RPC helper's borrowed `PolicyConfig` is not the source of staging policy: the existing `ManagedNetworkHandle::prepare_mempool_recovery_at` API reads the authoritative current policy from the handle. The parameter was retained for its existing caller contract but deliberately excluded from persisted decoding; node replay regressions prove the current handle policy still determines final membership.
- The obsolete policy-overflow retention test was converted to an explicit hostile zero-limit decoder fixture. That storage-layer bound failure correctly records `StoreCorruption` through the existing `StorageError::recovery_category` mapping and leaves the snapshot intact.
- Executor commits used `--no-verify` because the canonical verification report is intentionally quarantined during parallel execution. Every focused Plan 10 gate still ran manually and passed.

## Known Stubs

None. The changed paths contain no placeholder runtime or UI data flow.

## Security and Threat Review

- Untrusted Fjall bytes are rejected by encoded-size preflight and streaming count, per-transaction, and aggregate transaction-byte counters before unbounded allocation.
- Recovery topology uses finite vertices plus checked per-record and aggregate input-edge counters before graph growth.
- Persisted input supplies no policy authority; authenticated chainstate and fresh live policy still determine admission, expiry, trimming, final membership, rebuilt projections, and atomic install.
- Errors remain low-cardinality and do not expose raw snapshot bytes, transactions, or identities. No unplanned threat surface was introduced beyond the plan's registered storage and topology boundaries.

## User Setup Required

None - no external service configuration or credentials are required.

## Deferred Issues

- WR-01 remains a non-blocking deferred warning: current live preflight is unconditional, while outer-cfg mutation hardening remains outside this gap closure.

## Next Phase Readiness

- Plan 10's focused compiling evidence is ready for Plan 11's independent checker, parity, generated-LOC, and full verifier transaction.
- Phase status, roadmap/state mutation, requirement promotion, and canonical `135-VERIFICATION.md` remain owned by the parent orchestrator and Plan 11 verifier.

## Self-Check: PASSED

All ten authorized Rust paths exist, this summary exists, and commits `e46c799e`, `703ede93`, and `2fb22879` resolve in repository history. The implementation commit union is exactly the Plan 10 frontmatter inventory; protected artifacts are absent from every Plan 10 commit, their recorded baseline hashes are unchanged, canonical `135-VERIFICATION.md` remains absent, and no goal-blocking stub or unplanned threat surface remains.
