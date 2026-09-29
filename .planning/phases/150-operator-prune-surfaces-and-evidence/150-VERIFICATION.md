---
phase: 150-operator-prune-surfaces-and-evidence
verified: 2026-09-29T17:40:00Z
status: passed
score: 19/19 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 150-2026-09-28T17-08-23
generated_at: 2026-09-29T17:40:00Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 150: Operator Prune Surfaces and Evidence Verification Report

**Phase Goal:** Operators can inspect prune state, request a manual prune, manage prune locks, and read sanitized support evidence.
**Verified:** 2026-09-29T17:40:00Z
**Status:** passed
**Re-verification:** No — initial verification

Lifecycle provenance matches across `150-CONTEXT.md`, all eight plans, and all eight summaries: `lifecycle_mode: yolo` and `phase_lifecycle_id: 150-2026-09-28T17-08-23`. None are marked `direct-fallback`.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | Operator can read pruned, prune height, automatic pruning, and prune target from RPC, CLI, and dashboard. | ✓ VERIFIED | `GetBlockchainInfoResponse` emits `pruned`, `pruneheight`, `automatic_pruning`, and `prune_target_size`. CLI status and the dashboard reuse `prune_rows`. |
| 2 | Disabled mode is `pruned: false` and omits `pruneheight`, `automatic_pruning`, and `prune_target_size`. | ✓ VERIFIED | `project_prune_status` clears the optional fields. Default `getblockchaininfo` key order in the phase 127 composition test includes `pruned` and not the optional keys. |
| 3 | Manual-only with nothing pruned is `pruned: true`, `pruneheight: 0`, `automatic_pruning: false`, and no target. | ✓ VERIFIED | `phase150_blockchaininfo_manual_only_emits_pruneheight_zero` asserts that JSON shape, including absence of `prune_target_size`. |
| 4 | Automatic 550 MiB projects `prune_target_size` 576716800 bytes. | ✓ VERIFIED | `target_mib * 1024 * 1024` in `project_prune_status`, asserted by `phase150_blockchaininfo_automatic_550_emits_576716800`. |
| 5 | A hole under a complete tip is `GetPruneHeight` of the last incomplete suffix height, and the info field is that value plus one. | ✓ VERIFIED | `get_prune_height` walks the tip suffix; `info_pruneheight` adds one. Hole case is asserted in `phase150_blockchaininfo_hole_emits_first_complete_height`. |
| 6 | A raw `pruneblockchain` argument above 1000000000 is a timestamp minus 7200 seconds, and height 0 stays the zero sentinel. | ✓ VERIFIED | `resolve_manual_prune_argument` and the zero early-return in `prune_blockchain` (returns `0` before planning). |
| 7 | Missing JSONC `prune` and explicit `0` are disabled; `1` is manual-only; `550` is automatic 550 MiB; `2`, `549`, and negatives fail load. | ✓ VERIFIED | `OpenBitcoinConfig::default` sets `prune: 0`. `config/prune.rs` calls `parse_prune_arg`. Loader assigns `RuntimeConfig.prune_mode`. |
| 8 | Startup applies `RuntimeConfig.prune_mode` through `set_prune_mode` before the context is published. | ✓ VERIFIED | `from_runtime_config` and `from_runtime_config_with_network_handle` both call `set_prune_mode`. |
| 9 | Named locks survive reopen, the same name replaces one range, and clear removes only that name. | ✓ VERIFIED | `replace_prune_lock` inserts by name and `sync_prune_locks`. `clear_named_prune_lock` returns false without a write when the name is missing. Tests cover reopen. |
| 10 | Startup resume loads locks and passes that slice into `resume_prune_intent`. | ✓ VERIFIED | `FlushLifecycle::initialize` calls `resume_prune_intent(store, &locks)` after `load_prune_locks`. |
| 11 | Support counters start at zero, then increment one batch and the deleted-height count, with last height equal to the max of that batch only. | ✓ VERIFIED | `record_successful_prune_batch` ignores an empty slice and otherwise adds one batch, the slice length, and `deleted_heights.iter().max()`. Applied from `DeletedLiveMate` heights in `prune_apply.rs`. |
| 12 | Status snapshot carries configured facts, live locks, manual outcome, and support counts; a stopped collector uses the JSONC mode and marks those live facts unavailable. | ✓ VERIFIED | `PruneOperatorStatus` is a sibling of chainstate durability. Live collector copies `network_status.prune`. `from_stopped_config` parses the JSONC integer and marks height, locks, manual prune, and counts unavailable. |
| 13 | `pruneblockchain` in disabled mode returns `MiscError` and does not delete or set have-pruned. A keep-window target returns `InvalidParameter` and is not clamped. A legal target returns `GetPruneHeight` (or `-1`) after `flush_applying_prune_plan`. | ✓ VERIFIED | Refusal returns before flush. Keep-window maps to `ManualPruneRefusal::TargetInsideKeepWindow`. Success path flushes, then returns `configured_last_pruned_height` without the plus-one info offset, or `-1`. |
| 14 | `listprunelocks`, `setprunelock`, and `clearprunelock` are registered and dispatched, and the CLI `prune run` / `prune lock` commands call those RPC methods without opening Fjall. | ✓ VERIFIED | Method registry names the four methods. `operator/prune.rs` builds `pruneblockchain`, `listprunelocks`, `setprunelock`, and `clearprunelock` JSON-RPC calls. No Fjall open in that module. `runtime.rs` routes `OperatorCommand::Prune` to `execute_prune_command`. |
| 15 | The dashboard appends a read-only Prune section at index 5, leaves Mempool and Wallet at index 2, and has no payload-deleting action. | ✓ VERIFIED | `dashboard_sections` pushes `prune_section` last. Tests assert section titles and action keys `r s t o x i u e d q`. `DashboardAction` has no prune or delete variant. The section only renders rows. |
| 16 | The dashboard lists locks and shows the manual-prune height or a typed refusal, including the keep-window text that says refused. | ✓ VERIFIED | `lock_rows` and `refusal_text` in `prune_section.rs`. CLI status lines come from the same `prune_rows`. |
| 17 | Support Markdown has `## Prune` immediately after `## Chainstate Durability`, with batch and height counts, and omits last prune height until a successful batch exists. | ✓ VERIFIED | `render.rs` calls `push_prune` right after `push_chainstate_durability`. `prune_fact_lines` pushes last height only when `maybe_last_prune_height` is set. |
| 18 | Support JSON clears `prune.locks` while the caller snapshot keeps them. | ✓ VERIFIED | `support_status_for_bundle` takes the snapshot by value and `clear_support_prune_locks` replaces an available lock list with an empty vector. `support_prune_section_reports_counts_and_omits_lock_datadir_and_hash` clones the caller snapshot, then asserts `secret-lock` remains on that snapshot and is absent from the JSON and Markdown. |
| 19 | Support JSON and Markdown do not contain a datadir, a Fjall key name, a block hash, or a lock name in the prune evidence. | ✓ VERIFIED | Same test forbids `secret-lock`, the planted datadir, the planted hash, `prune_locks`, and `prune_summary` in both rendered forms, while `successful_batch_count` and `pruned_height_count` remain. |

