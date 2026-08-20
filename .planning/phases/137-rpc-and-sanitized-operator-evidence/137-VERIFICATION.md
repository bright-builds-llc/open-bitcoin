---
phase: 137-rpc-and-sanitized-operator-evidence
verified: 2026-08-20T00:11:10Z
status: passed
score: 4/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-20T00:11:10Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 137: RPC and Sanitized Operator Evidence Verification Report

**Phase Goal:** Operators can inspect and exercise the scoped package and long-lived mempool behavior through one stable, redacted evidence contract.
**Verified:** 2026-08-20T00:07:11Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

The phase delivers one shared, identifier-free evidence contract on status, dashboard, metrics, logs, and support bundles, plus originating authenticated RPC/CLI responses for package dry-run, package submit, and mempool info. Local admission and relay/fanout stay independent. Public/default relay is not claimed. Phase 138 parity, adversarial, benchmark, and claim-guardrail closeout is out of scope.

### Observable Truths

| #   | Truth   | Status     | Evidence       |
| --- | ------- | ---------- | -------------- |
| 1   | RPC and CLI expose package dry-run, package submission, and mempool information with stable errors and per-transaction results matching the authoritative core. | ✓ VERIFIED | `testmempoolaccept` / `submitpackage` are `BaselineParity` and project `PackageReport` via `project_testmempoolaccept` / `project_submitpackage`. `openbitcoinpackage` plus `open-bitcoin package {dry-run,submit}` carry the typed report. `open-bitcoin-cli` forwards baseline method names generically. RPC-level errors use Knots codes -8 / -22 / -25; member policy stays in the result body. |
| 2   | Status, dashboard, metrics, logs, and support bundles separately report vsize, accounted usage, capacity, fee floors, pressure/decay, eviction, checkpoint, recovery, and retry outcomes with fixed low-cardinality fields. | ✓ VERIFIED | `MempoolStatus` groups are published on `openbitcoinnetworkstatus`, copied by the CLI collector, rendered as UI-SPEC rows, sampled as fixed `MetricKind`s, logged under `mempool_policy`, and copied into shareable support Markdown. Eviction stays distinct from pressure via `relay.outcome_counters.evicted_count` / `RelayEvictedCount`. |
| 3   | Shared evidence distinguishes accepted, still-present, eligible, queued, attempted, emitted, requested, served, suppressed, and cleared states without leaking identifiers or per-member details beyond the authenticated direct response. | ✓ VERIFIED | Admission and retry groups plus dashboard/status/support "Admission states" / "Relay states" rows expose those tokens as counts. Dual-state proofs forbid `members` / `fingerprint` / `txid` / `wtxid` on `openbitcoinnetworkstatus.mempool`. Recovery mapping copies counts only. |
| 4   | Relay-disabled or peer-policy-suppressed operation can show successful local admission while truthfully showing that no public/default relay or propagation result was achieved. | ✓ VERIFIED | Relay-disabled submit can be `accepted` / `still-present` with member `relay=relay_disabled`. Shared aggregates increment those counts. Knots `sendrawtransaction` stays `{txid_hex, replaced_txids, evicted_txids}`. Forbidden keys `propagated` / `broadcast` / `public_relay` are rejected on the typed report and status groups. |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected    | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-rpc/src/package_projection.rs` | Pure PackageReport projector family | ✓ VERIFIED | `project_testmempoolaccept`, `project_submitpackage`, `project_open_bitcoin_package`; uses `report.members()`; no `SystemTime` / hex decode / mempool mutation. |
| `packages/open-bitcoin-node/src/network/admission_bridge/local_package.rs` | Local dry-run and submit | ✓ VERIFIED | `dry_run_local_package` uses `DryRunPackageCommand` only; `submit_local_package` uses `SubmitPackageCommand` then `LifecycleCommand::PackageAdmission` with `AdmissionProjectionSource::Local`. |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | Handle facades | ✓ VERIFIED | `dry_run_local_package` / `submit_local_package` take shell-sampled time and `RelayIntent`. |
| `packages/open-bitcoin-rpc/src/method/package.rs` | Knots + extension request types | ✓ VERIFIED | `TestMempoolAcceptRequest`, `SubmitPackageRequest`, `OpenBitcoinPackageRequest` with `deny_unknown_fields` and required `mode`. |
| `packages/open-bitcoin-rpc/src/dispatch/package.rs` | Decode, count, topology, project, dispatch | ✓ VERIFIED | Wired from `dispatch.rs`; dry-run vs submit vs extension projectors. |
| `packages/open-bitcoin-node/src/status/mempool_groups.rs` | Identifier-free D-10 groups | ✓ VERIFIED | resources, fee_floors, pressure, eviction, checkpoint, recovery, retry, admission. |
| `packages/open-bitcoin-rpc/src/method/node.rs` | Status RPC mempool + unchanged sendraw | ✓ VERIFIED | `pub mempool` on network status; `SendRawTransactionResponse` has no dual-state fields. |
| `packages/open-bitcoin-node/src/network/types.rs` | Operator snapshot fields | ✓ VERIFIED | Checkpoint / retry / admission published through `ManagedNetworkOperatorSnapshot`. |
| `packages/open-bitcoin-cli/src/operator/package.rs` | clap package dry-run/submit | ✓ VERIFIED | Calls `openbitcoinpackage`; prints fingerprint, members, dual-state; submit disclaimer denies public/default relay. |
| `packages/open-bitcoin-cli/src/operator/status/render/mempool_policy.rs` | Human policy lines | ✓ VERIFIED | Locked UI-SPEC labels including `Virtual size` and `Admission states`. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs` | Dashboard rows from snapshot groups | ✓ VERIFIED | Extends `mempool_policy_entries`; `MAX_DASHBOARD_CHARTS` stays 8. Literal `Admission states` lives in the shared renderer, not this file. |
| `packages/open-bitcoin-node/src/metrics.rs` | Fixed MetricKind additions | ✓ VERIFIED | `MempoolVirtualSize` through retry/admission kinds; `as_str` stems match snapshot fields. |
| `packages/open-bitcoin-node/src/logging.rs` | Allowlisted mempool policy log source | ✓ VERIFIED | `MEMPOOL_POLICY_LOG_SOURCE` + `mempool_policy_log_record`. |
| `packages/open-bitcoin-cli/src/operator/support/redaction.rs` | Redaction of new groups | ✓ VERIFIED | `redact_mempool_policy_groups` on `support_status_for_bundle`. |
| `packages/open-bitcoin-cli/src/operator/support/render/relay.rs` | Support Markdown policy bullets | ✓ VERIFIED | Count-only bullets plus locked next-action sentence. Literal is split across const + format. |
| `packages/open-bitcoin-rpc/src/dispatch/tests/dual_state.rs` | Relay-disabled dual-state proofs | ✓ VERIFIED | Accept + still-present + no status member table + no propagation keys. |
| `docs/parity/catalog/rpc-cli-config.md` | Package RPC/CLI catalog rows | ✓ VERIFIED | BaselineParity vs extension split; no public/default relay claim. |
| `docs/parity/source-breadcrumbs.json` | Breadcrumbs for new sources | ✓ VERIFIED | Registers `package_projection.rs`, `local_package.rs`, `mempool_groups.rs`, `mempool_policy` metrics/logs, CLI renderers. |

