---
phase: 143-honest-stored-block-availability
verified: 2026-09-17T06:40:57Z
status: passed
score: 7/7 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 143-2026-09-17T01-47-45
generated_at: 2026-09-17T06:40:57Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 143: Honest Stored-Block Availability Verification Report

**Phase Goal:** The node serves or reports a stored block only when the payload bytes are present and refuses cleanly when they are not.
**Verified:** 2026-09-17T06:40:57Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

The production serve path classifies `Available` only from a cache-or-store payload-byte probe, reports missing payload as `Unavailable` (never injected `Pruned`), and keeps the three HAVL-03 facts distinguishable on the report seam. Inventory, durable inbound RPC, compact inheritance, and post-gate lookup failure all refuse with existing `NotFound` machinery. Docs and the Phase 111 checker no longer describe missing payload as prune-mode.

### Observable Truths

| #   | Truth   | Status     | Evidence       |
| --- | ------- | ---------- | -------------- |
| 1   | Inventory, serve, and RPC classify a block as Available only after a payload-byte probe succeeds (cache `blocks_by_hash` or store `has_block`). | ✓ VERIFIED | `managed_block_serve_input` sets `payload_present = cache_present \|\| durable_payload_present` and `data_availability = Available` only then. Durable inbound RPC threads `source.has_block(hash).unwrap_or(false)` into `receive_message_for_durable_serving`. Tests: `managed_serve_input_cache_hit_sets_payload_present_true`, `durable_cache_miss_store_hit_is_payload_present`, `has_block_is_true_after_save_block_without_calling_load_block`, `durable_getdata_without_store_body_is_unavailable_at_gate`. |
| 2   | When the payload is absent, the node refuses cleanly as Unavailable and does not emit Pruned unless prune mode actually deleted files. | ✓ VERIFIED | Production `inventory.rs` has no `BlockServingDataAvailability::Pruned`. Missing cache + `durable_payload_present=false` is `Unavailable`. Phase 111 missing-body test now expects `NotFound` + `Unavailable` and `pruned_count == 0`. Durable missing-body inbound plan has no `DurableBlock` intent and resolves `NotFound` with `served_count == 0`. |
| 3   | Operator-visible facts distinguish `payload_present`, `index_known`, and `validated_on_active_chain`; coins tip or header index alone cannot authorize a serve. | ✓ VERIFIED | `BlockServingPresenceFacts` sits on `ManagedBlockServeInput` and `ManagedBlockServeDecision`. Test `managed_serve_input_exposes_presence_facts_without_authorizing_available_from_index` asserts `index_known=true`, `validated_on_active_chain=true`, `payload_present=false`, and `data_availability=Unavailable`. `inventory.rs` does not call `best_block()`. Full status/RPC/CLI/dashboard rollout of these facts remains Phase 144 (D-10); HAVL-03 is the report-seam contract (D-08). |
| 4   | Missing-payload refuse does not change public-default serving or claim archive-node behavior; no `getblock` product was added. | ✓ VERIFIED | Claim-scanned Phase 111 docs still contain the existing "does not add ... archive-node behavior ... public block serving by default" sentences. No `"getblock"` / `fn getblock` product method in RPC or CLI source. Phase 111 checker still rejects compact/archive/public/production overclaims (`bun test` 8/8, live checker exit 0). |
| 5   | `classify_block_serving_status` stays I/O-free; `Pruned` remains a reserved classifier mapping. | ✓ VERIFIED | `open-bitcoin-network/src/block_serving.rs` contains no `fjall`, `blocks_by_hash`, or `has_block`. Classifier still maps injected `Pruned` → `BlockServingStatusLabel::Pruned`. Rustdoc: "Reserved for a future prune-mode delete... Missing payload is Unavailable". Network-core `block_serving::` tests still pass (20/20). |
| 6   | `gate_inventory_for_durable_serving` no longer passes `durable_availability: true`; it uses the probe result. | ✓ VERIFIED | Gate takes `durable_payload_present: impl Fn(BlockHash) -> bool` and calls `durable_payload_present(block_hash)`. `inventory.rs` contains no `durable_availability` and no `block_hash, false, true`. RPC context supplies the fail-closed `has_block` closure. Test `gate_inventory_source_does_not_pass_literal_true_durable_override` passes. |
| 7   | A successful classify followed by a failed byte read reports Unavailable + existing NotFound, not a fabricated Block body and not Available. | ✓ VERIFIED | `LookupUnavailable` completion forces `status_label=Unavailable`, `payload_present=false`, `missing_inventory=true`. `serve_managed_block_request(..., |_| None)` returns no `Block`. `resolve_block_intent` still emits `WireNetworkMessage::NotFound` on `Ok(None)`, `Err(_)`, and missing source. Tests: `lookup_unavailable_completion_reports_unavailable_not_available`, `serve_managed_block_request_lookup_none_is_unavailable`, inbound missing-body / redaction tests. |

