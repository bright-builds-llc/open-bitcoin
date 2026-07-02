---
phase: 105-operator-rpc-metrics-logs-and-support-evidence
type: phase-summary
status: complete
requirements:
  - OBS-01
  - OBS-02
  - OBS-03
  - OBS-04
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 105-2026-07-01T20-32-29
generated_at: 2026-07-02T01:59:09Z
completed: 2026-07-02
---

# Phase 105 Summary: Operator, RPC, Metrics, Logs, And Support Evidence

Phase 105 projects local relay and mempool participation through one shared, sanitized evidence contract. Operator status, dashboard rows, the Open Bitcoin network-status RPC extension, metrics, structured logs, and support bundles now consume `RelayEvidenceStatus` instead of reconstructing relay state independently.

## Requirement Evidence

| Requirement | Status | Evidence |
| --- | --- | --- |
| OBS-01 | Complete | `RelayEvidenceStatus`, `RelayEvidenceCounters`, `RelayEvidenceField`, and `RelayEvidenceCapability` define the classified contract in `packages/open-bitcoin-node/src/status/relay_evidence.rs`; status and RPC tests prove JSON projection through `mempool.relay` and `openbitcoinnetworkstatus.relay`. |
| OBS-02 | Complete | Operator status and dashboard render `mempool.relay` through `packages/open-bitcoin-cli/src/operator/status/render/relay.rs` and `packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs`, with tests for fixed counters and classified capability rows. |
| OBS-03 | Complete | `packages/open-bitcoin-node/src/metrics.rs` maps each fixed relay counter to low-cardinality `MetricKind` values, and `packages/open-bitcoin-node/src/logging.rs` emits `relay_mempool` structured logs with aggregate counts only. |
| OBS-04 | Complete | `packages/open-bitcoin-cli/src/operator/support/redaction.rs` sanitizes relay reason strings before JSON/Markdown support output, and `packages/open-bitcoin-cli/src/operator/support/render/relay.rs` renders bounded local troubleshooting and parity-review guidance. |

## Parity And Documentation

- Registered `v2-0-operator-rpc-metrics-logs-support-evidence` in `docs/parity/index.json` and `docs/parity/checklist.md`.
- Updated P2P, mempool-policy, and RPC/CLI/config catalogs to classify Phase 105 as local operator evidence, not public relay readiness.
- Updated architecture and runtime docs with the field-state vocabulary: `implemented`, `unavailable`, `deferred`, and `intentionally_different`.
- Added repo-local Cargo and Bazel commands for operator status, RPC extension status, and support-bundle UAT.
- Updated README status to reflect bounded v2.0 relay and mempool evidence while preserving deferred public/default/production scope.

## Verification

- Focused Rust tests covered RPC projection, status/dashboard rendering, metrics/logging, and support sanitization.
- `scripts/check-phase105-operator-relay-evidence.ts` guards Phase 105 parity roots, source breadcrumbs, fixed counters, support sanitization fixtures, repo-local runtime commands, verifier order, and forbidden relay/production claims.
- `scripts/verify.sh` runs the Phase 105 checker immediately after Phase 104 and before pure-core checks.
- Final evidence is recorded in `105-VERIFICATION.md`.

## Residual Scope

- Public relay defaults, public-network relay UAT, compact block relay, package relay, bloom/filter serving, production-service proof, production full-node readiness proof, and production-funds wallet safety proof remain future Phase 106 or later scope.
- Phase 105 reports local status evidence; it does not guarantee public transaction propagation.

## Self-Check

- Complete: OBS-01, OBS-02, OBS-03, and OBS-04 are mapped to source, tests, docs, parity roots, checker coverage, and final verification.
- Passed: focused checker, fast verifier, cargo checks, repository verifier, state validation, and lifecycle validation all passed.

*Phase: 105-operator-rpc-metrics-logs-and-support-evidence*
*Completed: 2026-07-02*
