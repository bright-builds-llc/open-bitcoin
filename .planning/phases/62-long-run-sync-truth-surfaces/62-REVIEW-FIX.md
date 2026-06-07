---
status: fixed
phase: 62-long-run-sync-truth-surfaces
generated_at: 2026-06-07T00:08:22Z
review_path: .planning/phases/62-long-run-sync-truth-surfaces/62-REVIEW.md
fix_scope: critical_warning
findings_fixed:
  - WR-01
  - WR-02
commits:
  - 2d358be
  - 3d7e03c
---

# Phase 62 Review Fix Summary

## Fixed Findings

### WR-01: Structured sync logs drift from Phase 62 truth labels

Fixed in `2d358be fix(62): WR-01 align structured sync log labels`.

- Renamed bounded sync structured-log labels from `stop_reason=` and `signal=`
  to `latest_stop_reason=` and `progress_signal=`.
- Updated Phase 62 Rust tests that assert structured-log field names.
- Tightened `scripts/check-phase62-sync-truth-surfaces.ts` so default
  verification checks those labels in `summary.rs` instead of filtering them out.

### WR-02: Live-smoke reports hide unavailable progress and peer facts behind zeroes

Fixed in `3d7e03c fix(62): preserve unavailable live smoke truth`.

- Carried `maybeSyncProgressUnavailableReason` and
  `maybePeerCountsUnavailableReason` through live-smoke snapshots, restart
  summaries, and final status summaries.
- Changed unavailable sync progress and peer-count numeric fields to `null`
  instead of synthesized `0` values.
- Updated Markdown rendering for snapshot rows, final durable status, progress
  evidence, restart evidence, peer health, and bounded counters to render
  `Unavailable: {reason}`.
- Added deterministic fixture coverage that fails if unavailable progress or
  peer-count facts regress to zero substitution.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase62 --all-features` - passed.
- `bash scripts/test-run-live-mainnet-smoke.sh` - passed.
- `bun run scripts/check-phase62-sync-truth-surfaces.ts` - passed.
- `bash -c 'if rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh; then exit 1; fi'` - passed.
- `git diff --check` - passed.
- Normal commit hooks passed for both fix commits, including the repo-native
  `bash scripts/verify.sh` gate.

## Residual Risk

No known review findings remain from the Phase 62 standard review. Public-network
live-smoke remains opt-in UAT evidence and was not run as part of default
verification.
