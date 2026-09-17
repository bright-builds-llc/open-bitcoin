---
phase: 144-operator-flush-and-availability-evidence
verified: 2026-09-17T21:30:00.000Z
status: passed
score: 16/16 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 144-2026-09-17T11-23-17
generated_at: 2026-09-17T21:30:00.000Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 144: Operator Flush and Availability Evidence Verification Report

**Phase Goal:** Operators can see flush, recovery, cache-size, and have-bytes versus do-not through sanitized surfaces.
**Verified:** 2026-09-17T17:45:58Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

Operators can inspect flush, coins-marker recovery, current cache-size, last flush reason, and have-bytes versus do-not from one dedicated `chainstate_durability` field. That field is retained on the manager and serve seam, then cloned through status, `openbitcoinnetworkstatus`, CLI JSON/human, dashboard rows, fixed metrics, allowlisted logs, and redacted support. Locked decisions hold: one snapshot field, no `getblock`, no ninth chart, no `Pruned` on the new field, no peer ids or coin dumps. Phase 145 still owns CSVFY.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Status, RPC, CLI, dashboard, metrics, logs, and support expose flush, recovery, and have-bytes versus do-not using sanitized low-cardinality fields. | ✓ VERIFIED | Dedicated `OpenBitcoinStatusSnapshot.chainstate_durability` after `block_relay`; RPC clones `snapshot.chainstate_durability()`; CLI/dashboard/support format that field; seven fixed `MetricKind` series; log source `chainstate_durability`. |
| 2 | Operator evidence reports cache-size state (OK / LARGE / CRITICAL) and last flush reason. | ✓ VERIFIED | Machine `ok`/`large`/`critical` plus `last_flush_reason` on the contract; human/dashboard/support map to `OK`/`LARGE`/`CRITICAL`; metrics sample cache-size class 0/1/2 and last-flush-reason class 0..4. |
| 3 | Coins best-block, interrupted-flush or replay outcome, and per-request availability labels are visible without peer identifiers or raw coin dumps. | ✓ VERIFIED | Height+64-hex only; `recovery_outcome` includes `replayed`/`fail_closed`; have-bytes last labels plus three counters; logs omit hash; support redacts peer/hex/credential/outpoint needles. |
| 4 | OpenBitcoinStatusSnapshot carries one dedicated `chainstate_durability` FieldAvailability field distinct from `recovery_evidence` and `block_relay`. | ✓ VERIFIED | `status.rs` field order is `block_relay` → `chainstate_durability` → … → `recovery_evidence`. New struct does not fold those fields. |
| 5 | Stopped or unprojected runtimes expose Unavailable with `CHAINSTATE_DURABILITY_UNAVAILABLE_REASON` and never fabricate occupancy or payload facts. | ✓ VERIFIED | `default_unavailable()` uses the stable reason; managed snapshot stays Unavailable when `NotReady` and recovery is unset; RPC-failure CLI fallback uses inbound-style `openbitcoinnetworkstatus unavailable`. |
| 6 | Current `cache_size` is `decide_flush(FlushMode::None)` at projection; `last_flush_reason` and `write_kind` come from the last real `execute_flush`. | ✓ VERIFIED | `execute_flush` stores the decision only when `mode != FlushMode::None`; projector classifies occupancy via `FlushMode::None` and maps retained write separately. Retention test asserts `periodic`/`sync` after a later None classify. |
| 7 | Fail-closed and interrupted-without-B keep coins best-block fields None and never publish interrupted H hashes. | ✓ VERIFIED | Projector omits tip when outcome is fail-closed/interrupted and `B` is missing; snapshot looks up `coins_best_block()`, not header `H`; initialize missing-`B` maps to `FailClosed`. |
| 8 | Have-bytes last status is unavailable whenever payload is absent; accumulator stores last labels plus three counters only. | ✓ VERIFIED | `record_have_bytes` sets Unavailable and increments `unavailable_count` / `index_known_without_payload_count` when `payload_present` is false. No `HashMap`, `pruned_count`, or `peer_id`. |
| 9 | `openbitcoinnetworkstatus` JSON includes top-level `chainstate_durability`; `getblockchaininfo` and `getnetworkinfo` stay unchanged. | ✓ VERIFIED | Field lives only on `OpenBitcoinNetworkStatusResponse`. Baseline structs have no Open Bitcoin key. Schema test asserts exact keys and unchanged baseline shapes. No `getblock` method. |
| 10 | Live CLI JSON copies the RPC field; human status prints the six locked lines after block-relay and before Wallet. | ✓ VERIFIED | `let chainstate_durability = network_status.chainstate_durability`; render extends after `block_relay_evidence_lines` then prints `Wallet:`. |
| 11 | Dashboard appends the same six rows after `block_relay_rows` inside Mempool and Wallet; `MAX_DASHBOARD_CHARTS` remains 8. | ✓ VERIFIED | `mempool_and_wallet.extend(chainstate_durability_rows(snapshot))`; `MAX_DASHBOARD_CHARTS: usize = 8`; `DASHBOARD_METRIC_KINDS` length 8. |
| 12 | Fixed MetricKind series expose cache-size 0/1/2, last-flush-reason 0..4, write-kind 0..3, recovery 0..3, and the three counters with no dynamic labels. | ✓ VERIFIED | `MetricKind::ALL: [Self; 81]`; `chainstate_durability_metric_samples` emits exactly those seven kinds from the snapshot; Unavailable emits no samples. Persist appends from `operator_snapshot()`. |
| 13 | Structured log source `chainstate_durability` uses cause / outcome / label plus counts and optional height — never coins-best-block hash, peer ids, or outpoints. | ✓ VERIFIED | `available_message` formats locked labels and counts; optional `height=` only; no `maybe_coins_best_block_hash`. Runtime emit lives in `sync/runtime_state/operator_logs.rs`. |
| 14 | Support Markdown renders `## Chainstate Durability` with the six locked bullets plus the exact Next action; free-text reasons go through redaction. | ✓ VERIFIED | `push_chainstate_durability` after block-relay; `redact_chainstate_durability` called from `support_status_for_bundle`; Next action forbids prune/archive/public-default/production-readiness. |
| 15 | Docs document the shared field, cache-size OK/LARGE/CRITICAL, last flush reason, and have-bytes versus do-not without prune/archive/public-default/production-readiness claims; runtime-guide includes Cargo and Bazel commands. | ✓ VERIFIED | `status-snapshot.md`, `operator-observability.md`, and `runtime-guide.md` carry the contract. Runtime-guide lists the four status Cargo/Bazel commands plus support-bundle twins. Phase 145 still owns CSVFY. |
| 16 | Deterministic checker proves surfaces agree on CSOBS fields; new first-party Rust files have breadcrumbs; `verify.sh` runs the checker after Phase 116. | ✓ VERIFIED | Live `bun test` + `bun run` of `scripts/check-phase144-operator-flush-availability-evidence.ts` passed. `verify.sh` inserts the pair immediately after Phase 116. Breadcrumb JSON lists contract, flush, HAVL, metrics, logs, CLI, dashboard, support, and extracted helper files. |

