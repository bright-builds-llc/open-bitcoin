---
phase: 38
phase_name: "Block Download, Connect, and Restart Recovery"
generated_by: gsd-verify-work
lifecycle_mode: yolo
phase_lifecycle_id: "38-2026-05-02T01-10-10"
generated_at: "2026-05-02T04:00:21Z"
status: passed
lifecycle_validated: true
---

# Phase 38 Verification

## Result

Passed. Phase 38 now gives the durable sync runtime a bounded block-download and restart-recovery path: missing best-chain blocks can be requested with explicit per-peer and global caps, already-stored blocks reconnect on restart before re-request, and locally available better branches can take over through the existing undo-backed chainstate reorg path.

## Commands

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed | Final rerun after the `block_reconcile.rs` extraction and peer-test coverage additions. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed | Clean on the final tree. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed | Full workspace build succeeded after the final network-file size cleanup. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed | Full workspace tests and doctests passed on the final tree. |
| `bash scripts/verify.sh` | Passed | Final repo-native pass covered file-size gates, panic-site checks, benchmark smoke, Bazel smoke, and coverage gates. |
| `git diff --check` | Passed | No whitespace or conflict-marker issues remain. |
| `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 38 --require-plans --require-verification --raw` | Passed | Lifecycle provenance is complete for context, plan, summary, and verification artifacts. |

## Evidence

- [`packages/open-bitcoin-node/src/sync/block_reconcile.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/block_reconcile.rs) owns bounded missing-block request planning plus durable block reconnect/reorg reconciliation.
- [`packages/open-bitcoin-node/src/sync.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync.rs) now invokes reconciliation before and during peer sessions, persists block bodies, and reuses the same bounded request path after restart.
- [`packages/open-bitcoin-node/src/network.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network.rs) exposes runtime-facing helpers for best-chain entries, local block-hash tracking, reconnecting stored blocks, and branch reorg application.
- [`packages/open-bitcoin-network/src/peer.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer.rs) now exposes explicit requested-block inspection and runtime-owned missing-block request helpers without widening the generic peer protocol boundary.
- [`packages/open-bitcoin-node/src/sync/tests.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests.rs) proves restart reconnect and locally available better-branch takeover.
- [`packages/open-bitcoin-network/src/peer/tests.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/tests.rs) and [`packages/open-bitcoin-network/src/header_store.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/header_store.rs) cover the new helper behavior directly so repo-native coverage stays green.

## Residual Risks

- The daemon’s operator-facing run surface is still limited by the existing `open-bitcoind` preflight boundary; later phases still own richer runtime controls and truth surfaces.
- The current header-store chain-work heuristic is good enough for deterministic tests and this phase’s restart/reorg mechanics, but not yet proven against full mainnet parity expectations.