**Score:** 19/19 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-chainstate/src/prune/status.rs` | I/O-free `GetPruneHeight`, quartet, timestamp pre-step | ✓ VERIFIED | `project_prune_status`, `get_prune_height`, `resolve_manual_prune_argument` exist and are re-exported from `prune.rs`. |
| `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` | Top-level JSONC `prune` integer, default 0 | ✓ VERIFIED | `pub prune: i64`, default `0`. |
| `packages/open-bitcoin-rpc/src/config/prune.rs` | `parse_prune_arg` boundary | ✓ VERIFIED | Loader assigns `prune_mode` from `resolve_prune_mode`. |
| `packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs` | Durable locks and support summary | ✓ VERIFIED | `prune_locks` records and `record_successful_prune_batch`. |
| `packages/open-bitcoin-node/src/status/prune_operator.rs` | Serializable operator prune facts | ✓ VERIFIED | `PruneOperatorStatus` carries quartet, locks, manual outcome, and counts. |
| `packages/open-bitcoin-rpc/src/method/node.rs` | Optional `getblockchaininfo` quartet keys | ✓ VERIFIED | Serde renames and `skip_serializing_if` on the three optional keys; `pruned` is always present. |
| `packages/open-bitcoin-rpc/src/dispatch/prune.rs` | `pruneblockchain` plus list/set/clear | ✓ VERIFIED | Wired from `dispatch.rs` and implemented with planner refusal before flush. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model/prune_section.rs` | Read-only Prune section | ✓ VERIFIED | Rows only; appended at dashboard index 5. |
| `packages/open-bitcoin-cli/src/operator/prune.rs` | CLI prune run and prune lock commands | ✓ VERIFIED | Contains the four RPC method names. |
| `packages/open-bitcoin-cli/src/operator/support/render/prune.rs` | Sanitized support prune section | ✓ VERIFIED | Emits `## Prune` counts and omits last height until present. |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `prune.rs` | `prune/status.rs` | `pub use project_prune_status` | WIRED | Re-export present. |
| `config/loader.rs` | `config/prune.rs` | `prune_mode:` assignment | WIRED | `prune_mode: super::prune::resolve_prune_mode(prune)?`. |
| `context/network.rs` | `ManagedNetworkHandle::set_prune_mode` | startup | WIRED | Called in both context constructors. |
| `flush_lifecycle.rs` | `resume_prune_intent` | loaded lock slice | WIRED | `resume_prune_intent(store, &locks)?`. |
| `prune_apply.rs` | `record_successful_prune_batch` | `DeletedLiveMate` heights | WIRED | Collects deleted heights, then records the batch. |
| `dispatch/node.rs` | `project_prune_status` | both blockchain-info paths | WIRED | In-memory and durable info both project the quartet. |
| `operator/status.rs` | network status `prune` | live collector copy | WIRED | `let prune = network_status.prune.clone()`. |
| `dispatch/prune.rs` | `plan_manual_prune` then `flush_applying_prune_plan` | refusal before flush | WIRED | Error returns before flush; success flushes then returns height. |
| dashboard `model.rs` | `prune_section` | sections push | WIRED | Last section, index 5. |
| `operator/prune.rs` | `pruneblockchain` | JSON-RPC client | WIRED | No store open in the CLI module. |
| `operator/runtime.rs` | `execute_prune_command` | `OperatorCommand::Prune` | WIRED | Single match arm. |
| `support/render.rs` | `push_prune` | after chainstate durability | WIRED | Immediate next call. |
| `support/redaction.rs` | caller snapshot | by-value clear of locks | WIRED | Redaction mutates the owned bundle copy; the test retains the original locks. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `getblockchaininfo` | quartet | `prune_mode` plus `get_prune_height` over active-chain payload and undo presence | Yes | ✓ FLOWING |
| Dashboard Prune section | `snapshot.prune` | live `openbitcoinnetworkstatus` prune object, or stopped JSONC projection | Yes | ✓ FLOWING |
| RPC lock methods | lock rows | `load_prune_locks` / `sync_prune_locks` on the Fjall block-index record | Yes | ✓ FLOWING |
| `pruneblockchain` | returned height | post-flush `configured_last_pruned_height` (`GetPruneHeight`, or `-1`) | Yes | ✓ FLOWING |
| Support bundle | counts and cleared locks | status snapshot copied by value, locks emptied, counts serialized | Yes | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Quartet, refusal, locks, redaction, dashboard keys | Source assertions in `dispatch/node.rs`, `dispatch/prune/tests.rs`, `support/tests/recovery_progress_inbound.rs`, `dashboard/model/tests/prune.rs` | Assertions match the implementation paths reviewed above | ? SKIP |

