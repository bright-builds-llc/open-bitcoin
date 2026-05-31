---
phase: 51-live-smoke-fresh-status-integration
plan: 01
subsystem: live-smoke-fresh-status
tags:
  - live-mainnet-smoke
  - sync-status
  - parity
requires:
  - .planning/phases/51-live-smoke-fresh-status-integration/51-CONTEXT.md
  - .planning/phases/51-live-smoke-fresh-status-integration/51-RESEARCH.md
provides:
  - scripts/run-live-mainnet-smoke.ts
  - scripts/test-run-live-mainnet-smoke.sh
  - .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md
  - docs/parity/release-readiness.md
  - docs/parity/checklist.md
  - docs/parity/index.json
affects:
  - docs/operator/runtime-guide.md
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/STATE.md
  - docs/metrics/lines-of-code.md
tech-stack:
  added: []
  patterns:
    - opt-in live public-mainnet smoke remains outside deterministic verification
    - fresh daemon sync-control metadata feeds progress snapshots
    - generated live-smoke evidence remains local and untracked
key-files:
  created:
    - .planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md
    - .planning/phases/51-live-smoke-fresh-status-integration/51-VERIFICATION.md
  modified:
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh
    - docs/operator/runtime-guide.md
    - .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md
    - docs/parity/release-readiness.md
    - docs/parity/checklist.md
    - docs/parity/index.json
    - .planning/REQUIREMENTS.md
    - .planning/ROADMAP.md
    - .planning/STATE.md
    - docs/metrics/lines-of-code.md
key-decisions:
  - Live-smoke per-poll status now calls `openbitcoinsyncstatus`.
  - The report parser accepts both RPC `{ metadata: ... }` responses and raw runtime metadata.
  - Phase 50 evidence was amended instead of committing generated live reports.
requirements-completed:
  - PROOF-03
  - PROOF-04
  - PROOF-05
  - OBS-02
  - SEC-03
duration: 30 min
completed: 2026-05-31
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 51-2026-05-31T21-21-57
generated_at: 2026-05-31T21:54:57Z
---

# Phase 51 Plan 01: Live Smoke Fresh Status Integration Summary

Closed the v1.3 G-01 audit gap by changing live-smoke progress polling from the
stored `getblockchaininfo` projection to fresh daemon sync-control status.

## Execution

- Started: 2026-05-31T21:25:05Z
- Finished: 2026-05-31T21:54:57Z
- Tasks completed: 3/3
- Files changed: 12 tracked files

## Implementation

- Changed `scripts/run-live-mainnet-smoke.ts` so the status command records and
  calls `openbitcoinsyncstatus`.
- Added a runtime metadata parser that accepts both RPC sync-control responses
  (`{ metadata: ... }`) and raw operator status metadata.
- Built `SyncStatusSnapshot` from fresh runtime metadata fields: lifecycle,
  phase, header height, connected block height, latest error, outbound peers,
  paused state, and updated timestamp.
- Kept final status on `open-bitcoin sync status --format json`, sharing the
  same metadata normalization path for final report fields.
- Updated `scripts/test-run-live-mainnet-smoke.sh` so mocked progress and
  no-progress paths assert `openbitcoinsyncstatus`, runtime lifecycle/phase,
  outbound peer counts, paused state, and updated timestamps.

## Evidence Updates

- Added a Phase 51 fresh-status amendment to the Phase 50 UAT artifact, naming
  the historical stale snapshot mismatch (`phase=rpc_getblockchaininfo`,
  `lifecycle=synced`) and recording the Phase 51 fix.
- Updated the operator runtime guide so the live smoke workflow describes fresh
  daemon sync-control polling.
- Updated v1.3 parity roots to point the evidence closeout at the Phase 51
  fresh-status fix and this summary.
- Marked PROOF-03, PROOF-04, PROOF-05, OBS-02, and SEC-03 complete.

## Verification Run

```bash
bash scripts/test-run-live-mainnet-smoke.sh
bun run scripts/check-v1.3-release-boundaries.ts
cargo fmt --manifest-path packages/Cargo.toml --all
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bash scripts/verify.sh
```

`bash scripts/verify.sh` completed successfully in 3m 21.707s and covered the
repo-native deterministic checks, Rust tests, benchmark smoke validation, Bazel
smoke build, and coverage gate.

## Deviations from Plan

None.

## Residual Risk

This phase proves that future opt-in live-smoke reports use fresh daemon status.
It does not claim new public-mainnet progress, inbound serving, relay,
packaging, GUI, hosted dashboard, or production-funds wallet readiness.

## Self-Check: PASSED

- Live-smoke status polling uses `openbitcoinsyncstatus`.
- Offline regression proves progress and no-progress reports use fresh runtime
  metadata.
- Phase 50 UAT and parity roots explain how the stale snapshot gap is closed.
- Requirements traceability for PROOF-03, PROOF-04, PROOF-05, OBS-02, and
  SEC-03 is complete.
- Full repo verification passed.