**Score:** 16/16 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/status/chainstate_durability.rs` | Shared contract and `as_str` labels | ✓ VERIFIED | Exists, substantive (`ChainstateDurabilityEvidence`, projector, labels), wired from flush lifecycle and operator snapshot. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | Retained last write and recovery | ✓ VERIFIED | `maybe_last_write_decision` / `maybe_recovery_outcome`; `project_chainstate_durability` copies retained facts. |
| `packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs` | Sibling HAVL accumulator | ✓ VERIFIED | `record_have_bytes` + `index_known_without_payload_count`; hooked from `record_block_serving_evidence`. |
| `packages/open-bitcoin-node/src/network/operator_snapshot.rs` | Managed snapshot copy | ✓ VERIFIED | `chainstate_durability: self.project_chainstate_durability()`. |
| `packages/open-bitcoin-rpc/src/method/node.rs` | RPC response field | ✓ VERIFIED | `pub chainstate_durability` on `OpenBitcoinNetworkStatusResponse`. |
| `packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs` | Six human lines | ✓ VERIFIED | `Chainstate durability:` plus five siblings; `OK`/`LARGE`/`CRITICAL`. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs` | Six dashboard rows | ✓ VERIFIED | Exact title-case labels; no chart/color additions. |
| `packages/open-bitcoin-node/src/metrics/chainstate_durability.rs` | Metric samples | ✓ VERIFIED | `chainstate_durability_metric_samples` reads `FieldAvailability`. |
| `packages/open-bitcoin-node/src/logging/chainstate_durability.rs` | Allowlisted log | ✓ VERIFIED | `CHAINSTATE_DURABILITY_LOG_SOURCE` + `chainstate_durability_log_record`. |
| `packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs` | Support heading | ✓ VERIFIED | `## Chainstate Durability` and locked Next action. |
| `packages/open-bitcoin-cli/src/operator/support/redaction.rs` | Support redaction | ✓ VERIFIED | `redact_chainstate_durability` wired in `support_status_for_bundle`. |
| `docs/architecture/status-snapshot.md` | Shared field contract | ✓ VERIFIED | Dedicated field, locked names, fail-closed tip rule, Phase 145 CSVFY note. |
| `docs/operator/runtime-guide.md` | Cargo/Bazel UAT commands | ✓ VERIFIED | Human/JSON Cargo and Bazel `status --format` twins plus support bundle. |
| `scripts/check-phase144-operator-flush-availability-evidence.ts` | Cross-surface checker | ✓ VERIFIED | Requires `CSOBS-01`/`CSOBS-02`; forbids `getblock`/`pruned` on the new-field corpus. |
| `scripts/check-phase144-operator-flush-availability-evidence.test.ts` | Checker fixtures | ✓ VERIFIED | Live-repo pass plus CSOBS-missing and getblock/pruned fail fixtures. |
| `scripts/verify.sh` | Default verifier wiring | ✓ VERIFIED | Bun test/run after Phase 116 in both the visible list and `run_step` list. |