gsd-tools pattern misses on 137-01 (`report.members\\(\\)`), 137-07 (`Admission states` in `relay.rs`), and 137-09 (`Next action: Treat package` as one literal) are string-match false positives. The wiring exists.

### Key Link Verification

| From | To  | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `package_projection.rs` | `package/report.rs` | `report.members()` | WIRED | Used for both Knots projectors and the typed report. |
| `local_package.rs` | `open-bitcoin-mempool` package commands | `DryRunPackageCommand` vs `SubmitPackageCommand` | WIRED | Dry-run never applies lifecycle. |
| `dispatch/package.rs` | `package_projection.rs` | `project_testmempoolaccept` / `project_submitpackage` | WIRED | Also `project_open_bitcoin_package`. |
| `dispatch/node.rs` | `status.rs` | `OpenBitcoinNetworkStatusResponse.mempool` | WIRED | Fills all eight groups from the operator snapshot. |
| `mempool_groups.rs` | recovery summary | count-only `recovered_count` | WIRED | `recovery_group_from_summary` copies counts, not txids. |
| `operator/package.rs` | `method/package.rs` | HTTP JSON-RPC `openbitcoinpackage` | WIRED | |
| `operator/status.rs` | network status RPC | `network_status.mempool` clone | WIRED | Collector copies groups instead of discarding them. |
| `metrics.rs` | snapshot field stems | `mempool_virtual_size` | WIRED | |
| `inbound_metrics.rs` | `mempool_policy_metric_samples` | persist path | WIRED | Also wired from `DurableSyncRuntime::persist_metrics`. |
| `support/redaction.rs` | `MempoolStatus` | `redact_mempool_policy_groups` | WIRED | |
| `dual_state.rs` | typed package report | `openbitcoinpackage` | WIRED | |
| `rpc-cli-config.md` | `dispatch/package.rs` | documented split | WIRED | |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `openbitcoinnetworkstatus.mempool` | resources / fee_floors / pressure / eviction / checkpoint / recovery / retry / admission | `authoritative_operator_snapshot()` → `ManagedMempoolInfo` + lifecycle evidence | Yes — live occupancy, floors, and count aggregates | ✓ FLOWING |
| Human status / dashboard rows | `mempool_policy_entries` | cloned `network_status.mempool` | Yes — same snapshot groups | ✓ FLOWING |
| Metrics | `mempool_policy_metric_samples` | Available `MempoolStatus` groups | Yes — omitted when unavailable, not hardcoded empty | ✓ FLOWING |
| Support Markdown | `mempool_policy_entries` after redaction | same snapshot | Yes — counts kept; free-text reasons sanitized | ✓ FLOWING |
| Knots / extension package JSON | `PackageReport` members | `dry_run_local_package` / `submit_local_package` | Yes — authoritative Phase 132 report | ✓ FLOWING |
| `MempoolRetryGroup.attempted` | attempted | passed as `0` in `operator_snapshot.rs` | Field exists and is distinct; no separate live leftover counter yet | ℹ️ STATIC (documented; not a Phase 137 goal failure) |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Baseline + extension methods registered | grep `SupportedMethod` names and `origin()` | `testmempoolaccept` / `submitpackage` BaselineParity; `openbitcoinpackage` OpenBitcoinExtension | ✓ PASS |
| CLI operator path calls extension RPC | grep `operator/package.rs` | `method: "openbitcoinpackage"` plus dry-run/submit clap | ✓ PASS |
| Baseline CLI forwards method names | read `client.rs` `CliCommand::RpcMethod` | Generic forwarder; no Open Bitcoin package parser | ✓ PASS |
| Relay-disabled dual-state proof exists | read `dispatch/tests/dual_state.rs` | Accept/still-present + `relay_disabled` + no status member table | ✓ PASS |
| UAT command forms documented | grep `docs/parity/service-operation-expectations.md` | Cargo and Bazel forms for `testmempoolaccept`, `submitpackage`, and `open-bitcoin package` | ✓ PASS |

