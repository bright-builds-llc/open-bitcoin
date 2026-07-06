---
phase: 116-operator-evidence-metrics-logs-and-support-boundary
subsystem: block-relay-operator-evidence
tags:
  - operator-evidence
  - rpc
  - cli
  - dashboard
  - metrics
  - logs
  - support
requirements-completed:
  - OBS-01
  - OBS-02
  - OBS-03
  - OBS-04
  - OBS-05
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 116-2026-07-06T03-46-36
generated_at: 2026-07-06T05:35:20Z
status: passed
---

# Phase 116 Summary

Phase 116 completed the operator evidence boundary for block serving and compact block relay by projecting one shared `block_relay` contract through RPC, CLI, dashboard, metrics, logs, support bundles, deterministic checker coverage, and final repository verification.

## OBS Mapping

- `OBS-01`: `BlockRelayEvidenceStatus`, managed network projection, and RPC `openbitcoinnetworkstatus.block_relay` now expose aggregate-only block-relay evidence.
- `OBS-02`: CLI status and dashboard surfaces render the shared `block_relay` contract with explicit unavailable states and no renderer-local heuristics.
- `OBS-03`: Fixed `_count` metrics plus stable `block_relay` structured logs read the shared status projection and avoid dynamic labels or sensitive identifiers.
- `OBS-04`: Support JSON and Markdown share one redacted block-relay projection that rejects raw compact payloads, hashes, endpoints, permission strings, credentials, and dynamic labels.
- `OBS-05`: Architecture docs, runtime guide UAT commands, and the Phase 116 Bun checker document and enforce the completed operator-evidence surface.

## Delivered Artifacts

- Shared status and runtime projection: `packages/open-bitcoin-node/src/status/block_relay_evidence.rs`, `packages/open-bitcoin-node/src/network/block_relay_evidence.rs`, and RPC wiring in `packages/open-bitcoin-rpc/src/`.
- CLI and dashboard rendering: `packages/open-bitcoin-cli/src/operator/status/render/block_relay.rs` and `packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs`.
- Metrics and logs: `packages/open-bitcoin-node/src/metrics.rs`, `packages/open-bitcoin-node/src/metrics/block_relay.rs`, and `packages/open-bitcoin-node/src/logging.rs`.
- Support, docs, and verification: `packages/open-bitcoin-cli/src/operator/support/render/block_relay.rs`, `scripts/check-phase116-operator-block-relay-evidence.ts`, `docs/operator/runtime-guide.md`, and `116-VERIFICATION.md`.

## Residual Risks And Deferred Scope

- Phase 116 intentionally stays within deterministic local verification and opt-in operator review; public-network review, broader parity closeout, and release-boundary packaging remain Phase 117 scope.
- The status and observability surfaces remain aggregate-only by design, so peer-level or block-level diagnostics continue to be intentionally unavailable on these operator-facing paths.

## Self-Check

- Complete: OBS-01 through OBS-05 are implemented and summarized with per-plan closeout files.
- Passed: `bash scripts/verify.sh` completed successfully for the final working tree.
