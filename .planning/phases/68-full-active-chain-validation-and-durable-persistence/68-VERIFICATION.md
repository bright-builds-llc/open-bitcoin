---
phase: 68-full-active-chain-validation-and-durable-persistence
status: passed
generated_by: gsd-verify-work
lifecycle_mode: yolo
phase_lifecycle_id: 68-2026-06-11T11-56-49
generated_at: 2026-06-11T13:41:49Z
lifecycle_validated: true
requirements: [SYNC-01, SYNC-02, SYNC-03, SYNC-04]
---

# Phase 68 Verification

status: passed

## Commands

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node connected_active_chain_progress_survives_runtime_reopen --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_progress --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync:: --all-features` passed with 75 passed and 1 ignored public-network smoke.
- `bun run scripts/check-phase68-active-chain-persistence.ts` passed.
- `bash scripts/verify.sh` passed in 39m 43s.

## Evidence

- `SyncProgress` now exposes `validated_active_chain_height`, `maybe_validated_active_chain_hash`, and `maybe_validated_active_chain_work`.
- `DurableSyncRuntime` status projection derives validated active-chain fields from connected chainstate progress and persists them through runtime metadata.
- `connected_active_chain_progress_survives_runtime_reopen` proves connected active-chain height, hash, chain work, downloaded block body, and chainstate snapshot survive reopening the same store.
- Docs distinguish downloaded-only block bodies from validated active-chain progress credit.
- `scripts/check-phase68-active-chain-persistence.ts` is wired into `bash scripts/verify.sh`.

## Residual Risks

- Public-mainnet live smoke remains opt-in UAT and is not part of default verification.
- Stay-current tip agreement, broader reorg/no-progress recovery, resource-bound proof, and cross-surface observability closeout remain deferred to later v1.6 phases.
