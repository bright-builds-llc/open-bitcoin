---
phase: 72-operator-observability-and-support-evidence
verified: 2026-06-13T21:02:04Z
status: passed
score: "8/8 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 72-2026-06-13T16-25-04
generated_at: 2026-06-13T21:02:04Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 72: Operator Observability and Support Evidence Verification Report

**Phase Goal:** Operators can inspect and share one coherent full-sync truth contract across CLI, dashboard, RPC, metrics, logs, live-smoke reports, and support bundles.
**Verified:** 2026-06-13T21:02:04Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | CLI status, dashboard, RPC, metrics, structured logs, live-smoke reports, and support bundles share one full-sync truth contract. | VERIFIED | CLI and dashboard use shared `sync_truth_render` helpers over `snapshot.sync`; RPC exposes the Open Bitcoin sync-status contract; metrics/logs emit `validated_active_chain_height`; live-smoke/support carry the same final-status fields. |
| 2 | Redacted support evidence includes initial/final tip, connected height/hash/work, restart/resume checkpoints, stay-current window, peer contribution, no-progress/reorg events, resource pressure, and final verdict. | VERIFIED | `derive_full_sync_evidence` builds `FullSyncEvidence`; `support/live_smoke.rs` allowlists summary-only Phase 72 keys; `operator_binary.rs` asserts JSON and Markdown evidence fields. |
| 3 | Cross-surface comparison confirms agreement on connected chain progress, tip freshness, recovery category, peer health, and next action. | VERIFIED | `scripts/check-phase72-observability-evidence.ts` validates surface agreement and now keeps RPC baseline exclusion separate from support verdict evidence. |
| 4 | Operator guidance explains whether evidence proves sync-to-tip, stay-current behavior, diagnosed blocker, inconclusive evidence, or deferred production-node scope. | VERIFIED | Runtime guide documents `sync_to_tip_proven`, `stay_current_proven`, `diagnosed_blocker`, `inconclusive`, and explicit deferred scope boundaries. |
| 5 | CLI human status renders validated active-chain height/hash/work separately from header/downloaded/connected progress. | VERIFIED | `status/render.rs` prints sync progress with validated active-chain fields; `phase72_cli_status_renders_full_sync_truth_contract` asserts exact human output and unavailable reasons. |
| 6 | Dashboard rows expose the same best-known tip, stay-current, no-progress, reorg, reconcile, resource-pressure, and validated-work facts. | VERIFIED | `dashboard/model.rs` renders rows for best-known tip, stay-current, no-progress, pressure, reorg, reconcile, and progress; dashboard tests assert the validated active-chain fields. |
| 7 | Open Bitcoin-specific RPC sync status preserves durable Phase 68-71 fields while baseline `getblockchaininfo` remains Knots-compatible and excludes support-only fields. | VERIFIED | RPC tests assert Phase 72 sync metadata and verify `getblockchaininfo` excludes `best_known_tip`, `stay_current`, `support_evidence`, `evidence_verdict`, and `validated_active_chain_work`. |
| 8 | Default verification runs Phase 72 deterministic checks after Phase 71 without public-network, service-manager, long-sync, or current-tip timing dependencies. | VERIFIED | `scripts/verify.sh` runs `check-phase71-resource-restart.ts` then `check-phase72-observability-evidence.ts`; the checker forbids live-smoke/manual-peer/restart/service-manager/default-mainnet dependencies. |

