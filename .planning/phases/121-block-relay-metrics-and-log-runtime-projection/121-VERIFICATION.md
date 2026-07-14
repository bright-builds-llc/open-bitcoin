---
phase: 121-block-relay-metrics-and-log-runtime-projection
verified: 2026-07-14T08:12:59Z
status: passed
score: 6/6 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 121-2026-07-14T04-25-57
generated_at: 2026-07-14T08:12:59Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 121: Block Relay Metrics and Log Runtime Projection Verification Report

**Phase Goal:** Project block-relay metric samples and structured log records through the sync runtime persist and logging path.
**Verified:** 2026-07-14T08:12:59Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `DurableSyncRuntime::persist_metrics` appends `block_relay_metric_samples` when Available | ✓ VERIFIED | `sync/metrics.rs` Available-gates provider then `samples.extend(block_relay_metric_samples(...))`; cargo test `persist_metrics_appends_block_relay_status_samples_with_sync_samples` ok |
| 2 | Structured sync/logging emits `block_relay_log_record` with fixed low-cardinality labels | ✓ VERIFIED | `write_block_relay_log` calls `block_relay_log_record` + `append_structured_record`; tick wires after `write_summary_logs` in `sync.rs:219`; emit test asserts `outcome=projected` / `cause=status_projection` / `label=block_relay` |
| 3 | Runtime tests prove projection beyond helper-only coverage | ✓ VERIFIED | Six DurableSyncRuntime tests in `sync/tests.rs` (available/unavailable/unset metrics + emit/omit/leakage logs); 6/6 cargo filters passed |
| 4 | No raw peer, permission, credential, or transaction payload leakage | ✓ VERIFIED | `write_block_relay_log_omits_sensitive_markers` asserts absence of `127.0.0.1`, `peer_id`, `permission_string`, `credential`, `cookie`, `secret`, `0123456789abcdef`; helpers unchanged; fixtures use aggregate `with_components` only |
| 5 | open-bitcoind wiring + Phase 121 checker in verify.sh | ✓ VERIFIED | Daemon `set_block_relay_metric_status_provider` from `ManagedRpcContext::block_relay_evidence_status` with activation outer gate; verify.sh dual-region (lines 342–343 + 460–461); bun checker 8/8 + corpus pass |
| 6 | OBS-03 marked complete | ✓ VERIFIED | REQUIREMENTS.md `[x] **OBS-03`** and traceability `OBS-03 \| Phase 121 \| Complete`; operator-observability Phase 121 projection note present |

**Score:** 6/6 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `packages/open-bitcoin-node/src/sync/metrics.rs` | Provider + Available-gated persist append | ✓ VERIFIED | `set_block_relay_metric_status_provider` + Available match before helper |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | `write_block_relay_log` via shared provider | ✓ VERIFIED | Available gate → `block_relay_log_record` → `append_structured_record` |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Runtime available/unavailable/leakage proofs | ✓ VERIFIED | Six named tests present and passing |
| `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` | ManagedRpcContext provider wiring | ✓ VERIFIED | Arc-cloned context; activation outer Available gate; no twin worker |
| `scripts/check-phase121-block-relay-metrics-log-runtime.ts` | Deterministic Phase 121 corpus checker | ✓ VERIFIED | 252 lines; bun run passed |
| `scripts/verify.sh` | Dual-region Phase 121 wiring | ✓ VERIFIED | Visible bun test/run + matching `run_step` entries |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `set_block_relay_metric_status_provider` | `persist_metrics` | Available gate before `block_relay_metric_samples` | ✓ WIRED | `metrics.rs:43-47` |
| `write_block_relay_log` | `block_relay_log_record` | Available gate + `append_structured_record` | ✓ WIRED | `runtime_state.rs:115-128` |
| sync tick | `write_block_relay_log` | same tick as `persist_metrics` / `write_summary_logs` | ✓ WIRED | `sync.rs:207-219` |
| open-bitcoind `start_daemon_sync_worker` | `ManagedRpcContext::block_relay_evidence_status` | Arc-cloned provider closure | ✓ WIRED | `open-bitcoind.rs:379-397` |
| `scripts/verify.sh` | Phase 121 checker | bun test + bun run visible + run_step | ✓ WIRED | 4 matches |
| REQUIREMENTS OBS-03 | Phase 121 closeout | checkbox + Complete row | ✓ WIRED | lines 51 + 135 |

Note: `gsd-tools verify key-links` reported "Source file not found" for symbolic from/to entries without paths; wiring confirmed by direct file reads/greps above.

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `persist_metrics` block-relay samples | `FieldAvailability<BlockRelayEvidenceStatus>` | provider → Available → `block_relay_metric_samples` → `append_metric_samples` | Yes — fixture aggregates + daemon ManagedRpcContext evidence | ✓ FLOWING |
| `write_block_relay_log` record | same shared provider status | Available → `block_relay_log_record` → `append_structured_record` | Yes — structured log file records with fixed labels | ✓ FLOWING |
| open-bitcoind provider | `block_relay_evidence_status()` | ManagedRpcContext lock + activation outer gate | Yes — real status aggregates; Unavailable on lock/activation miss | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Persist appends when Available | `cargo test -p open-bitcoin-node persist_metrics_appends_block_relay` | 1 passed | ✓ PASS |
| Persist omits when Unavailable/unset | `cargo test -p open-bitcoin-node persist_metrics_omits_block_relay` | 2 passed | ✓ PASS |
| Log emit/omit/leakage | `cargo test -p open-bitcoin-node write_block_relay_log` | 3 passed | ✓ PASS |
| Checker corpus + mutations | `bun test scripts/check-phase121-…test.ts` | 8 pass / 0 fail | ✓ PASS |
| Checker on live corpus | `bun run scripts/check-phase121-….ts` | passed | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| OBS-03 | 121-01, 121-02 | Fixed low-cardinality labels for served/suppressed/compact outcomes projected through metrics + structured logs | ✓ SATISFIED | Runtime projection + daemon wiring + checker + REQUIREMENTS Complete |

No orphaned Phase 121 requirements found.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | None blocking | — | No TODO/FIXME/placeholder stubs in metrics/runtime_state paths; `clippy::todo` allow in open-bitcoind is pre-existing lint attribute, not a stub |

Also confirmed:
- Helpers `pub fn block_relay_metric_samples` / `pub fn block_relay_log_record` still present (reuse, not rewrite)
- No `start_block_relay_metrics_worker` / `persist_block_relay_metrics_once` twin
- No claim-creep phrases in operator-observability Phase 121 section

### Human Verification Required

None — projection, omission, leakage, daemon wiring, and verifier inclusion are covered by automated Rust + Bun checks.

### Gaps Summary

None. Phase goal achieved: Available-gated block-relay metric samples and structured logs project through DurableSyncRuntime, production open-bitcoind wires ManagedRpcContext evidence, Phase 121 checker is dual-wired in verify.sh, and OBS-03 is Complete.

---

_Verified: 2026-07-14T08:12:59Z_
_Verifier: Claude (gsd-verifier)_
