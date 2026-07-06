---
phase: 115-missing-transaction-round-trip-fallback-and-validation-handoff
plan: 01
subsystem: network-compact-download
tags: [rust, bip152, getblocktxn, scheduler]
requires:
  - phase: 114-compact-block-reconstruction-from-mempool-state
    provides: PartialCompactBlock Ready missing_indexes outcomes
provides:
  - CompactDownloadPeerState and in-flight matching
  - schedule_missing_transaction_request eligibility gates
  - absolute_indexes_to_differential_deltas and build_get_block_transactions_request
affects: [phase-115-plan-02, phase-115-plan-03]
key-files:
  created:
    - packages/open-bitcoin-network/src/compact_download.rs
    - packages/open-bitcoin-network/src/compact_download/tests.rs
  modified:
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Pure compact_download policy stays beside compact_reconstruction with typed outcomes."
  - "Scheduler converts Phase 114 absolute missing indexes to BIP152 differential deltas."
requirements-completed: [RCN-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 115-2026-07-06T00-26-47
generated_at: 2026-07-06T01:00:00Z
duration: 30m
completed: 2026-07-06
---

# Phase 115 Plan 01: Missing Transaction Request Scheduler Summary

Added pure compact-download in-flight state, BIP152 differential index request building, and gated getblocktxn scheduling.

## Accomplishments

- Introduced `CompactDownloadPeerState`, `CompactDownloadInFlight`, and suppression reason enums.
- Implemented `schedule_missing_transaction_request` with activation, capability, in-flight, missing-index, and duplicate gates.
- Added differential delta builder verified against codec index expansion.
- Registered parity breadcrumbs for new first-party Rust files.
