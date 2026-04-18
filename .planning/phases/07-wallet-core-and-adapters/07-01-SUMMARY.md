---
phase: 07-wallet-core-and-adapters
plan: 01
subsystem: wallet-crate-and-descriptors
tags: [wallet, descriptors, addresses, pure-core]
provides:
  - pure-core open-bitcoin-wallet crate
  - WIF and address encoders
  - single-key descriptor parsing
affects: [wallet, core, workspace]
requirements_completed: [WAL-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 07-2026-04-18T00-33-56
generated_at: 2026-04-18T00:33:56Z
completed: 2026-04-17
---

# Phase 7 Plan 01 Summary

Established the pure-core wallet crate and its first descriptor or address
surface.

## Accomplishments

- Added `open-bitcoin-wallet` to Cargo, Bazel, `open-bitcoin-core`, and the
  pure-core crate allowlist.
- Implemented wallet error types, WIF parsing, Base58Check encoding, segwit
  address encoding, and the single-key descriptor parser for `pkh`,
  `sh(wpkh)`, `wpkh`, and `tr`.
- Anchored the first address fixtures to vendored Knots behavior for legacy,
  nested segwit, native segwit, and taproot output derivation.
