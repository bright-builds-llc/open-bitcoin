---
phase: 38
phase_name: "Block Download, Connect, and Restart Recovery"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "38-2026-05-02T01-10-10"
generated_at: "2026-05-02T01:10:10Z"
---

# Phase 38 Discussion Log

## Mode

This phase was discussed in `--yolo --chain` mode from the wrapper `gsd-yolo-discuss-plan-execute-commit-and-push`. The wrapper auto-selected Phase 38 because Phase 37 artifacts exist and Phase 38 is the first pending phase without discussion artifacts.

## Gray Areas Resolved

| Gray Area | Resolution |
| --- | --- |
| Should Phase 38 create a separate block-sync subsystem? | No. Keep the work inside `DurableSyncRuntime` and `ManagedPeerNetwork` so it extends the shipped Phase 37 runtime rather than forking it. |
| Where should block-request scheduling live? | In the runtime, using small peer/header-store helpers, so restart recovery and fresh-header progress share the same bounded request logic. |
| How should restart recovery treat persisted-but-unconnected blocks? | As durable local inputs that should be reconciled against the durable best header chain on reopen before asking the network for the same blocks again. |
| How should branch takeovers be applied? | Use the existing chainstate undo-backed `reorg` path when the locally available replacement branch matches the durable best header chain and outranks the active chain. |
| What resource bounds matter in this phase? | Explicit per-peer and global block in-flight limits with typed operator guidance when the runtime cannot make safe forward progress. |

## Carry-Forward Constraints

- Phase 37 remains the owner of contextual header validation and deterministic best-header-chain selection.
- Pure-core crates must stay free of direct storage, transport, and runtime side effects.
- Default verification must remain hermetic and must not require public mainnet access.
- Phase 39 owns richer operator-facing progress/control surfaces; Phase 38 should only keep block versus header truthfulness intact.
- Phase 40 owns live mainnet smoke/benchmark commands and milestone closeout docs.

## Required Verification

- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 38 --require-plans --raw`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`
- `git diff --check`
- Final diff review before commit/push.
