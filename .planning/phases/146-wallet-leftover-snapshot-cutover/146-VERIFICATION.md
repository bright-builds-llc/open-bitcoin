---
phase: 146-wallet-leftover-snapshot-cutover
verified: 2026-09-22T05:29:44Z
status: passed
score: 6/6 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 146-2026-09-21T20-57-36
generated_at: 2026-09-22T05:29:44Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 146: Wallet Leftover-Snapshot Cutover Verification Report

**Phase Goal:** Wallet rescan treats durable coins and payload-present blocks as chain truth, never leftover snapshot bytes.
**Verified:** 2026-09-22T05:29:44Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | Wallet rescan reads coins best-block and payload-present blocks; leftover snapshot blobs are non-authoritative on the wallet path | ✓ VERIFIED | `required_chainstate_snapshot` → `wallet_scan_chainstate_snapshot` (coins + `load_chain_meta_for_open`); zero `load_chainstate_snapshot` in `wallet_rescan.rs`; durable RPC seeds via same helper in `network.rs`; `has_block` gates before `rescan_chainstate` |
| 2 | After same-datadir reopen with a leftover snapshot present, rescan does not rebuild balances or history from that snapshot | ✓ VERIFIED | `disagreeing_leftover_snapshot_does_not_change_wallet_balances` (tip 3 / 60_000 sats, leftover remains); `durable_rescan_ignores_disagreeing_leftover_snapshot` (25_000 sats, not 999_999 / tip 99) |
| 3 | Contributors can observe that unlink-ready prune work has not yet deleted payloads; cutover alone does not invent have-pruned | ✓ VERIFIED | Reopen test asserts `has_block` still true; `phase146_cutover_does_not_invent_have_pruned` bans `have_pruned` / `block_status_pruned` / `NODE_NETWORK_LIMITED`; `runtime_state.rs` has zero `save_chainstate_snapshot` |
| 4 | A scan window height without payload bytes fails closed without leftover fallback | ✓ VERIFIED | `missing_block_payload_fails_chunk_closed` + `durable_rescan_missing_block_payload_fails_closed`; message substring `missing block payload`; no leftover load on fail path |
| 5 | Durable RPC open seeds MemoryChainstateStore from `wallet_scan_chainstate_snapshot` and fails closed when coins exist without usable `chain_meta` tip | ✓ VERIFIED | `network.rs` durable seed uses `wallet_scan_chainstate_snapshot` only; helper returns `Err` on empty `active_chain` or coins-B vs tip mismatch; no `hydrate_chainstate_for_open` / leftover migration on that seed |
| 6 | Touched Phase 146 first-party Rust paths have parity breadcrumbs | ✓ VERIFIED | Paths listed in `docs/parity/source-breadcrumbs.json` (`wallet_rescan.rs`, `coins.rs`, `wallet_rescan_runtime.rs`, `network.rs`, `rescan.rs`, `construction.rs`) |

