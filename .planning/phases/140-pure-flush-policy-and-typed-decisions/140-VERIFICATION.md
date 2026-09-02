---
phase: 140-pure-flush-policy-and-typed-decisions
verified: 2026-09-02T10:35:00Z
status: passed
score: 18/18 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 140-2026-09-02T02-14-08
generated_at: 2026-09-02T10:35:00Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 140: Pure Flush Policy and Typed Decisions Verification Report

**Phase Goal:** Flush and recovery decisions are a typed, injectable policy the later manager can execute without clocks or I/O in core.
**Verified:** 2026-09-02T10:35:00Z
**Status:** passed
**Re-verification:** No — initial verification

Provenance: `140-CONTEXT.md`, `140-01-PLAN.md`, `140-02-PLAN.md`, `140-03-PLAN.md`, and all three SUMMARYs share `lifecycle_mode: yolo` and `phase_lifecycle_id: 140-2026-09-02T02-14-08`. None are `direct-fallback`.

## Goal Achievement

The phase goal holds in the codebase. `open-bitcoin-chainstate` owns an I/O-free `decide_flush` / `decide_recovery` machine. Adapters inject occupancy, time, pressure, disk-free, and head-marker count. Core returns exclusive `FlushDecision` / `RecoveryDecision` values. Node persist still writes leftover snapshots and does not call the policy. Phase 142 owns IfNeeded / Periodic / Always execution.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Given injected cache-size, time, disk-space, and mode facts, the node decides None, IfNeeded, Periodic, or Always without reading a clock or disk itself. | ✓ VERIFIED | `FlushPolicyInput` carries `mode`, cache bytes/limit/leftover, `now`/`next_write`, `memory_pressure`, `disk_free_bytes`, and `cache_entry_count`. `decide_flush` matches on those facts only. `flush.rs` has no `Instant`, `SystemTime`, `std::fs`, `fjall`, or `tokio`. |
| 2 | Disk-space refusal is a first-class decision outcome, not a later adapter surprise. | ✓ VERIFIED | Exclusive `FlushDecision::RefuseDiskSpace` with `LastFlushReason::FailedDisk`. Write variants cannot carry a refuse flag. Guard is `disk_free_bytes < 192 * entry_count` after an intended write. |
| 3 | Cache-size state is classified as OK, LARGE, or CRITICAL from injected occupancy facts. | ✓ VERIFIED | `classify_cache_size` uses saturating total and `u128` 90% math with strict `>`. None-mode tests bind 50 MiB at 90%, 200 MiB at `total - 10 MiB`, leftover-added total, and CRITICAL over cap. |
| 4 | Flush and Sync remain distinct decisions, and FlushForPrune is not implemented. | ✓ VERIFIED | Exclusive `Flush` vs `Sync` variants. Private `FlushWriteKind` maps Periodic LARGE/CRITICAL → Flush and Periodic OK due → Sync. `rg` finds no `FlushForPrune` in `flush.rs`. |
| 5 | Contributors can construct `FlushPolicyInput` from injected cache bytes, limits, leftover, `FlushPolicyTime` now/next_write, memory_pressure, disk-free bytes, and entry count without `Instant::now()` or disk I/O. | ✓ VERIFIED | Public struct fields plus `FlushPolicyTime::from_unix_seconds`. `periodic_due` is `now >= next_write`. Tests construct inputs with injected unix seconds. |
| 6 | None-mode `decide_flush` classifies OK at the 90% threshold on a 50 MiB budget, LARGE one byte over that threshold, and CRITICAL when cache_bytes exceed total space. | ✓ VERIFIED | Tests `none_mode_classifies_ok_at_small_budget_ninety_percent_threshold`, `..._large_one_byte_over_small_budget_ninety_percent`, `..._critical_when_cache_bytes_exceed_total` passed. |
| 7 | None-mode `decide_flush` classifies OK at 180 MiB and at 190 MiB on a 200 MiB budget, and LARGE only one byte over total-minus-10-MiB headroom. | ✓ VERIFIED | Tests `none_mode_classifies_ok_at_180_mib_on_200_mib_budget`, `..._ok_at_exactly_total_minus_10_mib_on_200_mib_budget`, `..._large_one_byte_over_headroom_on_200_mib_budget` passed. |
| 8 | None mode still classifies cache size and never returns `RefuseDiskSpace`, even when `disk_free_bytes` is 0. | ✓ VERIFIED | `FlushMode::None` short-circuits in `maybe_intended_write`. Tests `none_mode_never_refuses_disk_even_when_free_bytes_are_zero` and `none_mode_critical_with_pressure_and_low_disk_still_returns_none` passed. |
| 9 | `open-bitcoin-chainstate` `coins/flush.rs` stays I/O-free: no Fjall, `std::fs`, Tokio, Instant, or SystemTime. | ✓ VERIFIED | Source grep empty. Crate `Cargo.toml` depends only on `open-bitcoin-consensus` and `open-bitcoin-primitives`. Purity test passed. |
| 10 | Always mode returns Flush with `LastFlushReason::Always`, or `RefuseDiskSpace` with `FailedDisk` when free_bytes is below `192 * entry_count`. | ✓ VERIFIED | Tests `always_mode_returns_flush_with_reason_always`, `always_mode_refuses_disk_when_free_below_192_times_entries`, and `always_mode_writes_flush_when_free_equals_192_times_entries` passed. Guard uses strict `<`. |
| 11 | Periodic + LARGE or CRITICAL returns Flush even when `periodic_due` is false; Periodic + OK + due returns Sync; Periodic + OK + not due returns None. | ✓ VERIFIED | Tests `periodic_large_returns_flush_even_when_not_due`, `periodic_critical_returns_flush_even_when_not_due`, `periodic_ok_due_returns_sync`, `periodic_ok_not_due_returns_none` passed. |
| 12 | IfNeeded + CRITICAL, or IfNeeded + memory_pressure, returns Flush with `LastFlushReason::Needed`. | ✓ VERIFIED | Tests `ifneeded_critical_returns_flush_with_reason_needed` and `ifneeded_ok_with_memory_pressure_returns_flush` passed. |
| 13 | IfNeeded + LARGE without CRITICAL or pressure returns None even when `disk_free_bytes` is 0. | ✓ VERIFIED | Tests `ifneeded_large_without_pressure_returns_none` and `ifneeded_large_with_low_disk_still_returns_none` passed. No `large_always_flushes` test or rule. |
| 14 | Disk-space refusal is checked only after a write would otherwise happen and replaces Flush or Sync; it never coexists with a write. | ✓ VERIFIED | `decide_flush` returns None before the guard when `maybe_intended_write` is empty, then maybe `RefuseDiskSpace`, else Flush/Sync. Exclusive enum. Periodic OK due + 1919 bytes refuses instead of Sync. |
| 15 | `decide_recovery` maps head-marker count 0/1/2/other to `ConsistentEmptyHeads` / `OneHead` / `InterruptedTwoHeads` / `InconsistentOtherCount` without hashes or I/O. | ✓ VERIFIED | Four-arm match in `decide_recovery`. One-element is `OneHead`, not `InconsistentOtherCount { count: 1 }`. Count-only; no `BlockHash` or `ReplayBlocks`. |
| 16 | Crate-root `open-bitcoin-chainstate` re-exports `FlushDecision`, `decide_flush`, `RecoveryDecision`, and `decide_recovery` next to `CoinsCache`. | ✓ VERIFIED | `lib.rs` `pub use coins::{..., FlushDecision, RecoveryDecision, decide_flush, decide_recovery}`. Test `crate_root_reexports_flush_and_recovery_types` constructs through `crate::`. `classify_cache_size` stays crate-private. |
| 17 | `ManagedChainstate::persist` still calls `save_snapshot` and does not call `decide_flush`. | ✓ VERIFIED | `packages/open-bitcoin-node/src/chainstate.rs` lines 249–251 write `self.store.save_snapshot(...)`. Grep for `decide_flush` / `RefuseDiskSpace` in that file is empty. Guard test passed. |
| 18 | `DurableSyncRuntime::persist_progress` still writes `save_chainstate_snapshot` and does not call `decide_flush`. | ✓ VERIFIED | `runtime_state.rs` lines 88–94 still save header entries and the chainstate snapshot. Grep for `decide_flush` / `FlushMode` is empty. Guard test passed. |

