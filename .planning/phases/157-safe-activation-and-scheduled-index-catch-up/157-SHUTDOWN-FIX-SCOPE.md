---
phase: 157-safe-activation-and-scheduled-index-catch-up
generated_by: gsd-code-review-fix
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:35:00Z
status: verified
finding: WR-01
---

# Review correction: settle shutdown before reporting index failure

The independently reviewed daemon path returns a retained BASIC error after the coins owner has completed Always. The outer `?` then bypasses final mempool settlement and retry join. Preserve the failure and saved Active protection while completing shutdown effects; a failed shutdown must not publish a clean marker.

## Checkable correction plan

- [x] Capture a behavioral RED through the actual outer shutdown coordinator: retained index error with successful Always still requires mempool settlement and retry join, returns failure, and withholds the clean marker.
- [x] Separate final mempool settlement/join from clean-marker publication and make the actual outer shutdown path attempt all required workers before propagating failures. Preserve error visibility and existing checkpoint/prune cadence; do not special-case index success or add a new service.
- [x] Retarget the historical Phase 135 shutdown/source assertion to the actual production ordering, preserving positive and independent negative controls. Coordinate the Phase 157 guard's corresponding regression with Plan 10.
- [x] Run affected behavioral and inherited daemon/checkpoint tests, Phase 135 live/mutation controls, strict RPC Clippy, scoped formatting and diff review.
- [x] Root independently re-reviews the corrected source before full native verification.

## Ownership and boundaries

The fixer may change `open-bitcoind.rs`, `open_bitcoind/coins_flush.rs`, `open_bitcoind/checkpoint.rs`, `open_bitcoind/tests/checkpoint.rs`, `open_bitcoind/tests/filter_index.rs`, `open_bitcoind/coins_flush/tests/filter_index.rs`, and the existing `scripts/check-phase135-snapshot-recovery.ts`/`.test.ts`. Existing Phase 157 guard/docs/catalog paths remain Plan 10-owned. Report any extra path or architectural scope before changing it. Prefer existing files; any new first-party Rust path requires immediate breadcrumbs, explicit scope amendment and final registry coverage.

All task/TDD/metadata commits and ordinary content staging remain deferred to root's clean whole-phase gate. This correction is part of the authorized GSD review-fix workflow, not a new milestone phase. Source fingerprints and review artifacts must be refreshed after the fix. No shared STATE/ROADMAP/REQUIREMENTS/config mutation is delegated.
