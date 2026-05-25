---
phase: 44-peer-contribution-attribution
generated_by: gsd-code-review-fix
lifecycle_mode: yolo
phase_lifecycle_id: "44-2026-05-25T16-03-34"
generated_at: "2026-05-25T16:52:03Z"
review_path: .planning/phases/44-peer-contribution-attribution/44-REVIEW.md
status: fixed_with_deferral
fixed:
  critical: 1
  warning: 3
  info: 1
deferred:
  info: 1
---

# Phase 44 Review Fix Report

## Fixed Findings

- **CR-01:** Replaced interpolated shell command detection with positional `sh -c` arguments and added a shell regression that verifies metacharacters in `OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN` are not executed.
- **WR-01:** Counted only `PeerSyncState::Connected` outcomes as connected peers and asserted stalled peers keep `connected_peers` at zero.
- **WR-02:** Converted retry backoff milliseconds to rounded-up Unix seconds for idle-loop timestamps and peer retry scheduling, with regression coverage for expected wait seconds.
- **WR-03:** Skipped live-smoke binary builds only when both daemon and status binaries are overridden.
- **IN-01:** Required full decimal strings for ports and positive integer options, with regressions for trailing junk in `--timeout-seconds` and manual peer ports.

## Deferred Findings

- **IN-02:** The smoke runner file-size refactor remains deferred. This phase added focused report fields and regressions only; splitting the existing runner into modules would be a separate maintainability phase.

## Verification

- `cargo fmt --all --manifest-path packages/Cargo.toml`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features peer_contribution`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features backoff`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- `bun run scripts/run-live-mainnet-smoke.ts --help`
- `git diff --check`