gsd-tools: artifacts 16/16 passed; key links 12/12 verified.

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `flush_lifecycle.rs` | `status/chainstate_durability.rs` | `project_chainstate_durability` | ✓ WIRED | Lifecycle method forwards retained write/recovery into the I/O-free projector. |
| `block_relay_evidence.rs` | `chainstate_durability_evidence.rs` | `record_have_bytes` | ✓ WIRED | `record_block_serving_evidence` records `decision.presence` on the sibling accumulator. |
| `operator_snapshot.rs` | `status.rs` | `chainstate_durability:` | ✓ WIRED | Managed snapshot field populated by `project_chainstate_durability()`. |
| `rpc/dispatch/node.rs` | `network/types.rs` | `snapshot.chainstate_durability().clone()` | ✓ WIRED | Thin clone; inbound-status getter forwards the managed field. |
| `cli/operator/status.rs` | `rpc/method/node.rs` | live collect copies RPC field | ✓ WIRED | `let chainstate_durability = network_status.chainstate_durability`. |
| `dashboard/model.rs` | `dashboard/model/chainstate_durability.rs` | `chainstate_durability_rows` | ✓ WIRED | Extended after `block_relay_rows`. |
| `status/chainstate_durability.rs` | `metrics/chainstate_durability.rs` | `chainstate_durability_metric_samples` | ✓ WIRED | Samples read `FieldAvailability<ChainstateDurabilityEvidence>`. |
| `sync/metrics.rs` | `metrics/chainstate_durability.rs` | persist append | ✓ WIRED | After mempool_policy, from `operator_snapshot().chainstate_durability()`. |
| `support/render.rs` | `support/render/chainstate_durability.rs` | `push_chainstate_durability` | ✓ WIRED | After block-relay evidence. |
| checker | `verify.sh` | bun test then bun run after Phase 116 | ✓ WIRED | Lines 394–395 and `run_step` 560–561. |
| `runtime-guide.md` | checker | `status --format` corpus | ✓ WIRED | Required Cargo/Bazel commands present in the guide. |
| `source-breadcrumbs.json` | `status/chainstate_durability.rs` | `node-status-contract` | ✓ WIRED | Path registered; sibling new files also grouped. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| Status / managed snapshot | `chainstate_durability` | Retained `FlushDecision` + recovery + `decide_flush(FlushMode::None)` + `HaveBytesAccumulator::snapshot()` | Yes — manager/serve facts, not hardcoded empty Available | ✓ FLOWING |
| RPC `openbitcoinnetworkstatus` | `chainstate_durability` | `snapshot.chainstate_durability().clone()` | Yes — clone of managed field | ✓ FLOWING |
| CLI human/JSON | `snapshot.chainstate_durability` | Live RPC copy; stopped/error use Unavailable | Yes — formatters read snapshot enums only | ✓ FLOWING |
| Dashboard rows | `snapshot.chainstate_durability` | Same snapshot | Yes — six rows from Available or wrapper reason | ✓ FLOWING |
| Metrics | seven `MetricSample`s | Snapshot occupancy/write/recovery/counters | Yes — Unavailable returns `[]`, not fabricated 0/0/0 | ✓ FLOWING |
| Structured log | message labels/counts | Snapshot; optional height only | Yes — no hash field | ✓ FLOWING |
| Support Markdown/JSON | bullets + `status.chainstate_durability` | Snapshot after `redact_chainstate_durability` | Yes — tip-hash exception is the existing 64-hex pattern | ✓ FLOWING |

