---
phase: 55-outbound-handshake-compatibility-fixes
plan: 01
subsystem: p2p-sync
tags:
  - rust
  - p2p
  - sync
  - compatibility
requires:
  - phase: 54-peer-compatibility-baseline-and-diagnostic-harness
    provides: deterministic compatibility transcript diagnosis
provides:
  - daemon sync accepts completed outbound handshakes as connected peers
  - duplicate-version peer disconnects become typed compatibility failures
  - deterministic manual, DNS, wrong-network, malformed-data, and replacement peer tests
affects:
  - header-ibd-convergence
  - operator-evidence-threat-model-and-release-boundaries
tech-stack:
  added: []
  patterns:
    - sync shell reads pure peer handshake state before classifying idle peers
key-files:
  created:
    - .planning/phases/55-outbound-handshake-compatibility-fixes/55-CONTEXT.md
    - .planning/phases/55-outbound-handshake-compatibility-fixes/55-DISCUSSION-LOG.md
    - .planning/phases/55-outbound-handshake-compatibility-fixes/55-RESEARCH.md
    - .planning/phases/55-outbound-handshake-compatibility-fixes/55-01-PLAN.md
    - .planning/phases/55-outbound-handshake-compatibility-fixes/55-REVIEW.md
    - .planning/phases/55-outbound-handshake-compatibility-fixes/55-01-SUMMARY.md
  modified:
    - docs/parity/catalog/p2p.md
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs
key-decisions:
  - "Completed outbound version/verack handshakes count as connected peers even if the peer idles before headers or blocks."
  - "Peer-manager disconnect decisions are surfaced to durable sync as typed failures after peer removal."
  - "Duplicate-version compatibility failures receive no accepted header or block progress credit."
patterns-established:
  - "Handshake completion is checked from pure peer state at the sync shell boundary."
  - "Compatibility failures can be represented separately from generic network failures in sync telemetry."
requirements-completed:
  - COMPAT-03
  - COMPAT-05
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 55-2026-06-02T22-36-24
generated_at: 2026-06-03T01:42:37.341Z
duration: 3h 06m
completed: 2026-06-03
---

# Phase 55 Plan 01 Summary

**Daemon sync now accepts completed outbound handshakes while rejecting incompatible peers as typed, uncredited outcomes.**

## Performance

- **Duration:** 3h 06m
- **Started:** 2026-06-02T22:36:24Z
- **Completed:** 2026-06-03T01:42:37Z
- **Tasks:** 4
- **Files modified:** 5 source/doc files plus GSD artifacts

## Accomplishments

- Changed daemon sync so peers that complete the outbound `version`/`verack`
  handshake are recorded as `Connected` when they later idle.
- Preserved pre-handshake idle behavior as `Stalled` with the existing warning
  and retry-backoff path.
- Propagated peer-manager disconnect decisions through the managed network shell
  so duplicate-version peers become typed compatibility failures.
- Added deterministic tests for manual-peer handshake completion, DNS-peer
  handshake completion, duplicate-version replacement, wrong-network failure,
  malformed data, and coherent durable state.
- Refreshed the P2P parity catalog to claim the new daemon-integrated handshake
  behavior while keeping unattended public-mainnet full sync deferred.

## Task Commits

Final strict wrapper commit will capture the phase as a single verified outcome.

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` - surfaces disconnect actions as
  typed network errors after removing the peer.
- `packages/open-bitcoin-node/src/sync.rs` - classifies post-handshake idle peers
  as connected while preserving pre-handshake stalls.
- `packages/open-bitcoin-node/src/sync/types.rs` - adds sync compatibility
  failure telemetry and recovery guidance.
- `packages/open-bitcoin-node/src/sync/tests.rs` - adds deterministic Phase 55
  regression coverage and updates the header-sync post-handshake idle
  expectation.
- `docs/parity/catalog/p2p.md` - documents daemon-integrated handshake and
  incompatible-peer behavior.

## Decisions Made

- Use existing `PeerState` booleans as the handshake source of truth rather than
  introducing a parallel runtime state machine.
- Keep useful progress tied to accepted headers and blocks, not handshake
  activity.
- Keep public-network checks opt-in and outside default verification.

## Deviations from Plan

None - plan executed as written. The implementation used the optional
`PeerFailureReason::Compatibility` and `SyncRuntimeError::PeerCompatibility`
variants anticipated by the plan.

**Total deviations:** 0.
**Impact on plan:** No scope change.

## Issues Encountered

- The original `scripted_headers_sync_persists_progress_and_status` test
  expected post-handshake idle to be `Stalled`; this was updated to the intended
  Phase 55 `Connected` outcome while keeping a separate pre-handshake stall
  regression test.
- A duplicate-version test initially expected zeroed resource-pressure fields,
  but durable sync metadata correctly preserves configured runtime limits; the
  assertion was updated to prove coherent durable state.
- Full repo verification initially stopped on a stale tracked LOC report; the
  report was regenerated and the full `bash scripts/verify.sh` contract passed.

## User Setup Required

None - no external service configuration required.

## Verification

Passed:

```bash
cargo fmt --all --manifest-path packages/Cargo.toml
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync --all-features
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features
bun run scripts/check-parity-breadcrumbs.ts --check
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bash scripts/verify.sh
```

`cargo test --manifest-path packages/Cargo.toml --workspace --all-features`
passed with one ignored opt-in live-network smoke test.
`bash scripts/verify.sh` passed after regenerating the tracked LOC report with
`bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`.

## Next Phase Readiness

Phase 56 can build on a daemon sync runtime that now distinguishes compatible
completed handshakes from incompatible or stalled peers without public-network
verification gates.

## Self-Check: PASSED

---

*Phase: 55-outbound-handshake-compatibility-fixes*
*Completed: 2026-06-03*
