---
phase: 142-manager-flush-lifecycle-and-restart
verified: 2026-09-07T02:00:00Z
status: passed
score: 5/5 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 142-2026-09-06T16-23-52
generated_at: 2026-09-07T02:00:00Z
lifecycle_validated: true
overrides_applied: 0
re_verification: false
---

# Phase 142: Manager Flush Lifecycle and Restart Verification Report

**Phase Goal:** One manager owns coins init, flush points, interrupted-flush recovery, and restart from durable coins best-block.
**Verified:** 2026-09-07T02:00:00Z
**Status:** passed
**Re-verification:** No — initial verification

Provenance: `142-CONTEXT.md`, all six PLAN files, and all six SUMMARY files share `lifecycle_mode: yolo` and `phase_lifecycle_id: 142-2026-09-06T16-23-52`.

## Goal Achievement

### Observable Truths

Roadmap success criteria are the contract. PLAN frontmatter truths were merged as supporting detail and did not subtract scope.

| #   | Truth   | Status     | Evidence       |
| --- | ------- | ---------- | -------------- |
| 1   | One manager initializes the coins database, health-check, and cache, and reports CanFlush-style readiness only after that sequence for the single active chainstate. | ✓ VERIFIED | `initialize` in `flush_lifecycle.rs` starts `NotReady`, opens `FjallCoinsView`, runs `decide_recovery`, replays or fail-closes, then `CoinsCache::from_parent` and `ReadyToFlush`. `execute_flush` refuses before ready. Production open stores that lifecycle via `from_chainstate`. Dual-chainstate stays out of scope (D-12). |
| 2   | After same-datadir restart, tip and UTXO view come from durable coins best-block, not a leftover snapshot blob. | ✓ VERIFIED | `open_runtime.rs` calls `initialize` + `Chainstate::from_coins_cache` + three-arg `from_chainstate`. It does not call `hydrate_chainstate_for_open`. `FjallChainstateStore::load_snapshot` returns `None`; `save_snapshot` is a no-op. Restart test `same_datadir_reopen_tip_matches_coins_best_block` asserts tip == coins `B` and leftover UTXOs stay unread. |
| 3   | The node flushes IfNeeded after connect/reorg, Periodic on injected ticks, and Always on shutdown, writing block, undo, and index before coins. | ✓ VERIFIED | `persist()` always calls `execute_flush(IfNeeded)` from connect/disconnect/reorg/commit paths. Daemon `coins_flush.rs` drives `FlushMode::Periodic` on ticks and `FlushMode::Always` on shutdown through `ManagedNetworkHandle`. `persist_ordered_prefix` writes block → undo → headers before `cache.flush`/`sync`. Undo-fail abort test proves coins/`B` do not advance. |
| 4   | A mid-flush crash is recovered by interrupted-flush replay from stored undo and block bodies, or fails closed without inventing a consistent tip. | ✓ VERIFIED | Two-element `H` without `B` is `MarkerState::Interrupted` and `head_blocks() -> Ok([new, old])`. `ensure_schema_two` classifies interrupted `H` before leftover-empty (WR-02). `replay_interrupted_flush` rolls back/forward apply-only then `PersistMode::Sync`, or returns `StorageError::InterruptedWrite` and leaves `H`. Hydrate leftover scanners still fail closed. |
| 5   | Progress credit and `persist_progress` no longer rewrite the full UTXO snapshot as live truth. | ✓ VERIFIED | `persist_progress` writes headers and runtime metadata only — no `save_chainstate_snapshot` / `seed_coins_from_snapshot`. `gate_progress_credit_on_coins_best` requires coins `B` hex == claimed tip; unflushed memory connects stay uncredited. Phase 140/141 leftover-write guards are flipped. |

**Score:** 5/5 truths verified

Supporting PLAN truths checked and held: typed `InterruptedWrite` remap (IN-01 / D-10); empty `H` + present `B` is consistent and does not replay (D-09); `Chainstate<V: CoinsView = MemoryCoinsView>` with first-party `estimated_cache_bytes`; core crate stays I/O-free; shell defaults 450 / 4 / 8 MiB and 442 MiB cache limit; `RefuseDiskSpace` / `FlushDecision::None` do not write coins; Periodic resample is 50–70 minutes via `set_next_write`; no every-connect `Always` (D-02).