**Score:** 18/18 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `packages/open-bitcoin-chainstate/src/coins/flush.rs` | Types, `classify_cache_size`, Knots `decide_flush`, `decide_recovery` | ✓ VERIFIED | 231 lines. Exists, substantive, wired from `coins.rs` and `lib.rs`. gsd-tools artifacts 3/3 + 2/2 + 3/3 passed. |
| `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` | Classification, mode matrix, recovery, persist guards | ✓ VERIFIED | 745 lines, 36 tests, all passing. |
| `packages/open-bitcoin-chainstate/src/lib.rs` | Crate-root flush and recovery re-exports | ✓ VERIFIED | `pub use coins` includes `decide_flush`, `decide_recovery`, `FlushDecision`, `RecoveryDecision`. |
| `packages/open-bitcoin-chainstate/src/coins.rs` | `mod flush` plus public re-exports | ✓ VERIFIED | `mod flush;` and `pub use flush::{...}`. Does not re-export `classify_cache_size`. |
| `docs/parity/source-breadcrumbs.json` | `chainstate-engine` files for flush sources | ✓ VERIFIED | Lists `coins/flush.rs` and `coins/tests/flush.rs`. |
| `packages/open-bitcoin-node/src/chainstate.rs` | Leftover persist unchanged | ✓ VERIFIED | Read-only guard: still `save_snapshot`, no policy call. |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | Leftover persist unchanged | ✓ VERIFIED | Still `save_chainstate_snapshot`, no `decide_flush`. |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| `coins.rs` | `coins/flush.rs` | `mod flush;` plus public re-exports | WIRED | Pattern present; `pub use flush::{...}`. |
| `coins/flush.rs` | `coins/tests/flush.rs` | None-mode `decide_flush` returns classified cache_size | WIRED | Tests call `crate::coins::decide_flush` and assert `cache_size()`. |
| `coins/flush.rs` | `coins/flush.rs` | classify → intended write → maybe `RefuseDiskSpace` | WIRED | `decide_flush` body matches the planned shape. |
| `coins/tests/flush.rs` | `coins/flush.rs` | IfNeeded LARGE is not a write trigger | WIRED | `ifneeded_large_without_pressure_returns_none` exists and passed. |
| `lib.rs` | `coins/flush.rs` | `pub use coins::{..., RecoveryDecision, decide_flush, decide_recovery}` | WIRED | Crate-root export list includes all planned names. |
| `coins/tests/flush.rs` | `open-bitcoin-node/src/chainstate.rs` | `include_str` persist still `save_snapshot` | WIRED | Guard test passed against live persist source. |