Runtime tests were not re-executed. The relevant cases open Fjall stores and exceed the 10-second spot-check bound. The assertions were read in source against the production functions they call.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| OPER-01 | 150-01, 150-02, 150-04, 150-06 | Operator can read pruned, prune height, automatic pruning, and prune target from RPC, CLI, and dashboard. | ✓ SATISFIED | Quartet projection, JSONC mode, `getblockchaininfo`, status snapshot, CLI lines, dashboard section. |
| OPER-02 | 150-05, 150-06, 150-07 | Operator can request a manual prune when prune mode is on. | ✓ SATISFIED | `pruneblockchain` refusal and height return; CLI `prune run`; dashboard shows the outcome and does not delete. |
| OPER-03 | 150-03, 150-08 | Support evidence reports prune counts and the last prune height without raw storage paths. | ✓ SATISFIED | Durable summary counters; Markdown `## Prune`; JSON omits last height until a batch exists and strips lock names, key names, hashes, and planted paths. |
| LOCK-02 | 150-03, 150-05, 150-06, 150-07 | Operator can list and set prune locks. | ✓ SATISFIED | Durable replace-by-name, RPC list/set/clear, CLI list/set/clear, dashboard list. Clear-by-name is included, matching D-13. |

REQUIREMENTS.md maps OPER-01, OPER-02, OPER-03, and LOCK-02 to Phase 150 and no other IDs. Every plan `requirements` entry is one of those four IDs. No orphaned Phase 150 requirement.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-rpc/src/context/network.rs` | 144 | `expect` on fresh startup `set_prune_mode` | ℹ️ Info | The resolved mode is applied on the startup path. This is not a missing prune surface. |
| `packages/open-bitcoin-rpc/src/context/prune.rs` | 29 | `list_prune_locks` returns an empty vector when no metrics store is attached | ℹ️ Info | Durable contexts load the Fjall record. The empty result is the no-store case, not the lock implementation. |

No TODO, FIXME, or placeholder prune surfaces were found in the phase implementation files.

### Human Verification Required

None. The dashboard contract is a deterministic row model and a fixed action-key list, both asserted in unit tests. No visual or external-service behavior remains untraced.

### Gaps Summary

No gaps. Phase 150 success criteria hold in the codebase: operators can read the prune quartet, request a manual prune and see the refusal or height, list and set locks from RPC and CLI, and read support counts that do not carry lock names or raw storage paths.

---

_Verified: 2026-09-29T17:40:00Z_
_Verifier: Claude (gsd-verifier)_