CLI/dashboard/support durability modules contain no `decide_flush`, Fjall, `getblock`, or `pruned` emit strings.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Cross-surface checker passes on the live repo | `bun test scripts/check-phase144-operator-flush-availability-evidence.test.ts` | 4 pass / 0 fail | ✓ PASS |
| Checker validates CSOBS corpus | `bun run scripts/check-phase144-operator-flush-availability-evidence.ts` | `Phase 144 operator flush and availability evidence validated.` | ✓ PASS |
| New-field files do not emit getblock/pruned/peer_id | `rg` on durability modules | Matches only rustdoc forbids | ✓ PASS |
| HAVL accumulator has no request-log types | `rg HashMap\|pruned_count\|peer_id` on accumulator | No matches | ✓ PASS |
| Cargo unit tests for named behaviors | `cargo test …` | SKIP — compile exceeds the 10s spot-check bound; named tests exist in-tree and are required by the checker | ? SKIP |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| CSOBS-01 | 144-01, 144-02, 144-03, 144-04 | Status, RPC, CLI, dashboard, metrics, logs, and support expose flush, recovery, and have-bytes vs do-not using sanitized low-cardinality fields. | ✓ SATISFIED | Shared field plus all seven surfaces; checker `REQUIRED_REQUIREMENTS` includes `CSOBS-01`. |
| CSOBS-02 | 144-01, 144-02, 144-03, 144-04 | Operator evidence reports cache-size state (OK / LARGE / CRITICAL) and last flush reason. | ✓ SATISFIED | Machine labels, human tokens, metric classes, and docs; checker requires `CSOBS-02`, `cache_size`, `last_flush_reason`, `ok`/`large`/`critical`. |

No orphaned Phase 144 requirements. REQUIREMENTS.md maps only CSOBS-01 and CSOBS-02 to this phase; both appear in every plan's `requirements:` frontmatter.

CSVFY-01 and CSVFY-02 remain Phase 145. This phase documents have-bytes versus do-not and forbids prune/archive/public-default/production-readiness claims on the new field without closing those later guardrails.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | `coins_recovery_outcome_after_success` | `CoinsRecoveryOutcome::Interrupted` is never assigned on the initialize success path | ℹ️ Info | Successful two-head replay becomes `Replayed`; failed replay returns `StorageError` with `fail_closed`/`interrupted` in Display. Operators still see replay vs fail-closed. Projector/JSON still accept `Interrupted` and omit tip without `B`. Not a goal failure. |

No TODO/FIXME/placeholder stubs in the new durability modules. Empty `HaveBytesEvidence::empty()` and Unavailable defaults are overwritten by real projection or rendered as Unavailable.

### Confirmation Bias Notes

1. **Partial production assignment:** `Interrupted` exists on the shared enum and in fixture tests, but `initialize` never stores `CoinsRecoveryOutcome::Interrupted`. Distinguishability for a live Available `interrupted` label is therefore contract-ready, not a runtime initialize outcome. Replay success and fail-closed errors still satisfy the roadmap “interrupted-flush or replay outcome” wording.
2. **Fixture-constructed Interrupted JSON:** `interrupted_without_tip` builds the enum directly. It proves serialization/omission, not that initialize emits that variant.
3. **Replay-without-B path:** `InterruptedTwoHeads` success does not inspect `best_block()`. If `B` is missing after replay, outcome is `Replayed` and tip fields stay `None` via the generic missing-hash branch. No interrupted `H` substitution.

### Human Verification Required

None. Surfaces are text/JSON/metrics/log/support contracts with locked strings. Default verification is deterministic and public-network-free (D-23). Live daemon UAT stays opt-in per `runtime-guide.md`.

### Gaps Summary

No gaps. Phase goal achieved.

### Locked-Decision Check

- One `chainstate_durability` snapshot field — yes.
- No `getblock` product — no method added; forbidden on the new-field corpus.
- No ninth chart — `MAX_DASHBOARD_CHARTS = 8`.
- No `Pruned` on the new field — accumulator and renderers omit it.
- No peer ids or coin dumps — last labels + three counters; height+hash only.
- Phase 145 still owns CSVFY — no Phase 145 checker; docs say so explicitly.

---

_Verified: 2026-09-17T17:45:58Z_
_Verifier: Claude (gsd-verifier)_
