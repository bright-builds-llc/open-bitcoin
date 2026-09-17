---
phase: 144-operator-flush-and-availability-evidence
reviewed: 2026-09-17T18:05:00Z
depth: standard
files_reviewed: 42
files_reviewed_list:
  - packages/open-bitcoin-node/src/status/chainstate_durability.rs
  - packages/open-bitcoin-node/src/status/chainstate_durability/tests.rs
  - packages/open-bitcoin-node/src/status.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/durability.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs
  - packages/open-bitcoin-node/src/network/chainstate_durability_evidence/tests.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - packages/open-bitcoin-node/src/network/operator_snapshot.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/network/peer_network_clone.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/metrics/chainstate_durability.rs
  - packages/open-bitcoin-node/src/metrics/tests/relay_projection.rs
  - packages/open-bitcoin-node/src/logging.rs
  - packages/open-bitcoin-node/src/logging/chainstate_durability.rs
  - packages/open-bitcoin-node/src/logging/tests/redaction.rs
  - packages/open-bitcoin-node/src/sync/metrics.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/operator_logs.rs
  - packages/open-bitcoin-rpc/src/method/node.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/context/inbound_status.rs
  - packages/open-bitcoin-cli/src/operator/status.rs
  - packages/open-bitcoin-cli/src/operator/status/render.rs
  - packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/metric_labels.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/projection.rs
  - packages/open-bitcoin-cli/src/operator/support/redaction.rs
  - packages/open-bitcoin-cli/src/operator/support/render.rs
  - packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs
  - packages/open-bitcoin-cli/src/operator/support/render/text.rs
  - scripts/check-phase144-operator-flush-availability-evidence.ts
  - scripts/check-phase144-operator-flush-availability-evidence.test.ts
findings:
  critical: 0
  warning: 1
  info: 2
  total: 3
status: issues_found
---

# Phase 144: Code Review Report

**Reviewed:** 2026-09-17T18:05:00Z
**Depth:** standard
**Files Reviewed:** 42
**Status:** issues_found

## Summary

Phase 144's production path is sound. The dedicated `chainstate_durability` field copies retained flush/recovery/have-bytes facts; RPC/CLI/dashboard/metrics/logs/support format those facts instead of re-deriving occupancy; durable open uses `initialize` rather than inventing a consistent tip. The special-concern scan (coin dumps, peer ids, pruned labels, `getblock`, ninth chart, `unwrap()`, high-cardinality metrics) did not find a production defect.

One advisory warning: the Phase 144 checker includes itself in the requirement/symbol/test-name corpus, so those checks can pass after the real docs or tests drop the strings. Two info notes cover an unwired fail-closed log helper and human tip rendering that requires both height and hash.

Advisory only — do not block.

## Warnings

### WR-01: Checker corpus is self-satisfying for required strings

**File:** `scripts/check-phase144-operator-flush-availability-evidence.ts:213-263`
**Issue:** `checkRequiredText` joins every `TARGET_FILES` body, and that list includes this checker script. `REQUIRED_REQUIREMENTS`, `REQUIRED_SYMBOLS`, and `REQUIRED_BEHAVIOR_TESTS` are defined as string constants in the same file, so CSOBS-01/CSOBS-02, locked symbols, and behavior-test names stay present even if docs or the Rust tests that should contain them are deleted. The fixture test only fails after those strings are stripped from *all* files, including the checker.
**Fix:** Build the requirement/symbol/test-name corpus from product files only (docs, packages, `verify.sh`). Keep the checker in `TARGET_FILES` for its own needles and verify-order checks, but omit it from `checkRequiredText`:

```typescript
const REQUIREMENT_CORPUS_FILES = TARGET_FILES.filter(
  (file) =>
    file !== "scripts/check-phase144-operator-flush-availability-evidence.ts" &&
    file !== "scripts/check-phase144-operator-flush-availability-evidence.test.ts",
);

function checkRequiredText(texts: TextCorpus, failures: string[]): void {
  const corpus = REQUIREMENT_CORPUS_FILES.map((file) => texts.get(file) ?? "").join("\n");
  // ... existing requirement / symbol / test-name loops
}
```

## Info

### IN-01: Fail-closed StorageError log helper is not on the emit path

**File:** `packages/open-bitcoin-node/src/logging/chainstate_durability.rs:39-52`
**Issue:** `chainstate_durability_fail_closed_log_record` is exported and unit-tested, but `write_block_relay_log` only calls `chainstate_durability_log_record` on an operator snapshot. A failed `initialize` in `open_runtime.rs` returns `Err` before the log loop runs, so a fail-closed `StorageError` Display never reaches this helper in production.
**Fix:** Call the helper from the initialize-error path if startup should emit `recovery_outcome=fail_closed`, or keep it as a tested mapper and document that live fail-closed-without-B after a successful open is covered by `chainstate_durability_log_record` (`recovery_outcome=fail_closed`, no hash).

### IN-02: Human coins-best-block line hides hash-only tips

**File:** `packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs:60-68`
**Issue:** Human CLI, dashboard, and support require both `maybe_coins_best_block_height` and `maybe_coins_best_block_hash`. `operator_snapshot` can publish a hash with `height = None` when `coins_best_block()` succeeds but the header store has no entry. JSON then shows the hash while human lines print `Unavailable: coins best-block unavailable`.
**Fix:** If hash-only should stay visible, render `hash={hash}` when height is missing. If the current fail-closed human line is intended, no change.

---

## Special-concern checklist

| Concern | Result |
| --- | --- |
| Coin dumps | Not present. Payload is height + 64-hex tip only; no outpoints, values, scripts, or UTXO maps. |
| Peer ids | Not on the new field. `HaveBytesAccumulator` stores last labels and three counters only. Logs/support sanitize `peer_id=`. |
| Pruned labels on the new field | Not present. `pruned` appears only in rustdoc must-not text. Block-relay `pruned_count` is the pre-existing Phase 143 surface. |
| `getblock` product | Not added. `getblockchaininfo` / `getnetworkinfo` stay unchanged; RPC clones `snapshot.chainstate_durability()`. |
| Ninth dashboard chart | Not added. `MAX_DASHBOARD_CHARTS` and `DASHBOARD_METRIC_KINDS` stay 8; the seven durability series are labeled but not chart candidates. |
| `unwrap()` | Not in production Phase 144 files. Node crate denies `clippy::unwrap_used` outside tests. |
| High-cardinality metrics | Not present. Seven closed `MetricKind` series; class integers and counters; no dynamic labels. |
| Invented consistent tip after fail-closed | Not on the durable path. Empty/missing-B `initialize` is `FailClosed` with both tip fields `None`. Durable open uses `initialize`, not `FlushLifecycle::ready()`. Projector does not substitute interrupted `H` hashes (`coins_best_block()` only). |
| Renderer re-deriving cache-size | Not present. CLI, dashboard, support, and metrics format `evidence.cache_size` / snapshot class. Occupancy stays `decide_flush(FlushMode::None)` in the projector. |

---

_Reviewed: 2026-09-17T18:05:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