gsd-tools key-links: 2/2, 2/2, and 2/2 verified.

### Data-Flow Trace (Level 4)

Not a UI phase. Policy data still flows from injected facts, not hardcoded empties.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `decide_flush` | `FlushPolicyInput` fields | Caller-injected occupancy, time, pressure, disk facts | Yes — used by `classify_cache_size`, `maybe_intended_write`, `disk_guard_fails` | ✓ FLOWING |
| `FlushDecision` | `cache_size`, `reason` | Shared `FlushDecisionFacts` on every variant | Yes — accessors read the same payload | ✓ FLOWING |
| `decide_recovery` | `head_marker_count` | Injected `usize` | Yes — four-arm match returns count-bearing `InconsistentOtherCount` | ✓ FLOWING |

`decide_flush` is intentionally not called from node persist (D-22). That is a leftover-snapshot guard, not a hollow production path for this phase.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Flush policy unit suite | `cargo test -p open-bitcoin-chainstate --lib coins::tests::flush` | `36 passed; 0 failed` in 0.00s (already built) | ✓ PASS |
| Forbidden I/O / prune tokens in `flush.rs` | `rg Instant\|SystemTime\|std::fs\|fjall\|tokio\|FlushForPrune\|ReplayBlocks\|BlockHash\|empty_cache\|disk_ok\|52428800` | No matches | ✓ PASS |
| Node persist not retargeted | `rg decide_flush\|RefuseDiskSpace\|FlushMode` on persist files | No matches; `save_snapshot` / `save_chainstate_snapshot` still present | ✓ PASS |
| Named matrix tests exist | `-- --list` | 36 tests including Always/Periodic/IfNeeded/recovery/persist guards | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| **FLUSH-01** | 140-01, 140-02 | Node flushes coins using IfNeeded, Periodic, and Always policy, including disk-space refusal, from injected cache, time, and disk facts. | ✓ SATISFIED | Phase contract is the injectable decision surface: mode matrix, first-class `RefuseDiskSpace`, no clock/disk in core. REQUIREMENTS.md checkbox is already Complete. Actual IfNeeded-after-connect / Periodic-tick / Always-on-shutdown I/O is Phase 142 success criterion 3, not a Phase 140 gap. |
| **MGR-03** | 140-01, 140-03 | Flush and recovery decisions are a typed pure-core state machine; adapters perform I/O. | ✓ SATISFIED | `FlushDecision` + `RecoveryDecision` live in `open-bitcoin-chainstate`. Crate root exports both machines. Adapters still perform leftover snapshot I/O and do not sample clocks inside core. REQUIREMENTS.md / ROADMAP coverage still list MGR-03 Pending until this verification exists — tracker lag, not missing code. |

Orphaned Phase 140 requirements: none. REQUIREMENTS.md maps only FLUSH-01 and MGR-03 to this phase; both appear in plan frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | No TODO/FIXME/placeholder/stub returns in `flush.rs` or flush tests | — | None |
| `open-bitcoin-node` persist files | — | `decide_flush` unused by production persist | ℹ️ Info | Intentional D-22 leftover-snapshot guard. Phase 142 wires flush points. |

Confirmation-bias notes (do not fail the goal):
- `classify_does_not_panic_when_limit_and_leftover_are_u64_max` only asserts a return, not a specific size class.
- Always + `cache_entry_count == 0` never refuses (`required == 0`). Not a stated must-have.

### Human Verification Required

None. This is a pure-core decision crate with executable unit coverage. No UI, external service, or operator-flow check is required for the phase goal.

### Gaps Summary

No actionable gaps. Later phases own execution and encoding that this phase explicitly deferred:

- Phase 141 — `head_blocks` encoding and Fjall coins records
- Phase 142 — manager flush call sites, ordered flush, replay versus fail-closed
- Phase 144 — operator cache-size / last-flush-reason surfaces

Those are roadmap-owned follow-ons, not failed Phase 140 truths.

---

_Verified: 2026-09-02T10:35:00Z_
_Verifier: Claude (gsd-verifier)_

## VERIFICATION PASSED
