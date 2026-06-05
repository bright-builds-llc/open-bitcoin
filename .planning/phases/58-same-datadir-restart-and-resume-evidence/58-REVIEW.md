---
phase: 58-same-datadir-restart-and-resume-evidence
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 58-2026-06-05T12-58-05
generated_at: 2026-06-05T13:50:35Z
status: resolved
---

# Phase 58 Code Review

## Findings

1. Restart evidence could pass unchanged downloaded or connected heights even when the corresponding hash changed.
2. A missing post-restart status snapshot could be reported as if no pre-restart progress occurred.
3. Top-level report commands could show preview RPC ports instead of the actual session command specs.

## Resolutions

1. `restartStatus` now requires downloaded and connected hashes to stay stable when heights are unchanged, and the duplicate-connect verdict reports `duplicate_connect_suspected` for same-height hash changes.
2. Restart-mode result classification now distinguishes no pre-restart progress, post-restart runtime failure, missing post-restart snapshots, and post-restart height/hash mismatch.
3. Reports now include actual first-session commands at the existing top-level command fields plus `daemon_sessions` entries and Markdown rows for every daemon session.

## Verification

- `bash scripts/test-run-live-mainnet-smoke.sh` passed with added regression fixtures for same-height hash mismatch and second-session status failure.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node same_datadir --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node restart --all-features` passed.
- `bun run scripts/run-live-mainnet-smoke.ts --help` passed.
- `rg -n "run-live-mainnet-smoke|--restart-after-progress" scripts/verify.sh` returned no matches.
