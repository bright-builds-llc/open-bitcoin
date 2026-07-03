---
phase: 107-runtime-relay-activation-and-download-eligibility-integration
plan: 03
subsystem: operator-relay-evidence
tags:
  - relay
  - status
  - eligibility
  - support-redaction

requires:
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-01 pure relay download eligibility and typed scheduler suppressions
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-02 runtime relay activation propagation into managed PeerManager state
  - phase: 105-operator-rpc-metrics-logs-and-support-evidence
    provides: Shared sanitized RelayEvidenceStatus contract and support redaction path
provides:
  - Relay activation evidence serialized through Open Bitcoin-specific relay status
  - Aggregate download eligibility counters derived from managed peer eligibility decisions
  - Support redaction coverage for new relay evidence fields
  - sendrawtransaction response-shape regression coverage for no public propagation claims
affects:
  - operator status relay evidence
  - support bundle relay redaction
  - managed network status projection
  - RPC local submission evidence

tech-stack:
  added: []
  patterns:
    - Boolean and numeric aggregate relay evidence fields only
    - Managed eligibility counters derived from relay_serving_context_for_peer
    - Support redaction covers every RelayEvidenceField reason-bearing branch

key-files:
  created:
    - .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-03-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/status/relay_evidence.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/relay_fanout.rs
    - packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Expose relay activation as a boolean RelayActivationEvidence field, defaulting to implemented false."
  - "Expose download eligibility as fixed aggregate counters rather than per-peer reasons or identifiers."
  - "Project managed counters through relay_serving_context_for_peer so status uses the same eligibility model as relay serving."
  - "Keep sendrawtransaction success response limited to txid_hex, replaced_txids, and evicted_txids."

patterns-established:
  - "RelayEvidenceStatus::with_activation_and_counters preserves the existing counter constructor while adding Plan 107 evidence."
  - "Managed relay status derives activation from resolved relay_activation and eligibility from current managed peers."
  - "Support status redaction sanitizes activation and download_eligibility reason states before JSON or Markdown rendering."

requirements-completed:
  - ACT-01
  - ACT-02
  - REL-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 107-2026-07-03T02-54-20
generated_at: 2026-07-03T04:45:06Z

duration: 20m
completed: 2026-07-03
---

# Phase 107 Plan 03: Runtime Relay Activation and Download Eligibility Integration Summary

**Open Bitcoin relay status now exposes resolved activation and aggregate download eligibility without peer, permission, or transaction material.**

## Performance

- **Duration:** 20m
- **Started:** 2026-07-03T04:25:25Z
- **Completed:** 2026-07-03T04:45:06Z
- **Tasks:** 2
- **Files modified/created:** 11, including this summary and refreshed LOC metrics

## Accomplishments

- Added `RelayActivationEvidence` and `RelayDownloadEligibilityCounters` to the shared `RelayEvidenceStatus` contract with serde defaults for backward-compatible deserialization.
- Projected activation and aggregate peer eligibility from managed network state through `ManagedPeerNetwork::relay_evidence_status()`.
- Extended node, operator status, support, managed-network, and RPC tests for fixed fields, redaction, and local submission no-propagation response shape.
- Refreshed `docs/metrics/lines-of-code.md` after Rust source and test changes.

## Task Commits

No commits were created. The execution context explicitly instructed this executor not to commit or push; the parent workflow owns any later commit.

1. **Task 1: Extend shared relay status with activation and eligibility aggregates** - complete, not committed here.
2. **Task 2: Project managed eligibility and preserve local submission evidence** - complete, not committed here.

## Files Created/Modified

- `packages/open-bitcoin-node/src/status/relay_evidence.rs` - Adds activation and download eligibility evidence fields plus the combined status constructor.
- `packages/open-bitcoin-node/src/status/tests.rs` - Covers default and populated fixed-field serialization and sensitive-material absence.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Adds managed peer eligibility counter projection from existing relay-serving decisions.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - Threads activation and eligibility counters into `RelayEvidenceStatus`.
- `packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs` - Covers default-off disabled counts, enabled eligible counts, and protected-only ineligible counts.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Covers operator JSON evidence for activation and eligibility counters.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - Extends relay support redaction to the new reason-bearing fields.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Covers support JSON evidence and redaction for activation and eligibility fields.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Confirms `sendrawtransaction` keeps its bounded success response and exposes relay evidence only through status.
- `docs/metrics/lines-of-code.md` - Regenerated from the current worktree.

