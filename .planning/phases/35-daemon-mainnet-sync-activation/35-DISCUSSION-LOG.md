---
phase: 35
phase_name: "Daemon Mainnet Sync Activation"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "35-2026-05-01T21-26-04"
generated_at: "2026-05-01T21:29:26.254Z"
---

# Phase 35 Discussion Log

## Mode

This phase was discussed in `--yolo --chain` mode from the wrapper `gsd-yolo-discuss-plan-execute-commit-and-push`. The user requested implementation of the new milestone intent, and the selected phase was `35 Daemon Mainnet Sync Activation`.

## Gray Areas Resolved

| Gray Area | Resolution |
| --- | --- |
| Should the daemon start public-network sync now? | No. Phase 35 only wires opt-in config, validation, durable runtime construction, and pre-sync status. Live peer transport starts in later phases. |
| Where should activation live? | Open Bitcoin-only activation lives in `open-bitcoin.jsonc` and daemon CLI flags, never in `bitcoin.conf`. |
| What prevents accidental mainnet networking? | Defaults keep sync disabled. Enabling requires `sync.network_enabled = true`, `sync.mode = "mainnet-ibd"`, and the mainnet chain. |
| What should `open-bitcoind` do when enabled? | Resolve config, validate safety gates, open the selected datadir as a `FjallNodeStore`, construct `DurableSyncRuntime`, and print a concise bootstrap summary before serving RPC. |
| How should docs frame the result? | Current docs must call this a mainnet sync activation/preflight foundation, not an operator-ready unattended full-sync daemon. |

## Carry-Forward Constraints

- Phase 36 owns peer discovery/outbound lifecycle.
- Phase 37 owns headers-first IBD.
- Phase 38 owns block connection and recovery.
- Phase 39 owns observability/control.
- Phase 40 owns live smoke validation and full operator docs closeout.

## Required Verification

- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 35 --require-plans --raw`
- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo build --all-targets --all-features`
- `cargo test --all-features`
- `bash scripts/verify.sh`
- `git diff --check`
- Final diff review before commit/push.
