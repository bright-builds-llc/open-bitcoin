---
phase: 59-operator-evidence-threat-model-and-release-boundaries
verified: 2026-06-05T19:55:39Z
status: passed
score: 6/6 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 59-2026-06-05T15-10-59
generated_at: 2026-06-05T19:55:39Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 59: Operator Evidence, Threat Model, and Release Boundaries Verification Report

**Phase Goal:** Close v1.4 with coherent operator evidence, support artifacts, docs, security analysis, and scoped release claims.
**Verified:** 2026-06-05T19:55:39Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | OBS-01: Status, dashboard, metrics, structured logs, RPC-facing blockchain info, and live-smoke snapshots preserve shared operator evidence fields. | VERIFIED | `summary.rs` has `sync_summary_projects_consistent_operator_evidence_fields` and structured log evidence for `header=840100 downloaded=840006 connected=840004 signal=block_progress`; status renderer asserts `headers=840100 downloaded_blocks=840006 connected_blocks=840004`, `awaiting_blocks`, latest peer error, and peer source; dashboard test moved to `model/tests.rs` and asserts the same fields plus `inbound=0 outbound=2`; RPC test asserts `headers=840100`, `blocks=840004`, `initialblockdownload=true`, and latest warning; live-smoke fixture checks `firstHeaderProgress`, `firstBlockProgress`, and `restartResumeEvidence.recoveryDiagnosis`. |
| 2 | OBS-02: Support bundle v1.4 live-smoke summary is allowlisted/redacted and surfaces first-header, first-block, restart/resume, recovery, peer outcome, final status, and unavailable evidence. | VERIFIED | `support.rs` delegates to `live_smoke::summary`; `support/live_smoke.rs` allowlists `firstHeaderProgress`, `firstBlockProgress`, `restartResumeEvidence`, `finalStatus`, `peerOutcomeSummary`, and sanitizes retained values; render labels include `First header progress`, `First block progress`, `Restart/resume evidence`, `Recovery diagnosis`, and `Final status`; `operator_binary.rs` fixture asserts summarized fields and absence of `live-smoke-secret`, raw daemon tails, endpoint tables, manual peers, raw endpoint address, cookie-like text, wallet-like text, and raw `rpcpassword` from JSON and Markdown. |
| 3 | OBS-03: Runtime and architecture docs provide repo-local Cargo/Bazel/UAT commands and field-level interpretation. | VERIFIED | `docs/operator/runtime-guide.md` includes `bash scripts/verify.sh`, deterministic fixture smoke, manual-peer live smoke, restart/resume, Cargo/Bazel sync status, and Cargo/Bazel support bundle commands; it interprets `result.status`, `result.progressDetected`, `result.firstHeaderProgress`, `result.firstBlockProgress`, `result.restartResumeEvidence`, final status counters, `support-evidence.json`, and `support-evidence.md`; architecture docs name `OpenBitcoinStatusSnapshot` as shared truth for status, dashboard, support evidence, RPC-facing blockchain info, metrics, structured logs, and live-smoke snapshots; config docs preserve metadata-only credential wording. |
| 4 | SEC-01: Reviewer-facing v1.4 threat model covers public peer compatibility, header/block input, resource bounds, restart/resume evidence, report redaction, support evidence, and operator-facing live evidence. | VERIFIED | `docs/parity/threat-model-v1.4.md` contains required sections, `V14-TM-01` through `V14-TM-08`, OWASP ASVS v5.0.0 L1 mapping, evidence acceptance, release boundary matrix, requirements traceability, and residual risks. |
| 5 | SEC-02: Parity/release docs distinguish v1.4 opt-in outbound IBD progress from deferred surfaces and preserve v1.3 history. | VERIFIED | `docs/parity/index.json` has done surface `v1-4-operator-evidence-release-boundaries` with all six OBS/SEC requirements and evidence paths; `checklist.md`, `README.md`, and `release-readiness.md` link `threat-model-v1.4.md` while preserving `v1-3-threat-model-release-boundaries`; release/deviations/P2P docs explicitly defer inbound serving, transaction relay, production-funds wallet use, migration apply mode, packaging, hosted dashboard, GUI, Windows service support, and unattended production-node operation. |
| 6 | SEC-03: Deterministic default verification and public-network exclusion are enforced. | VERIFIED | `scripts/check-v1.4-release-boundaries.ts` validates v1.4 roots/docs and asserts `scripts/verify.sh` excludes `run-live-mainnet-smoke` and `--restart-after-progress`; `scripts/verify.sh` runs both v1.3 and v1.4 boundary checkers and contains no live-smoke/manual-peer/restart public-network command. Known final aggregate evidence records `bash scripts/verify.sh` passed after code-review follow-up. |

