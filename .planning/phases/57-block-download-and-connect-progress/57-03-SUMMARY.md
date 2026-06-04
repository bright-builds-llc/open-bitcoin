---
phase: 57-block-download-and-connect-progress
plan: 03
subsystem: sync
tags: [rust, sync, block-download, status, live-smoke-evidence]

requires:
  - phase: 57-block-download-and-connect-progress
    plan: 02
    provides: typed block connect dispositions and peer-attributed block response outcomes
provides:
  - downloaded and connected block hash fields in durable sync progress
  - durable status projection tests for connected, downloaded-only, and unavailable hash states
  - compatibility coverage proving legacy block_height remains the connected-height alias
affects: [sync, operator-status, live-smoke, BLK-02, BLK-03]

tech-stack:
  added: []
  patterns:
    - durable sync summaries carry block progress identity from runtime state into status JSON
    - status hash fields stay additive while legacy height aliases remain stable

key-files:
  created:
    - .planning/phases/57-block-download-and-connect-progress/57-03-SUMMARY.md
  modified:
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs

key-decisions:
  - "Expose downloaded and connected block hashes as optional status fields so live-smoke evidence can quote exact block identity when available."
  - "Keep `block_height` as the connected-height compatibility alias instead of redefining it to downloaded height."
  - "Derive missing hash evidence as JSON null rather than manufacturing placeholder hashes."

patterns-established:
  - "Fresh durable sync status can now report downloaded and connected block height/hash pairs from a single summary projection."
  - "Operator-facing status additions remain backward-compatible by adding optional fields and preserving existing aliases."

requirements-completed: [BLK-02, BLK-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 57-2026-06-03T13-56-54
generated_at: 2026-06-04T09:50:16Z

duration: ~35m
completed: 2026-06-04
---

# Phase 57 Plan 03: Durable Block Progress Evidence Summary

**Durable sync status now reports downloaded and connected block height/hash evidence while preserving the legacy connected-height alias.**

## Performance

- **Duration:** ~35m
- **Tasks:** 2
- **Files created/modified:** 10

## Accomplishments

- Added optional `maybe_downloaded_block_hash` and `maybe_connected_block_hash` fields to durable sync progress projections.
- Derived downloaded block identity from contiguous best-chain block bodies and connected block identity from the active chain tip.
- Added deterministic sync progress tests for connected hash evidence, downloaded-only hash evidence, and unavailable hash serialization.
- Added compatibility coverage proving `block_height` remains equal to connected height while downloaded height and hashes are surfaced separately.
- Updated first-party status literals so CLI, RPC, and node tests compile against the additive status contract.

## Task Commits

1. **Task 1: Add block hashes to durable sync progress** - `418bccf`
2. **Task 2: Keep projections and existing operator status compatibility green** - `eea2037`

## Verification

Passed:

```bash
cargo fmt --all --manifest-path packages/Cargo.toml
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_progress --all-features
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status --all-features
bun run scripts/check-parity-breadcrumbs.ts --check
```

The Task 2 commit also completed the repo pre-commit hook successfully, including `bash scripts/verify.sh`.

## Deviations from Plan

- The executor process hit a usage limit after Task 1. The orchestrator completed Task 2 locally, reran the targeted checks, and committed the summary evidence.

## Issues Encountered

- No functional blockers remain for Plan 03.

## Next Phase Readiness

Plan 04 can consume fresh status snapshots containing downloaded and connected block heights plus optional hashes, so the live-mainnet smoke report can cite before/after durable progress without inferring it from peer activity.

## Self-Check: PASSED

- `FOUND:418bccf`
- `FOUND:eea2037`
- `FOUND:.planning/phases/57-block-download-and-connect-progress/57-03-SUMMARY.md`
- `PASS:cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_progress --all-features`
- `PASS:cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status --all-features`
- `PASS:bun run scripts/check-parity-breadcrumbs.ts --check`

---
*Phase: 57-block-download-and-connect-progress*
*Completed: 2026-06-04*