**Score:** 7/7 truths verified

### Required Artifacts

| Artifact | Expected    | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-node/src/network/block_serving.rs` | `BlockServingPresenceFacts` on Input/Decision; LookupUnavailable rewrite | ✓ VERIFIED | Exists (375 lines). `struct BlockServingPresenceFacts` with the three bools. `completion(LookupUnavailable)` rewrites Unavailable + `payload_present: false`. Presence copied onto every Decision via `missing(...)` / `eligible_decision`. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | Honest `durable_payload_present` assembly; gate uses probe | ✓ VERIFIED | Exists (425 lines). No `Pruned` inject. `data_availability` is Available only when `payload_present`. Gate calls `durable_payload_present(block_hash)`. |
| `packages/open-bitcoin-node/src/network/tests/block_serving.rs` | Unavailable-not-Pruned + presence-fact tests | ✓ VERIFIED | Contains renamed `..._unavailable_notfound`, presence-fact tests, no-Pruned source scan, no-literal-true scan, I/O-free classifier scan. 18/18 tests pass. |
| `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` | `FjallNodeStore::has_block` contains_key probe | ✓ VERIFIED | Exists (18 lines, substantive). `contains_key(super::block_key(block_hash))`. No `load_block` / `parse_block`. Wired from `DurableBlockSource` impl. |
| `packages/open-bitcoin-rpc/src/context/inbound_wire.rs` | `DurableBlockSource::has_block`; NotFound on load miss | ✓ VERIFIED | Trait + `FjallNodeStore` impl. `resolve_block_intent` still uses `LookupUnavailable` + `WireNetworkMessage::NotFound`. |
| `packages/open-bitcoin-node/src/network/block_serving/tests.rs` | LookupUnavailable and compact-inheritance tests | ✓ VERIFIED | Contains `lookup_unavailable_completion_reports_unavailable_not_available`, `serve_managed_block_request_lookup_none_is_unavailable`, `compact_paths_reuse_managed_block_serve_input_without_second_probe`. 14/14 tests pass. |
| `docs/architecture/status-snapshot.md` | Reserved-Pruned copy | ✓ VERIFIED | Contains `reserved for a future prune-mode delete`. No `pruned active non-tip blocks`. Keeps "does not add" / archive-node / public-default sentences. |
| `docs/operator/runtime-guide.md` | Same reserved wording + repo-local commands | ✓ VERIFIED | Contains `block_status_unavailable`, reserved-Pruned sentence, and the three verbatim commands (`cargo run --manifest-path ...`, `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, `bash scripts/verify.sh`). |
| `docs/parity/catalog/p2p.md` | Request-path paragraph is not prune-mode | ✓ VERIFIED | Contains `reserved for a future prune-mode delete`. No `pruned active non-tip blocks`. |
| `docs/parity/index.json` | `v2-1-full-block-serving-request-path` rationale updated | ✓ VERIFIED | Rationale includes reserved-Pruned wording plus `WireNetworkMessage::NotFound`, `block_status_pruned`, `block_status_unavailable`. |

gsd-tools `verify artifacts` reported 12/12 plan artifacts passed (3+3+2+4).

