---
phase: 37
phase_name: "Header-First Mainnet Sync Integration"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "37-2026-05-02T00-08-13"
generated_at: "2026-05-02T00:08:13.525Z"
---

# Phase 37 Discussion Log

## Mode

This phase was discussed in `--yolo --chain` mode from the wrapper `gsd-yolo-discuss-plan-execute-commit-and-push`. The wrapper auto-selected Phase 37 because Phase 36 is complete and Phase 37 is the first pending phase without discussion artifacts.

## Gray Areas Resolved

| Gray Area | Resolution |
| --- | --- |
| Should Phase 37 continue using the existing peer/runtime seams or add a new sync subsystem? | Extend the existing `DurableSyncRuntime` and `ManagedPeerNetwork` seams so header sync stays on the shipped Phase 36 outbound-peer foundation. |
| Should headers immediately trigger block download? | No. Phase 37 is strictly header-first and continues header batching without requesting blocks yet. |
| How should restart recovery avoid replaying header work? | Reload the durable header snapshot on open and continue from the persisted locator/tip rather than rebuilding from genesis. |
| How should competing branches be handled? | Keep deterministic active-header-chain selection inside the header store and reject invalid contextual headers with typed failures. |
| What operator truthfulness matters now? | Status and sync summaries must keep header height and block height distinct so Phase 37 does not imply chainstate progress it has not earned. |

## Carry-Forward Constraints

- Phase 36 remains the owner of resolver injection, outbound-peer lifecycle, and basic peer telemetry.
- Phase 37 must keep pure-core crates free of direct filesystem, network, and runtime side effects.
- Phase 38 will own block download, block connect, and restart recovery for partial block state.
- Phase 39 will own richer operator-surface presentation of sync phases and health.
- Default verification must remain hermetic and must not require public mainnet access.

## Required Verification

- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 37 --require-plans --raw`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`
- `git diff --check`
- Final diff review before commit/push.