Runtime cargo tests were not executed in this pass (file/wiring verification only).

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| MPOBS-01 | 137-01, 137-02, 137-03, 137-06, 137-11 | RPC and CLI expose scoped package dry-run, submit, and mempool-info with stable errors and per-tx results matching the core. | ✓ SATISFIED | Projectors + handle methods + BaselineParity dispatch + extension CLI + `getmempoolinfo` aliases + catalog/UAT. |
| MPOBS-02 | 137-04, 137-05, 137-07, 137-08, 137-09, 137-11 | Status, dashboard, metrics, logs, and support bundles distinguish vsize, usage, capacity, floors, pressure/decay, eviction, checkpoint, recovery, and retry with fixed low-cardinality fields. | ✓ SATISFIED | Shared `MempoolStatus` groups rendered and sampled on every listed surface. |
| MPOBS-03 | 137-05, 137-06, 137-07, 137-09, 137-10, 137-11 | Shared evidence is redacted and distinguishes admission/relay states; identifiers stay on the originating authenticated response. | ✓ SATISFIED | Dual-state tokens on extension/operator path; aggregates only on shared surfaces; support redaction + locked next-action copy. |

No orphaned Phase 137 requirements. MPVFY-01..04 belong to Phase 138 and were not treated as gaps.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-node/src/network/operator_snapshot.rs` | 85 | `attempted` passed as `0` | ℹ️ Info | Documented in `mempool_groups.rs`: no separate live leftover counter yet. The `attempted` key still exists on shared evidence. |
| `packages/open-bitcoin-node/src/logging/mempool_policy.rs` | builder only | No new persist hook | ℹ️ Info | Matches existing `relay_mempool_log_record` pattern (no production writer). Builder is allowlisted and tested. |
| `packages/open-bitcoin-rpc/src/package_projection/tests.rs` | — | No dedicated `Reconsiderable` projector case | ℹ️ Info | Production match arm exists and uses the same Knots reject-reason body path as other failures. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs` | 55-58 | Pre-existing `Public relay` capability row | ℹ️ Info | Phase 105 capability evidence (`public_relay_readiness`), not a Phase 137 propagation claim. Dual-state tests forbid a `public_relay` result field. |

No blocker stubs, empty handlers, or identifier-bearing shared DTOs were found.

### Human Verification Required

None. Frontmatter status is restricted to `passed` | `gaps_found`. Visual dashboard layout is locked by UI-SPEC unit tests on labels, order, and chart count.

### Gaps Summary

No actionable gaps. Automated gsd-tools misses were pattern-escape / split-literal false positives. `attempted` remaining zero and the log builder lacking a new persist hook follow documented Phase 137 / prior-phase contracts and are not Phase 138 work.

---

_Verified: 2026-08-20T00:07:11Z_
_Verifier: Claude (gsd-verifier)_
