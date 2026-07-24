---
phase: 130-resource-time-and-fee-primitives
verified: 2026-07-24T08:55:04Z
status: passed
score: 4/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-24T08:55:04Z
lifecycle_validated: true
overrides_applied: 0
requirements_verified:
  - FEEP-01
  - FEEP-02
  - FEEP-03
  - FEEP-04
  - FEEP-05
deferred:
  - truth: "Accounted-memory capacity enforcement, sustained-pressure trimming, and rolling-fee bump/decay mechanics"
    addressed_in: "Phase 131"
    evidence: "ROADMAP Phase 131 goal: enforce sustained-pressure capacity, eviction, expiry, and block-gated rolling-fee decay; CONTEXT D-03 and parity index deferred_boundaries keep Phase 130 on the accounting/evidence contract with capacityenforcement=legacy_vsize"
  - truth: "Complete cross-cache projection of MempoolLifecycleDelta through every dependent runtime cache"
    addressed_in: "Phase 134"
    evidence: "ROADMAP Phase 134 goal: one runtime authority and lifecycle delta govern every package/mempool consequence across dependent state; CONTEXT D-16 limits Phase 130 to semantic facts and current-cache consumption"
  - truth: "Full mempool snapshot/checkpoint schema evolution, cadence, and crash-loss window beyond truthful legacy metadata compatibility"
    addressed_in: "Phase 135"
    evidence: "ROADMAP Phase 135 goal: persist source mempool records and recover through policy-aware replay; Plan 130-08 keeps SchemaVersion::CURRENT unchanged and only adds optional all-or-none metadata fields"
---

# Phase 130: Resource, Time, and Fee Primitives Verification Report

**Phase Goal:** Operators and contributors can reason about mempool capacity, fee floors, time-dependent policy, and lifecycle results through explicit, non-overloaded contracts.
**Verified:** 2026-07-24T08:55:04Z
**Status:** passed
**Re-verification:** No — initial verification
**Commit checked:** `3550ab50` (working tree otherwise clean aside from staged `130-REVIEW.md`)

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Operator evidence reports transaction virtual size, accounted mempool memory usage, and configured capacity as separate values. | ✓ VERIFIED | Distinct newtypes `TransactionVirtualSize`, `AccountedMempoolMemory`, and `MempoolCapacity` in `packages/open-bitcoin-mempool/src/resource.rs`. RPC `GetMempoolInfoResponse` maps `bytes`/`usage`/`maxmempool` separately (`method/node.rs:42-55`); dispatch projects from `ManagedMempoolInfo` (`dispatch/node.rs:114-120`). Unequal-value assertions live in `get_mempool_info_exposes_truthful_resource_and_fee_evidence` (`dispatch/tests.rs:1451-1490`) and 52 mempool resource/fee/context/lifecycle unit tests passed. Phase 130 checker FEEP-01 mutation fails when resource types disappear. |
| 2 | Operators can distinguish the static relay floor, incremental relay fee, rolling mempool floor, and effective admission floor, and package fees cannot bypass the wrong boundary. | ✓ VERIFIED | Role wrappers `StaticRelayFeeRate`, `IncrementalRelayFeeRate`, `RollingMempoolFeeRate`, and derived `EffectiveAdmissionFeeRate` in `fee.rs`; `effective_admission_fee_rate` is `max(static, rolling)` only. RPC exposes `minrelaytxfee`, `incrementalrelayfee`, `rollingmempoolfee`, `mempoolminfee`/`effectiveadmissionfee`. Tests `incremental_relay_fee_is_not_an_admission_floor` and `package_member_below_static_fails_even_when_aggregate_exceeds_rolling` passed. Checker FEEP-02 + incremental-exclusion mutations pass. |
| 3 | Expiry, recovery, and retry outcomes consistently use explicit acceptance time plus typed local-origin and relay-request metadata. | ✓ VERIFIED | `MempoolEntryMetadata` + `AdmissionContext::{peer,local,reorg,legacy_unknown}` in `context.rs`; retry eligibility requires known local + requested + current membership. Snapshots preserve or fail-closed decode metadata (`snapshot_codec/mempool.rs` `maybe_accepted_at_unix_seconds` all-or-none). Tests `legacy_unknown_metadata_is_not_retry_eligible`, `legacy_mempool_snapshot_decodes_to_fail_closed_metadata`, and `managed_reorg_reacceptance_uses_explicit_event_time` passed. RPC `sendrawtransaction` samples time in shell then calls `_at` (`dispatch/node.rs:284-286`). |
| 4 | Contributors can reproduce admission, replacement, expiry, pressure, block, reorg, and retry decisions from explicit time, block, occupancy, and jitter inputs with stable typed outcomes. | ✓ VERIFIED | Operation contexts in `context.rs` (`AdmissionContext`, `PressureDecisionContext`, `BlockLifecycleContext`, `ReorgLifecycleContext`); `MempoolLifecycleDelta` with independent cause/role plus retry-clear vocabulary in `pool/lifecycle.rs`; injected `RetryDecisionContext`/`RetryJitterSeconds` in network `retry.rs` (0..=300, no clock/RNG in pure crates — rg finds none). Lifecycle delta ordering/dedup tests passed (12+ cases). Pure mempool crate has no `SystemTime`/`rand` usage. |

