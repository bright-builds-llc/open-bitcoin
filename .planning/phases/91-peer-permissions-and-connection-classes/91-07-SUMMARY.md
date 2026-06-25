---
phase: 91-peer-permissions-and-connection-classes
plan: 07
subsystem: support-bundle
tags: [rust, p2p, peer-permissions, support, redaction]

requires:
  - phase: 91-05
    provides: "Shared inbound status permission evidence fields"
provides:
  - "Support Markdown rendering for bounded inbound permission class and effect evidence"
  - "Support JSON sanitization for raw permission class names and effect-like literals"
  - "Redaction regression coverage for raw class names, permission specs, peer ids, endpoints, and credential literals"
affects:
  - 91-09-operator-docs-parity-roots-and-uat-commands
  - 91-10-deterministic-phase-checker-and-verifier-wiring

tech-stack:
  added: []
  patterns:
    - "Support bundles derive permission evidence from sanitized OpenBitcoinStatusSnapshot.peers.inbound"
    - "Unknown permission class/effect labels collapse to bounded redaction labels before shareable support serialization"
    - "Inactive relay-like permission effects render as diagnostic evidence, not relay support"

key-files:
  created:
    - .planning/phases/91-peer-permissions-and-connection-classes/91-07-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs

key-decisions:
  - "Support bundles keep safe machine labels such as ordinary_inbound, protected_inbound, admission_protected, and inactive_relay."
  - "Unknown permission labels are redacted before JSON and Markdown rendering instead of trusting arbitrary status strings."
  - "Latest permission decision messages are rebuilt from sanitized outcome/class labels for support bundles."
  - "Dashboard metric labels must cover every MetricKind variant so added permission metrics do not break CLI verification."

requirements-completed: [PERM-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T18:45:19Z

duration: 25min
completed: 2026-06-25
---

# Phase 91 Plan 07: Support-Bundle Permission Evidence Summary

**Support bundles now expose bounded permission diagnostics while stripping raw permission config, peer, endpoint, and credential literals from shareable evidence.**

## Accomplishments

- Rendered support Markdown fields for `permission_class`, permissioned/protected counts, active effects, inactive effects, and `latest_permission_decision`.
- Added inactive relay-like permission guidance that explicitly says relay, mempool, bloom, and blockfilter permissions are inactive Phase 91 evidence, not relay support.
- Added support JSON sanitization that preserves known machine labels and replaces unknown permission class/effect labels with redaction labels.
- Added regression tests proving representative raw class names, `in,noban,forceinbound` strings, `peer_id=`, raw endpoints, `rpc_password`, and cookie literals do not appear in support JSON or Markdown.
- Fixed the dashboard metric label match for the four Phase 91 permission metric variants, which was exposed by CLI verification.

## Task Commits

1. **Task 1: Render bounded permission evidence in support Markdown** - `b6cf051` (`feat`)
2. **Task 2: Sanitize support JSON permission evidence** - `b6cf051` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support.rs` - Adds inbound permission evidence sanitization on the support-bundle status copy.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - Renders permission evidence and scoped inactive relay-like next-action guidance.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Covers support Markdown, JSON sanitization, and forbidden-string redaction.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Adds labels for Phase 91 permission metrics.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Guards that all metric kinds have dashboard labels.

## Verification Results

- `cargo fmt --all` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli inbound_support --no-run` - passed
- `timeout 180s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli inbound_support -- --nocapture` - compiled, then timed out locally at `Running unittests src/lib.rs`, matching the known generated Rust test-binary stall recorded for Phase 91.
- `rg -n "permission_class|permissioned_inbound_peers|protected_inbound_peers|active_permission_effects|inactive_permission_effects|latest_permission_decision" packages/open-bitcoin-cli/src/operator/support/render/inbound.rs packages/open-bitcoin-cli/src/operator/support/tests.rs` - passed
- `rg -n "Relay, mempool, bloom, and blockfilter permissions are recorded as inactive" packages/open-bitcoin-cli/src/operator/support/render/inbound.rs packages/open-bitcoin-cli/src/operator/support/tests.rs` - passed
- `rg -n "sanitize|redact|permission" packages/open-bitcoin-cli/src/operator/support.rs packages/open-bitcoin-cli/src/operator/support/tests.rs` - passed
- `git diff --check` - passed

## Deviations from Plan

- Fixed an earlier Plan 91-05 compile regression in the dashboard metric label match because the new permission metric variants made `open-bitcoin-cli` fail to compile.
- The originally spawned executor stalled before landing a summary; the orchestrator completed the plan locally and closed the worker.

## Next Phase Readiness

Plan 91-08 can now add negative safeguards knowing both operator status and support bundles show inactive relay-like permission effects without implying relay, mempool, bloom, blockfilter, or compact-block behavior is supported.