**Score:** 6/6 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-node/src/sync/types/summary.rs` | Sync summary, metrics, logs consistency | VERIFIED | Test and projection evidence present; latest peer error flows into status evidence. |
| `packages/open-bitcoin-cli/src/operator/status/render.rs` | Human status shared-field rendering | VERIFIED | Fixture asserts header/downloaded/connected, signal, peer failure, and latest error. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` + `model/tests.rs` | Dashboard projection consistency | VERIFIED | Plan literal pattern moved to child test module after file-length cleanup; `model.rs` wires `mod tests;` and production projection reads `OpenBitcoinStatusSnapshot`. |
| `packages/open-bitcoin-rpc/src/dispatch/tests.rs` | RPC connected-height regression | VERIFIED | `getblockchaininfo` test preserves connected `blocks` alias and latest warning. |
| `scripts/test-run-live-mainnet-smoke.sh` | Deterministic live-smoke fixture assertions | VERIFIED | Fixture suite passed and checks schema v2 nested progress/restart fields. |
| `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` | Allowlisted live-smoke support projection | VERIFIED | Compact summary extraction and redaction are present; raw manual-peer fallback keys absent from production projection. |
| `packages/open-bitcoin-cli/src/operator/support.rs` / `support/render.rs` / `tests/operator_binary.rs` | Support bundle integration, Markdown rendering, and redaction tests | VERIFIED | Delegation, labels, unavailable states, and forbidden raw-field assertions are present. |
| `docs/operator/runtime-guide.md` and architecture docs | Repo-local commands and interpretation | VERIFIED | Required Cargo/Bazel/UAT commands and shared snapshot/redaction wording are present. |
| `docs/parity/threat-model-v1.4.md`, parity roots, release docs | v1.4 threat and claim boundary | VERIFIED | Threat IDs, roots, matrix, deferred surfaces, and v1.3 historical links verified. |
| `scripts/check-v1.4-release-boundaries.ts` and `scripts/verify.sh` | Deterministic SEC-03 enforcement | VERIFIED | Boundary checker passed; verify script wiring and public-network exclusion verified. |
| `docs/metrics/lines-of-code.md` | Fresh tracked LOC report | VERIFIED | Plan checker expected literal `Generated`, but the artifact is substantive and fresh: regenerated comparison had identical input fingerprint and total line count, differing only in embedded `--output` path. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| Sync summary | Status/dashboard/RPC/live-smoke surfaces | Shared status and deterministic fixtures | VERIFIED | Header/downloaded/connected/progress/error fields are asserted across Rust renderers, RPC tests, and live-smoke fixture script. |
| Support command | `support/live_smoke.rs` | `live_smoke::summary(&value)` | VERIFIED | Support bundle reads selected report and stores only compact summary output. |
| Support tests | Support projection/rendering | Forbidden raw-field fixture | VERIFIED | Tests assert raw report bodies and secret-like markers are absent from both JSON and Markdown. |
| Runtime guide | Live-smoke/support/status commands | Exact repo-local command strings | VERIFIED | Cargo and Bazel command forms are present with field-level pass/fail wording. |
| Parity index/checklist/README | `threat-model-v1.4.md` and release readiness | v1.4 surface and audit roots | VERIFIED | Machine and human roots include all six Phase 59 requirement IDs and evidence paths. |
| `scripts/verify.sh` | `scripts/check-v1.4-release-boundaries.ts` | Bun command | VERIFIED | Verify script invokes v1.4 checker and excludes public-network live-smoke invocations. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| Sync summary/status/RPC surfaces | Header, downloaded block, connected block, peer state, progress signal, latest error | `SyncRunSummary`, `DurableSyncState`, `OpenBitcoinStatusSnapshot` fixtures | Yes - tests assert concrete non-empty values and latest peer error | VERIFIED |
| Dashboard projection | `OpenBitcoinStatusSnapshot.sync` and `peers` fields | `DashboardModel::from_snapshot` / `dashboard_sections` | Yes - child module fixture asserts rendered rows from snapshot values | VERIFIED |
| Support bundle live-smoke summary | `live_smoke.summary` | Operator-provided live-smoke JSON report path | Yes - allowlisted projection keeps required v1.4 fields and redacts raw input | VERIFIED |
| Docs/parity release roots | Requirement/evidence IDs | `docs/parity/index.json`, checklist, release docs | Yes - parsed JSON root contains done v1.4 surface with all requirements and evidence | VERIFIED |
| Default verification boundary | Verify script commands | `scripts/verify.sh` plus v1.4 checker | Yes - deterministic checker is wired and asserts public-network exclusions | VERIFIED |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Lifecycle provenance | `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 59 --require-plans --raw` | `valid` | PASS |
| v1.4 release-boundary checker | `bun run scripts/check-v1.4-release-boundaries.ts` | `validated v1.4 release boundary parity roots` | PASS |
| Live-smoke deterministic fixtures | `bash scripts/test-run-live-mainnet-smoke.sh` | Exit 0 | PASS |
| Parity breadcrumbs | `bun run scripts/check-parity-breadcrumbs.ts --check` | `Parity breadcrumbs verified for 222 Rust file(s).` | PASS |
| File length | `bash scripts/check-file-lengths.sh` | Production Rust file-length check passed | PASS |
| Parity JSON | `jq empty docs/parity/index.json` | Exit 0 | PASS |
| Diff hygiene | `git diff --check` | Exit 0 | PASS |
| Aggregate repo verification | `bash scripts/verify.sh` | Known final aggregate evidence: passed after code-review follow-up | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| OBS-01 | `59-01-PLAN.md` | Operator evidence fields agree across status, dashboard, metrics, logs, RPC, and live-smoke snapshots | SATISFIED | Rust renderer/RPC/summary tests and live-smoke fixture checks verify shared field values. |
| OBS-02 | `59-02-PLAN.md` | Redacted v1.4 support evidence summarizes diagnostics without raw sensitive data | SATISFIED | Allowlisted support projection, Markdown rendering, unavailable-state tests, and forbidden raw-field tests verified. |
| OBS-03 | `59-03-PLAN.md` | Operator docs include repo-local commands and pass/fail interpretation | SATISFIED | Runtime guide and architecture docs contain required commands, exact field names, local-only artifacts, and metadata-only credential reporting. |
| SEC-01 | `59-04-PLAN.md` | v1.4 threat model covers scoped public-peer and evidence risks | SATISFIED | `threat-model-v1.4.md` contains STRIDE IDs, ASVS mapping, evidence acceptance, release matrix, and residual risks. |
| SEC-02 | `59-04-PLAN.md` | Parity/release docs distinguish current v1.4 claim from deferred surfaces | SATISFIED | Parity root, checklist, README, release-readiness, deviations, and P2P catalog contain current v1.4 and historical v1.3 boundaries. |
| SEC-03 | `59-05-PLAN.md` | Default verification remains deterministic; public-network checks stay opt-in | SATISFIED | v1.4 checker passed; verify script wiring and exclusion checks verified; aggregate verify passed in known final evidence. |