**Score:** 4/4 truths verified

### Deferred Items

Items intentionally outside Phase 130 scope and owned by later roadmap phases. Not verification gaps.

| # | Item | Addressed In | Evidence |
|---|------|-------------|----------|
| 1 | Accounted-memory enforcement, pressure trim, rolling-fee bump/decay | Phase 131 | ROADMAP Phase 131; `capacityenforcement: "legacy_vsize"`; CONTEXT D-03 |
| 2 | Complete cross-cache lifecycle-delta projection | Phase 134 | ROADMAP Phase 134; CONTEXT D-16 |
| 3 | Full snapshot/checkpoint schema and cadence | Phase 135 | ROADMAP Phase 135; Plan 130-08 schema-stability contract |

### Required Artifacts

Plan-level `gsd-tools verify artifacts` across Plans 01–13: substantive artifacts exist. Two literal `contains` path mismatches are modularization, not missing work:

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-mempool/src/resource.rs` | Resource newtypes + ledger/oracle | ✓ VERIFIED | `MempoolResourceLedger`, distinct vsize/usage/capacity types |
| `packages/open-bitcoin-mempool/src/fee.rs` | Fee role wrappers + package floor contract | ✓ VERIFIED | Four roles + `evaluate_package_fee_floors` |
| `packages/open-bitcoin-mempool/src/context.rs` | Entry metadata + operation contexts | ✓ VERIFIED | Metadata + admission/pressure/block/reorg contexts |
| `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` | `MempoolLifecycleDelta` | ✓ VERIFIED | Cause/role split, final membership, retry clears |
| `packages/open-bitcoin-mempool/src/pool.rs` (+ `pool/admission.rs`) | Ledger ownership + fee wiring | ✓ VERIFIED | `resource_ledger` in `pool.rs`; `effective_admission_fee_rate` called from `pool/admission.rs` (gsd-tools path check against `pool.rs` alone is a false negative) |
| `packages/open-bitcoin-node/src/network/admission_bridge.rs` | Typed peer/local admission + delta apply | ✓ VERIFIED | `apply_admitted_transition`, `submit_local_transaction_outcome_at` |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | Explicit-time local admission | ✓ VERIFIED | `submit_local_transaction_outcome_at`; deprecated no-time `AdmissionResult` path retained fail-closed for wallet (Plan 11 SUMMARY deviation) |
| `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` (+ `mempool.rs`) | Optional all-or-none metadata wire fields | ✓ VERIFIED | `maybe_accepted_at_unix_seconds` lives in `snapshot_codec/mempool.rs` submodule (gsd-tools path check against root file alone is a false negative) |
| `packages/open-bitcoin-rpc/src/method/node.rs` + `dispatch/node.rs` | Truthful getmempoolinfo projection | ✓ VERIFIED | Distinct resource + fee fields + `capacityenforcement` |
| `packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs` | Injected retry time/jitter | ✓ VERIFIED | `RetryDecisionContext`, bounded jitter |
| `docs/parity/index.json` + `catalog/mempool-policy.md` | Unique FEEP ownership + deferred boundaries | ✓ VERIFIED | Surface `v2-2-resource-time-fee-primitives` |
| `scripts/check-phase130-resource-time-fee-primitives.ts` | Mutation-tested FEEP guard | ✓ VERIFIED | Live check exits 0; 24/24 mutation tests pass |
| `scripts/verify.sh` | 129 → 130 → 117 ordering | ✓ VERIFIED | Heredoc + `run_step` lines 418–421 / 570–573 |

### Key Link Verification

All plan key-links reported `verified: true` via `gsd-tools verify key-links` for Plans 01–13. Manual spot-checks confirm:

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `pool.rs` / `pool/admission.rs` | `resource.rs` / `fee.rs` / `context.rs` | Admission updates ledger, derives effective fee, copies metadata | ✓ WIRED | Imports + call sites present |
| `admission_bridge.rs` | `pool/lifecycle.rs` | `apply_admitted_transition` consumes `MempoolTransition.delta` | ✓ WIRED | Uses `MempoolLifecycleDelta` facts |
| `dispatch/node.rs` | `context/network.rs` → authority | Shell-sampled seconds → `_at` | ✓ WIRED | `current_timestamp_unix_seconds` then `submit_local_transaction_with_relay_evidence_at` |
| `block_reconcile.rs` | `runtime_authority` / lifecycle | Explicit reorg timestamp | ✓ WIRED | `ReorgLifecycleContext` + passing reorg test |
| `snapshot_codec/mempool.rs` | `mempool_snapshot.rs` | All-or-none metadata ↔ domain | ✓ WIRED | Known / legacy-unknown / corruption paths |
| checker test → checker → `verify.sh` | Guard integration | 129 then 130 then 117 | ✓ WIRED | Live ordering + mutation coverage |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `getmempoolinfo` RPC | `bytes`/`usage`/`maxmempool`/fee fields | `ManagedMempoolInfo` from live mempool ledger + fee roles | Yes — unequal fixture values in dispatch test | ✓ FLOWING |
| Lifecycle bridge | removals / final membership | `MempoolLifecycleDelta` from committed pool transitions | Yes — admission/block/reorg tests assert typed causes | ✓ FLOWING |
| Snapshot recovery | `MempoolEntryMetadata` | Encoded optional fields or explicit legacy-unknown | Yes — round-trip + legacy decode tests | ✓ FLOWING |
| Retry inputs | `observed_at` + `jitter` | Shell-injected `RetryDecisionContext` | Yes — bounded constructor; scheduling deferred to Phase 136 | ✓ FLOWING (inputs only) |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Phase 130 mutation suite | `bun test scripts/check-phase130-resource-time-fee-primitives.test.ts` | 24 pass, 0 fail (~88s) | ✓ PASS |
| Phase 130 live checker | `bun run scripts/check-phase130-resource-time-fee-primitives.ts` | `Phase 130 resource time and fee primitives validated.` | ✓ PASS |
| Mempool FEEP unit tests | `cargo test -p open-bitcoin-mempool --lib -- resource_cases fee_cases context_cases lifecycle_delta_cases` | 52 passed, 0 failed | ✓ PASS |
| Node reorg + legacy snapshot | `cargo test -p open-bitcoin-node --lib -- managed_reorg_reacceptance legacy_mempool_snapshot` | 2 passed | ✓ PASS |
| RPC unequal getmempoolinfo | `cargo test -p open-bitcoin-rpc --lib get_mempool_info_exposes_truthful_resource_and_fee_evidence` | Blocked this session by concurrent `verify-full` cooperative lock (pid 22378) | ? SKIP — test body inspected; FEEP-01/02 checker mutations cover the same anchors |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| FEEP-01 | 01, 09, 12, 13 | Distinguish vsize, accounted usage, capacity | ✓ SATISFIED | Resource newtypes, ledger/oracle, RPC bytes/usage/maxmempool, checker mutation |
| FEEP-02 | 02, 09, 12, 13 | Distinguish fee roles; packages cannot bypass wrong floor | ✓ SATISFIED | Fee role types, package floor assessment, RPC fee fields, fee_cases tests |
| FEEP-03 | 03, 05–09, 11–13 | Explicit acceptance time + origin + relay metadata | ✓ SATISFIED | Entry metadata, snapshot compatibility, local `_at` admission, retry eligibility rules |
| FEEP-04 | 03, 05–07, 10–13 | Explicit time/block/occupancy/jitter; no pure-crate clock/RNG | ✓ SATISFIED | Operation contexts, retry jitter inputs, rg clean of wall-clock/rand in mempool + relay retry |
| FEEP-05 | 04–09, 11–13 | Stable typed lifecycle outcomes | ✓ SATISFIED | `MempoolLifecycleDelta` cause/role/membership/retry-clear; bridge consumption |

No orphaned Phase 130 requirements: REQUIREMENTS.md maps exactly FEEP-01..05 to Phase 130, and every ID appears in plan frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `runtime_authority.rs` / `admission_bridge.rs` | deprecated `submit_local_transaction` | Fail-closed legacy-unknown adapter retained for wallet `AdmissionResult` callers | ℹ️ Info | Documented Plan 11 deviation; outcome no-time shims removed; not retry-eligible |
| `pool.rs` legacy vsize trim loop | trim still on `legacy_vsize_trim_limit` | Intentional Phase 131 boundary | ℹ️ Info | Evidence field `capacityenforcement=legacy_vsize`; not a Phase 130 gap |
| Core FEEP modules | — | No TODO/FIXME/placeholder stubs in resource/fee/context/lifecycle | — | Clean |

### Human Verification Required

None. Phase 130 deliverables are typed contracts, RPC aggregate fields, and deterministic guards covered by automated tests.

### Gaps Summary

No actionable gaps. Modularization moved two `contains` patterns into submodules (`pool/admission.rs`, `snapshot_codec/mempool.rs`) while preserving wiring. Deferred enforcement, cross-cache projection, and checkpoint schema remain correctly owned by Phases 131/134/135.

### Confirmation-Bias Notes

1. **Partial path still present:** wallet/deprecated no-time `AdmissionResult` admission assigns `legacy_unknown` rather than shell-sampled local metadata — intentional and fail-closed, not counted as a goal failure.
2. **Tests match claims:** fee_cases and getmempoolinfo assertions check unequal roles/values, not mere field presence.
3. **Error path covered:** snapshot partial-metadata corruption and retry jitter out-of-range both have dedicated fail-closed tests/mutations.

---

_Verified: 2026-07-24T08:55:04Z_
_Verifier: Claude (gsd-verifier)_