### Required Artifacts

gsd-tools artifact check passed plans 01–05. Plan 06 flagged `sync.rs` missing `from_chainstate` because open was extracted to `open_runtime.rs`. That is a module split, not a missing implementation — wiring still exists.

| Artifact | Expected    | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-chainstate/src/error.rs` | Typed `InterruptedWrite` | ✓ VERIFIED | Variant + Display `"interrupted coins write"`; remap matches the variant, not Display text. |
| `packages/open-bitcoin-node/src/storage/coins_view.rs` | `MarkerState::Interrupted`, `head_blocks` Ok | ✓ VERIFIED | `(2, None)` → `Interrupted { new, old }`; `head_blocks` returns those hashes. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | Interrupted-first schema-2 + typed remap | ✓ VERIFIED | `ensure_schema_two` returns `Ok` on `InterruptedTwoHeads` before leftover-empty. `map_heads_error` matches `ChainstateError::InterruptedWrite`. |
| `packages/open-bitcoin-chainstate/src/engine.rs` | Generic `Chainstate<V>` + `from_parent` | ✓ VERIFIED | `pub struct Chainstate<V: CoinsView = MemoryCoinsView>`, `from_parent`, `from_coins_cache`. |
| `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs` | Generic overlay apply | ✓ VERIFIED | `apply_connect_on_overlay` / `apply_disconnect_on_overlay` generic over `V`. |
| `packages/open-bitcoin-chainstate/src/coins/cache.rs` | Occupancy estimators | ✓ VERIFIED | `cache_entry_count`, `estimated_cache_bytes` (first-party overhead). |
| `packages/open-bitcoin-node/src/chainstate/replay.rs` | ReplayBlocks owner | ✓ VERIFIED | `replay_interrupted_flush`; apply-only; no `connect_block`. |
| `packages/open-bitcoin-node/src/chainstate/replay/tests.rs` | Bodies-present + fail-closed fixtures | ✓ VERIFIED | Named tests for rollback/rollforward, zero-old first flush, missing body/undo keep `H`. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | CanFlush owner + ordered flush | ✓ VERIFIED | `ManagerReadiness`, `initialize`, `execute_flush`, `FlushPersistSink`, Knots defaults. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs` | Sequence + abort tests | ✓ VERIFIED | Includes `execute_flush_aborts_coins_when_undo_save_fails`. |
| `packages/open-bitcoin-node/src/chainstate.rs` | Required `FlushLifecycle`; IfNeeded persist | ✓ VERIFIED | `flush_lifecycle: FlushLifecycle` is required; `persist` → `execute_flush(IfNeeded)`. |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs` | Periodic + Always worker | ✓ VERIFIED | `ManagedNetworkHandle`, 50–70 min resample, shutdown Always. |
| `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` | Flipped leftover-write guards | ✓ VERIFIED | Persist and persist_progress guards assert decide_flush / no snapshot write. |
| `packages/open-bitcoin-node/src/sync.rs` | Open delegates to initialize + `from_chainstate` | ✓ VERIFIED | `mod open_runtime`; `open` → `open_with_runtime_activation`. Pattern lives in extracted module. |
| `packages/open-bitcoin-node/src/sync/open_runtime.rs` | Production attach of lifecycle + Fjall cache | ✓ VERIFIED | `initialize`, `from_coins_cache`, `from_chainstate(store, chainstate, lifecycle)`. |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | Headers/runtime persist + B-gated credit | ✓ VERIFIED | `persist_progress` headers/runtime only; credit via `gate_progress_credit_on_coins_best`. |

### Key Link Verification

| From | To  | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `fjall_store/coins.rs` | `coins_view.rs` | `decide_recovery` before leftover-empty | ✓ WIRED | `ensure_schema_two` and hydrate classify interrupted `H` first. |
| `fjall_store/coins.rs` | `error.rs` | `ChainstateError::InterruptedWrite` | ✓ WIRED | `map_heads_error` matches the variant. |
| `engine.rs` | `cache.rs` | `CoinsCache<V>` | ✓ WIRED | `Chainstate.coins` is `CoinsCache<V>`. |
| `cache.rs` | `flush.rs` | `estimated_cache_bytes` | ✓ WIRED | Occupancy feeds `FlushPolicyInput`. |
| `replay.rs` | `fjall_store.rs` | `load_undo` / `load_block` | ✓ WIRED | `require_undo` / `require_block`. |
| `replay.rs` | `flush.rs` | `decide_recovery` | ✓ WIRED | Replay classifies heads before apply. |
| `flush_lifecycle.rs` | `replay.rs` | `replay_interrupted_flush` | ✓ WIRED | `apply_recovery_decision` on `InterruptedTwoHeads`. |
| `flush_lifecycle.rs` | `flush.rs` | `decide_flush` then flush/sync | ✓ WIRED | Write kind taken from decision; not re-chosen. |
| `flush_lifecycle.rs` | `fjall_store.rs` | `impl FlushPersistSink for FjallNodeStore` | ✓ WIRED | Also implemented for `FjallChainstateStore`. |
| `chainstate.rs` | `flush_lifecycle.rs` | `self.flush_lifecycle.execute_flush` | ✓ WIRED | `persist` / `flush_with_mode`. |
| `coins_flush.rs` | `runtime_authority.rs` | `ManagedNetworkHandle` | ✓ WIRED | `flush_coins` / `set_coins_next_write`. |
| `open-bitcoind.rs` | `coins_flush.rs` | `start_coins_flush_worker` | ✓ WIRED | Started beside mempool checkpoint. |
| `sync.rs` / `open_runtime.rs` | `chainstate.rs` | `from_chainstate(...)` | ✓ WIRED | gsd-tools regex `from_chainstate(` was invalid; manual grep confirms the call. |
| `open_runtime.rs` | `flush_lifecycle.rs` | `initialize(` | ✓ WIRED | Same regex-tool miss; source has the call. |
| `runtime_state.rs` | `coins_view.rs` | `best_block` credit gate | ✓ WIRED | `store.coins_view().best_block()` into `gate_progress_credit_on_coins_best`. |

### Data-Flow Trace (Level 4)

Not a UI-render phase. Traced production data sources that become tip, UTXO, flush, and credit.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `open_runtime.rs` | `cache` / tip / UTXOs | `initialize` → `FjallCoinsView` markers + coins `B` | Yes — durable coins keyspace, leftover unread | ✓ FLOWING |
| `flush_lifecycle.rs` `execute_flush` | `decision` / writes | Injected occupancy + `decide_flush`; sink persist; `cache.flush`/`sync` | Yes — live cache + Fjall sink | ✓ FLOWING |
| `runtime_state.rs` `persist_progress` | headers / metadata | `network.header_entries()`, `save_runtime_metadata` | Yes — no snapshot rewrite | ✓ FLOWING |
| `helpers.rs` progress credit | `progress_credit` | `classify_progress_credit` gated on coins `B` | Yes — unavailable when `B` lags claimed tip | ✓ FLOWING |

`ManagedChainstate::from_store` still hydrates `load_snapshot` into `MemoryCoinsView`. That path is the in-memory test parent (D-12). Production uses `FjallChainstateStore` (`load_snapshot` → `None`) plus `initialize`.

### Behavioral Spot-Checks

Commands are read-only source probes (no servers, no mutations). Full Cargo suites exceed the 10s spot-check budget.

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Production open attaches initialize + lifecycle | grep `initialize` / `from_coins_cache` / `from_chainstate` in `open_runtime.rs` | All three present; no `hydrate_chainstate_for_open` | ✓ PASS |
| IfNeeded after connect/reorg | grep `FlushMode::IfNeeded` + `execute_flush` in `chainstate.rs` | `persist()` calls both from connect/reorg/commit | ✓ PASS |
| Periodic + Always daemon wiring | grep `FlushMode::Periodic` / `Always` / `start_coins_flush_worker` | Worker + `open-bitcoind.rs` start site | ✓ PASS |
| persist_progress has no leftover writes | grep `save_chainstate_snapshot` / `seed_coins_from_snapshot` in `runtime_state.rs` | Zero matches in persist path | ✓ PASS |
| Replay is apply-only | grep `connect_block` in `replay.rs` | Zero matches | ✓ PASS |
| Typed remap, not Display | grep `detail.contains("interrupted write")` in `fjall_store/coins.rs` | Zero matches; `ChainstateError::InterruptedWrite` present | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| MGR-01 | 142-02, 142-04, 142-05 | One manager owns coins-database init, health-check, cache init, and CanFlush-style readiness for the single active chainstate. | ✓ SATISFIED | `FlushLifecycle` + `initialize` + required field on `ManagedChainstate`; production open stores that instance. |
| MGR-02 | 142-02, 142-06 | After restart, tip and UTXO view come from durable coins best-block, not a leftover snapshot blob. | ✓ SATISFIED | `from_coins_cache` on Fjall parent; leftover hydrate unused; restart tests assert coins `B`. |
| FLUSH-02 | 142-01, 142-03 | A mid-flush crash is recovered by interrupted-flush replay using stored undo and block bodies, or fails closed without inventing a consistent tip. | ✓ SATISFIED | Observable interrupted `H`; replay or `InterruptedWrite`; leftover scanners still fail closed. |

No orphaned Phase 142 IDs. REQUIREMENTS.md maps only MGR-01, MGR-02, and FLUSH-02 to this phase. HAVL-*, CSOBS-01/02, and CSVFY-* belong to 143–145. FUT-21–FUT-23 stay deferred.

REQUIREMENTS.md checkboxes and the coverage table still say Pending. That is bookkeeping for phase closeout, not missing implementation.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-node/src/chainstate.rs` | ~420 | `let _ = self.flush_with_mode(...)` swallows IfNeeded errors | ℹ️ Info | Allowed crash-loss window (D-01). Failed coins writes still abort inside `execute_flush` and do not advance `B`; credit stays gated on `B`. |
| `packages/open-bitcoin-node/src/chainstate.rs` | ~390 | `flush_window` passes empty `block_payloads`; persist passes empty header entries | ℹ️ Info | Blocks are saved at download (`block_response.rs`); headers via `persist_progress`. `execute_flush` still writes any provided block/undo/index before coins; undo window is live. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | ~196 | `persist_chain_meta` runs after coins write | ℹ️ Info | Documented 142-06 choice so reopen hydrates `active_chain` from the flushed tip. Block/undo/index prefix still precedes coins. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | ~236 | `probe_disk_free_bytes` always `u64::MAX` | ℹ️ Info | D-04 allows injected facts; crate forbids `unsafe` `statvfs`. Callers can still inject a real value. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | ~207 | Comment still says production persist writes leftover | ℹ️ Info | Stale comment. Production persist/open paths no longer write leftover as live truth. |
| `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` | 705–741 | Source-contains leftover-write guards | ℹ️ Info | Guards match D-18. Behavioral restart/credit tests also exist and are the stronger evidence. |

No blocker stubs. No TODO/FIXME/placeholder in the phase-owned manager, replay, flush-lifecycle, open, or coins-flush modules.

### Human Verification Required

None. Behaviors are backend Rust with source wiring plus unit fixtures (interrupted `H` open, replay success/fail-closed, CanFlush sequence, ordered-write abort, same-datadir reopen from coins `B`, B-gated credit). No visual, operator-surface, or external-service items in this phase (those are 144).

### Gaps Summary

No actionable gaps. Later phases 143–145 own honest payload-present availability, operator flush evidence, and parity/no-claim closeout — none of those are missing Phase 142 deliverables.

Confirmation-bias notes (do not fail the goal): `persist()` is fire-and-forget IfNeeded; leftover-write guards are partly source-contains; `probe_disk_free_bytes` is a non-probing stub. Each is consistent with locked D-01/D-04/D-18.

---

_Verified: 2026-09-07T02:00:00Z_
_Verifier: Claude (gsd-verifier)_

## VERIFICATION PASSED