No orphaned Phase 59 requirements were found: all six Phase 59 requirement IDs appear in plan frontmatter and in `.planning/REQUIREMENTS.md` traceability.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---:|---|---|---|
| `docs/parity/deviations-and-unknowns.md` | 139 | Folded todo audit wording | Info | Historical audit language, not an implementation stub. |
| `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` | 295 | `rpcpassword` string | Info | Redaction trigger only; production support output tests assert the raw key is not leaked. |
| `scripts/check-v1.4-release-boundaries.ts` | 238 | `console.log` | Info | Expected success message for a checker script. |
| `scripts/test-run-live-mainnet-smoke.sh` | 973 | `console.log` in Bun eval | Info | Fixture extraction helper, not a stub handler. |

No blocker anti-patterns, TODO/FIXME placeholders, hollow props, or raw support-evidence leakage were found in Phase 59 production surfaces.

### Human Verification Required

None. This phase is documentation, deterministic fixtures, support projection, and release-boundary enforcement. Public-network live smoke remains opt-in UAT by design and is not required for Phase 59 verification.

### Gaps Summary

No gaps found. Two automatic checker misses were manually classified as stale plan-literal patterns rather than failures: dashboard assertions moved into `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` during the file-length fix, and the LOC report lacks the literal word `Generated` while still carrying a fresh deterministic fingerprint.

### Residual Risks

- This verifier did not re-run the full `bash scripts/verify.sh`; it used the known final aggregate evidence that the command passed after code-review follow-up and ran targeted deterministic spot-checks.
- Live public-network behavior is intentionally outside default verification and remains opt-in UAT evidence, matching SEC-03.
- `.planning/REQUIREMENTS.md` still marks Phase 59 requirement traceability rows as `Pending`, but the implementation evidence and phase plans verify the requirements as satisfied.

---

_Verified: 2026-06-05T19:55:39Z_
_Verifier: the agent (gsd-verifier)_