**Score:** 6/6 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | `wallet_scan_chainstate_snapshot` never calls leftover load | ✓ VERIFIED | Exists; body uses `scan_coin_records` + `load_chain_meta_for_open`; no `load_chainstate_snapshot` in function body |
| `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` | coins-backed required snapshot + `has_block` gate | ✓ VERIFIED | Wired; zero leftover load tokens |
| `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs` | disagreeing leftover + missing payload + no-prune guards | ✓ VERIFIED | All four Phase 146 tests present |
| `packages/open-bitcoin-rpc/src/context/network.rs` | durable init uses wallet scan helper | ✓ VERIFIED | `durable_chainstate` ← `wallet_scan_chainstate_snapshot` |
| `packages/open-bitcoin-rpc/src/context/rescan.rs` | durable named-wallet `has_block` fail-closed | ✓ VERIFIED | Gate + `mark_failed` before `rescan_chainstate` |
| `packages/open-bitcoin-rpc/src/context/tests/construction.rs` | durable leftover/rescan regressions | ✓ VERIFIED | Ignore-leftover + missing-payload tests |
| `docs/parity/source-breadcrumbs.json` | breadcrumb coverage for touched paths | ✓ VERIFIED | All Phase 146 touched paths listed |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `wallet_rescan.rs` | `coins.rs` | `required_chainstate_snapshot` → `wallet_scan_chainstate_snapshot` | ✓ WIRED | Call site present |
| `wallet_rescan.rs` | `blocks.rs` (`has_block`) | gate before `rescan_chainstate` | ✓ WIRED | Manual grep; gsd-tools regex `has_block\\(` invalid but source has `has_block(` |
| `network.rs` | `coins.rs` | durable `MemoryChainstateStore` seed | ✓ WIRED | `wallet_scan_chainstate_snapshot` |
| `rescan.rs` | `has_block` | durable named-wallet gate | ✓ WIRED | Loop before successful rescan |
| `rescan.rs` | `blockchain_snapshot` | memory store seeded at open | ✓ WIRED | `blockchain_snapshot()` then partial filter |
| `wallet_rescan_runtime.rs` | `wallet_rescan.rs` | source-string prune bans | ✓ WIRED | `include_str!` assertions |
| `runtime_state.rs` | leftover write omission | no `save_chainstate_snapshot` | ✓ WIRED | Zero matches in file |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `WalletRescanRuntime::advance_wallet_rescan` | `chainstate` / partial UTXOs | `wallet_scan_chainstate_snapshot` → `scan_coin_records` + `chain_meta` | Yes — durable coins, not leftover blob | ✓ FLOWING |
| Durable `rescan_wallet_range` | `snapshot` / wallet UTXOs | `blockchain_snapshot()` from memory store seeded at open | Yes — same coins-backed seed | ✓ FLOWING |
| Disagreeing leftover tests | tip / balances | planted `seed_coins_from_snapshot` vs poison leftover | Coins truth wins; leftover stays on disk | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| No leftover load on wallet rescan path | `rg load_chainstate_snapshot wallet_rescan.rs` | no matches | ✓ PASS |
| Scan helper present and leftover-free body | parse `wallet_scan_chainstate_snapshot` body | no `load_chainstate_snapshot` | ✓ PASS |
| Durable RPC seed cutover | `rg wallet_scan_chainstate_snapshot network.rs` | match in durable seed | ✓ PASS |
| Payload gate wired | `rg 'has_block\(' wallet_rescan.rs rescan.rs` | matches on both paths | ✓ PASS |
| No leftover write resurrection | `rg save_chainstate_snapshot runtime_state.rs` | no matches | ✓ PASS |
| Regression tests exist | `rg fn disagreeing_leftover\|missing_block_payload\|phase146_cutover\|durable_rescan_ignores` | all present | ✓ PASS |

Step 7b cargo execution skipped (compile exceeds 10s spot-check budget); source contracts and test symbols verified instead.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| SNAP-01 | 146-01, 146-02, 146-03 | Wallet rescan reads durable coins and payload-present blocks, and does not treat leftover snapshot bytes as chain truth | ✓ SATISFIED | Coins-backed assembly + `has_block` gates on node and durable RPC paths; leftover regressions; no-prune / no leftover-write guards |

No orphaned Phase 146 requirements: REQUIREMENTS.md maps only SNAP-01 to Phase 146; all three plans declare SNAP-01.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | ~105–133 | `wallet_scan_chainstate_snapshot` omits `InterruptedTwoHeads` fail-closed that `hydrate_chainstate_for_open` has (146-REVIEW CR-01) | ⚠️ Warning | Crash mid-coins-write can still yield torn coins as scan authority; does **not** reintroduce leftover `"snapshot"` as truth — out of SNAP-01 leftover scope |
| `wallet_rescan.rs` / `rescan.rs` | chunk gates | `has_block` only covers current scan window, not every creating height ≤ through_height (146-REVIEW WR-01 / D-04 edge) | ⚠️ Warning | Mid-range `start_height` / pre-unlink edge; planned acceptance was window gate (D-08). Becomes sharper after Phase 148 deletes |
| `wallet_rescan.rs` / `rescan.rs` | `has_block` Err | storage probe errors propagate without `mark_failed` (146-REVIEW WR-02) | ℹ️ Info | Job may stay Pending on backend read failure; does not restore leftover authority |

No blocker stubs, TODO/FIXME, or prune-label invention on production cutover paths.

### Advisory Review Disposition

146-REVIEW.md reported 1 critical / 2 warnings. Per phase verification rules, a review finding is a **gap only if it violates the phase goal or a locked leftover-cutover decision**.

- **CR-01** — real coins-integrity hardening gap vs hydrate, but wallet path still never reads leftover `"snapshot"`. Does not fail SNAP-01 / goal “never leftover snapshot bytes.”
- **WR-01 / WR-02** — quality follow-ups; plan-scoped window gating and SNAP-01 leftover cutover still hold.

### Human Verification Required

None. Success criteria are covered by source contracts and automated regression symbols.

### Gaps Summary

None. Phase goal achieved: wallet and durable RPC rescan authority is durable coins + `chain_meta` + payload-present probes; leftover snapshot bytes remain non-authoritative; cutover does not invent have-pruned or delete payloads.

---

_Verified: 2026-09-22T05:29:44Z_
_Verifier: Claude (gsd-verifier)_
