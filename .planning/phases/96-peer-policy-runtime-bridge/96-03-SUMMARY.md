---
phase: 96-peer-policy-runtime-bridge
plan: 03
subsystem: operator-evidence
tags: [rust, status, logging, support, cli, redaction]
requires:
  - phase: 96-peer-policy-runtime-bridge
    provides: Plan 02 managed runtime peer-policy projection and scoped reconnect suppression.
provides:
  - Shared inbound status projection tests for runtime peer-policy bridge outcomes.
  - Sanitized structured log records for inbound peer-policy events.
  - CLI/support rendering and redaction tests for Phase 96 peer-policy bridge evidence.
affects: [operator-status, support-bundle, structured-logs, parity-breadcrumbs]
tech-stack:
  added: []
  patterns: [shared-status-evidence, bounded-structured-logs, support-redaction-boundary]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/peer_policy.rs
  modified:
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
    - packages/open-bitcoin-node/src/network/inbound.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
key-decisions:
  - "Peer-policy structured logs use a dedicated inbound_peer_policy source and redacted_peer_policy_field literal."
  - "Runtime bridge source labels project as source_peer_policy_runtime_bridge when the managed runtime records Phase 96-originated decisions."
  - "Support Markdown uses the Phase 96 scoped runtime bridge next-action wording for peer-policy evidence."
patterns-established:
  - "RPC context structured-log appenders can expose public event-at methods while test-only helpers derive latest events from current status."
  - "Support redaction preserves safe low-cardinality policy labels while removing raw peer-policy material."
requirements-completed: [EVICT-03, EVICT-04, DOS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 96-2026-06-28T02-38-04
generated_at: 2026-06-28T04:07:45Z
duration: 11min
completed: 2026-06-28
---

# Phase 96 Plan 03: Operator Evidence Summary

**Runtime peer-policy bridge evidence now flows through shared status, sanitized structured logs, CLI status, and support redaction.**

## Performance

- **Duration:** 11min
- **Started:** 2026-06-28T03:56:30Z
- **Completed:** 2026-06-28T04:07:45Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- Added shared status projection coverage for runtime ban, unban, and protected misbehavior bridge outcomes.
- Added `inbound_peer_policy_log_record` and RPC context append support for sanitized peer-policy structured logs.
- Added runtime-decision status-and-log coverage through `ManagedRpcContext`.
- Updated support Markdown to use the Phase 96 scoped runtime bridge next-action sentence.
- Added CLI/support tests proving safe labels remain visible while raw peer-policy material is redacted.

## Task Commits

Deferred until the wrapper-level clean verification gate. The user-invoked wrapper requires no commit or push before final verification is clean.

## Files Created/Modified

- `packages/open-bitcoin-node/src/logging.rs` - Adds peer-policy structured log source, record builder, and redaction.
- `packages/open-bitcoin-node/src/logging/tests.rs` - Adds allowlist and redaction tests for peer-policy log records.
- `packages/open-bitcoin-node/src/network/inbound.rs` - Maps Phase 96 runtime bridge decisions to `source_peer_policy_runtime_bridge`.
- `packages/open-bitcoin-rpc/src/context.rs` - Wires the new peer-policy context module.
- `packages/open-bitcoin-rpc/src/context/peer_policy.rs` - Adds peer-policy structured log append support with parity breadcrumbs.
- `packages/open-bitcoin-rpc/src/context/tests.rs` - Adds status projection and structured-log append tests.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Adds Phase 96 safe-label status rendering coverage.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - Adds Phase 96 scoped runtime bridge next-action text.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Adds support Markdown/redaction coverage for Phase 96.

## Decisions Made

- Keep the shared status schema unchanged because existing peer-policy fields already represent the required bridge evidence.
- Make `record_inbound_peer_policy_event_at` a real `ManagedRpcContext` method, not a dead test-only helper.
- Keep raw marker strings only in tests and assert decoded/rendered output does not contain them.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Add runtime bridge source label mapping**
- **Found during:** Task 2 (structured logs for peer-policy bridge events)
- **Issue:** Existing managed ban sources collapsed unknown runtime bridge sources to `source_ban_policy`.
- **Fix:** Added `peer_policy_runtime_bridge -> source_peer_policy_runtime_bridge` mapping.
- **Files modified:** `packages/open-bitcoin-node/src/network/inbound.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc record_inbound_peer_policy_runtime_decision_projects_status_and_log -- --nocapture`
- **Committed in:** Deferred until final wrapper gate.

**Total deviations:** 1 auto-fixed (Rule 2).
**Impact on plan:** The change preserves low-cardinality evidence while making Phase 96 runtime bridge provenance explicit.

## Issues Encountered

The first status projection test expected the latest event to remain `protected_no_action`, but the existing projection order applies unban decisions after misbehavior decisions. The test now asserts the required counters and expects the latest `unbanned` event, matching the implemented projection contract.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc current_inbound_status_projects_runtime_peer_policy_bridge -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status::inbound --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound_peer_policy_log_record -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc record_inbound_peer_policy_event_appends_inbound_peer_policy_log_record -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc record_inbound_peer_policy_runtime_decision_projects_status_and_log -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::render --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support --no-fail-fast`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features -- -D warnings`
- `bun run scripts/check-parity-breadcrumbs.ts --check`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 96-04. The deterministic checker and docs can now guard the completed shared evidence path.

---
*Phase: 96-peer-policy-runtime-bridge*
*Completed: 2026-06-28*
