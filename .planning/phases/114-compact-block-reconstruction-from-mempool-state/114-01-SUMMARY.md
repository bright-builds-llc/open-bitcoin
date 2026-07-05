---
phase: 114-compact-block-reconstruction-from-mempool-state
plan: 01
subsystem: consensus-codec-network
tags: [rust, bip152, siphash, compact-reconstruction]
requires: []
provides:
  - SipHash-2-4 helper for BIP152 witness-hash short IDs
  - ShortIdSelector and match-key helpers in codec
  - PartialCompactBlock volatile state model and outcome enums
affects: [phase-114-plan-02, phase-115-missing-transaction-fallback]
tech-stack:
  added: []
  patterns:
    - SipHash lives in consensus; selector derivation lives in codec
    - Six-byte short IDs use explicit match keys for hash maps
key-files:
  created:
    - packages/open-bitcoin-consensus/src/crypto/siphash.rs
    - packages/open-bitcoin-network/src/compact_reconstruction.rs
  modified:
    - packages/open-bitcoin-consensus/src/crypto.rs
    - packages/open-bitcoin-consensus/src/lib.rs
    - packages/open-bitcoin-codec/src/compact_block.rs
    - packages/open-bitcoin-codec/src/lib.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "SipHash-2-4 stays in open-bitcoin-consensus to preserve codec dependency direction."
  - "PartialCompactBlock is volatile and cleared on invalid init paths."
requirements-completed: [RCN-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 114-2026-07-05T20-44-12
generated_at: 2026-07-05T20:55:00Z
duration: 25m
completed: 2026-07-05
---

# Phase 114 Plan 01: Short-ID Helper and Reconstruction State Model Summary

Added Knots-aligned SipHash short-ID math, codec selector helpers, and the `PartialCompactBlock` outcome model.

## Accomplishments

- Implemented `siphash_uint256` with a Knots vector test.
- Added `ShortIdSelector`, `short_id_selector_from_header_and_nonce`, and `short_id_match_key`.
- Introduced `PartialCompactBlock` plus `CompactReconstructionOutcome` invalid/failed reason enums.
- Registered parity breadcrumbs for new first-party Rust files.