### Key Link Verification

| From | To  | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `inventory.rs` | `open-bitcoin-network/src/block_serving.rs` | Available only when `payload_present` | ✓ WIRED | `data_availability = if payload_present { Available } else { Unavailable }`. Classifier `may_serve_block` is true only for `Available`. |
| `block_serving.rs` | `inventory.rs` | Presence facts copied onto Decision | ✓ WIRED | Gate/deny/missing paths pass `input.presence`. LookupUnavailable rewrites presence with `payload_present: false`. |
| `context.rs` | `runtime_authority.rs` | `prepare_inbound_wire_message` passes `has_block` into durable receive | ✓ WIRED | Closure `|hash| source.has_block(hash).unwrap_or(false)` (or `false` when no source). |
| `inventory.rs` | `block_serving.rs` | Gate feeds probe result as `durable_payload_present` | ✓ WIRED | Manual: `managed_block_serve_input(..., false, durable_payload_present(block_hash))`. gsd-tools reported this pattern missing because it searches both endpoints; `block_serving.rs` does not contain the call site. The inventory call site is present and used. |
| `block_serving.rs` | `block_relay_evidence.rs` | `complete_block_serve` records completion `status_label` | ✓ WIRED | `complete_block_serve` → `record_block_serving_evidence(..., completion.decision())` increments `unavailable_count` for `Unavailable`. |
| `inbound_wire.rs` | `block_serving.rs` | Load miss stays `NotFound` + `LookupUnavailable` | ✓ WIRED | `Some(Ok(None)) \| Some(Err(_)) \| None` completion + `WireNetworkMessage::NotFound`. |
| `docs/parity/index.json` | Phase 111 checker | Required terms including `block_status_pruned` | ✓ WIRED | Checker REQUIRED_TERMS still include `block_status_pruned`; live checker exit 0. |
| `docs/operator/runtime-guide.md` | Phase 111 checker | REQUIRED_RUNTIME_COMMANDS remain verbatim | ✓ WIRED | Cargo, Bazel, and `bash scripts/verify.sh` lines present. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `ManagedBlockServeInput.presence.payload_present` | `cache_present \|\| durable_payload_present` | `blocks_by_hash.contains_key` OR `has_block` / caller bool | Yes — live cache/store probe, not a hardcoded true on durable gate | ✓ FLOWING |
| `ManagedBlockServeInput.presence.index_known` | header-store membership | `peer_manager.header_store().contains` | Yes | ✓ FLOWING |
| `ManagedBlockServeInput.presence.validated_on_active_chain` | active-chain index | `active_chain.iter().position(...)` | Yes; coins `best_block()` is not consulted | ✓ FLOWING |
| `ManagedBlockServeDecision.status_label` | classifier output | `classify_block_serving_status(&BlockServingStatusFacts { data_availability, ... })` | Yes — Available only when payload_present; LookupUnavailable rewrites to Unavailable | ✓ FLOWING |
| `FjallNodeStore::has_block` | key presence | `block_index.contains_key("block:" + 64-hex)` | Yes — no body decode | ✓ FLOWING |
| Inbound durable plan | `DurableBlock` vs immediate `NotFound` | `gate_managed_block_request` after probe | Missing store body produces no `DurableBlock` intent | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Presence facts, Unavailable-not-Pruned, no literal true, I/O-free classifier | `cargo test -p open-bitcoin-node --lib network::tests::block_serving` | 18 passed | ✓ PASS |
| LookupUnavailable rewrite, no fabricated body, compact inheritance | `cargo test -p open-bitcoin-node --lib network::block_serving::` | 14 passed | ✓ PASS |
| `has_block` contains_key without decode | `cargo test -p open-bitcoin-node --lib storage::fjall_store::tests::block_presence` | 1 passed | ✓ PASS |
| Durable gate Unavailable + inbound NotFound/redaction | `cargo test -p open-bitcoin-rpc --lib inbound_listener::tests::block_serving` | 14 passed | ✓ PASS |
| Reserved Pruned classifier mapping still works | `cargo test -p open-bitcoin-network --lib block_serving::` | 20 passed | ✓ PASS |
| Phase 111 checker + no-claim scan | `bun test scripts/check-phase111-full-block-serving-request-path.test.ts` and `bun run scripts/check-phase111-full-block-serving-request-path.ts` | 8/8 tests; `validated Phase 111 full block-serving request path` | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| HAVL-01 | 143-02, 143-03 | Node serves or reports a stored block as Available only when the payload bytes are present. | ✓ SATISFIED | Cache-or-store probe authorizes Available. Durable gate uses `has_block`, not literal true. Post-gate miss rewrites Unavailable. |
| HAVL-02 | 143-01, 143-03, 143-04 | When the payload is absent, the node refuses cleanly with Unavailable and does not emit Pruned unless prune mode actually deleted files. | ✓ SATISFIED | Production inventory no longer injects Pruned. Missing-payload paths use Unavailable + NotFound. Docs/rustdoc mark Pruned reserved. |
| HAVL-03 | 143-01 | Operator evidence distinguishes payload_present, index_known, and validated_on_active_chain. | ✓ SATISFIED | Three typed bools on the classification/report seam; tests prove they can diverge and that index/active-chain membership cannot authorize Available. Operator UI surfaces for these facts are Phase 144 (CSOBS-01/CSOBS-02), not this phase. |