**Score:** 8/8 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `packages/open-bitcoin-cli/src/operator/sync_truth_render.rs` | Shared full-sync rendering helpers | VERIFIED | Formats progress, best-known tip, stay-current, no-progress, reorg, and reconcile fields. |
| `packages/open-bitcoin-cli/src/operator/status/render.rs` | CLI status projection | VERIFIED | Consumes `snapshot.sync` and renders Phase 72 status lines. |
| `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` | CLI regression coverage | VERIFIED | Asserts contract fields and unavailable reasons. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` | Dashboard projection | VERIFIED | Adds rows for shared sync truth fields. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` | Dashboard regression coverage | VERIFIED | Asserts row labels and validated active-chain output. |
| `packages/open-bitcoin-rpc/src/dispatch/tests.rs` | RPC contract and baseline exclusion tests | VERIFIED | Covers Open Bitcoin sync status and support-only field exclusion. |
| `packages/open-bitcoin-cli/src/operator/support.rs` | Support bundle wiring | VERIFIED | Calls `derive_full_sync_evidence` and stores `full_sync_evidence`; artifact checker pattern miss for `SupportEvidenceVerdict` is satisfied by `support/evidence.rs`. |
| `packages/open-bitcoin-cli/src/operator/support/evidence.rs` | Typed support evidence verdicts | VERIFIED | Defines verdict enum and derives sync-to-tip/stay-current/blocker/inconclusive labels. |
| `packages/open-bitcoin-cli/src/operator/support/render.rs` | Support Markdown rendering | VERIFIED | Renders verdict and partial active-chain fields with exact unavailable reasons. |
| `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` | Summary-only live-smoke ingestion | VERIFIED | Allowlists Phase 72 final-status keys and rejects raw report/log fields. |
| `packages/open-bitcoin-cli/src/operator/support/tests.rs` | Verdict regression tests | VERIFIED | Includes peer-shortfall-without-blocking regression and normal resource-pressure inconclusive test. |
| `scripts/run-live-mainnet-smoke.ts` | Live-smoke schema/report projection | VERIFIED | Emits schema v2 final status with validated active-chain, tip, stay-current, no-progress, reorg, reconcile, resource, and peer fields. |
| `scripts/test-run-live-mainnet-smoke.sh` | Deterministic live-smoke fixture checks | VERIFIED | Asserts full Phase 72 evidence and missing validated height remains null with an unavailable reason. |
| `packages/open-bitcoin-node/src/metrics.rs` | Metrics kind | VERIFIED | Adds `validated_active_chain_height`. |
| `packages/open-bitcoin-node/src/sync/types/summary.rs` | Metric/log data source | VERIFIED | Emits validated active-chain metric/log fields, resource pressure, peer contribution, stop reason, and recovery category. Test-name pattern is in `summary/tests.rs`, not the source file. |
| `packages/open-bitcoin-node/src/sync/types/summary/tests.rs` | Metric/log regression coverage | VERIFIED | Asserts Phase 72 metric and structured-log dimensions. |
| `scripts/check-phase72-observability-evidence.ts` | Deterministic contract checker | VERIFIED | Validates plans, source/test anchors, redaction, docs, parity breadcrumbs, and default verification boundaries. |
| `scripts/verify.sh` | Repo-native verification wiring | VERIFIED | Runs Phase 72 checker after Phase 71 checker. |
| `docs/operator/runtime-guide.md` | Operator guidance | VERIFIED | Documents verdict meanings, evidence fields, commands, and deferred scope. |
| `docs/architecture/status-snapshot.md` | Status contract documentation | VERIFIED | Documents Phase 72 comparison fields. |
| `docs/architecture/operator-observability.md` | Cross-surface observability documentation | VERIFIED | Documents shared fields across status, telemetry, live-smoke, and support evidence. |
| `docs/parity/catalog/*.md` and `docs/parity/source-breadcrumbs.json` | Parity/deferred-scope evidence | VERIFIED | Catalogs record Phase 72 boundaries; breadcrumbs cover new first-party source anchors. |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| CLI status/dashboard | `OpenBitcoinStatusSnapshot.sync` | Shared render helpers over `snapshot.sync` | WIRED | Key-link verifier passed for Plan 72-01. |
| RPC tests | RPC node method serialization | `OpenBitcoinSyncControlResponse` over runtime metadata | WIRED | Phase 72 fields exposed only on Open Bitcoin status; baseline RPC exclusion tested. |
| Support bundle | Status snapshot and live-smoke summary | `derive_full_sync_evidence(&status, &live_smoke)` | WIRED | Key-link verifier passed for Plan 72-02. |
| Support Markdown | Support bundle evidence | `bundle.full_sync_evidence` | WIRED | Markdown renders typed verdict and active-chain evidence. |
| Sync summary | Metrics registry | `SyncRunSummary::metric_samples` emits `MetricKind::ValidatedActiveChainHeight` | WIRED | Key-link verifier passed for Plan 72-03. |
| Live-smoke script | Support live-smoke ingestion | Schema v2 `final_status` keys match support allowlist | WIRED | Deterministic fixture and support ingestion tests cover the link. |
| Phase 72 checker | `scripts/verify.sh` | Checker-order assertion and verify-script invocation | WIRED | Key-link verifier passed for Plan 72-04. |
| Phase 72 checker | Docs and support evidence source | Required anchors and forbidden defaults | WIRED | Direct `bun run scripts/check-phase72-observability-evidence.ts` passed. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| CLI status renderer | `snapshot.sync.*` | `OpenBitcoinStatusSnapshot` from node status | Yes | FLOWING |
| Dashboard model | `snapshot.sync.*` | Same status snapshot model as CLI | Yes | FLOWING |
| RPC sync status | `sync_state.sync.*` | Runtime metadata serialized by Open Bitcoin RPC status | Yes | FLOWING |
| Support evidence | `full_sync_evidence` | Status snapshot plus summary-only live-smoke evidence | Yes | FLOWING |
| Support Markdown | `bundle.full_sync_evidence` | Generated support bundle JSON model | Yes | FLOWING |
| Metrics/logs | `SyncRunSummary` | Durable sync summary values | Yes | FLOWING |
| Live-smoke report | `final_status` | Parsed status/RPC values and deterministic fixtures | Yes | FLOWING |
| Deterministic checker | Source/docs/test file anchors | Repository files read by Bun script | Yes | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| TypeScript checker is syntactically valid | `bun --check scripts/check-phase72-observability-evidence.ts` | `validated Phase 72 observability/support evidence` | PASS |
| Deterministic Phase 72 evidence checker passes | `bun run scripts/check-phase72-observability-evidence.ts` | `validated Phase 72 observability/support evidence` | PASS |
| Formal plan key links are wired | `gsd-tools verify key-links` for all four Phase 72 plans | 12/12 verified | PASS |
| Formal plan artifacts exist/substantive | `gsd-tools verify artifacts` for all four Phase 72 plans | 12/14 direct pattern checks; 2/2 manual supplements verified in adjacent implementation/test files | PASS |
| Orchestrator post-fix verification | User-provided evidence: focused Cargo tests, live-smoke fixture, Phase 71/72 checkers, parity breadcrumbs, file lengths, and full `bash scripts/verify.sh` | Full verify passed in 14m 11.756s | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|-------------|--------|----------|
| OBS-01 | 72-01, 72-03, 72-04 | Shared full-sync truth contract across CLI, dashboard, RPC, metrics, logs, live-smoke reports, and support bundles | SATISFIED | Shared status fields, checker anchors, tests, metrics/logs, and docs verified. |
| OBS-02 | 72-02, 72-03, 72-04 | Redacted support evidence with full-sync proof fields and final verdict | SATISFIED | Support evidence JSON/Markdown, live-smoke allowlist, redaction checks, and operator binary test verified. |
| OBS-03 | 72-01, 72-03, 72-04 | Cross-surface agreement on progress, freshness, recovery, peer health, and next action | SATISFIED | Checker validates surface agreement; CLI/dashboard/RPC/live-smoke/metrics share field names and values. |
| OBS-04 | 72-02, 72-04 | Guidance explaining proof states and deferred production-node scope | SATISFIED | Runtime guide and parity catalogs document verdict meanings and non-claims. |

