---
phase: 91-peer-permissions-and-connection-classes
plan: 06
subsystem: cli-status
tags: [rust, p2p, peer-permissions, status, operator]

requires:
  - phase: 91-05
    provides: "Shared inbound status permission evidence fields"
provides:
  - "Human status rendering for inbound permission class and effect labels"
  - "JSON status projection tests for shared permission evidence"
  - "Status non-leak coverage for raw class names, raw permission strings, peer ids, and credentials"
affects:
  - 91-07-support-bundle-permission-evidence-and-redaction
  - 91-09-operator-docs-parity-roots-and-uat-commands

tech-stack:
  added: []
  patterns:
    - "Render permission evidence from OpenBitcoinStatusSnapshot.peers.inbound only"
    - "Use none for empty active/inactive permission effect lists"
    - "Keep peer count rendering separate from inbound permission diagnostics"

key-files:
  created:
    - .planning/phases/91-peer-permissions-and-connection-classes/91-06-SUMMARY.md
    - .planning/phases/91-peer-permissions-and-connection-classes/deferred-items.md
  modified:
    - packages/open-bitcoin-cli/src/operator/status/render/inbound.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs

key-decisions:
  - "Render shared machine labels only: permission_class, active_permission_effects, inactive_permission_effects, and latest_permission_decision."
  - "Keep raw config names, raw permission specs, peer ids, and credential strings out of operator status output."
  - "Preserve the existing Peers: in/out line as peer counts, not permission diagnostics."

requirements-completed: [PERM-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T18:14:00Z

duration: stalled verification
completed: 2026-06-25
---

# Phase 91 Plan 06: Operator Status Permission Rendering Summary

**Operator status now renders Phase 91 permission evidence from the shared status contract without exposing raw config or credential data.**

## Accomplishments

- Added human status text for `permission_class`, `permissioned_inbound_peers`, `protected_inbound_peers`, `active_permission_effects`, `inactive_permission_effects`, and `latest_permission_decision`.
- Added renderer coverage for comma-separated effect labels and `none` for empty effect lists.
- Added status collection assertions proving live RPC permission evidence maps through `OpenBitcoinStatusSnapshot.peers.inbound`.
- Extended non-leak coverage for representative raw class names, permission strings, peer/config wording, RPC password labels, and cookie strings.

## Task Commits

1. **Task 1: Render permission fields in human status** - `144a61f` (`feat`)
2. **Task 2: Preserve JSON status projection and redaction tests** - `e1f0438` (`test`)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - Renders permission evidence from shared inbound status fields.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Covers rendered permission labels and empty-list fallback text.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Covers JSON status projection and forbidden-string redaction.
- `.planning/phases/91-peer-permissions-and-connection-classes/deferred-items.md` - Records the local CLI Cargo verification stall.

## Verification Results

- `rg -n "permission_class|permissioned_inbound_peers|protected_inbound_peers|active_permission_effects|inactive_permission_effects|latest_permission_decision" packages/open-bitcoin-cli/src/operator/status/render/inbound.rs packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - passed
- `! rg -n "raw_config|class_name|peer_id|rpc_password|cookie" packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - passed
- `rg -n "active_permission_effects|inactive_permission_effects|latest_permission_decision|operator_loopback|rpc_password|cookie" packages/open-bitcoin-cli/src/operator/status/tests.rs` - passed
- `rg -n "OpenBitcoinStatusSnapshot" packages/open-bitcoin-cli/src/operator/status/tests.rs` - passed
- Bounded Cargo verification for `open-bitcoin-cli` stalled locally before useful diagnostics; details are recorded in `deferred-items.md`.

## Deviations from Plan

- The executor committed the code and tests but stalled before generating this summary. The orchestrator added the summary and progress metadata afterward.
- Focused Cargo verification for `open-bitcoin-cli` hit a broader local toolchain stall, not just the known test-binary launch hang. No Cargo or rustc processes were left running.

## Next Phase Readiness

Plan 91-07 can render the same shared permission evidence into support-bundle Markdown and JSON with endpoint redaction preserved.