## Decisions Made

- Eligibility counters are computed from `relay_serving_context_for_peer` rather than a second peer-class model.
- `Disabled` and `ActivationRequired` both count as `relay_disabled_count`, while permission-required and inactive permission-effect outcomes count as `permission_required_count`.
- The existing `with_counters` constructor remains default-off and zero-eligibility for compatibility; new call sites use `with_activation_and_counters`.
- Baseline-compatible `sendrawtransaction` response shape stays unchanged; activation and eligibility evidence remain in Open Bitcoin-specific status.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Extended support redaction for new reason-bearing relay fields**
- **Found during:** Task 1 (Extend shared relay status with activation and eligibility aggregates)
- **Issue:** The plan added two new `RelayEvidenceField` values. Non-implemented states can carry reason strings, so leaving support redaction unchanged could leak sensitive relay material through support JSON or Markdown.
- **Fix:** Added `activation` and `download_eligibility` to `redact_relay_mempool_evidence`, and added support tests proving sensitive reason strings are redacted.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support/redaction.rs`, `packages/open-bitcoin-cli/src/operator/support/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support -- --nocapture`
- **Committed in:** Not committed by this executor per execution context.

**Total deviations:** 1 auto-fixed missing-critical issue.
**Impact on plan:** The adjacent edit was required by the plan threat model and kept the behavior inside the existing support redaction boundary.

## Issues Encountered

- The Task 1 RED run failed as intended because `RelayActivationEvidence`, `RelayDownloadEligibilityCounters`, and `with_activation_and_counters` did not exist yet.
- The existing sensitive-material status test used a broad `permission` substring, which conflicted with the planned fixed field `permission_required_count`; it now forbids `permission_string` while allowing the aggregate counter.
- The Task 2 RED run failed as intended because `relay_evidence_status()` still reported default activation and zero eligibility counters before projection wiring.
- The protected-only inbound counter test needed reserved inbound capacity in the fixture, matching existing managed admission behavior.

## Known Stubs

None. A targeted scan of Plan 03 modified files found no TODO, FIXME, placeholder, coming-soon, not-available text, or hardcoded empty UI/data stubs.

## Threat Flags

None. This plan changes an existing serialized status boundary and support redaction path only. It adds no network endpoint, auth path, filesystem trust boundary, schema change, service-bit change, compact block behavior, package relay, bloom/filter serving, or public relay default.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-rpc -p open-bitcoin-cli --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib relay_evidence_status -- --nocapture` - passed, 7 tests
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_fanout_cases -- --nocapture` - passed, 5 tests
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_local_submission_cases -- --nocapture` - passed, 3 tests
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc sendrawtransaction -- --nocapture` - passed, 4 matching library tests plus empty filtered targets
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator_status -- --nocapture` - passed, 2 matching library tests plus empty filtered targets
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support -- --nocapture` - passed, 55 matching library tests plus 7 matching operator binary tests
- `bash -c '! rg -n "propagated|broadcast|production_ready" packages/open-bitcoin-rpc/src/method packages/open-bitcoin-rpc/src/dispatch/node.rs'` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed, 339 Rust files verified
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` - failed before refresh, then passed after regeneration
- `git diff --check` - passed

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 107-04 can use Open Bitcoin-specific relay status to prove default-off activation, explicit activation, aggregate eligible/ineligible peers, and local submission evidence without expanding baseline RPC response shapes or leaking raw relay material.

## Self-Check: PASSED

- Created summary file: `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-03-SUMMARY.md`
- Verified `RelayActivationEvidence`, `RelayDownloadEligibilityCounters`, and `with_activation_and_counters` exist in `packages/open-bitcoin-node/src/status/relay_evidence.rs`.
- Verified `ManagedPeerNetwork::relay_evidence_status()` threads activation and managed download eligibility counters into shared status.
- Verified `sendrawtransaction` tests assert the bounded response shape and Open Bitcoin-specific relay evidence path.
- Verified `git diff --check` passed after summary creation.
- No commits were created, matching the execution context.

*Phase: 107-runtime-relay-activation-and-download-eligibility-integration*
*Completed: 2026-07-03*