No orphaned Phase 72 requirements were found in `.planning/REQUIREMENTS.md`; OBS-01 through OBS-04 are all claimed by Phase 72 plans.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `scripts/run-live-mainnet-smoke.ts` | multiple | `return null` / `return []` | Info | Legitimate nullable/missing-evidence modeling and empty collection defaults, covered by deterministic fixture checks. |
| `scripts/check-phase72-observability-evidence.ts` | 561 | `console.log` | Info | Success output for a repo-owned checker, not a stub. |
| `scripts/check-phase71-resource-restart.ts` | 168 | `console.log` | Info | Existing checker success output, not part of Phase 72 behavior. |

No blocker anti-patterns, placeholders, hollow props, or unwired stubs were found in the Phase 72 changed source/docs/checker files.

### Human Verification Required

None. The phase goal is a deterministic data contract and support-evidence surface; visual/public-network UAT remains explicitly outside Phase 72 and is covered by later roadmap scope.

### Gaps Summary

No blocking gaps found. Phase 72 achieves the roadmap goal: the operator-facing status, telemetry, live-smoke, support bundle, and documentation surfaces share one coherent full-sync truth contract, and the review-fix regressions are covered by code and tests.

---

_Verified: 2026-06-13T21:02:04Z_
_Verifier: the agent (gsd-verifier)_