No orphaned Phase 143 requirements. REQUIREMENTS.md maps HAVL-01, HAVL-02, and HAVL-03 only to this phase. Later-phase CSOBS/CSVFY IDs are not claimed here.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` | 76-78 | `ScriptedDurableBlockSource::has_block` returns `Ok(true)` | ℹ️ Info | Intentional test double so D-04 load-failure tests still reach `load_block`. Real Fjall missing keys return `Ok(false)`. |
| `packages/open-bitcoin-rpc/src/context.rs` | 320 | `has_block` `Err` mapped with `unwrap_or(false)` | ℹ️ Info | Fail-closed as planned (not `unwrap()`). No dedicated unit test for a failing `has_block` Result; load-error redaction tests cover post-gate `load_block` errors instead. |

No TODO/FIXME/PLACEHOLDER stubs in the production files for this phase. No leftover `durable_availability` override. No production `Pruned` inject.

### Human Verification Required

None. This phase is classification, store-probe, and refuse-path behavior with deterministic unit/checker coverage. No new operator UI, visual surface, or public-network behavior was added.

### Gaps Summary

No gaps. Later Phase 144 exposes sanitized operator surfaces for flush and have-bytes facts; that work is not a Phase 143 hole. Phase 145 owns remaining no-claim closeout.

### Confirmation-Bias Notes

- gsd-tools key-link check for `durable_payload_present(block_hash)` failed because the pattern lives only in `inventory.rs`. Manual wiring is present and tested.
- `durable_cache_miss_store_hit_is_payload_present` feeds `durable_payload_present=true` directly rather than opening Fjall inside the node unit test. The store probe itself is covered by `has_block_is_true_after_save_block_without_calling_load_block`, and the RPC durable-gate test uses a real missing-key store.
- Off-active durable-only hashes stay `Unknown` chain position (SideChain still uses `cache_present`). That is the documented 143-01 deviation and does not authorize Available.

### Commit Evidence

Documented plan commits exist and match SUMMARY hashes:

- `9cde0089` feat(143-01): assemble presence facts and stop injecting Pruned
- `bd8b7f77` feat(143-02): replace durable override with has_block probe
- `507c9167` feat(143-03): report Unavailable on LookupUnavailable
- `0baf0b7f` docs(143-04): rewrite reserved-Pruned Phase 111 claim copy

Provenance: CONTEXT.md, all four PLAN.md files, and all four SUMMARY.md files share `lifecycle_mode: yolo` and `phase_lifecycle_id: 143-2026-09-17T01-47-45`.

---

_Verified: 2026-09-17T06:40:57Z_
_Verifier: Claude (gsd-verifier)_
